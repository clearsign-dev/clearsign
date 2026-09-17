//! Address display. Addresses are always shown in full, EIP-55 checksummed,
//! and chunked into groups of four so look-alike addresses are easier to spot.

use alloc::string::String;

use crate::hex;
use crate::keccak::keccak256;

pub type Address = [u8; 20];

pub const ZERO: Address = [0u8; 20];

/// EIP-55 mixed-case checksum encoding with a `0x` prefix.
pub fn checksummed(addr: &Address) -> String {
    let lower = hex::encode(addr);
    let hash = keccak256(lower.as_bytes());
    let mut out = String::with_capacity(42);
    out.push_str("0x");
    for (i, c) in lower.chars().enumerate() {
        let hash_byte = hash.get(i / 2).copied().unwrap_or(0);
        let nibble = if i % 2 == 0 {
            hash_byte >> 4
        } else {
            hash_byte & 0x0f
        };
        if c.is_ascii_alphabetic() && nibble >= 8 {
            out.push(c.to_ascii_uppercase());
        } else {
            out.push(c);
        }
    }
    out
}

/// Checksummed and split into groups of four after the prefix:
/// `0x1234 5678 9aBC ...`
pub fn display(addr: &Address) -> String {
    let full = checksummed(addr);
    let body = full.get(2..).unwrap_or("");
    let mut out = String::with_capacity(52);
    out.push_str("0x");
    for (i, c) in body.chars().enumerate() {
        if i != 0 && i % 4 == 0 {
            out.push(' ');
        }
        out.push(c);
    }
    if *addr == ZERO {
        out.push_str("  (zero address)");
    }
    out
}
