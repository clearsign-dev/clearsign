//! Safe multisig transactions that an owner is asked to sign off-chain.
//!
//! This is the flow the Bybit signers used in February 2025. The owner signs an
//! EIP-712 hash of a `SafeTx` struct. The interface that prepares it cannot be
//! trusted, so the signer recomputes the hash from the fields (INV-5) and
//! decodes the call those fields describe.
//!
//! Constants were confirmed against safe-global/safe-smart-account at tags
//! v1.1.1, v1.3.0 and v1.4.1. The hashes are computed from the type strings at
//! run time, and tests pin them to the published constants.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::abi::{word_from_address, word_from_u8};
use crate::address::{self, Address};
use crate::calls::{self, CallContext, Operation};
use crate::keccak::{keccak256, keccak256_concat};
use crate::review::{Review, Section, Severity};
use crate::{U256, hex};

pub const SAFE_TX_TYPE: &str = "SafeTx(address to,uint256 value,bytes data,uint8 operation,uint256 safeTxGas,uint256 baseGas,uint256 gasPrice,address gasToken,address refundReceiver,uint256 nonce)";
pub const DOMAIN_TYPE_V1_3: &str = "EIP712Domain(uint256 chainId,address verifyingContract)";
pub const DOMAIN_TYPE_LEGACY: &str = "EIP712Domain(address verifyingContract)";

/// Which EIP-712 domain the Safe contract uses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DomainVersion {
    /// Safe v1.3.0 and later: domain includes the chain ID.
    V1_3Plus,
    /// Safe v1.1.x: domain is only the Safe address. (v1.0.0 used a different
    /// struct field name and is not supported.)
    Legacy,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SafeTransaction {
    pub chain_id: U256,
    pub safe: Address,
    pub to: Address,
    pub value: U256,
    pub data: Vec<u8>,
    pub operation: u8,
    pub safe_tx_gas: U256,
    pub base_gas: U256,
    pub gas_price: U256,
    pub gas_token: Address,
    pub refund_receiver: Address,
    pub nonce: U256,
}

pub fn domain_separator(tx: &SafeTransaction, version: DomainVersion) -> [u8; 32] {
    let safe_word = word_from_address(&tx.safe);
    match version {
        DomainVersion::V1_3Plus => {
            let typehash = keccak256(DOMAIN_TYPE_V1_3.as_bytes());
            keccak256_concat(&[&typehash, &tx.chain_id.0, &safe_word])
        }
        DomainVersion::Legacy => {
            let typehash = keccak256(DOMAIN_TYPE_LEGACY.as_bytes());
            keccak256_concat(&[&typehash, &safe_word])
        }
    }
}

pub fn struct_hash(tx: &SafeTransaction) -> [u8; 32] {
    let typehash = keccak256(SAFE_TX_TYPE.as_bytes());
    let data_hash = keccak256(&tx.data);
    keccak256_concat(&[
        &typehash,
        &word_from_address(&tx.to),
        &tx.value.0,
        &data_hash,
        &word_from_u8(tx.operation),
        &tx.safe_tx_gas.0,
        &tx.base_gas.0,
        &tx.gas_price.0,
        &word_from_address(&tx.gas_token),
        &word_from_address(&tx.refund_receiver),
        &tx.nonce.0,
    ])
}

/// The hash each owner actually signs: `keccak256(0x19 || 0x01 || domainSeparator || structHash)`.
pub fn safe_transaction_hash(tx: &SafeTransaction, version: DomainVersion) -> [u8; 32] {
    let domain = domain_separator(tx, version);
    let message = struct_hash(tx);
    keccak256_concat(&[&[0x19, 0x01], &domain, &message])
}

pub fn review_safe_transaction(tx: &SafeTransaction, version: DomainVersion) -> Review {
    let mut review = Review::new("Safe transaction (owner signature)");

    let mut s = Section::new("Safe");
    match version {
        // The chain ID is part of what is hashed, so it is part of what is signed.
        DomainVersion::V1_3Plus => s.field("Network chain ID", tx.chain_id.to_decimal()),
        // Under the v1.1.x domain it is not. Printing it next to fields that are
        // signed would tell the reader this signature belongs to one chain, and
        // it belongs to all of them.
        DomainVersion::Legacy => s.field(
            "Network chain ID",
            format!("{} — NOT part of this signature", tx.chain_id.to_decimal()),
        ),
    }
    s.field("Safe", address::display(&tx.safe));
    s.field("Safe nonce", tx.nonce.to_decimal());
    s.field(
        "Contract version domain",
        String::from(match version {
            DomainVersion::V1_3Plus => "v1.3.0 or later",
            DomainVersion::Legacy => "v1.1.x",
        }),
    );
    if !tx.gas_price.is_zero()
        || tx.gas_token != address::ZERO
        || tx.refund_receiver != address::ZERO
    {
        s.field("safeTxGas", tx.safe_tx_gas.to_grouped_decimal());
        s.field("baseGas", tx.base_gas.to_grouped_decimal());
        s.field("gasPrice", tx.gas_price.to_grouped_decimal());
        s.field("gasToken", address::display(&tx.gas_token));
        s.field("refundReceiver", address::display(&tx.refund_receiver));
    }
    review.sections.push(s);

    if matches!(version, DomainVersion::Legacy) {
        review.find(
            Severity::Critical,
            "SIGNATURE_NOT_CHAIN_BOUND",
            String::from(
                "The v1.1.x Safe domain does not include the chain ID, so this signature is valid \
                 for the same Safe address on every chain it exists on, not only this one. Anyone \
                 holding it can replay it elsewhere while the nonce still matches.",
            ),
        );
    }

    calls::refund_finding(&mut review, tx.gas_price, tx.gas_token, tx.refund_receiver);

    calls::review_call(
        &mut review,
        "Call executed by the Safe",
        tx.to,
        tx.value,
        &tx.data,
        CallContext {
            safe: Some(tx.safe),
            operation: Operation::from_u8(tx.operation),
            chain_id: tx.chain_id.to_u64(),
            depth: 1,
        },
    );

    let safe_tx_hash = safe_transaction_hash(tx, version);
    review.set_signing_target(crate::review::SigningTarget {
        kind: crate::review::TargetKind::SafeTransaction,
        digest: safe_tx_hash,
    });
    review.digests.push((
        String::from("Safe transaction hash"),
        hex::encode_prefixed(&safe_transaction_hash(tx, version)),
    ));
    review.digests.push((
        String::from("Domain separator"),
        hex::encode_prefixed(&domain_separator(tx, version)),
    ));
    review.digests.push((
        String::from("Message hash"),
        hex::encode_prefixed(&struct_hash(tx)),
    ));
    review.find(
        Severity::Info,
        "COMPARE_SAFE_TX_HASH",
        String::from(
            "Before signing, confirm the Safe transaction hash matches what every other signer sees on their own \
             device, over a channel the preparing computer does not control.",
        ),
    );
    review
}
