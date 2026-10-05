//! Differential property tests for what was added on 5 October 2026: the
//! transaction types that used to be refused (EIP-2930, EIP-4844, EIP-7702),
//! and the administration and permission calls the decoder now reads.
//!
//! Same two directions as `alloy.rs`: agreement on valid encodings, and never
//! accepting calldata as canonical when alloy's validating decoder would not
//! re-encode it to the exact same bytes.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use alloy_consensus::{SignableTransaction, TxEip2930, TxEip4844, TxEip7702};
use alloy_eips::eip2930::{AccessList, AccessListItem};
use alloy_eips::eip7702::{Authorization, SignedAuthorization};
use alloy_primitives::{
    Address, B256, Bytes, FixedBytes, TxKind, U256 as AU256, aliases::U48, aliases::U160,
};
use alloy_sol_types::{SolCall, sol};
use clearsign::{DomainVersion, SafeTransaction, Severity, TxType, U256};
use proptest::prelude::*;

sol! {
    function transferOwnership(address newOwner);
    function grantRole(bytes32 role, address account);
    function revokeRole(bytes32 role, address account);
    function upgradeTo(address implementation);
    function upgradeToAndCall(address implementation, bytes data);
    function upgradeAndCall(address proxy, address implementation, bytes data);
    function changeProxyAdmin(address proxy, address newAdmin);
    function increaseAllowance(address spender, uint256 addedValue);
    function setApprovalForAll(address operator, bool approved);
    function approve(address token, address spender, uint160 amount, uint48 expiration);
    function multicall(bytes[] data);
    function schedule(address target, uint256 value, bytes data, bytes32 predecessor, bytes32 salt, uint256 delay);
    function execute(address target, uint256 value, bytes payload, bytes32 predecessor, bytes32 salt);
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
            proptest::collection::vec(any::<[u8; 32]>(), 0..3),
        ),
        0..3,
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

fn arb_blob_hashes() -> impl Strategy<Value = Vec<B256>> {
    proptest::collection::vec(any::<[u8; 31]>(), 1..7).prop_map(|hs| {
        hs.into_iter()
            .map(|tail| {
                let mut h = [0u8; 32];
                h[0] = 0x01;
                h[1..].copy_from_slice(&tail);
                B256::from(h)
            })
            .collect()
    })
}

fn arb_authorizations() -> impl Strategy<Value = Vec<SignedAuthorization>> {
    proptest::collection::vec(
        (
            arb_u256(),
            arb_address(),
            any::<u64>(),
            0u8..2,
            arb_u256(),
            arb_u256(),
        ),
        1..5,
    )
    .prop_map(|tuples| {
        tuples
            .into_iter()
            .map(|(chain_id, address, nonce, y, r, s)| {
                SignedAuthorization::new_unchecked(
                    Authorization {
                        chain_id,
                        address,
                        nonce,
                    },
                    y,
                    r,
                    s,
                )
            })
            .collect()
    })
}

fn safe_call(to: [u8; 20], data: Vec<u8>) -> SafeTransaction {
    SafeTransaction {
        chain_id: U256::from_u64(1),
        safe: [0x11; 20],
        to,
        value: U256::ZERO,
        data,
        operation: 0,
        safe_tx_gas: U256::ZERO,
        base_gas: U256::ZERO,
        gas_price: U256::ZERO,
        gas_token: [0; 20],
        refund_receiver: [0; 20],
        nonce: U256::ZERO,
    }
}

fn review(to: [u8; 20], data: Vec<u8>) -> clearsign::Review {
    clearsign::review_safe_transaction(&safe_call(to, data), DomainVersion::V1_3Plus)
}

proptest! {
    #![proptest_config(ProptestConfig { cases: 2000, .. ProptestConfig::default() })]

    #[test]
    fn eip2930_matches_alloy(
        chain_id in any::<u64>(), nonce in any::<u64>(), gas_price in any::<u128>(), gas_limit in any::<u64>(),
        to in arb_kind(), value in arb_u256(), input in arb_bytes(300), access_list in arb_access_list(),
    ) {
        let tx = TxEip2930 { chain_id, nonce, gas_price, gas_limit, to, value, access_list: access_list.clone(), input: Bytes::from(input.clone()) };
        let bytes = tx.encoded_for_signing();
        let parsed = clearsign::parse_unsigned_transaction(&bytes).unwrap();
        prop_assert_eq!(parsed.tx_type, TxType::Eip2930);
        prop_assert_eq!(parsed.chain_id, Some(U256::from_u64(chain_id)));
        prop_assert_eq!(parsed.nonce, U256::from_u64(nonce));
        prop_assert_eq!(parsed.gas_price, Some(cu(AU256::from(gas_price))));
        prop_assert_eq!(parsed.value, cu(value));
        prop_assert_eq!(&parsed.data, &input);
        prop_assert_eq!(parsed.access_list_entries, access_list.0.len());
        prop_assert_eq!(parsed.signing_hash, tx.signature_hash().0);
        let r = clearsign::review_transaction_bytes(&bytes).unwrap();
        prop_assert_eq!(r.signing_target().unwrap().digest, tx.signature_hash().0);
    }

    #[test]
    fn eip4844_matches_alloy(
        chain_id in any::<u64>(), nonce in any::<u64>(), gas_limit in any::<u64>(),
        max_fee in any::<u128>(), prio in any::<u128>(), blob_fee in any::<u128>(), to in arb_address(),
        value in arb_u256(), input in arb_bytes(200), access_list in arb_access_list(), hashes in arb_blob_hashes(),
    ) {
        let tx = TxEip4844 {
            chain_id, nonce, gas_limit, max_fee_per_gas: max_fee, max_priority_fee_per_gas: prio, to, value,
            access_list, blob_versioned_hashes: hashes.clone(), max_fee_per_blob_gas: blob_fee,
            input: Bytes::from(input.clone()),
        };
        let bytes = tx.encoded_for_signing();
        let parsed = clearsign::parse_unsigned_transaction(&bytes).unwrap();
        prop_assert_eq!(parsed.tx_type, TxType::Eip4844);
        prop_assert_eq!(parsed.to, Some(to.into_array()));
        prop_assert_eq!(parsed.max_fee_per_blob_gas, Some(cu(AU256::from(blob_fee))));
        prop_assert_eq!(parsed.blob_versioned_hashes.len(), hashes.len());
        for (a, b) in parsed.blob_versioned_hashes.iter().zip(hashes.iter()) {
            prop_assert_eq!(a, &b.0);
        }
        prop_assert_eq!(parsed.signing_hash, tx.signature_hash().0);
        let r = clearsign::review_transaction_bytes(&bytes).unwrap();
        prop_assert!(r.has("BLOB_TRANSACTION"));
        prop_assert_eq!(r.signing_target().unwrap().digest, tx.signature_hash().0);
    }

    #[test]
    fn eip7702_matches_alloy(
        chain_id in any::<u64>(), nonce in any::<u64>(), gas_limit in any::<u64>(),
        max_fee in any::<u128>(), prio in any::<u128>(), to in arb_address(), value in arb_u256(),
        input in arb_bytes(200), access_list in arb_access_list(), auths in arb_authorizations(),
    ) {
        let tx = TxEip7702 {
            chain_id, nonce, gas_limit, max_fee_per_gas: max_fee, max_priority_fee_per_gas: prio, to, value,
            access_list, authorization_list: auths.clone(), input: Bytes::from(input.clone()),
        };
        let bytes = tx.encoded_for_signing();
        let parsed = clearsign::parse_unsigned_transaction(&bytes).unwrap();
        prop_assert_eq!(parsed.tx_type, TxType::Eip7702);
        prop_assert_eq!(parsed.authorizations.len(), auths.len());
        for (a, b) in parsed.authorizations.iter().zip(auths.iter()) {
            prop_assert_eq!(a.chain_id, cu(b.inner().chain_id));
            prop_assert_eq!(a.address, b.inner().address.into_array());
            prop_assert_eq!(a.nonce, U256::from_u64(b.inner().nonce));
            prop_assert_eq!(a.y_parity, b.y_parity());
            prop_assert_eq!(a.r, cu(b.r()));
            prop_assert_eq!(a.s, cu(b.s()));
        }
        prop_assert_eq!(parsed.signing_hash, tx.signature_hash().0);

        let r = clearsign::review_transaction_bytes(&bytes).unwrap();
        prop_assert_eq!(r.signing_target().unwrap().digest, tx.signature_hash().0);
        // Every delegation to real code is CRITICAL; every one to zero clears.
        let delegating = auths.iter().filter(|a| a.inner().address != Address::ZERO).count();
        let critical = r.findings().iter().filter(|f| f.code == "ACCOUNT_DELEGATION").count();
        prop_assert_eq!(critical, delegating);
        if delegating > 0 {
            prop_assert_eq!(r.highest_severity(), Some(Severity::Critical));
        }
    }

    /// The administration calls decode to what alloy encoded, at the right severity.
    #[test]
    fn administration_calls_match_alloy(
        target in any::<[u8; 20]>(), who in arb_address(), other in arb_address(), role in any::<[u8; 32]>(),
        init in arb_bytes(120),
    ) {
        let r = review(target, transferOwnershipCall { newOwner: who }.abi_encode());
        prop_assert!(r.has("OWNERSHIP_TRANSFER"));
        prop_assert!(r.render().contains(&clearsign::address::display(&who.into_array())));

        let r = review(target, grantRoleCall { role: FixedBytes(role), account: who }.abi_encode());
        prop_assert_eq!(r.findings().iter().find(|f| f.code == "ROLE_GRANT").unwrap().severity, Severity::Critical);

        let r = review(target, revokeRoleCall { role: FixedBytes(role), account: who }.abi_encode());
        prop_assert!(r.has("ROLE_REVOKE"));

        let r = review(target, upgradeToCall { implementation: who }.abi_encode());
        prop_assert!(r.has("PROXY_UPGRADE"));

        let r = review(target, upgradeToAndCallCall { implementation: who, data: Bytes::from(init.clone()) }.abi_encode());
        prop_assert!(r.has("PROXY_UPGRADE"));
        prop_assert_eq!(r.has("UPGRADE_CALL_NOT_DECODED"), !init.is_empty());

        let r = review(target, upgradeAndCallCall { proxy: who, implementation: other, data: Bytes::from(init.clone()) }.abi_encode());
        prop_assert!(r.has("PROXY_UPGRADE"));
        prop_assert!(!r.has("MALFORMED_ARGUMENTS"));

        let r = review(target, changeProxyAdminCall { proxy: who, newAdmin: other }.abi_encode());
        prop_assert!(r.has("PROXY_ADMIN_CHANGE"));
    }

    #[test]
    fn permission_calls_match_alloy(
        target in any::<[u8; 20]>(), who in arb_address(), amount in arb_u256(), approved in any::<bool>(),
        p2_amount in any::<[u8; 20]>(), expiry in any::<[u8; 6]>(),
    ) {
        let r = review(target, increaseAllowanceCall { spender: who, addedValue: amount }.abi_encode());
        let unlimited = cu(amount).is_max() || cu(amount).is_effectively_unlimited();
        prop_assert_eq!(r.has("UNLIMITED_APPROVAL"), unlimited);
        prop_assert_eq!(r.has("TOKEN_APPROVAL"), !unlimited && !amount.is_zero());

        let r = review(target, setApprovalForAllCall { operator: who, approved }.abi_encode());
        prop_assert_eq!(r.has("APPROVAL_FOR_ALL"), approved);

        let amount160 = U160::from_be_bytes(p2_amount);
        let exp48 = U48::from_be_bytes(expiry);
        let r = review(target, approveCall { token: who, spender: who, amount: amount160, expiration: exp48 }.abi_encode());
        prop_assert!(!r.has("MALFORMED_ARGUMENTS"));
        // Unlimited from 2^144 up, as for ERC-20 near its maximum: no one
        // chooses an amount that size for its value.
        let threshold = U160::from(1u8) << 144usize;
        prop_assert_eq!(r.has("UNLIMITED_APPROVAL"), amount160 >= threshold);
    }

    /// Whatever multicall elements alloy encodes, clearsign reads the same
    /// number of carried calls and never calls a canonical encoding malformed.
    #[test]
    fn multicall_matches_alloy(elements in proptest::collection::vec(arb_bytes(90), 0..40), target in any::<[u8; 20]>()) {
        let data = multicallCall { data: elements.iter().cloned().map(Bytes::from).collect() }.abi_encode();
        let r = review(target, data);
        prop_assert!(!r.has("MALFORMED_ARGUMENTS"), "{}", r.render());
        let shown = r.sections.iter().filter(|s| s.title.starts_with("Call bundled by multicall")).count();
        prop_assert_eq!(shown, elements.len().min(32));
    }

    /// Never more permissive: when clearsign accepts mutated calldata for one of
    /// the new carrying or upgrade calls, alloy must accept it and re-encode it
    /// to the same bytes.
    #[test]
    fn mutated_carrying_calls_are_never_accepted_when_alloy_would_not(
        elements in proptest::collection::vec(arb_bytes(70), 1..5),
        flips in proptest::collection::vec((any::<usize>(), any::<u8>()), 0..4),
        cut in any::<usize>(), extend in arb_bytes(40), which in 0u8..4,
    ) {
        let payload = Bytes::from(elements[0].clone());
        let mut data = match which {
            0 => multicallCall { data: elements.iter().cloned().map(Bytes::from).collect() }.abi_encode(),
            1 => upgradeToAndCallCall { implementation: Address::repeat_byte(3), data: payload }.abi_encode(),
            2 => scheduleCall { target: Address::repeat_byte(4), value: AU256::from(1), data: payload, predecessor: B256::ZERO, salt: B256::repeat_byte(7), delay: AU256::from(60) }.abi_encode(),
            _ => executeCall { target: Address::repeat_byte(4), value: AU256::ZERO, payload, predecessor: B256::ZERO, salt: B256::repeat_byte(7) }.abi_encode(),
        };
        for (pos, byte) in flips {
            if data.len() > 4 {
                let i = 4 + pos % (data.len() - 4);
                data[i] ^= byte;
            }
        }
        match cut % 4 {
            0 => { let n = 4 + cut % (data.len() - 3); data.truncate(n); }
            1 => data.extend_from_slice(&extend),
            _ => {}
        }
        let r = review([0x55; 20], data.clone());
        let accepted = !r.has("MALFORMED_ARGUMENTS") && !r.has("UNKNOWN_SELECTOR") && !r.has("TOO_MANY_CARRIED_CALLS");
        if accepted {
            let reencoded = match which {
                0 => multicallCall::abi_decode_validate(&data).map(|c| c.abi_encode()),
                1 => upgradeToAndCallCall::abi_decode_validate(&data).map(|c| c.abi_encode()),
                2 => scheduleCall::abi_decode_validate(&data).map(|c| c.abi_encode()),
                _ => executeCall::abi_decode_validate(&data).map(|c| c.abi_encode()),
            };
            prop_assert!(reencoded.is_ok(), "clearsign accepted what alloy rejects: {}", r.render());
            prop_assert_eq!(reencoded.unwrap(), data);
        }
    }
}

/// Found by mutation testing, 5 Oct 2026: the blob-hash limit, at its boundary.
/// A list of exactly `MAX_BLOB_HASHES` is read, one more is refused, and the
/// limit is checked before any hash is.
#[test]
fn blob_hash_lists_are_read_up_to_the_limit_and_no_further() {
    let tx = |n: usize| TxEip4844 {
        chain_id: 1,
        nonce: 0,
        gas_limit: 21_000,
        max_fee_per_gas: 1,
        max_priority_fee_per_gas: 1,
        to: Address::repeat_byte(9),
        value: AU256::ZERO,
        access_list: AccessList::default(),
        blob_versioned_hashes: (0..n)
            .map(|i| {
                let mut h = [0u8; 32];
                h[0] = 0x01;
                h[31] = i as u8;
                B256::from(h)
            })
            .collect(),
        max_fee_per_blob_gas: 1,
        input: Bytes::new(),
    };
    let limit = clearsign::evm::MAX_BLOB_HASHES;
    let at = tx(limit);
    let parsed = clearsign::parse_unsigned_transaction(&at.encoded_for_signing()).unwrap();
    assert_eq!(parsed.blob_versioned_hashes.len(), limit);
    assert_eq!(parsed.signing_hash, at.signature_hash().0);
    assert_eq!(
        clearsign::parse_unsigned_transaction(&tx(limit + 1).encoded_for_signing()).map(|_| ()),
        Err(clearsign::Error::TooDeep)
    );
}

/// Found by mutation testing, 5 Oct 2026: the authorization limit, at its
/// boundary, the same way.
#[test]
fn authorization_lists_are_read_up_to_the_limit_and_no_further() {
    let tx = |n: usize| TxEip7702 {
        chain_id: 1,
        nonce: 0,
        gas_limit: 50_000,
        max_fee_per_gas: 1,
        max_priority_fee_per_gas: 1,
        to: Address::repeat_byte(9),
        value: AU256::ZERO,
        access_list: AccessList::default(),
        authorization_list: (0..n)
            .map(|i| {
                SignedAuthorization::new_unchecked(
                    Authorization {
                        chain_id: AU256::from(1),
                        address: Address::repeat_byte(7),
                        nonce: i as u64,
                    },
                    0,
                    AU256::from(1),
                    AU256::from(1),
                )
            })
            .collect(),
        input: Bytes::new(),
    };
    let limit = clearsign::evm::MAX_AUTHORIZATIONS;
    let at = tx(limit);
    let parsed = clearsign::parse_unsigned_transaction(&at.encoded_for_signing()).unwrap();
    assert_eq!(parsed.authorizations.len(), limit);
    assert_eq!(parsed.signing_hash, at.signature_hash().0);
    assert_eq!(
        clearsign::parse_unsigned_transaction(&tx(limit + 1).encoded_for_signing()).map(|_| ()),
        Err(clearsign::Error::TooDeep)
    );
}
