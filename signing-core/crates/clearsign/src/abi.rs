//! Strict Solidity ABI argument reading for calldata.
//!
//! INV-4: only the canonical layout produced by standard encoders is accepted.
//! Dirty high bits in addresses and small integers are rejected (the Solidity
//! ABI v2 decoder reverts on them too). Dynamic `bytes` must sit at the standard
//! offset, have zero padding, and nothing may follow the last argument. When a
//! known function's arguments are not canonical, the caller refuses to
//! interpret them rather than guessing.

use alloc::vec::Vec;

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

    /// A `bool`: the word must be exactly 0 or 1, as Solidity's decoder requires.
    pub fn boolean(&self, index: usize) -> Result<bool, Error> {
        match self.uint8(index)? {
            0 => Ok(false),
            1 => Ok(true),
            _ => Err(Error::NonCanonical("bool that is neither 0 nor 1")),
        }
    }

    /// An unsigned integer narrower than 256 bits, such as Permit2's `uint160`
    /// amounts and `uint48` expiries. Bits above the declared width must be
    /// zero, as Solidity's decoder requires.
    pub fn uint_bits(&self, index: usize, bits: usize) -> Result<U256, Error> {
        let w = self.word(index)?;
        if bits == 0 || bits > 256 || bits % 8 != 0 {
            return Err(Error::IntegerOverflow);
        }
        let zero_bytes = 32usize.saturating_sub(bits / 8);
        let (high, _) = w.split_at(zero_bytes);
        if high.iter().any(|b| *b != 0) {
            return Err(Error::NonCanonical("integer wider than its declared type"));
        }
        U256::from_be_slice(w)
    }

    /// Read a dynamic `bytes[]` argument whose offset word is at `head_index`,
    /// requiring the array at `expected_offset` and every element exactly where
    /// a standard encoder puts it: element offsets in order, each element
    /// directly after the previous one, zero padding, nothing in between.
    /// Returns the elements and the offset just past the array.
    ///
    /// More than `max_elements` is refused with [`Error::TooDeep`] before any
    /// element is read, so a declared length cannot drive an allocation.
    pub fn bytes_array_at(
        &self,
        head_index: usize,
        expected_offset: usize,
        max_elements: usize,
    ) -> Result<(Vec<&'a [u8]>, usize), Error> {
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
        let at = |pos: usize| -> Result<usize, Error> {
            let end = pos.checked_add(WORD).ok_or(Error::IntegerOverflow)?;
            let w = self.data.get(pos..end).ok_or(Error::Truncated)?;
            U256::from_be_slice(w)?
                .to_u64()
                .and_then(|v| usize::try_from(v).ok())
                .ok_or(Error::IntegerOverflow)
        };
        let count = at(offset)?;
        if count > max_elements {
            return Err(Error::TooDeep);
        }
        // Element offsets are relative to the word after the length.
        let base = offset.checked_add(WORD).ok_or(Error::IntegerOverflow)?;
        let mut expected = count.checked_mul(WORD).ok_or(Error::IntegerOverflow)?;
        let mut elements = Vec::with_capacity(count);
        for i in 0..count {
            let slot = i
                .checked_mul(WORD)
                .and_then(|o| base.checked_add(o))
                .ok_or(Error::IntegerOverflow)?;
            if at(slot)? != expected {
                return Err(Error::NonCanonical(
                    "array element not at the standard offset",
                ));
            }
            let start = base.checked_add(expected).ok_or(Error::IntegerOverflow)?;
            let len = at(start)?;
            let data_start = start.checked_add(WORD).ok_or(Error::IntegerOverflow)?;
            let data_end = data_start.checked_add(len).ok_or(Error::IntegerOverflow)?;
            let payload = self
                .data
                .get(data_start..data_end)
                .ok_or(Error::Truncated)?;
            let padded = padded_len(len)?;
            let pad_end = data_start
                .checked_add(padded)
                .ok_or(Error::IntegerOverflow)?;
            let padding = self.data.get(data_end..pad_end).ok_or(Error::Truncated)?;
            if padding.iter().any(|b| *b != 0) {
                return Err(Error::NonCanonical("non-zero padding after bytes element"));
            }
            elements.push(payload);
            expected = expected
                .checked_add(WORD)
                .and_then(|e| e.checked_add(padded))
                .ok_or(Error::IntegerOverflow)?;
        }
        let end = base.checked_add(expected).ok_or(Error::IntegerOverflow)?;
        Ok((elements, end))
    }

    /// Read a `(address, uint256, bytes)[]` argument, the call list ERC-7579
    /// and ERC-7821 smart accounts execute, with the same canonical-layout
    /// rules as [`Args::bytes_array_at`]: offsets in order, each tuple's
    /// `bytes` at the standard 0x60, zero padding, nothing in between.
    #[allow(clippy::type_complexity)]
    pub fn call_tuple_array_at(
        &self,
        head_index: usize,
        expected_offset: usize,
        max_elements: usize,
    ) -> Result<(Vec<(Address, U256, &'a [u8])>, usize), Error> {
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
        let word_at = |pos: usize| -> Result<&'a [u8], Error> {
            let end = pos.checked_add(WORD).ok_or(Error::IntegerOverflow)?;
            self.data.get(pos..end).ok_or(Error::Truncated)
        };
        let usize_at = |pos: usize| -> Result<usize, Error> {
            U256::from_be_slice(word_at(pos)?)?
                .to_u64()
                .and_then(|v| usize::try_from(v).ok())
                .ok_or(Error::IntegerOverflow)
        };
        let count = usize_at(offset)?;
        if count > max_elements {
            return Err(Error::TooDeep);
        }
        let base = offset.checked_add(WORD).ok_or(Error::IntegerOverflow)?;
        let mut expected = count.checked_mul(WORD).ok_or(Error::IntegerOverflow)?;
        let mut calls = Vec::with_capacity(count);
        for i in 0..count {
            let slot = i
                .checked_mul(WORD)
                .and_then(|o| base.checked_add(o))
                .ok_or(Error::IntegerOverflow)?;
            if usize_at(slot)? != expected {
                return Err(Error::NonCanonical(
                    "array element not at the standard offset",
                ));
            }
            let t = base.checked_add(expected).ok_or(Error::IntegerOverflow)?;
            let addr_word = word_at(t)?;
            let (high, low) = addr_word.split_at(12);
            if high.iter().any(|b| *b != 0) {
                return Err(Error::InvalidAddress);
            }
            let mut target = [0u8; 20];
            target.copy_from_slice(low);
            let value =
                U256::from_be_slice(word_at(t.checked_add(WORD).ok_or(Error::IntegerOverflow)?)?)?;
            let bytes_offset_pos = t.checked_add(64).ok_or(Error::IntegerOverflow)?;
            if usize_at(bytes_offset_pos)? != 96 {
                return Err(Error::NonCanonical(
                    "call data not at the standard offset in its tuple",
                ));
            }
            let len_pos = t.checked_add(96).ok_or(Error::IntegerOverflow)?;
            let len = usize_at(len_pos)?;
            let data_start = t.checked_add(128).ok_or(Error::IntegerOverflow)?;
            let data_end = data_start.checked_add(len).ok_or(Error::IntegerOverflow)?;
            let data = self
                .data
                .get(data_start..data_end)
                .ok_or(Error::Truncated)?;
            let padded = padded_len(len)?;
            let pad_end = data_start
                .checked_add(padded)
                .ok_or(Error::IntegerOverflow)?;
            let padding = self.data.get(data_end..pad_end).ok_or(Error::Truncated)?;
            if padding.iter().any(|b| *b != 0) {
                return Err(Error::NonCanonical("non-zero padding after call data"));
            }
            calls.push((target, value, data));
            expected = expected
                .checked_add(128)
                .and_then(|e| e.checked_add(padded))
                .ok_or(Error::IntegerOverflow)?;
        }
        let end = base.checked_add(expected).ok_or(Error::IntegerOverflow)?;
        Ok((calls, end))
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
