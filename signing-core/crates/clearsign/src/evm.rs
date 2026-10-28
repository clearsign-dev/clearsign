//! Unsigned EVM transactions: parsing and review.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use k256::ecdsa::{RecoveryId, Signature, VerifyingKey};

use crate::address::{self, Address};
use crate::calls::{self, CallContext, Operation};
use crate::keccak::keccak256;
use crate::review::{Review, Section, Severity};
use crate::rlp::{self, Item};
use crate::{Error, U256, hex};

/// Maximum encoded transaction accepted by the public raw-byte parser.
///
/// This matches the record limit used by the JSON, native and browser
/// boundaries. Keeping it here makes the library safe for embedders that do
/// not add a transport-specific cap of their own.
pub const MAX_TRANSACTION_BYTES: usize = 1024 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TxType {
    /// Pre-EIP-155: no chain ID, replayable on other chains.
    Legacy,
    /// EIP-155 legacy with chain ID.
    LegacyEip155,
    /// EIP-2930, type 1: legacy pricing with an access list.
    Eip2930,
    /// EIP-1559, type 2.
    Eip1559,
    /// EIP-4844, type 3: carries blobs, committed to by hash.
    Eip4844,
    /// EIP-7702, type 4: carries authorizations that delegate accounts to code.
    Eip7702,
}

/// One EIP-7702 authorization tuple: the account that signed it lets the code
/// at `address` run as that account, on `chain_id` (0 meaning every chain).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Authorization {
    pub chain_id: U256,
    pub address: Address,
    pub nonce: U256,
    pub y_parity: u8,
    pub r: U256,
    pub s: U256,
}

/// Why an EIP-7702 authorization signature cannot identify an authority.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AuthorizationSignatureError {
    InvalidYParity,
    InvalidScalar,
    HighS,
    RecoveryFailed,
}

impl Authorization {
    /// `keccak256(0x05 || rlp([chain_id, address, nonce]))`, as defined by EIP-7702.
    pub fn signature_hash(&self) -> [u8; 32] {
        let mut payload = Vec::with_capacity(64);
        append_rlp_uint(&mut payload, &self.chain_id);
        append_rlp_bytes(&mut payload, &self.address);
        append_rlp_uint(&mut payload, &self.nonce);

        let mut encoded = Vec::with_capacity(payload.len().saturating_add(3));
        encoded.push(0x05);
        append_rlp_list_header(&mut encoded, payload.len());
        encoded.extend_from_slice(&payload);
        keccak256(&encoded)
    }

    /// Recover the account that signed this tuple after enforcing EIP-2 low-S
    /// and secp256k1 scalar/parity constraints.
    pub fn recover_authority(&self) -> Result<Address, AuthorizationSignatureError> {
        if self.y_parity > 1 {
            return Err(AuthorizationSignatureError::InvalidYParity);
        }
        let recovery_id = RecoveryId::from_byte(self.y_parity)
            .ok_or(AuthorizationSignatureError::InvalidYParity)?;
        let signature = Signature::from_scalars(self.r.0, self.s.0)
            .map_err(|_| AuthorizationSignatureError::InvalidScalar)?;
        if signature.normalize_s() != signature {
            return Err(AuthorizationSignatureError::HighS);
        }
        let key =
            VerifyingKey::recover_from_prehash(&self.signature_hash(), &signature, recovery_id)
                .map_err(|_| AuthorizationSignatureError::RecoveryFailed)?;
        let point = key.to_sec1_point(false);
        let coordinates = point
            .as_bytes()
            .get(1..)
            .ok_or(AuthorizationSignatureError::RecoveryFailed)?;
        let hash = keccak256(coordinates);
        let mut authority = [0u8; 20];
        authority.copy_from_slice(
            hash.get(12..)
                .ok_or(AuthorizationSignatureError::RecoveryFailed)?,
        );
        Ok(authority)
    }
}

fn significant_bytes(value: &U256) -> &[u8] {
    let first = value.0.iter().position(|byte| *byte != 0).unwrap_or(32);
    value.0.get(first..).unwrap_or(&[])
}

fn append_rlp_uint(out: &mut Vec<u8>, value: &U256) {
    append_rlp_bytes(out, significant_bytes(value));
}

fn append_rlp_bytes(out: &mut Vec<u8>, bytes: &[u8]) {
    if bytes.len() == 1 && bytes.first().is_some_and(|byte| *byte < 0x80) {
        out.extend_from_slice(bytes);
        return;
    }
    let len = u8::try_from(bytes.len()).unwrap_or(u8::MAX);
    out.push(0x80u8.saturating_add(len));
    out.extend_from_slice(bytes);
}

fn append_rlp_list_header(out: &mut Vec<u8>, payload_len: usize) {
    let len = u8::try_from(payload_len).unwrap_or(u8::MAX);
    if payload_len <= 55 {
        out.push(0xc0u8.saturating_add(len));
    } else {
        // The three fixed authorization fields encode to at most 63 bytes.
        out.push(0xf8);
        out.push(len);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EvmTransaction {
    pub tx_type: TxType,
    pub chain_id: Option<U256>,
    pub nonce: U256,
    pub gas_price: Option<U256>,
    pub max_priority_fee_per_gas: Option<U256>,
    pub max_fee_per_gas: Option<U256>,
    pub gas_limit: U256,
    /// `None` means contract creation.
    pub to: Option<Address>,
    pub value: U256,
    pub data: Vec<u8>,
    pub access_list_entries: usize,
    /// EIP-4844 only.
    pub max_fee_per_blob_gas: Option<U256>,
    /// EIP-4844 only: the versioned hashes of the blobs the transaction carries.
    pub blob_versioned_hashes: Vec<[u8; 32]>,
    /// EIP-7702 only.
    pub authorizations: Vec<Authorization>,
    /// Keccak-256 of the exact bytes that will be signed.
    pub signing_hash: [u8; 32],
}

/// Parse the unsigned transaction bytes a wallet asks the signer to sign.
/// Signed transactions are rejected: a signer only ever signs unsigned payloads.
pub fn parse_unsigned_transaction(bytes: &[u8]) -> Result<EvmTransaction, Error> {
    if bytes.len() > MAX_TRANSACTION_BYTES {
        return Err(Error::TooLarge("transaction exceeds 1 MiB"));
    }
    let first = *bytes.first().ok_or(Error::Truncated)?;
    let signing_hash = keccak256(bytes);
    let typed_fields = || -> Result<Item<'_>, Error> {
        let payload = bytes.get(1..).ok_or(Error::Truncated)?;
        rlp::decode(payload)
    };
    match first {
        0x01 => {
            let item = typed_fields()?;
            let f = item.as_list()?;
            let [
                chain_id,
                nonce,
                gas_price,
                gas,
                to,
                value,
                data,
                access_list,
            ] = f
            else {
                return Err(Error::WrongFieldCount {
                    expected: 8,
                    found: f.len(),
                });
            };
            Ok(EvmTransaction {
                tx_type: TxType::Eip2930,
                chain_id: Some(chain_id.as_uint()?),
                nonce: nonce.as_uint()?,
                gas_price: Some(gas_price.as_uint()?),
                max_priority_fee_per_gas: None,
                max_fee_per_gas: None,
                gas_limit: gas.as_uint()?,
                to: to.as_optional_address()?,
                value: value.as_uint()?,
                data: data.as_bytes()?.to_vec(),
                access_list_entries: validate_access_list(access_list)?,
                max_fee_per_blob_gas: None,
                blob_versioned_hashes: Vec::new(),
                authorizations: Vec::new(),
                signing_hash,
            })
        }
        0x02 => {
            let item = typed_fields()?;
            let f = item.as_list()?;
            let [
                chain_id,
                nonce,
                prio,
                max_fee,
                gas,
                to,
                value,
                data,
                access_list,
            ] = f
            else {
                return Err(Error::WrongFieldCount {
                    expected: 9,
                    found: f.len(),
                });
            };
            Ok(EvmTransaction {
                tx_type: TxType::Eip1559,
                chain_id: Some(chain_id.as_uint()?),
                nonce: nonce.as_uint()?,
                gas_price: None,
                max_priority_fee_per_gas: Some(prio.as_uint()?),
                max_fee_per_gas: Some(max_fee.as_uint()?),
                gas_limit: gas.as_uint()?,
                to: to.as_optional_address()?,
                value: value.as_uint()?,
                data: data.as_bytes()?.to_vec(),
                access_list_entries: validate_access_list(access_list)?,
                max_fee_per_blob_gas: None,
                blob_versioned_hashes: Vec::new(),
                authorizations: Vec::new(),
                signing_hash,
            })
        }
        0x03 => {
            let item = typed_fields()?;
            let f = item.as_list()?;
            let [
                chain_id,
                nonce,
                prio,
                max_fee,
                gas,
                to,
                value,
                data,
                access_list,
                max_fee_per_blob_gas,
                blob_hashes,
            ] = f
            else {
                return Err(Error::WrongFieldCount {
                    expected: 11,
                    found: f.len(),
                });
            };
            Ok(EvmTransaction {
                tx_type: TxType::Eip4844,
                chain_id: Some(chain_id.as_uint()?),
                nonce: nonce.as_uint()?,
                gas_price: None,
                max_priority_fee_per_gas: Some(prio.as_uint()?),
                max_fee_per_gas: Some(max_fee.as_uint()?),
                gas_limit: gas.as_uint()?,
                to: Some(required_destination(to)?),
                value: value.as_uint()?,
                data: data.as_bytes()?.to_vec(),
                access_list_entries: validate_access_list(access_list)?,
                max_fee_per_blob_gas: Some(max_fee_per_blob_gas.as_uint()?),
                blob_versioned_hashes: parse_blob_hashes(blob_hashes)?,
                authorizations: Vec::new(),
                signing_hash,
            })
        }
        0x04 => {
            let item = typed_fields()?;
            let f = item.as_list()?;
            let [
                chain_id,
                nonce,
                prio,
                max_fee,
                gas,
                to,
                value,
                data,
                access_list,
                authorization_list,
            ] = f
            else {
                return Err(Error::WrongFieldCount {
                    expected: 10,
                    found: f.len(),
                });
            };
            Ok(EvmTransaction {
                tx_type: TxType::Eip7702,
                chain_id: Some(chain_id.as_uint()?),
                nonce: nonce.as_uint()?,
                gas_price: None,
                max_priority_fee_per_gas: Some(prio.as_uint()?),
                max_fee_per_gas: Some(max_fee.as_uint()?),
                gas_limit: gas.as_uint()?,
                to: Some(required_destination(to)?),
                value: value.as_uint()?,
                data: data.as_bytes()?.to_vec(),
                access_list_entries: validate_access_list(access_list)?,
                max_fee_per_blob_gas: None,
                blob_versioned_hashes: Vec::new(),
                authorizations: parse_authorizations(authorization_list)?,
                signing_hash,
            })
        }
        0xc0..=0xff => {
            let item = rlp::decode(bytes)?;
            let f = item.as_list()?;
            match f {
                [nonce, gas_price, gas, to, value, data] => Ok(EvmTransaction {
                    tx_type: TxType::Legacy,
                    chain_id: None,
                    nonce: nonce.as_uint()?,
                    gas_price: Some(gas_price.as_uint()?),
                    max_priority_fee_per_gas: None,
                    max_fee_per_gas: None,
                    gas_limit: gas.as_uint()?,
                    to: to.as_optional_address()?,
                    value: value.as_uint()?,
                    data: data.as_bytes()?.to_vec(),
                    access_list_entries: 0,
                    max_fee_per_blob_gas: None,
                    blob_versioned_hashes: Vec::new(),
                    authorizations: Vec::new(),
                    signing_hash,
                }),
                [nonce, gas_price, gas, to, value, data, chain_id, r, s] => {
                    if !r.as_bytes()?.is_empty() || !s.as_bytes()?.is_empty() {
                        return Err(Error::NonCanonical(
                            "unsigned EIP-155 transaction must have empty r and s",
                        ));
                    }
                    Ok(EvmTransaction {
                        tx_type: TxType::LegacyEip155,
                        chain_id: Some(chain_id.as_uint()?),
                        nonce: nonce.as_uint()?,
                        gas_price: Some(gas_price.as_uint()?),
                        max_priority_fee_per_gas: None,
                        max_fee_per_gas: None,
                        gas_limit: gas.as_uint()?,
                        to: to.as_optional_address()?,
                        value: value.as_uint()?,
                        data: data.as_bytes()?.to_vec(),
                        access_list_entries: 0,
                        max_fee_per_blob_gas: None,
                        blob_versioned_hashes: Vec::new(),
                        authorizations: Vec::new(),
                        signing_hash,
                    })
                }
                other => Err(Error::WrongFieldCount {
                    expected: 9,
                    found: other.len(),
                }),
            }
        }
        0x00 | 0x05..=0x7f => Err(Error::UnsupportedTransactionType(first)),
        0x80..=0xbf => Err(Error::WrongItemKind),
    }
}

/// Blob and set-code transactions cannot create contracts: the destination
/// must be present and exactly 20 bytes.
fn required_destination(item: &Item<'_>) -> Result<Address, Error> {
    item.as_optional_address()?.ok_or(Error::NonCanonical(
        "this transaction type cannot create a contract, so it needs a destination",
    ))
}

/// The most blob hashes accepted. Each block carries a handful of blobs; this
/// is far above any real transaction and keeps the list from driving memory.
pub const MAX_BLOB_HASHES: usize = 64;

fn parse_blob_hashes(item: &Item<'_>) -> Result<Vec<[u8; 32]>, Error> {
    let list = item.as_list()?;
    if list.is_empty() {
        return Err(Error::NonCanonical(
            "a blob transaction must carry at least one blob",
        ));
    }
    if list.len() > MAX_BLOB_HASHES {
        return Err(Error::TooDeep);
    }
    let mut out = Vec::with_capacity(list.len());
    for h in list {
        let bytes = h.as_bytes()?;
        let hash: [u8; 32] = bytes
            .try_into()
            .map_err(|_| Error::NonCanonical("blob versioned hash is not 32 bytes"))?;
        // Version 0x01 is the KZG commitment scheme, the only one defined.
        if hash.first() != Some(&0x01) {
            return Err(Error::NonCanonical(
                "blob versioned hash has an unknown version",
            ));
        }
        out.push(hash);
    }
    Ok(out)
}

/// The most authorizations accepted in one transaction, so a declared list
/// cannot drive memory; real transactions carry one or a few.
pub const MAX_AUTHORIZATIONS: usize = 64;

fn parse_authorizations(item: &Item<'_>) -> Result<Vec<Authorization>, Error> {
    let list = item.as_list()?;
    if list.is_empty() {
        return Err(Error::NonCanonical(
            "a set-code transaction must carry at least one authorization",
        ));
    }
    if list.len() > MAX_AUTHORIZATIONS {
        return Err(Error::TooDeep);
    }
    let mut out = Vec::with_capacity(list.len());
    for tuple in list {
        let fields = tuple.as_list()?;
        let [chain_id, address, nonce, y_parity, r, s] = fields else {
            return Err(Error::WrongFieldCount {
                expected: 6,
                found: fields.len(),
            });
        };
        let address = address
            .as_optional_address()?
            .ok_or(Error::InvalidAddress)?;
        let nonce = nonce.as_uint()?;
        if nonce.to_u64().is_none() {
            return Err(Error::IntegerOverflow);
        }
        // The field itself is uint8. Values other than 0 and 1 make only this
        // tuple's signature invalid, so the review reports that tuple as skipped.
        let y_parity = y_parity
            .as_uint()?
            .to_u64()
            .and_then(|value| u8::try_from(value).ok())
            .ok_or(Error::IntegerOverflow)?;
        out.push(Authorization {
            chain_id: chain_id.as_uint()?,
            address,
            nonce,
            y_parity,
            r: r.as_uint()?,
            s: s.as_uint()?,
        });
    }
    Ok(out)
}

/// Each entry must be `[address, [slot, ...]]` with 20-byte address and 32-byte slots.
fn validate_access_list(item: &Item<'_>) -> Result<usize, Error> {
    let entries = item.as_list()?;
    for entry in entries {
        let [addr, slots] = entry.as_list()? else {
            return Err(Error::WrongFieldCount {
                expected: 2,
                found: entry.as_list()?.len(),
            });
        };
        if addr.as_bytes()?.len() != 20 {
            return Err(Error::InvalidAddress);
        }
        for slot in slots.as_list()? {
            if slot.as_bytes()?.len() != 32 {
                return Err(Error::NonCanonical(
                    "access list storage key is not 32 bytes",
                ));
            }
        }
    }
    Ok(entries.len())
}

/// Build a display-only review of an already-parsed transaction. It carries no
/// signing target; use [`review_transaction_bytes`] for anything that may be signed.
pub fn review_evm_transaction(tx: &EvmTransaction) -> Review {
    let mut review = Review::new(match tx.tx_type {
        TxType::Eip1559 => "EVM transaction (EIP-1559)",
        TxType::Eip2930 => "EVM transaction (EIP-2930, access list)",
        TxType::Eip4844 => "EVM transaction (EIP-4844, carries blobs)",
        TxType::Eip7702 => "EVM transaction (EIP-7702, delegates accounts to code)",
        TxType::LegacyEip155 => "EVM transaction (legacy, EIP-155)",
        TxType::Legacy => "EVM transaction (legacy, no chain ID)",
    });

    let mut s = Section::new("Transaction");
    s.field(
        "Network chain ID",
        match tx.chain_id {
            Some(id) => id.to_decimal(),
            None => String::from("none"),
        },
    );
    s.field("Nonce", tx.nonce.to_decimal());
    s.field("Gas limit", tx.gas_limit.to_grouped_decimal());
    if let Some(p) = tx.gas_price {
        s.field("Gas price (wei)", p.to_grouped_decimal());
    }
    if let Some(m) = tx.max_fee_per_gas {
        s.field("Max fee per gas (wei)", m.to_grouped_decimal());
    }
    if let Some(p) = tx.max_priority_fee_per_gas {
        s.field("Max priority fee (wei)", p.to_grouped_decimal());
    }
    if let Some(b) = tx.max_fee_per_blob_gas {
        s.field("Max fee per blob gas (wei)", b.to_grouped_decimal());
    }
    if !matches!(tx.tx_type, TxType::Legacy | TxType::LegacyEip155) {
        s.field("Access list entries", format!("{}", tx.access_list_entries));
    }
    review.sections.push(s);

    if !tx.blob_versioned_hashes.is_empty() {
        let mut b = Section::new("Blobs");
        b.field(
            "Blobs carried",
            format!("{}", tx.blob_versioned_hashes.len()),
        );
        for (i, h) in tx.blob_versioned_hashes.iter().enumerate() {
            b.field(
                &format!("Versioned hash {}", i.saturating_add(1)),
                hex::encode_prefixed(h),
            );
        }
        review.sections.push(b);
        review.find(
            Severity::Info,
            "BLOB_TRANSACTION",
            format!(
                "Carries {} blob(s). Only their hashes are signed; the blob contents travel \
                 separately and are not shown here.",
                tx.blob_versioned_hashes.len()
            ),
        );
    }

    review_authorizations(&mut review, tx);

    if tx.tx_type == TxType::Legacy {
        review.find(
            Severity::Warning,
            "REPLAYABLE_NO_CHAIN_ID",
            String::from("This transaction has no chain ID, so a signature for it can be replayed on other EVM networks."),
        );
    }
    if tx.access_list_entries > 0 {
        review.find(
            Severity::Info,
            "ACCESS_LIST",
            String::from(
                "An access list is present. It affects gas cost, not what the transaction does.",
            ),
        );
    }

    match tx.to {
        Some(to) => calls::review_call(
            &mut review,
            "Call",
            to,
            tx.value,
            &tx.data,
            CallContext {
                safe: None,
                authenticated_safe: None,
                operation: Operation::Call,
                chain_id: tx.chain_id.and_then(|c| c.to_u64()),
                depth: 0,
            },
        ),
        None => {
            let mut c = Section::new("Contract creation");
            c.field("Native value (wei)", tx.value.to_grouped_decimal());
            c.field("Initialisation code", format!("{} bytes", tx.data.len()));
            review.sections.push(c);
            review.find(
                Severity::Blind,
                "CONTRACT_CREATION",
                String::from("Deploys a new contract. Its initialisation code is not decoded, so what it does is unknown."),
            );
        }
    }

    review.digests.push((
        String::from("Transaction signing hash"),
        hex::encode_prefixed(&tx.signing_hash),
    ));
    review
}

fn signature_error(error: AuthorizationSignatureError) -> &'static str {
    match error {
        AuthorizationSignatureError::InvalidYParity => "yParity is not 0 or 1",
        AuthorizationSignatureError::InvalidScalar => {
            "r or s is zero or outside the secp256k1 scalar range"
        }
        AuthorizationSignatureError::HighS => "s is above the EIP-2 low-S limit",
        AuthorizationSignatureError::RecoveryFailed => "no authority can be recovered",
    }
}

/// EIP-7702 processing stops at the first failed check for each tuple. Checks
/// that require chain state are never inferred from the unsigned bytes.
fn review_authorizations(review: &mut Review, tx: &EvmTransaction) {
    let total = tx.authorizations.len();
    let recoveries: Vec<Result<Address, AuthorizationSignatureError>> = tx
        .authorizations
        .iter()
        .map(Authorization::recover_authority)
        .collect();
    for (i, a) in tx.authorizations.iter().enumerate() {
        let n = i.saturating_add(1);
        let chain_matches = tx
            .chain_id
            .is_some_and(|chain_id| a.chain_id.is_zero() || a.chain_id == chain_id);
        let nonce_is_max = a.nonce == U256::from_u64(u64::MAX);
        let recovery = recoveries
            .get(i)
            .copied()
            .unwrap_or(Err(AuthorizationSignatureError::RecoveryFailed));
        let will_skip = !chain_matches || nonce_is_max || recovery.is_err();
        let mut section = Section::new(&format!(
            "{} {n} of {total}",
            if will_skip {
                "Skipped account authorization"
            } else {
                "Account authorization"
            }
        ));
        section.field(
            "Valid on chain ID",
            if a.chain_id.is_zero() {
                String::from("0 — EVERY chain")
            } else {
                a.chain_id.to_decimal()
            },
        );
        section.field(
            "Stateless signature",
            match recovery {
                Ok(_) => String::from("VALID: canonical low-S signature"),
                Err(error) => format!("INVALID: {}", signature_error(error)),
            },
        );
        section.field(
            "Recovered authority",
            match recovery {
                Ok(authority) => address::display(&authority),
                Err(_) => String::from("UNAVAILABLE"),
            },
        );
        section.field(
            if will_skip {
                "Authorization address (not applied)"
            } else {
                "Requested code address"
            },
            address::display(&a.address),
        );
        section.field("Authorization nonce", a.nonce.to_decimal());
        section.field(
            "Transaction sender relationship",
            String::from("UNKNOWN: the unsigned transaction may be sponsored"),
        );

        let duplicate = match recovery {
            Ok(authority) => recoveries
                .iter()
                .take(i)
                .position(|earlier| *earlier == Ok(authority)),
            Err(_) => None,
        };
        if let Some(earlier) = duplicate {
            section.field(
                "Repeated authority",
                format!(
                    "YES: also authorization {}; the last valid occurrence wins",
                    earlier.saturating_add(1)
                ),
            );
        }

        let state_status = if !chain_matches {
            "UNKNOWN on its authorized chain; not checked because this transaction chain skips the tuple"
        } else if nonce_is_max {
            "UNKNOWN; not checked because the maximum nonce makes this tuple skip"
        } else if recovery.is_err() {
            "UNKNOWN; not checked because the signature makes this tuple skip"
        } else {
            "UNKNOWN: current authority nonce and code are not in the signed bytes"
        };
        section.field("State-dependent effectiveness", String::from(state_status));
        review.sections.push(section);

        if let Some(earlier) = duplicate {
            review.find(
                Severity::Warning,
                "DUPLICATE_AUTHORITY",
                format!(
                    "Authorization {n} and authorization {} recover to the same authority. EIP-7702 uses the last occurrence that passes every stateless and state-dependent check; these bytes do not prove which occurrences are effective.",
                    earlier.saturating_add(1)
                ),
            );
        }

        if !chain_matches {
            review.find(
                Severity::Warning,
                "AUTHORIZATION_CHAIN_MISMATCH",
                format!(
                    "Authorization {n} is skipped before its requested address is interpreted: its chain ID {} is neither 0 nor the transaction chain ID {}. Its state-dependent validity on the authorized chain is unknown.",
                    a.chain_id.to_decimal(),
                    tx.chain_id
                        .map_or_else(|| String::from("none"), |id| id.to_decimal())
                ),
            );
            continue;
        }
        if nonce_is_max {
            review.find(
                Severity::Warning,
                "AUTHORIZATION_WILL_BE_SKIPPED",
                format!(
                    "Authorization {n} has nonce 2^64 - 1. EIP-7702 processors skip that tuple before signature or state checks, so it does not change the recovered authority's delegation."
                ),
            );
            continue;
        }
        let authority = match recovery {
            Ok(authority) => authority,
            Err(error) => {
                review.find(
                    Severity::Warning,
                    "AUTHORIZATION_INVALID_SIGNATURE",
                    format!(
                        "Authorization {n} is skipped because its stateless signature is invalid ({}). No authority or delegation effect can be claimed.",
                        signature_error(error)
                    ),
                );
                continue;
            }
        };

        review.find(
            Severity::Blind,
            "AUTHORIZATION_STATE_UNKNOWN",
            format!(
                "Authorization {n} has a valid signature from {}, but whether it takes effect is UNKNOWN: the signed bytes do not prove the authority's current nonce or that its code is empty or already delegated.",
                address::checksummed(&authority)
            ),
        );
        if a.address == address::ZERO {
            review.find(
                Severity::Info,
                "DELEGATION_CLEAR_REQUESTED",
                format!(
                    "Authorization {n} requests clearing the delegation of recovered authority {} if every state-dependent check passes. The outer transaction sender may be a sponsor and is not assumed to be this authority.",
                    address::checksummed(&authority)
                ),
            );
            continue;
        }
        review.find(
            Severity::Critical,
            "ACCOUNT_DELEGATION",
            format!(
                "Authorization {n} requests that code at {} run as recovered authority {} if every state-dependent check passes. The transaction may be sponsored, so the authority is not assumed to be its sender. That code can move everything the authority holds, now and later, until the delegation is replaced. Nothing in these bytes says what it does.",
                address::checksummed(&a.address),
                address::checksummed(&authority)
            ),
        );
        if a.chain_id.is_zero() {
            review.find(
                Severity::Critical,
                "DELEGATION_ON_EVERY_CHAIN",
                format!(
                    "Authorization {n} carries chain ID 0, so it is valid on every EVM chain, not \
                     only this one."
                ),
            );
        }
    }
}

/// Parse and review raw unsigned transaction bytes. This is the only way to get
/// a signable review of an EVM transaction: the digest is recomputed here from
/// the bytes, never taken from a caller-supplied [`EvmTransaction`]. INV-1.
pub fn review_transaction_bytes(bytes: &[u8]) -> Result<Review, Error> {
    let tx = parse_unsigned_transaction(bytes)?;
    let mut review = review_evm_transaction(&tx);
    review.set_signing_target(crate::review::SigningTarget {
        kind: crate::review::TargetKind::EvmTransaction {
            tx_type: tx.tx_type,
            chain_id: tx.chain_id,
        },
        digest: keccak256(bytes),
    });
    Ok(review)
}
