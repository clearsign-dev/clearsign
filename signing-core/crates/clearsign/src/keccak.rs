//! Keccak-256, the only cryptographic primitive the decoder needs.

use sha3::{Digest, Keccak256};

pub fn keccak256(data: &[u8]) -> [u8; 32] {
    let digest = Keccak256::digest(data);
    let mut out = [0u8; 32];
    out.copy_from_slice(digest.as_slice());
    out
}

/// Keccak-256 over the concatenation of several slices, without allocating.
pub fn keccak256_concat(parts: &[&[u8]]) -> [u8; 32] {
    let mut hasher = Keccak256::new();
    for part in parts {
        hasher.update(part);
    }
    let digest = hasher.finalize();
    let mut out = [0u8; 32];
    out.copy_from_slice(digest.as_slice());
    out
}

/// First four bytes of Keccak-256 of a function signature.
pub fn selector(signature: &str) -> [u8; 4] {
    let h = keccak256(signature.as_bytes());
    let mut out = [0u8; 4];
    out.copy_from_slice(h.get(..4).unwrap_or(&[0u8; 4]));
    out
}
