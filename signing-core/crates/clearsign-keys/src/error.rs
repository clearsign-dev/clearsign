use core::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum KeyError {
    /// Dice input contained something other than the digits 1 to 6 and whitespace.
    InvalidDiceRoll,
    /// Too few dice rolls for the requested use.
    NotEnoughDiceRolls { required: usize, found: usize },
    /// Hardware entropy failed a basic sanity check.
    DegenerateHardwareEntropy,
    /// The recovery phrase is not a valid English BIP-39 phrase (wrong word, wrong count, bad checksum).
    InvalidMnemonic,
    /// Passphrases are limited to printable ASCII so every wallet derives the same keys.
    PassphraseNotPrintableAscii,
    /// Key derivation failed. Astronomically unlikely for valid input.
    Derivation,
    /// The review has no signable digest (it was built for display only).
    NotSignable,
    /// A BLIND or CRITICAL finding was not acknowledged.
    UnacknowledgedFinding(&'static str),
    /// An acknowledgement was given for a finding that is not present, or not one that needs acknowledging.
    UnexpectedAcknowledgement,
    /// More findings than can be acknowledged one by one; see
    /// `MAX_ACKNOWLEDGEABLE_FINDINGS`.
    TooManyFindings,
    /// Signing failed inside the ECDSA implementation.
    Signing,
    /// The produced signature did not verify against the signing key. Nothing was released.
    SignatureSelfCheckFailed,
    /// A chain ID too large to encode a legacy EIP-155 `v` value.
    ChainIdTooLarge,
}

impl fmt::Display for KeyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            KeyError::InvalidDiceRoll => {
                f.write_str("dice rolls may contain only the digits 1 to 6")
            }
            KeyError::NotEnoughDiceRolls { required, found } => {
                write!(f, "need at least {required} dice rolls, got {found}")
            }
            KeyError::DegenerateHardwareEntropy => {
                f.write_str("hardware entropy failed a sanity check and was refused")
            }
            KeyError::InvalidMnemonic => f.write_str("not a valid English BIP-39 recovery phrase"),
            KeyError::PassphraseNotPrintableAscii => {
                f.write_str("passphrase must contain only printable ASCII characters")
            }
            KeyError::Derivation => f.write_str("key derivation failed"),
            KeyError::NotSignable => {
                f.write_str("this review is display-only and cannot be signed")
            }
            KeyError::TooManyFindings => f.write_str(
                "this review has more findings than can be acknowledged one by one; it cannot be approved",
            ),
            KeyError::UnacknowledgedFinding(code) => {
                write!(
                    f,
                    "finding {code} must be explicitly acknowledged before signing"
                )
            }
            KeyError::UnexpectedAcknowledgement => {
                f.write_str("acknowledgements must match exactly the findings that require them")
            }
            KeyError::Signing => f.write_str("signing failed"),
            KeyError::SignatureSelfCheckFailed => {
                f.write_str("signature failed its self-check and was not released")
            }
            KeyError::ChainIdTooLarge => f.write_str("chain ID too large for a legacy v value"),
        }
    }
}
