//! Typed errors. INV-7: every failure is a value, never a panic.

use core::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// Input is not valid hexadecimal.
    InvalidHex,
    /// Input ended before a complete structure was read.
    Truncated,
    /// Bytes remain after a complete structure. INV-4.
    TrailingBytes,
    /// A valid structure was encoded in a non-canonical way. INV-4.
    NonCanonical(&'static str),
    /// Nesting exceeded the fixed limit.
    TooDeep,
    /// A structure had the wrong number of fields.
    WrongFieldCount { expected: usize, found: usize },
    /// Expected a byte string, found a list, or the reverse.
    WrongItemKind,
    /// An integer does not fit in its declared width.
    IntegerOverflow,
    /// An address was not exactly 20 bytes, or had non-zero high bits in ABI encoding.
    InvalidAddress,
    /// The transaction type byte is not supported.
    UnsupportedTransactionType(u8),
    /// A Safe operation value other than 0 (call) or 1 (delegatecall).
    InvalidOperation(u8),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidHex => f.write_str("input is not valid hexadecimal"),
            Error::Truncated => f.write_str("input ended before the structure was complete"),
            Error::TrailingBytes => f.write_str("unexpected bytes after the end of the structure"),
            Error::NonCanonical(what) => write!(f, "non-canonical encoding: {what}"),
            Error::TooDeep => f.write_str("nesting is deeper than allowed"),
            Error::WrongFieldCount { expected, found } => {
                write!(f, "expected {expected} fields, found {found}")
            }
            Error::WrongItemKind => {
                f.write_str("expected a byte string but found a list, or the reverse")
            }
            Error::IntegerOverflow => f.write_str("integer does not fit in its declared width"),
            Error::InvalidAddress => f.write_str("invalid address encoding"),
            Error::UnsupportedTransactionType(t) => {
                write!(f, "unsupported transaction type 0x{t:02x}")
            }
            Error::InvalidOperation(op) => write!(f, "invalid Safe operation value {op}"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {}
