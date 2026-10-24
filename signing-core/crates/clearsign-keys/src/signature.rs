//! ECDSA signing with a mandatory self-check.

use clearsign::{TargetKind, TxType};
use k256::ecdsa::{RecoveryId, SigningKey, VerifyingKey};

use crate::KeyError;
use crate::approval::Approval;

/// A recoverable secp256k1 signature, low-S normalised.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Signature {
    pub r: [u8; 32],
    pub s: [u8; 32],
    /// 0 or 1.
    pub recovery_id: u8,
    pub kind: TargetKind,
}

impl Signature {
    /// `r || s || v` with `v` = 27 or 28: the format a Safe owner signature uses,
    /// and what `cast wallet sign --no-hash` prints.
    pub fn to_rsv65(&self) -> [u8; 65] {
        let mut out = [0u8; 65];
        if let Some(r) = out.get_mut(..32) {
            r.copy_from_slice(&self.r);
        }
        if let Some(s) = out.get_mut(32..64) {
            s.copy_from_slice(&self.s);
        }
        if let Some(v) = out.get_mut(64) {
            *v = self.recovery_id.saturating_add(27);
        }
        out
    }

    /// The `v` / `y_parity` value to place in the signed transaction.
    ///
    /// - EIP-1559, EIP-2930, EIP-4844, EIP-7702: `y_parity`, 0 or 1
    /// - legacy EIP-155: `recovery_id + chain_id * 2 + 35`
    /// - legacy without chain ID: `27 + recovery_id`
    /// - Safe owner signature: `27 + recovery_id`
    pub fn v(&self) -> Result<u64, KeyError> {
        let rec = u64::from(self.recovery_id);
        match self.kind {
            // Every typed transaction carries y_parity directly.
            TargetKind::EvmTransaction {
                tx_type: TxType::Eip1559 | TxType::Eip2930 | TxType::Eip4844 | TxType::Eip7702,
                ..
            } => Ok(rec),
            TargetKind::EvmTransaction {
                tx_type: TxType::LegacyEip155,
                chain_id: Some(chain),
            } => {
                let chain = chain.to_u64().ok_or(KeyError::ChainIdTooLarge)?;
                chain
                    .checked_mul(2)
                    .and_then(|x| x.checked_add(35))
                    .and_then(|x| x.checked_add(rec))
                    .ok_or(KeyError::ChainIdTooLarge)
            }
            TargetKind::EvmTransaction {
                tx_type: TxType::LegacyEip155,
                chain_id: None,
            } => Err(KeyError::ChainIdTooLarge),
            TargetKind::EvmTransaction {
                tx_type: TxType::Legacy,
                ..
            }
            | TargetKind::SafeTransaction => Ok(rec.saturating_add(27)),
        }
    }
}

pub(crate) fn sign(key: &SigningKey, approval: &Approval<'_>) -> Result<Signature, KeyError> {
    // RFC 6979 deterministic nonce; k256 returns a low-S signature and the matching recovery ID.
    let (sig, rec) = key
        .sign_prehash_recoverable(&approval.digest)
        .map_err(|_| KeyError::Signing)?;

    // Self-check: recover the public key from the signature and compare. A fault,
    // a miscompilation or a library bug must not release a bad or leaking signature.
    let recovered = VerifyingKey::recover_from_prehash(&approval.digest, &sig, rec)
        .map_err(|_| KeyError::SignatureSelfCheckFailed)?;
    if recovered != *key.verifying_key() {
        return Err(KeyError::SignatureSelfCheckFailed);
    }
    if sig.normalize_s().is_some() {
        // A high-S signature is malleable; refuse rather than release it.
        return Err(KeyError::SignatureSelfCheckFailed);
    }

    let (r, s) = sig.split_bytes();
    let mut r_out = [0u8; 32];
    let mut s_out = [0u8; 32];
    r_out.copy_from_slice(r.as_slice());
    s_out.copy_from_slice(s.as_slice());
    Ok(Signature {
        r: r_out,
        s: s_out,
        recovery_id: recovery_byte(rec),
        kind: approval.kind,
    })
}

fn recovery_byte(rec: RecoveryId) -> u8 {
    u8::from(rec.is_y_odd())
}
