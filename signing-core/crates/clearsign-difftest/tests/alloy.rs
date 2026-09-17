//! Differential property tests: clearsign against alloy, an independent and widely
//! deployed Rust Ethereum implementation. Thousands of random cases per property.
//!
//! Two directions matter:
//! - **Agreement**: for valid encodings, both implementations extract the same
//!   fields and compute the same hashes.
//! - **Never more permissive**: whenever clearsign accepts calldata as canonical,
//!   alloy's validating decoder must accept it too and re-encode it to the exact
//!   same bytes. Otherwise clearsign could display something the EVM would reject
//!   or interpret differently.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use alloy_consensus::{SignableTransaction, TxEip1559, TxLegacy};
use alloy_eips::eip2930::{AccessList, AccessListItem};
use alloy_primitives::{Address, B256, Bytes, TxKind, U256 as AU256};
use alloy_sol_types::{Eip712Domain, SolCall, SolStruct, sol};
use clearsign::{DomainVersion, SafeTransaction, Severity, TxType, U256};
use proptest::prelude::*;

sol! {
    struct SafeTx {
        address to;
        uint256 value;
        bytes data;
        uint8 operation;
        uint256 safeTxGas;
        uint256 baseGas;
        uint256 gasPrice;
        address gasToken;
        address refundReceiver;
        uint256 nonce;
    }
    function transfer(address to, uint256 amount);
    function approve(address spender, uint256 amount);
    function transferFrom(address from, address to, uint256 amount);
    function execTransaction(
        address to,
        uint256 value,
        bytes data,
        uint8 operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address refundReceiver,
        bytes signatures
    );
}

fn cu(v: AU256) -> U256 {
    U256(v.to_be_bytes::<32>())
}

fn arb_address() -> impl Strategy<Value = Address> {
    prop_oneof![
        1 => Just(Address::ZERO),
        9 => any::<[u8; 20]>().prop_map(Address::from),
    ]
}

fn arb_u256() -> impl Strategy<Value = AU256> {
    prop_oneof![
        1 => Just(AU256::ZERO),
        1 => Just(AU256::MAX),
        4 => any::<u64>().prop_map(AU256::from),
        4 => any::<[u8; 32]>().prop_map(AU256::from_be_bytes),
    ]
}

fn arb_bytes(max: usize) -> impl Strategy<Value = Vec<u8>> {
    proptest::collection::vec(any::<u8>(), 0..max)
}

fn arb_access_list() -> impl Strategy<Value = AccessList> {
    proptest::collection::vec(
        (
            any::<[u8; 20]>(),
            proptest::collection::vec(any::<[u8; 32]>(), 0..4),
        ),
        0..4,
    )
    .prop_map(|items| {
        AccessList(
            items
                .into_iter()
                .map(|(a, keys)| AccessListItem {
                    address: Address::from(a),
                    storage_keys: keys.into_iter().map(B256::from).collect(),
                })
                .collect(),
        )
    })
}

fn arb_kind() -> impl Strategy<Value = TxKind> {
    prop_oneof![
        1 => Just(TxKind::Create),
        9 => arb_address().prop_map(TxKind::Call),
    ]
}

fn to_opt(kind: TxKind) -> Option<[u8; 20]> {
    match kind {
        TxKind::Create => None,
        TxKind::Call(a) => Some(a.into_array()),
    }
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 3000, .. ProptestConfig::default() })]

    /// Agreement on every EIP-1559 field and on the signing hash.
    #[test]
    fn eip1559_matches_alloy(
        chain_id in any::<u64>(), nonce in any::<u64>(), gas_limit in any::<u64>(),
        max_fee in any::<u128>(), prio in any::<u128>(), to in arb_kind(),
        value in arb_u256(), input in arb_bytes(400), access_list in arb_access_list(),
    ) {
        let tx = TxEip1559 {
            chain_id, nonce, gas_limit, max_fee_per_gas: max_fee, max_priority_fee_per_gas: prio,
            to, value, access_list: access_list.clone(), input: Bytes::from(input.clone()),
        };
        let bytes = tx.encoded_for_signing();
        let parsed = clearsign::parse_unsigned_transaction(&bytes).unwrap();
        prop_assert_eq!(parsed.tx_type, TxType::Eip1559);
        prop_assert_eq!(parsed.chain_id, Some(U256::from_u64(chain_id)));
        prop_assert_eq!(parsed.nonce, U256::from_u64(nonce));
        prop_assert_eq!(parsed.gas_limit, U256::from_u64(gas_limit));
        prop_assert_eq!(parsed.max_fee_per_gas, Some(cu(AU256::from(max_fee))));
        prop_assert_eq!(parsed.max_priority_fee_per_gas, Some(cu(AU256::from(prio))));
        prop_assert_eq!(parsed.to, to_opt(to));
        prop_assert_eq!(parsed.value, cu(value));
        prop_assert_eq!(&parsed.data, &input);
        prop_assert_eq!(parsed.access_list_entries, access_list.0.len());
        prop_assert_eq!(parsed.signing_hash, tx.signature_hash().0);

        let review = clearsign::review_transaction_bytes(&bytes).unwrap();
        prop_assert_eq!(review.signing_target().unwrap().digest, tx.signature_hash().0);
    }

    /// Agreement on legacy transactions, with and without EIP-155 chain IDs.
    #[test]
    fn legacy_matches_alloy(
        chain_id in proptest::option::of(any::<u64>()), nonce in any::<u64>(), gas_price in any::<u128>(),
        gas_limit in any::<u64>(), to in arb_kind(), value in arb_u256(), input in arb_bytes(300),
    ) {
        let tx = TxLegacy { chain_id, nonce, gas_price, gas_limit, to, value, input: Bytes::from(input.clone()) };
        let bytes = tx.encoded_for_signing();
        let parsed = clearsign::parse_unsigned_transaction(&bytes).unwrap();
        prop_assert_eq!(parsed.tx_type, if chain_id.is_some() { TxType::LegacyEip155 } else { TxType::Legacy });
        prop_assert_eq!(parsed.chain_id, chain_id.map(U256::from_u64));
        prop_assert_eq!(parsed.nonce, U256::from_u64(nonce));
        prop_assert_eq!(parsed.gas_price, Some(cu(AU256::from(gas_price))));
        prop_assert_eq!(parsed.to, to_opt(to));
        prop_assert_eq!(parsed.value, cu(value));
        prop_assert_eq!(&parsed.data, &input);
        prop_assert_eq!(parsed.signing_hash, tx.signature_hash().0);
    }

    /// The Safe transaction hash matches alloy's EIP-712 implementation, for both
    /// the v1.3.0+ domain and the v1.1.x domain.
    #[test]
    fn safe_tx_hash_matches_alloy_eip712(
        chain_id in any::<u64>(), safe in arb_address(), to in arb_address(), value in arb_u256(),
        data in arb_bytes(300), operation in any::<u8>(), safe_tx_gas in arb_u256(), base_gas in arb_u256(),
        gas_price in arb_u256(), gas_token in arb_address(), refund in arb_address(), nonce in arb_u256(),
    ) {
        let a = SafeTx {
            to, value, data: Bytes::from(data.clone()), operation, safeTxGas: safe_tx_gas, baseGas: base_gas,
            gasPrice: gas_price, gasToken: gas_token, refundReceiver: refund, nonce,
        };
        let c = SafeTransaction {
            chain_id: U256::from_u64(chain_id), safe: safe.into_array(), to: to.into_array(), value: cu(value),
            data, operation, safe_tx_gas: cu(safe_tx_gas), base_gas: cu(base_gas), gas_price: cu(gas_price),
            gas_token: gas_token.into_array(), refund_receiver: refund.into_array(), nonce: cu(nonce),
        };
        let v13 = Eip712Domain { chain_id: Some(AU256::from(chain_id)), verifying_contract: Some(safe), ..Default::default() };
        prop_assert_eq!(clearsign::safe_transaction_hash(&c, DomainVersion::V1_3Plus), a.eip712_signing_hash(&v13).0);
        let legacy = Eip712Domain { verifying_contract: Some(safe), ..Default::default() };
        prop_assert_eq!(clearsign::safe_transaction_hash(&c, DomainVersion::Legacy), a.eip712_signing_hash(&legacy).0);
        prop_assert_eq!(
            clearsign::review_safe_transaction(&c, DomainVersion::V1_3Plus).signing_target().unwrap().digest,
            a.eip712_signing_hash(&v13).0
        );
    }

    /// ERC-20 calls encoded by alloy are decoded, with the same values shown.
    #[test]
    fn erc20_calls_decode_like_alloy(which in 0u8..3, a1 in arb_address(), a2 in arb_address(), amount in arb_u256()) {
        let data = match which {
            0 => transferCall { to: a1, amount }.abi_encode(),
            1 => approveCall { spender: a1, amount }.abi_encode(),
            _ => transferFromCall { from: a1, to: a2, amount }.abi_encode(),
        };
        let tx = SafeTransaction {
            chain_id: U256::from_u64(1), safe: [0x11; 20], to: [0x22; 20], value: U256::ZERO, data,
            operation: 0, safe_tx_gas: U256::ZERO, base_gas: U256::ZERO, gas_price: U256::ZERO,
            gas_token: [0; 20], refund_receiver: [0; 20], nonce: U256::ZERO,
        };
        let review = clearsign::review_safe_transaction(&tx, DomainVersion::V1_3Plus);
        let text = review.render();
        prop_assert!(!review.has("MALFORMED_ARGUMENTS"), "{}", text);
        prop_assert!(!review.has("UNKNOWN_SELECTOR"), "{}", text);
        prop_assert!(text.contains(&clearsign::address::display(&a1.into_array())), "{}", text);
        if which == 1 && amount == AU256::MAX {
            prop_assert!(review.has("UNLIMITED_APPROVAL"));
        } else if which == 1 && amount == AU256::ZERO {
            prop_assert!(text.contains("0 (revokes approval)"));
        } else {
            prop_assert!(text.contains(&cu(amount).to_grouped_decimal()), "{}", text);
        }
    }

    /// execTransaction encoded by alloy is accepted and its inner call is handled
    /// according to the operation.
    #[test]
    fn exec_transaction_from_alloy_is_accepted(
        inner_to in arb_address(), recipient in arb_address(), amount in arb_u256(), operation in 0u8..2,
        gas_price in arb_u256(), signatures in arb_bytes(300), raw_inner in arb_bytes(200), use_raw in any::<bool>(),
    ) {
        let inner = if use_raw { raw_inner } else { transferCall { to: recipient, amount }.abi_encode() };
        let data = execTransactionCall {
            to: inner_to, value: AU256::ZERO, data: Bytes::from(inner), operation,
            safeTxGas: AU256::ZERO, baseGas: AU256::ZERO, gasPrice: gas_price,
            gasToken: Address::ZERO, refundReceiver: Address::ZERO, signatures: Bytes::from(signatures),
        }.abi_encode();
        let tx = TxEip1559 {
            chain_id: 1, nonce: 0, gas_limit: 500_000, max_fee_per_gas: 1, max_priority_fee_per_gas: 1,
            to: TxKind::Call(Address::repeat_byte(0x11)), value: AU256::ZERO, access_list: AccessList::default(),
            input: Bytes::from(data),
        };
        let review = clearsign::review_transaction_bytes(&tx.encoded_for_signing()).unwrap();
        let text = review.render();
        prop_assert!(!review.has("MALFORMED_ARGUMENTS"), "canonical alloy encoding refused:\n{}", text);
        prop_assert!(review.has("SAFE_EXEC_SUBMISSION"));
        if operation == 1 {
            prop_assert!(review.has("SAFE_DELEGATECALL"));
            prop_assert!(!text.contains("ERC-20 transfer"), "{}", text);
            prop_assert_eq!(review.highest_severity(), Some(Severity::Critical));
        }
    }

    /// Never more permissive than a strict decoder: mutate a canonical
    /// execTransaction, and whenever clearsign still treats it as well-formed,
    /// alloy's validating decoder must accept it and re-encode it byte-for-byte.
    #[test]
    fn clearsign_never_accepts_what_strict_abi_rejects(
        signatures in arb_bytes(140), inner in arb_bytes(140),
        mutations in proptest::collection::vec((any::<usize>(), any::<u8>()), 1..4),
        truncate in proptest::option::of(any::<usize>()), extend in arb_bytes(40),
    ) {
        let mut data = execTransactionCall {
            to: Address::repeat_byte(0x22), value: AU256::ZERO, data: Bytes::from(inner), operation: 0,
            safeTxGas: AU256::ZERO, baseGas: AU256::ZERO, gasPrice: AU256::ZERO,
            gasToken: Address::ZERO, refundReceiver: Address::ZERO, signatures: Bytes::from(signatures),
        }.abi_encode();
        for (pos, byte) in mutations {
            // Leave the selector intact so the execTransaction decoder is exercised.
            let i = 4 + pos % (data.len() - 4);
            data[i] = byte;
        }
        if let Some(t) = truncate { data.truncate(4 + t % (data.len() - 3)); }
        data.extend_from_slice(&extend);

        let tx = SafeTransaction {
            chain_id: U256::from_u64(1), safe: [0x11; 20], to: [0x33; 20], value: U256::ZERO, data: data.clone(),
            operation: 0, safe_tx_gas: U256::ZERO, base_gas: U256::ZERO, gas_price: U256::ZERO,
            gas_token: [0; 20], refund_receiver: [0; 20], nonce: U256::ZERO,
        };
        let review = clearsign::review_safe_transaction(&tx, DomainVersion::V1_3Plus);
        if !review.has("MALFORMED_ARGUMENTS") {
            let decoded = execTransactionCall::abi_decode_validate(&data);
            prop_assert!(decoded.is_ok(), "clearsign accepted calldata alloy rejects: 0x{}", clearsign::hex::encode(&data));
            prop_assert_eq!(decoded.unwrap().abi_encode(), data, "clearsign accepted a non-canonical encoding");
        }
    }
}
