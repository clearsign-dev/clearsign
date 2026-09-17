//! # clearsign
//!
//! Decode exactly what an EVM transaction or a Safe multisig transaction does,
//! using **only the bytes that will be signed**.
//!
//! This crate is the portable core of the signing project. It is `no_std` with
//! `alloc`, has one direct dependency (Keccak hashing from RustCrypto, eight
//! crates in total including transitive ones), forbids
//! `unsafe`, and is linted so that it cannot index, unwrap, panic or perform
//! unchecked arithmetic. See `docs/01-threat-model.md` for the invariants it
//! must uphold; each invariant is referenced by ID in the code and tests.
//!
//! Entry points:
//! - [`review_evm_transaction`] for an unsigned EVM transaction (EIP-1559 or legacy).
//! - [`review_safe_transaction`] for a Safe transaction an owner is asked to sign off-chain.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod abi;
pub mod address;
pub mod calls;
pub mod error;
pub mod evm;
pub mod hex;
pub mod keccak;
pub mod multisend;
pub mod review;
pub mod rlp;
pub mod safe;
pub mod text;
pub mod u256;

pub use error::Error;
pub use evm::{
    EvmTransaction, TxType, parse_unsigned_transaction, review_evm_transaction,
    review_transaction_bytes,
};
pub use review::{Finding, Review, Section, Severity, SigningTarget, TargetKind};
pub use safe::{DomainVersion, SafeTransaction, review_safe_transaction, safe_transaction_hash};
pub use text::escape_untrusted;
pub use u256::U256;
