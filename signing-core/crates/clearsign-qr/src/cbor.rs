//! A deliberately small, strict CBOR reader and writer (RFC 8949).
//!
//! UR requires deterministic CBOR, so this reader refuses everything that is
//! merely *allowed*: indefinite lengths, non-shortest integer encodings, and
//! trailing bytes. A transport that accepts two encodings of the same value is
//! a transport where the display and the signature can disagree.

use alloc::vec::Vec;

use crate::Error;

const MAJOR_UINT: u8 = 0;
const MAJOR_BYTES: u8 = 2;
const MAJOR_TEXT: u8 = 3;
const MAJOR_ARRAY: u8 = 4;
const MAJOR_MAP: u8 = 5;
const MAJOR_TAG: u8 = 6;

/// A cursor over CBOR bytes.
pub struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

/// What a CBOR item is, at the level this crate needs.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Uint(u64),
    Bytes,
    Text,
    Array(u64),
    Map(u64),
    Tag(u64),
}

impl<'a> Reader<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Reader { data, pos: 0 }
    }

    pub fn is_empty(&self) -> bool {
        self.pos >= self.data.len()
    }

    /// Require that every byte has been consumed.
    pub fn expect_end(&self) -> Result<(), Error> {
        if self.pos == self.data.len() {
            Ok(())
        } else {
            Err(Error::Cbor("trailing bytes"))
        }
    }

    fn take(&mut self, n: usize) -> Result<&'a [u8], Error> {
        let end = self.pos.checked_add(n).ok_or(Error::Cbor("length overflow"))?;
        let out = self.data.get(self.pos..end).ok_or(Error::Cbor("truncated"))?;
        self.pos = end;
        Ok(out)
    }

    /// Read one head: major type plus its argument, rejecting non-shortest forms.
    fn head(&mut self) -> Result<(u8, u64), Error> {
        let initial = *self.take(1)?.first().ok_or(Error::Cbor("truncated"))?;
        let major = initial >> 5;
        let info = initial & 0x1f;
        let value = match info {
            0..=23 => u64::from(info),
            24 => {
                let v = u64::from(*self.take(1)?.first().ok_or(Error::Cbor("truncated"))?);
                if v < 24 {
                    return Err(Error::Cbor("integer not in shortest form"));
                }
                v
            }
            25 => {
                let b: [u8; 2] = self.take(2)?.try_into().map_err(|_| Error::Cbor("truncated"))?;
                let v = u64::from(u16::from_be_bytes(b));
                if v <= u64::from(u8::MAX) {
                    return Err(Error::Cbor("integer not in shortest form"));
                }
                v
            }
            26 => {
                let b: [u8; 4] = self.take(4)?.try_into().map_err(|_| Error::Cbor("truncated"))?;
                let v = u64::from(u32::from_be_bytes(b));
                if v <= u64::from(u16::MAX) {
                    return Err(Error::Cbor("integer not in shortest form"));
                }
                v
            }
            27 => {
                let b: [u8; 8] = self.take(8)?.try_into().map_err(|_| Error::Cbor("truncated"))?;
                let v = u64::from_be_bytes(b);
                if v <= u64::from(u32::MAX) {
                    return Err(Error::Cbor("integer not in shortest form"));
                }
                v
            }
            31 => return Err(Error::Cbor("indefinite length is not deterministic CBOR")),
            _ => return Err(Error::Cbor("reserved additional information")),
        };
        Ok((major, value))
    }

    /// Peek at the next item's kind without consuming its payload.
    pub fn kind(&mut self) -> Result<Kind, Error> {
        let start = self.pos;
        let (major, value) = self.head()?;
        let kind = match major {
            MAJOR_UINT => Kind::Uint(value),
            MAJOR_BYTES => Kind::Bytes,
            MAJOR_TEXT => Kind::Text,
            MAJOR_ARRAY => Kind::Array(value),
            MAJOR_MAP => Kind::Map(value),
            MAJOR_TAG => Kind::Tag(value),
            _ => return Err(Error::Cbor("unsupported major type")),
        };
        self.pos = start;
        Ok(kind)
    }

    pub fn uint(&mut self) -> Result<u64, Error> {
        match self.head()? {
            (MAJOR_UINT, v) => Ok(v),
            _ => Err(Error::Cbor("expected an unsigned integer")),
        }
    }

    pub fn bytes(&mut self) -> Result<&'a [u8], Error> {
        match self.head()? {
            (MAJOR_BYTES, len) => {
                let n = usize::try_from(len).map_err(|_| Error::Cbor("length overflow"))?;
                self.take(n)
            }
            _ => Err(Error::Cbor("expected a byte string")),
        }
    }

    pub fn text(&mut self) -> Result<&'a str, Error> {
        match self.head()? {
            (MAJOR_TEXT, len) => {
                let n = usize::try_from(len).map_err(|_| Error::Cbor("length overflow"))?;
                core::str::from_utf8(self.take(n)?).map_err(|_| Error::Cbor("text is not UTF-8"))
            }
            _ => Err(Error::Cbor("expected a text string")),
        }
    }

    /// Enter an array, returning its length.
    pub fn array(&mut self) -> Result<u64, Error> {
        match self.head()? {
            (MAJOR_ARRAY, len) => Ok(len),
            _ => Err(Error::Cbor("expected an array")),
        }
    }

    /// Enter a map, returning its number of pairs.
    pub fn map(&mut self) -> Result<u64, Error> {
        match self.head()? {
            (MAJOR_MAP, len) => Ok(len),
            _ => Err(Error::Cbor("expected a map")),
        }
    }

    /// Consume a tag, requiring it to be `expected`.
    pub fn tag(&mut self, expected: u64) -> Result<(), Error> {
        match self.head()? {
            (MAJOR_TAG, v) if v == expected => Ok(()),
            (MAJOR_TAG, _) => Err(Error::Cbor("unexpected CBOR tag")),
            _ => Err(Error::Cbor("expected a CBOR tag")),
        }
    }

    /// Read a CBOR boolean (simple values 0xf4 and 0xf5).
    pub fn boolean(&mut self) -> Result<bool, Error> {
        match *self.take(1)?.first().ok_or(Error::Cbor("truncated"))? {
            0xf4 => Ok(false),
            0xf5 => Ok(true),
            _ => Err(Error::Cbor("expected a boolean")),
        }
    }

    /// Skip one complete item, whatever it is. Bounded by [`MAX_SKIP_DEPTH`].
    pub fn skip(&mut self) -> Result<(), Error> {
        self.skip_at(0)
    }

    fn skip_at(&mut self, depth: usize) -> Result<(), Error> {
        if depth > MAX_SKIP_DEPTH {
            return Err(Error::Cbor("nesting is deeper than allowed"));
        }
        let next = depth.saturating_add(1);
        match self.head()? {
            (MAJOR_UINT, _) => Ok(()),
            (MAJOR_BYTES | MAJOR_TEXT, len) => {
                let n = usize::try_from(len).map_err(|_| Error::Cbor("length overflow"))?;
                self.take(n).map(|_| ())
            }
            (MAJOR_ARRAY, len) => {
                for _ in 0..len {
                    self.skip_at(next)?;
                }
                Ok(())
            }
            (MAJOR_MAP, len) => {
                for _ in 0..len {
                    self.skip_at(next)?;
                    self.skip_at(next)?;
                }
                Ok(())
            }
            (MAJOR_TAG, _) => self.skip_at(next),
            // Major type 7 covers simple values; only the booleans and null are
            // accepted, and only so that an unknown field can be skipped.
            (7, 20..=22) => Ok(()),
            _ => Err(Error::Cbor("unsupported major type")),
        }
    }
}

/// Deepest CBOR nesting the reader will walk past.
pub const MAX_SKIP_DEPTH: usize = 8;

/// Append a head in shortest form.
fn write_head(out: &mut Vec<u8>, major: u8, value: u64) {
    let major = major << 5;
    match value {
        0..=23 => out.push(major | (value as u8)),
        24..=0xff => {
            out.push(major | 24);
            out.push(value as u8);
        }
        0x100..=0xffff => {
            out.push(major | 25);
            out.extend_from_slice(&(value as u16).to_be_bytes());
        }
        0x1_0000..=0xffff_ffff => {
            out.push(major | 26);
            out.extend_from_slice(&(value as u32).to_be_bytes());
        }
        _ => {
            out.push(major | 27);
            out.extend_from_slice(&value.to_be_bytes());
        }
    }
}

pub fn write_uint(out: &mut Vec<u8>, value: u64) {
    write_head(out, MAJOR_UINT, value);
}

pub fn write_bytes(out: &mut Vec<u8>, value: &[u8]) {
    write_head(out, MAJOR_BYTES, value.len() as u64);
    out.extend_from_slice(value);
}

pub fn write_text(out: &mut Vec<u8>, value: &str) {
    write_head(out, MAJOR_TEXT, value.len() as u64);
    out.extend_from_slice(value.as_bytes());
}

pub fn write_array(out: &mut Vec<u8>, len: u64) {
    write_head(out, MAJOR_ARRAY, len);
}

pub fn write_map(out: &mut Vec<u8>, len: u64) {
    write_head(out, MAJOR_MAP, len);
}

pub fn write_tag(out: &mut Vec<u8>, tag: u64) {
    write_head(out, MAJOR_TAG, tag);
}
