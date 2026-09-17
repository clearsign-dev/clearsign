//! # clearsign-qr
//!
//! Air-gapped transport. The signer never touches a network, a cable or a
//! filesystem shared with anything else: a request arrives as QR codes on a
//! screen, and a signature leaves as a QR code on the signer's own screen.
//!
//! Nothing here is invented. It implements the formats wallets already speak:
//!
//! - **Bytewords**, BCR-2020-012.
//! - **Uniform Resources**, BCR-2020-005, single-part and multipart.
//! - **Multipart UR fountain codes**, BCR-2024-001.
//! - **EIP-4527** `eth-sign-request` and `eth-signature`, as used by MetaMask
//!   and Keystone.
//!
//! Everything scanned is input from a computer the signer does not trust, so the
//! decoders are strict, bounded and total: no panics, no indexing, no
//! arithmetic that can overflow, and no accepting two encodings of one value.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;
#[cfg(feature = "std")]
extern crate std;

pub mod bytewords;
pub mod cbor;
pub mod crc32;
pub mod eth;
pub mod fountain;
pub mod ur;

pub use eth::{DataType, PathComponent, SignRequest, decode_sign_request, encode_signature};
pub use ur::{Decoder, Part, PartHeader};

use core::fmt;

/// Every failure in this crate is a value. INV-7.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    Bytewords(&'static str),
    Cbor(&'static str),
    Ur(&'static str),
    Eip4527(&'static str),
    /// The request decoded, but it is not something this signer will sign.
    Unsupported(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Bytewords(m) => write!(f, "bytewords: {m}"),
            Error::Cbor(m) => write!(f, "cbor: {m}"),
            Error::Ur(m) => write!(f, "uniform resource: {m}"),
            Error::Eip4527(m) => write!(f, "eth-sign-request: {m}"),
            Error::Unsupported(m) => write!(f, "unsupported request: {m}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
