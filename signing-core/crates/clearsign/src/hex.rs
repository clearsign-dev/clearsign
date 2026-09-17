//! Hexadecimal encoding and strict decoding.

use alloc::string::String;
use alloc::vec::Vec;

use crate::Error;

const DIGITS: &[u8; 16] = b"0123456789abcdef";

/// Lower-case hex with a `0x` prefix.
pub fn encode_prefixed(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().saturating_mul(2).saturating_add(2));
    out.push_str("0x");
    push_hex(&mut out, bytes);
    out
}

/// Lower-case hex without a prefix.
pub fn encode(bytes: &[u8]) -> String {
    let mut out = String::with_capacity(bytes.len().saturating_mul(2));
    push_hex(&mut out, bytes);
    out
}

fn push_hex(out: &mut String, bytes: &[u8]) {
    for &b in bytes {
        out.push(char::from(
            DIGITS.get(usize::from(b >> 4)).copied().unwrap_or(b'?'),
        ));
        out.push(char::from(
            DIGITS.get(usize::from(b & 0x0f)).copied().unwrap_or(b'?'),
        ));
    }
}

/// Decode hex, accepting an optional `0x` prefix and surrounding whitespace.
/// Rejects odd lengths and any non-hex character.
pub fn decode(input: &str) -> Result<Vec<u8>, Error> {
    let s = input.trim();
    let s = s
        .strip_prefix("0x")
        .or_else(|| s.strip_prefix("0X"))
        .unwrap_or(s);
    if s.len() % 2 != 0 {
        return Err(Error::InvalidHex);
    }
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len() / 2);
    for pair in bytes.chunks_exact(2) {
        let (hi, lo) = match pair {
            [h, l] => (nibble(*h)?, nibble(*l)?),
            _ => return Err(Error::InvalidHex),
        };
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn nibble(c: u8) -> Result<u8, Error> {
    match c {
        b'0'..=b'9' => c.checked_sub(b'0').ok_or(Error::InvalidHex),
        b'a'..=b'f' => c
            .checked_sub(b'a')
            .and_then(|v| v.checked_add(10))
            .ok_or(Error::InvalidHex),
        b'A'..=b'F' => c
            .checked_sub(b'A')
            .and_then(|v| v.checked_add(10))
            .ok_or(Error::InvalidHex),
        _ => Err(Error::InvalidHex),
    }
}
