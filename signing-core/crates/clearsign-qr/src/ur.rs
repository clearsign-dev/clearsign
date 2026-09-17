//! Parsing `ur:` strings and reassembling multipart messages.
//!
//! Everything arriving here was produced by a computer the signer does not
//! trust, so every field is bounded and every mismatch is a refusal.

use alloc::string::String;
use alloc::vec;
use alloc::vec::Vec;

use crate::Error;
use crate::bytewords;
use crate::cbor;
use crate::crc32::crc32;
use crate::fountain::choose_fragments;

/// Largest message the decoder will reassemble. A signing request is a few
/// hundred bytes; anything far larger is not something a person can review.
pub const MAX_MESSAGE_LEN: usize = 64 * 1024;

/// Largest number of fragments a message may be split into.
pub const MAX_SEQ_LEN: usize = 1024;

/// Largest number of parts the decoder will accept before giving up, so a
/// stream of junk QR codes cannot hold the device forever.
pub const MAX_PARTS_PROCESSED: usize = 8192;

/// One scanned QR code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub ur_type: String,
    /// `None` for a single-part UR.
    pub header: Option<PartHeader>,
    /// The message itself for a single-part UR, or this part's fragment.
    pub payload: Vec<u8>,
}

/// The fountain metadata carried by every part of a multipart UR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PartHeader {
    pub seq_num: u32,
    pub seq_len: usize,
    pub message_len: usize,
    pub checksum: u32,
}

/// Parse one `ur:type/…` or `ur:type/seq/…` string.
pub fn parse(s: &str) -> Result<Part, Error> {
    let body = s
        .strip_prefix("ur:")
        .or_else(|| s.strip_prefix("UR:"))
        .ok_or(Error::Ur("not a ur: string"))?;
    let mut fields = body.split('/');
    let ur_type = fields.next().ok_or(Error::Ur("missing type"))?;
    if ur_type.is_empty()
        || !ur_type
            .bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'-')
    {
        return Err(Error::Ur("type is not a lowercase UR type"));
    }
    let second = fields.next().ok_or(Error::Ur("missing payload"))?;
    let third = fields.next();
    if fields.next().is_some() {
        return Err(Error::Ur("too many path components"));
    }

    match third {
        // ur:type/message
        None => Ok(Part {
            ur_type: String::from(ur_type),
            header: None,
            payload: bytewords::decode(second)?,
        }),
        // ur:type/seqNum-seqLen/fragment
        Some(fragment) => {
            let (num, len) = second
                .split_once('-')
                .ok_or(Error::Ur("sequence is not seqNum-seqLen"))?;
            let seq_num: u32 = num.parse().map_err(|_| Error::Ur("bad sequence number"))?;
            let seq_len: usize = len.parse().map_err(|_| Error::Ur("bad sequence length"))?;
            if seq_num == 0 || seq_len == 0 || seq_len > MAX_SEQ_LEN {
                return Err(Error::Ur("sequence out of range"));
            }
            let cbor_bytes = bytewords::decode(fragment)?;
            let (header, data) = decode_part_cbor(&cbor_bytes)?;
            if header.seq_num != seq_num || header.seq_len != seq_len {
                return Err(Error::Ur("sequence in the path disagrees with the payload"));
            }
            // The fragments must be able to hold the message, and must not be so
            // large that the last fragment would be entirely padding.
            let capacity = data
                .len()
                .checked_mul(header.seq_len)
                .ok_or(Error::Ur("fragment size overflows"))?;
            if capacity < header.message_len
                || capacity.saturating_sub(data.len()) >= header.message_len
            {
                return Err(Error::Ur("fragment size disagrees with message length"));
            }
            Ok(Part {
                ur_type: String::from(ur_type),
                header: Some(header),
                payload: data,
            })
        }
    }
}

/// `part = [uint32 seqNum, uint seqLen, uint messageLen, uint32 checksum, bytes data]`
///
/// Structural decoding only. Whether the part is consistent with the message it
/// claims to belong to is decided by [`parse`] and [`Decoder::receive`].
pub fn decode_part_cbor(bytes: &[u8]) -> Result<(PartHeader, Vec<u8>), Error> {
    let mut r = cbor::Reader::new(bytes);
    if r.array()? != 5 {
        return Err(Error::Ur("part is not a five-element array"));
    }
    let seq_num = u32::try_from(r.uint()?).map_err(|_| Error::Ur("sequence number too large"))?;
    let seq_len = usize::try_from(r.uint()?).map_err(|_| Error::Ur("sequence length too large"))?;
    let message_len =
        usize::try_from(r.uint()?).map_err(|_| Error::Ur("message length too large"))?;
    let checksum = u32::try_from(r.uint()?).map_err(|_| Error::Ur("checksum too large"))?;
    let data = r.bytes()?.to_vec();
    r.expect_end()?;

    if seq_len == 0 || seq_len > MAX_SEQ_LEN {
        return Err(Error::Ur("sequence length out of range"));
    }
    if message_len == 0 || message_len > MAX_MESSAGE_LEN {
        return Err(Error::Ur("message length out of range"));
    }
    if data.is_empty() {
        return Err(Error::Ur("empty fragment"));
    }
    Ok((
        PartHeader {
            seq_num,
            seq_len,
            message_len,
            checksum,
        },
        data,
    ))
}

/// Reassembles a message from scanned parts.
///
/// A single-part UR completes on the first part. A multipart UR completes when
/// enough parts have arrived, whatever order they came in, and only if the
/// reassembled message matches the checksum every part agreed on.
pub struct Decoder {
    ur_type: Option<String>,
    expected: Option<PartHeader>,
    /// Fragments recovered so far, indexed by fragment number.
    fragments: Vec<Option<Vec<u8>>>,
    /// Parts still mixing more than one unknown fragment.
    mixed: Vec<(Vec<usize>, Vec<u8>)>,
    processed: usize,
    result: Option<Vec<u8>>,
}

impl Default for Decoder {
    fn default() -> Self {
        Self::new()
    }
}

impl Decoder {
    pub fn new() -> Self {
        Decoder {
            ur_type: None,
            expected: None,
            fragments: Vec::new(),
            mixed: Vec::new(),
            processed: 0,
            result: None,
        }
    }

    /// The UR type seen so far, once any part has been accepted.
    pub fn ur_type(&self) -> Option<&str> {
        self.ur_type.as_deref()
    }

    /// How many of the message's fragments are known, and how many there are.
    pub fn progress(&self) -> (usize, usize) {
        (
            self.fragments.iter().filter(|f| f.is_some()).count(),
            self.fragments.len(),
        )
    }

    /// The finished message, if it is complete.
    pub fn message(&self) -> Option<&[u8]> {
        self.result.as_deref()
    }

    /// Feed one scanned `ur:` string. Returns the message once complete.
    ///
    /// A part that does not belong to the message already being received is an
    /// error, not a silent reset: mixing two messages is how a scanner gets
    /// talked into assembling something nobody displayed.
    pub fn receive(&mut self, s: &str) -> Result<Option<&[u8]>, Error> {
        let part = parse(s)?;
        if self.result.is_some() {
            return Ok(self.result.as_deref());
        }
        self.processed = self.processed.saturating_add(1);
        if self.processed > MAX_PARTS_PROCESSED {
            return Err(Error::Ur("too many parts received without completing"));
        }
        match &self.ur_type {
            None => self.ur_type = Some(part.ur_type.clone()),
            Some(t) if *t == part.ur_type => {}
            Some(_) => return Err(Error::Ur("part belongs to a different UR type")),
        }

        let header = match part.header {
            None => {
                if self.expected.is_some() {
                    return Err(Error::Ur("single-part UR mixed with a multipart UR"));
                }
                if part.payload.len() > MAX_MESSAGE_LEN {
                    return Err(Error::Ur("message is larger than the limit"));
                }
                self.result = Some(part.payload);
                return Ok(self.result.as_deref());
            }
            Some(h) => h,
        };

        match self.expected {
            None => {
                self.expected = Some(header);
                self.fragments = vec![None; header.seq_len];
            }
            Some(first) => {
                if first.seq_len != header.seq_len
                    || first.message_len != header.message_len
                    || first.checksum != header.checksum
                {
                    return Err(Error::Ur("part belongs to a different message"));
                }
            }
        }
        if let Some(first_fragment) = self.fragments.iter().flatten().next() {
            if first_fragment.len() != part.payload.len() {
                return Err(Error::Ur("fragment length changed mid-message"));
            }
        }

        let indexes = choose_fragments(header.seq_num, header.seq_len, header.checksum);
        if indexes.is_empty() || indexes.iter().any(|i| *i >= header.seq_len) {
            return Err(Error::Ur("part names a fragment outside the message"));
        }
        self.absorb(indexes, part.payload);
        self.try_finish(header)
    }

    /// Reduce a new part against what is known, then use it to reduce the rest.
    fn absorb(&mut self, indexes: Vec<usize>, data: Vec<u8>) {
        let mut queue = vec![(indexes, data)];
        let mut guard = 0usize;
        while let Some((indexes, data)) = queue.pop() {
            guard = guard.saturating_add(1);
            if guard > MAX_SEQ_LEN.saturating_mul(4) {
                return;
            }
            let (indexes, data) = reduce_by_known(indexes, data, &self.fragments);
            match indexes.as_slice() {
                [] => {}
                [only] => {
                    let index = *only;
                    if self.fragments.get(index).map(Option::is_some) == Some(false) {
                        if let Some(slot) = self.fragments.get_mut(index) {
                            *slot = Some(data);
                        }
                        // A newly known fragment may unlock mixed parts.
                        let still_mixed = core::mem::take(&mut self.mixed);
                        for (mixed_indexes, mixed_data) in still_mixed {
                            if mixed_indexes.contains(&index) {
                                queue.push((mixed_indexes, mixed_data));
                            } else {
                                self.mixed.push((mixed_indexes, mixed_data));
                            }
                        }
                    }
                }
                _ => {
                    if !self.mixed.iter().any(|(i, _)| *i == indexes) {
                        self.mixed.push((indexes, data));
                    }
                }
            }
        }
    }

    fn try_finish(&mut self, header: PartHeader) -> Result<Option<&[u8]>, Error> {
        if self.fragments.iter().any(Option::is_none) {
            return Ok(None);
        }
        let mut message = Vec::with_capacity(header.message_len);
        for fragment in self.fragments.iter().flatten() {
            message.extend_from_slice(fragment);
        }
        if message.len() < header.message_len {
            return Err(Error::Ur("reassembled message is shorter than declared"));
        }
        message.truncate(header.message_len);
        if crc32(&message) != header.checksum {
            // Every part agreed on this checksum, so a mismatch means the
            // fragments were not the ones the sender encoded.
            return Err(Error::Ur("reassembled message fails its checksum"));
        }
        self.result = Some(message);
        Ok(self.result.as_deref())
    }
}

/// XOR out every fragment of `data` that is already known.
fn reduce_by_known(
    indexes: Vec<usize>,
    mut data: Vec<u8>,
    known: &[Option<Vec<u8>>],
) -> (Vec<usize>, Vec<u8>) {
    let mut remaining = Vec::with_capacity(indexes.len());
    for index in indexes {
        match known.get(index).and_then(Option::as_ref) {
            Some(fragment) if fragment.len() == data.len() => {
                for (a, b) in data.iter_mut().zip(fragment.iter()) {
                    *a ^= *b;
                }
            }
            _ => remaining.push(index),
        }
    }
    (remaining, data)
}

/// Encode a complete message as a single-part `ur:<type>/<bytewords>`.
pub fn encode_single(ur_type: &str, message: &[u8]) -> String {
    let mut out = String::from("ur:");
    out.push_str(ur_type);
    out.push('/');
    out.push_str(&bytewords::encode(message));
    out
}
