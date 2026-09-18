//! A minimal 256-bit unsigned integer: enough to parse, compare and print values.
//! No arithmetic beyond what display requires, to keep the audited surface small.

use alloc::string::String;
use alloc::vec::Vec;

use crate::Error;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct U256(pub [u8; 32]);

impl core::fmt::Debug for U256 {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.to_decimal())
    }
}

impl U256 {
    pub const ZERO: U256 = U256([0u8; 32]);
    pub const MAX: U256 = U256([0xff; 32]);

    /// Big-endian bytes of at most 32 bytes, left-padded with zeros.
    pub fn from_be_slice(bytes: &[u8]) -> Result<U256, Error> {
        if bytes.len() > 32 {
            return Err(Error::IntegerOverflow);
        }
        let mut out = [0u8; 32];
        let start = 32usize
            .checked_sub(bytes.len())
            .ok_or(Error::IntegerOverflow)?;
        out.get_mut(start..)
            .ok_or(Error::IntegerOverflow)?
            .copy_from_slice(bytes);
        Ok(U256(out))
    }

    pub fn from_u64(v: u64) -> U256 {
        let mut out = [0u8; 32];
        if let Some(tail) = out.get_mut(24..) {
            tail.copy_from_slice(&v.to_be_bytes());
        }
        U256(out)
    }

    pub fn is_zero(&self) -> bool {
        self.0.iter().all(|b| *b == 0)
    }

    /// Add, refusing to wrap. A total that silently wrapped would understate what
    /// a batch moves, which is the one number a person is most likely to check.
    pub fn checked_add(&self, other: &U256) -> Option<U256> {
        let mut out = [0u8; 32];
        let mut carry = 0u16;
        for i in (0..32).rev() {
            let a = u16::from(*self.0.get(i)?);
            let b = u16::from(*other.0.get(i)?);
            // Each term is at most 255 and the carry at most 1, so this cannot
            // exceed a u16 — but the decoder is linted to never rely on that
            // kind of reasoning being right.
            let sum = a.saturating_add(b).saturating_add(carry);
            *out.get_mut(i)? = (sum & 0xff) as u8;
            carry = sum >> 8;
        }
        if carry != 0 { None } else { Some(U256(out)) }
    }

    /// Whether this is so large that no real token supply could reach it, and
    /// an allowance of this size is unlimited in every practical sense.
    ///
    /// The threshold is 2^192. The largest plausible supply — a trillion trillion
    /// tokens with 18 decimals — is about 2^159, so anything above this is not a
    /// number anyone chose for its value.
    pub fn is_effectively_unlimited(&self) -> bool {
        self.0.iter().take(8).any(|b| *b != 0)
    }

    pub fn is_max(&self) -> bool {
        self.0.iter().all(|b| *b == 0xff)
    }

    /// Parse a decimal string. Rejects empty input, signs, separators and overflow.
    pub fn from_decimal(s: &str) -> Result<U256, Error> {
        if s.is_empty() {
            return Err(Error::IntegerOverflow);
        }
        let mut acc = [0u8; 32];
        for c in s.bytes() {
            let digit = match c {
                b'0'..=b'9' => c.checked_sub(b'0').ok_or(Error::IntegerOverflow)?,
                _ => return Err(Error::IntegerOverflow),
            };
            // acc = acc * 10 + digit, from the least significant byte upwards.
            let mut carry = u16::from(digit);
            for byte in acc.iter_mut().rev() {
                let v = u16::from(*byte)
                    .checked_mul(10)
                    .and_then(|x| x.checked_add(carry))
                    .ok_or(Error::IntegerOverflow)?;
                *byte = (v & 0xff) as u8;
                carry = v >> 8;
            }
            if carry != 0 {
                return Err(Error::IntegerOverflow);
            }
        }
        Ok(U256(acc))
    }

    /// Exact decimal representation.
    pub fn to_decimal(&self) -> String {
        let mut n = self.0;
        let mut digits: Vec<u8> = Vec::new();
        loop {
            if n.iter().all(|b| *b == 0) {
                break;
            }
            let mut rem: u16 = 0;
            for byte in n.iter_mut() {
                // cur <= 9 * 256 + 255 = 2559, always fits in u16.
                let cur = rem.saturating_mul(256).saturating_add(u16::from(*byte));
                *byte = (cur / 10) as u8;
                rem = cur % 10;
            }
            digits.push(b'0'.saturating_add(rem as u8));
        }
        if digits.is_empty() {
            return String::from("0");
        }
        digits.reverse();
        String::from_utf8(digits).unwrap_or_default()
    }

    /// Decimal with thin grouping for readability, e.g. `1_000_000`.
    pub fn to_grouped_decimal(&self) -> String {
        let plain = self.to_decimal();
        let len = plain.len();
        let mut out = String::with_capacity(len.saturating_add(len / 3));
        for (i, c) in plain.chars().enumerate() {
            let remaining = len.saturating_sub(i);
            if i != 0 && remaining % 3 == 0 {
                out.push('_');
            }
            out.push(c);
        }
        out
    }

    pub fn to_u64(&self) -> Option<u64> {
        if self.0.get(..24)?.iter().any(|b| *b != 0) {
            return None;
        }
        let mut buf = [0u8; 8];
        buf.copy_from_slice(self.0.get(24..)?);
        Some(u64::from_be_bytes(buf))
    }
}
