//! Strict RLP decoding.
//!
//! INV-4: every non-canonical form is rejected, so two parsers can never
//! disagree about what a transaction contains. Rules enforced:
//! - a single byte below 0x80 must be encoded as itself, not as a 1-byte string
//! - short forms must be used for payloads of 55 bytes or fewer
//! - length prefixes may not have leading zero bytes
//! - lengths must fit within the input, and nothing may follow the top-level item
//! - nesting is capped at [`MAX_DEPTH`]

use alloc::vec::Vec;

use crate::{Error, U256};

pub const MAX_DEPTH: usize = 16;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item<'a> {
    Bytes(&'a [u8]),
    List(Vec<Item<'a>>),
}

impl<'a> Item<'a> {
    pub fn as_bytes(&self) -> Result<&'a [u8], Error> {
        match self {
            Item::Bytes(b) => Ok(b),
            Item::List(_) => Err(Error::WrongItemKind),
        }
    }

    pub fn as_list(&self) -> Result<&[Item<'a>], Error> {
        match self {
            Item::List(l) => Ok(l),
            Item::Bytes(_) => Err(Error::WrongItemKind),
        }
    }

    /// A canonical RLP integer: no leading zero bytes, zero encoded as empty.
    pub fn as_uint(&self) -> Result<U256, Error> {
        let b = self.as_bytes()?;
        if b.first() == Some(&0) {
            return Err(Error::NonCanonical("integer with leading zero"));
        }
        U256::from_be_slice(b)
    }

    /// Either empty (contract creation) or exactly 20 bytes.
    pub fn as_optional_address(&self) -> Result<Option<[u8; 20]>, Error> {
        let b = self.as_bytes()?;
        match b.len() {
            0 => Ok(None),
            20 => {
                let mut a = [0u8; 20];
                a.copy_from_slice(b);
                Ok(Some(a))
            }
            _ => Err(Error::InvalidAddress),
        }
    }
}

/// Decode exactly one item that spans the entire input.
pub fn decode(input: &[u8]) -> Result<Item<'_>, Error> {
    let (item, rest) = decode_item(input, 0)?;
    if !rest.is_empty() {
        return Err(Error::TrailingBytes);
    }
    Ok(item)
}

fn decode_item(input: &[u8], depth: usize) -> Result<(Item<'_>, &[u8]), Error> {
    if depth > MAX_DEPTH {
        return Err(Error::TooDeep);
    }
    let (&prefix, after) = input.split_first().ok_or(Error::Truncated)?;
    match prefix {
        0x00..=0x7f => {
            let one = input.get(..1).ok_or(Error::Truncated)?;
            Ok((Item::Bytes(one), after))
        }
        0x80..=0xb7 => {
            let len = usize::from(prefix.checked_sub(0x80).ok_or(Error::Truncated)?);
            let (payload, rest) = take(after, len)?;
            if len == 1 && payload.first().is_some_and(|b| *b < 0x80) {
                return Err(Error::NonCanonical(
                    "single byte below 0x80 wrapped in a string",
                ));
            }
            Ok((Item::Bytes(payload), rest))
        }
        0xb8..=0xbf => {
            let len_of_len = usize::from(prefix.checked_sub(0xb7).ok_or(Error::Truncated)?);
            let (len, after_len) = read_long_length(after, len_of_len)?;
            let (payload, rest) = take(after_len, len)?;
            Ok((Item::Bytes(payload), rest))
        }
        0xc0..=0xf7 => {
            let len = usize::from(prefix.checked_sub(0xc0).ok_or(Error::Truncated)?);
            let (payload, rest) = take(after, len)?;
            Ok((Item::List(decode_list_payload(payload, depth)?), rest))
        }
        0xf8..=0xff => {
            let len_of_len = usize::from(prefix.checked_sub(0xf7).ok_or(Error::Truncated)?);
            let (len, after_len) = read_long_length(after, len_of_len)?;
            let (payload, rest) = take(after_len, len)?;
            Ok((Item::List(decode_list_payload(payload, depth)?), rest))
        }
    }
}

fn decode_list_payload(mut payload: &[u8], depth: usize) -> Result<Vec<Item<'_>>, Error> {
    let mut items = Vec::new();
    let child_depth = depth.checked_add(1).ok_or(Error::TooDeep)?;
    while !payload.is_empty() {
        let (item, rest) = decode_item(payload, child_depth)?;
        items.push(item);
        payload = rest;
    }
    Ok(items)
}

fn read_long_length(input: &[u8], len_of_len: usize) -> Result<(usize, &[u8]), Error> {
    let (len_bytes, rest) = take(input, len_of_len)?;
    if len_bytes.first() == Some(&0) {
        return Err(Error::NonCanonical("length prefix with leading zero"));
    }
    let mut len: usize = 0;
    for b in len_bytes {
        len = len
            .checked_mul(256)
            .and_then(|v| v.checked_add(usize::from(*b)))
            .ok_or(Error::IntegerOverflow)?;
    }
    if len <= 55 {
        return Err(Error::NonCanonical(
            "long form used for a payload of 55 bytes or fewer",
        ));
    }
    Ok((len, rest))
}

fn take(input: &[u8], len: usize) -> Result<(&[u8], &[u8]), Error> {
    if input.len() < len {
        return Err(Error::Truncated);
    }
    Ok(input.split_at(len))
}
