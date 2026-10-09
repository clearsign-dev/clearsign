//! Unsigned EVM transactions: parsing and review.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::address::{self, Address};
use crate::calls::{self, CallContext, Operation};
use crate::keccak::keccak256;
use crate::review::{Review, Section, Severity};
use crate::rlp::{self, Item};
use crate::{Error, U256, hex};

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
        let y_parity = match y_parity.as_uint()?.to_u64() {
            Some(0) => 0,
            Some(1) => 1,
            _ => return Err(Error::NonCanonical("authorization y_parity is not 0 or 1")),
        };
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

/// EIP-7702: each authorization lets code run as the account that signed it.
///
/// That account is recovered from the tuple's signature, which needs elliptic
/// curve arithmetic this crate deliberately does not carry; the review names it
/// as "the account that signed authorization N", which is the account sending
/// this transaction when it sponsors its own delegation.
fn review_authorizations(review: &mut Review, tx: &EvmTransaction) {
    let total = tx.authorizations.len();
    for (i, a) in tx.authorizations.iter().enumerate() {
        let n = i.saturating_add(1);
        let will_skip = a.nonce == U256::from_u64(u64::MAX);
        let mut s = Section::new(&format!(
            "{} {n} of {total}",
            if will_skip {
                "Skipped account authorization"
            } else {
                "Account delegation"
            }
        ));
        s.field(
            "Valid on chain ID",
            if a.chain_id.is_zero() {
                String::from("0 — EVERY chain")
            } else {
                a.chain_id.to_decimal()
            },
        );
        s.field(
            if will_skip {
                "Requested code address"
            } else {
                "Delegates to code at"
            },
            address::display(&a.address),
        );
        s.field("Authorization nonce", a.nonce.to_decimal());
        review.sections.push(s);
        if will_skip {
            review.find(
                Severity::Warning,
                "AUTHORIZATION_WILL_BE_SKIPPED",
                format!(
                    "Authorization {n} has nonce 2^64 - 1. EIP-7702 processors skip that tuple, so it does not change the account's delegation."
                ),
            );
            continue;
        }
        if a.address == address::ZERO {
            review.find(
                Severity::Info,
                "DELEGATION_CLEARED",
                format!(
                    "Authorization {n} clears the signing account's delegation, so it behaves as an \
                     ordinary account again."
                ),
            );
            continue;
        }
        review.find(
            Severity::Critical,
            "ACCOUNT_DELEGATION",
            format!(
                "Authorization {n} makes the code at {} run as the account that signed it, \
                 usually you. That code can move everything the account holds, now and later, \
                 until the delegation is replaced. Nothing in these bytes says what it does.",
                address::checksummed(&a.address)
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
