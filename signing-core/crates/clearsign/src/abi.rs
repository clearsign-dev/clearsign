//! Strict Solidity ABI argument reading for calldata.
//!
//! INV-4: only the canonical layout produced by standard encoders is accepted.
//! Dirty high bits in addresses and small integers are rejected (the Solidity
//! ABI v2 decoder reverts on them too). Dynamic `bytes` must sit at the standard
//! offset, have zero padding, and nothing may follow the last argument. When a
//! known function's arguments are not canonical, the caller refuses to
//! interpret them rather than guessing.

use crate::address::Address;
use crate::{Error, U256};

pub const WORD: usize = 32;

/// Arguments that follow the 4-byte selector.
#[derive(Clone, Copy)]
pub struct Args<'a> {
    data: &'a [u8],
}

impl<'a> Args<'a> {
    pub fn new(data: &'a [u8]) -> Self {
        Args { data }
    }

    pub fn len(&self) -> usize {
        self.data.len()
    }

    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }

    pub fn word(&self, index: usize) -> Result<&'a [u8], Error> {
        let start = index.checked_mul(WORD).ok_or(Error::Truncated)?;
        let end = start.checked_add(WORD).ok_or(Error::Truncated)?;
        self.data.get(start..end).ok_or(Error::Truncated)
    }

    pub fn uint256(&self, index: usize) -> Result<U256, Error> {
        U256::from_be_slice(self.word(index)?)
    }

    pub fn uint8(&self, index: usize) -> Result<u8, Error> {
        let w = self.word(index)?;
        let (high, low) = w.split_at(31);
        if high.iter().any(|b| *b != 0) {
            return Err(Error::NonCanonical("uint8 with non-zero high bits"));
        }
        low.first().copied().ok_or(Error::Truncated)
    }

    pub fn address(&self, index: usize) -> Result<Address, Error> {
        let w = self.word(index)?;
        let (high, low) = w.split_at(12);
        if high.iter().any(|b| *b != 0) {
            return Err(Error::InvalidAddress);
        }
        let mut a = [0u8; 20];
        a.copy_from_slice(low);
        Ok(a)
    }

    /// Require that the arguments are exactly `words` static words long.
    pub fn expect_static_len(&self, words: usize) -> Result<(), Error> {
        let expected = words.checked_mul(WORD).ok_or(Error::IntegerOverflow)?;
        match self.data.len() {
            n if n == expected => Ok(()),
            n if n < expected => Err(Error::Truncated),
            _ => Err(Error::TrailingBytes),
        }
    }

    /// Read a dynamic `bytes` argument whose offset word is at `head_index`,
    /// requiring it to start at `expected_offset`. Returns the payload and the
    /// canonical offset at which the next dynamic argument must begin.
    pub fn bytes_at(
        &self,
        head_index: usize,
        expected_offset: usize,
    ) -> Result<(&'a [u8], usize), Error> {
        let offset = self
            .uint256(head_index)?
            .to_u64()
            .and_then(|v| usize::try_from(v).ok())
            .ok_or(Error::IntegerOverflow)?;
        if offset != expected_offset {
            return Err(Error::NonCanonical(
                "dynamic argument not at the standard offset",
            ));
        }
        let len = self
            .data
            .get(offset..)
            .and_then(|tail| tail.get(..WORD))
            .ok_or(Error::Truncated)
            .and_then(U256::from_be_slice)?
            .to_u64()
            .and_then(|v| usize::try_from(v).ok())
            .ok_or(Error::IntegerOverflow)?;
        let start = offset.checked_add(WORD).ok_or(Error::IntegerOverflow)?;
        let end = start.checked_add(len).ok_or(Error::IntegerOverflow)?;
        let payload = self.data.get(start..end).ok_or(Error::Truncated)?;
        let padded = padded_len(len)?;
        let pad_end = start.checked_add(padded).ok_or(Error::IntegerOverflow)?;
        let padding = self.data.get(end..pad_end).ok_or(Error::Truncated)?;
        if padding.iter().any(|b| *b != 0) {
            return Err(Error::NonCanonical("non-zero padding after bytes argument"));
        }
        Ok((payload, pad_end))
    }

    /// Require that nothing follows `end`.
    pub fn expect_end(&self, end: usize) -> Result<(), Error> {
        match self.data.len() {
            n if n == end => Ok(()),
            n if n < end => Err(Error::Truncated),
            _ => Err(Error::TrailingBytes),
        }
    }
}

/// Length rounded up to a multiple of 32.
pub fn padded_len(len: usize) -> Result<usize, Error> {
    let rem = len % WORD;
    if rem == 0 {
        Ok(len)
    } else {
        len.checked_add(WORD.saturating_sub(rem))
            .ok_or(Error::IntegerOverflow)
    }
}

/// Encode a value as a single 32-byte word (for hashing).
pub fn word_from_address(a: &Address) -> [u8; 32] {
    let mut w = [0u8; 32];
    if let Some(tail) = w.get_mut(12..) {
        tail.copy_from_slice(a);
    }
    w
}

pub fn word_from_u8(v: u8) -> [u8; 32] {
    let mut w = [0u8; 32];
    if let Some(last) = w.last_mut() {
        *last = v;
    }
    w
}
