//! Unsigned EVM transactions: parsing and review.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::address::Address;
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
    /// EIP-1559, type 2.
    Eip1559,
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
    /// Keccak-256 of the exact bytes that will be signed.
    pub signing_hash: [u8; 32],
}

/// Parse the unsigned transaction bytes a wallet asks the signer to sign.
/// Signed transactions are rejected: a signer only ever signs unsigned payloads.
pub fn parse_unsigned_transaction(bytes: &[u8]) -> Result<EvmTransaction, Error> {
    let first = *bytes.first().ok_or(Error::Truncated)?;
    let signing_hash = keccak256(bytes);
    match first {
        0x02 => {
            let payload = bytes.get(1..).ok_or(Error::Truncated)?;
            let item = rlp::decode(payload)?;
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
                        signing_hash,
                    })
                }
                other => Err(Error::WrongFieldCount {
                    expected: 9,
                    found: other.len(),
                }),
            }
        }
        0x00..=0x7f => Err(Error::UnsupportedTransactionType(first)),
        0x80..=0xbf => Err(Error::WrongItemKind),
    }
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
    if tx.tx_type == TxType::Eip1559 {
        s.field("Access list entries", format!("{}", tx.access_list_entries));
    }
    review.sections.push(s);

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
