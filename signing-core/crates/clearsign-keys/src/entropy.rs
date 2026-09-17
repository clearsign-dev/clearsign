//! Seed generation.
//!
//! There is deliberately no function that makes a seed from hardware randomness
//! alone. A hardware or software random number generator can fail silently and
//! produce output that looks random but is predictable, which is exactly what
//! happened to Coldcard devices whose seeds were generated between March 2021 and
//! the July 2026 fix. No statistical test run on the device can detect a
//! deterministic generator. Dice rolls are an independent source the user can see.

use alloc::string::String;

use sha2::{Digest, Sha256};
use zeroize::Zeroizing;

use crate::KeyError;

/// Dice rolls needed when dice are the only source: 99 rolls carry about 256 bits.
pub const MIN_DICE_ROLLS_ALONE: usize = 99;
/// Dice rolls needed when mixed with hardware entropy: 50 rolls carry about 129 bits,
/// enough to protect the seed even if the hardware source is completely broken.
pub const MIN_DICE_ROLLS_MIXED: usize = 50;

const MIX_TAG: &[u8] = b"clearsign/seed-entropy/v1";

/// A validated sequence of six-sided dice rolls.
pub struct DiceRolls {
    digits: Zeroizing<String>,
}

impl DiceRolls {
    /// Accepts the digits 1 to 6; whitespace is ignored so rolls can be typed in groups.
    pub fn parse(input: &str) -> Result<DiceRolls, KeyError> {
        let mut digits = Zeroizing::new(String::with_capacity(input.len()));
        for c in input.chars() {
            match c {
                '1'..='6' => digits.push(c),
                c if c.is_whitespace() => {}
                _ => return Err(KeyError::InvalidDiceRoll),
            }
        }
        Ok(DiceRolls { digits })
    }

    pub fn count(&self) -> usize {
        self.digits.len()
    }

    fn require(&self, required: usize) -> Result<(), KeyError> {
        if self.count() < required {
            return Err(KeyError::NotEnoughDiceRolls {
                required,
                found: self.count(),
            });
        }
        Ok(())
    }
}

/// A 24-word recovery phrase from dice rolls alone.
///
/// Entropy is `SHA-256` of the roll digits as ASCII, for example `"4163..."`.
/// That lets anyone verify the phrase on a separate computer:
/// `printf '%s' "$ROLLS" | shasum -a 256`, then convert the 32 bytes to BIP-39
/// with any independent tool.
pub fn mnemonic_from_dice(rolls: &DiceRolls) -> Result<Zeroizing<String>, KeyError> {
    rolls.require(MIN_DICE_ROLLS_ALONE)?;
    let entropy = Zeroizing::new(Sha256::digest(rolls.digits.as_bytes()));
    phrase_from_entropy(entropy.as_slice())
}

/// A 24-word recovery phrase from hardware entropy mixed with dice rolls.
///
/// Entropy is `SHA-256(len(tag) || tag || len(hw) || hw || len(rolls) || rolls)`
/// with 32-bit big-endian lengths and tag `clearsign/seed-entropy/v1`. The seed
/// is safe if **either** source is good.
pub fn mnemonic_from_mixed_entropy(
    hardware: &[u8; 32],
    rolls: &DiceRolls,
) -> Result<Zeroizing<String>, KeyError> {
    rolls.require(MIN_DICE_ROLLS_MIXED)?;
    reject_degenerate(hardware)?;
    let mut h = Sha256::new();
    for part in [MIX_TAG, hardware.as_slice(), rolls.digits.as_bytes()] {
        let len = u32::try_from(part.len()).map_err(|_| KeyError::InvalidDiceRoll)?;
        h.update(len.to_be_bytes());
        h.update(part);
    }
    let entropy = Zeroizing::new(h.finalize());
    phrase_from_entropy(entropy.as_slice())
}

/// Catches only gross failures such as a stuck or zeroed source. It cannot detect
/// a deterministic generator, which is why dice are always mixed in.
fn reject_degenerate(bytes: &[u8; 32]) -> Result<(), KeyError> {
    let mut seen = [false; 256];
    let mut distinct = 0usize;
    for b in bytes {
        if let Some(slot) = seen.get_mut(usize::from(*b)) {
            if !*slot {
                *slot = true;
                distinct = distinct.saturating_add(1);
            }
        }
    }
    // 32 uniformly random bytes almost always contain 29 or more distinct values.
    if distinct < 16 {
        return Err(KeyError::DegenerateHardwareEntropy);
    }
    Ok(())
}

fn phrase_from_entropy(entropy: &[u8]) -> Result<Zeroizing<String>, KeyError> {
    let m = bip39::Mnemonic::from_entropy(entropy).map_err(|_| KeyError::InvalidMnemonic)?;
    let mut out = Zeroizing::new(String::new());
    for (i, w) in m.words().enumerate() {
        if i != 0 {
            out.push(' ');
        }
        out.push_str(w);
    }
    Ok(out)
}
