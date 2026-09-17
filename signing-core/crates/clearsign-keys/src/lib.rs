//! # clearsign-keys
//!
//! Seed generation, Ethereum key derivation, and signing — built so that the
//! only thing that can be signed is a digest a human has just reviewed.
//!
//! Three design rules, each traceable to a real loss:
//!
//! 1. **No signing without a review.** There is no public function that signs an
//!    arbitrary hash. [`approve`] takes a [`clearsign::Review`] and returns an
//!    [`Approval`] only when every BLIND and CRITICAL finding has been explicitly
//!    acknowledged. [`Account::sign`] accepts only an `Approval`. (Bybit, 2025:
//!    signers approved a hash they never saw decoded.)
//! 2. **No seed from a hardware random number generator alone.** Seeds come from
//!    dice rolls, or from hardware entropy *mixed with* dice rolls, so one
//!    silently broken source is not fatal. (Coldcard, 2026: a build error routed
//!    seed generation to a deterministic generator; seeds made from enough dice
//!    rolls were not affected.)
//! 3. **Verify every signature before releasing it.** A signature is recovered
//!    and checked against the signing key, catching faults and library bugs.
//!
//! This crate performs no cryptography of its own. It composes BIP-39
//! (rust-bitcoin `bip39`), BIP-32 (`bip32`) and secp256k1 ECDSA (RustCrypto
//! `k256`), and every output is tested against an independent implementation.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

mod approval;
mod entropy;
mod error;
mod signature;
mod wallet;

pub use approval::{Approval, approve};
pub use entropy::{
    DiceRolls, MIN_DICE_ROLLS_ALONE, MIN_DICE_ROLLS_MIXED, mnemonic_from_dice,
    mnemonic_from_mixed_entropy,
};
pub use error::KeyError;
pub use signature::Signature;
pub use wallet::{Account, Wallet};
