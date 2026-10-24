//! Differential tests against an independent implementation.
//!
//! Every expected value in this file was produced by Foundry's `cast` 1.7.1
//! (`cast sig`, `cast calldata`, `cast abi-encode`, `cast keccak`, `cast to-rlp`,
//! `cast to-check-sum-address`), not by clearsign. The commands are reproduced
//! in `tests/vectors/README.md`. If clearsign and cast disagree, a test fails.
//!
//! The Safe constants were additionally confirmed against the Safe contract
//! source at tags v1.1.1, v1.3.0 and v1.4.1.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use clearsign::address::checksummed;
use clearsign::calls::KNOWN_SIGNATURES;
use clearsign::keccak::{keccak256, selector};
use clearsign::safe::{
    DOMAIN_TYPE_LEGACY, DOMAIN_TYPE_V1_3, SAFE_TX_TYPE, domain_separator, struct_hash,
};
use clearsign::{DomainVersion, SafeTransaction, Severity, TxType, U256};
use clearsign::{
    hex, parse_unsigned_transaction, review_evm_transaction, review_safe_transaction,
    safe_transaction_hash,
};

fn addr(s: &str) -> [u8; 20] {
    hex::decode(s).unwrap().try_into().unwrap()
}

fn h(bytes: &[u8]) -> String {
    hex::encode_prefixed(bytes)
}

const SAFE: &str = "0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4";
const USDC: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const ZERO: [u8; 20] = [0u8; 20];

#[test]
fn selectors_match_cast_sig() {
    // (selector from `cast sig`, signature)
    let from_cast = [
        ("0xa9059cbb", "transfer(address,uint256)"),
        ("0x095ea7b3", "approve(address,uint256)"),
        ("0x23b872dd", "transferFrom(address,address,uint256)"),
        (
            "0x6a761202",
            "execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)",
        ),
        ("0x0d582f13", "addOwnerWithThreshold(address,uint256)"),
        ("0xf8dc5dd9", "removeOwner(address,address,uint256)"),
        ("0xe318b52b", "swapOwner(address,address,address)"),
        ("0x694e80c3", "changeThreshold(uint256)"),
        ("0x610b5925", "enableModule(address)"),
        ("0xe009cfde", "disableModule(address,address)"),
        ("0xe19a9dd9", "setGuard(address)"),
        ("0xf08a0323", "setFallbackHandler(address)"),
        ("0x7de7edef", "changeMasterCopy(address)"),
        ("0x8d80ff0a", "multiSend(bytes)"),
        // Added 5 Oct 2026, each printed by `cast sig` (Foundry 1.7.1).
        ("0x39509351", "increaseAllowance(address,uint256)"),
        ("0xa457c2d7", "decreaseAllowance(address,uint256)"),
        ("0xa22cb465", "setApprovalForAll(address,bool)"),
        ("0x87517c45", "approve(address,address,uint160,uint48)"),
        ("0xf2fde38b", "transferOwnership(address)"),
        ("0x715018a6", "renounceOwnership()"),
        ("0x79ba5097", "acceptOwnership()"),
        ("0x2f2ff15d", "grantRole(bytes32,address)"),
        ("0xd547741f", "revokeRole(bytes32,address)"),
        ("0x36568abe", "renounceRole(bytes32,address)"),
        ("0x3659cfe6", "upgradeTo(address)"),
        ("0x4f1ef286", "upgradeToAndCall(address,bytes)"),
        ("0x8f283970", "changeAdmin(address)"),
        ("0x99a88ec4", "upgrade(address,address)"),
        ("0x9623609d", "upgradeAndCall(address,address,bytes)"),
        ("0x7eff275e", "changeProxyAdmin(address,address)"),
        ("0xac9650d8", "multicall(bytes[])"),
        (
            "0x01d5062a",
            "schedule(address,uint256,bytes,bytes32,bytes32,uint256)",
        ),
        (
            "0x134008d3",
            "execute(address,uint256,bytes,bytes32,bytes32)",
        ),
        ("0x13af4035", "setOwner(address)"),
        ("0xd73dd623", "increaseApproval(address,uint256)"),
        ("0x66188463", "decreaseApproval(address,uint256)"),
        ("0xe9ae5c53", "execute(bytes32,bytes)"),
    ];
    assert_eq!(from_cast.len(), KNOWN_SIGNATURES.len());
    for (expected, sig) in from_cast {
        assert_eq!(h(&selector(sig)), expected, "keccak selector for {sig}");
        let (hardcoded, _) = KNOWN_SIGNATURES.iter().find(|(_, s)| *s == sig).unwrap();
        assert_eq!(h(hardcoded), expected, "hardcoded selector for {sig}");
    }
}

#[test]
fn typehashes_match_safe_contract_source() {
    assert_eq!(
        h(&keccak256(SAFE_TX_TYPE.as_bytes())),
        "0xbb8310d486368db6bd6f849402fdd73ad53d316b5a4b2644ad6efe0f941286d8"
    );
    assert_eq!(
        h(&keccak256(DOMAIN_TYPE_V1_3.as_bytes())),
        "0x47e79534a245952e8b16893a336b85a3d9ea9fa8c573f3d803afb92a79469218"
    );
    assert_eq!(
        h(&keccak256(DOMAIN_TYPE_LEGACY.as_bytes())),
        "0x035aff83d86937d35b32e04f0ddc6ff469290eef2f1b692d8a815c89404d4749"
    );
}

#[test]
fn checksums_match_cast() {
    assert_eq!(checksummed(&addr(SAFE)), SAFE);
    assert_eq!(checksummed(&addr(USDC)), USDC);
    assert_eq!(
        checksummed(&addr("0x00000000000000000000000000000000deadbeef")),
        "0x00000000000000000000000000000000DeaDBeef"
    );
}

fn safe_tx(chain: u64, to: &str, data: &str, operation: u8, nonce: u64) -> SafeTransaction {
    SafeTransaction {
        chain_id: U256::from_u64(chain),
        safe: addr(SAFE),
        to: addr(to),
        value: U256::ZERO,
        data: hex::decode(data).unwrap(),
        operation,
        safe_tx_gas: U256::ZERO,
        base_gas: U256::ZERO,
        gas_price: U256::ZERO,
        gas_token: ZERO,
        refund_receiver: ZERO,
        nonce: U256::from_u64(nonce),
    }
}

/// Vector A: an ordinary USDC transfer executed by a Safe on chain 1.
#[test]
fn safe_vector_a_plain_transfer() {
    let tx = safe_tx(
        1,
        USDC,
        "0xa9059cbb00000000000000000000000070997970c51812dc3a010c7d01b50e0d17dc79c8000000000000000000000000000000000000000000000000000000003b9aca00",
        0,
        42,
    );
    let v = DomainVersion::V1_3Plus;
    assert_eq!(
        h(&domain_separator(&tx, v)),
        "0x3abafb4fc69bc9effe736580564843cff706e076fa459a6490d7e3a1d5509f12"
    );
    assert_eq!(
        h(&struct_hash(&tx)),
        "0x2a559042b9548671c6bd4bc557b5cd035ccf7e98c6c111933c96b7725eaf694e"
    );
    assert_eq!(
        h(&safe_transaction_hash(&tx, v)),
        "0x62a251bfb5da9aacc458c4856329e7b78a88690d2407f4d16c2658ecff90f5c7"
    );

    let review = review_safe_transaction(&tx, v);
    assert!(
        review.highest_severity() < Some(Severity::Blind),
        "{}",
        review.render()
    );
    let text = review.render();
    assert!(text.contains("ERC-20 transfer"));
    assert!(text.contains("1_000_000_000"));
}

/// Vector B: modelled on the February 2025 Bybit pattern. A DELEGATECALL whose
/// calldata carries an innocent-looking `transfer` selector. The target address
/// is synthetic.
#[test]
fn safe_vector_b_delegatecall_disguised_as_transfer() {
    let tx = SafeTransaction {
        safe_tx_gas: U256::from_u64(45_746),
        ..safe_tx(
            1,
            "0x00000000000000000000000000000000DeaDBeef",
            "0xa9059cbb000000000000000000000000000000000000000000000000000000000000dead0000000000000000000000000000000000000000000000000000000000000000",
            1,
            71,
        )
    };
    let v = DomainVersion::V1_3Plus;
    assert_eq!(
        h(&struct_hash(&tx)),
        "0xc88d99df05108d4d76d4f3028a1cbc5f8417f34bf0ec3227ffe5039b89f12412"
    );
    assert_eq!(
        h(&safe_transaction_hash(&tx, v)),
        "0xa62b640da6d5c542052b0aa17fd0a8a549d0a02b48e2e5a0ec06aba49b47df2d"
    );

    let review = review_safe_transaction(&tx, v);
    assert_eq!(review.highest_severity(), Some(Severity::Critical));
    assert!(review.has("SAFE_DELEGATECALL"));
    let text = review.render();
    // INV-1 in spirit: the misleading function name must not be presented as a description.
    assert!(
        !text.contains("ERC-20 transfer"),
        "delegatecall must not be summarised as a transfer:\n{text}"
    );
    assert!(text.contains("DO NOT SIGN"));
}

/// Vector C: legacy (v1.1.x) domain, Safe adds an owner, with a gas refund, chain 100.
#[test]
fn safe_vector_c_legacy_domain_owner_change_with_refund() {
    let tx = SafeTransaction {
        safe_tx_gas: U256::from_u64(50_000),
        base_gas: U256::from_u64(21_000),
        gas_price: U256::from_u64(1_000_000_000),
        ..safe_tx(
            100,
            SAFE,
            "0x0d582f13000000000000000000000000000000000000000000000000000000000000dead0000000000000000000000000000000000000000000000000000000000000002",
            0,
            3,
        )
    };
    let v = DomainVersion::Legacy;
    assert_eq!(
        h(&domain_separator(&tx, v)),
        "0xb3ded2bdbff5db1a87f6d551fa256e9f2bd6517a3bb84f4c2ea863fb3a559622"
    );
    assert_eq!(
        h(&struct_hash(&tx)),
        "0xdc1fc7fc18a431f145eb32c3fe05f65386afe9e46002179c5ef7f4d880e59724"
    );
    assert_eq!(
        h(&safe_transaction_hash(&tx, v)),
        "0x02356044af5c03895a23b92c7b1abd284b0f00f5bc9ed76a4ad6cd2c837fa917"
    );

    let review = review_safe_transaction(&tx, v);
    assert!(review.has("SAFE_OWNER_CHANGE"));
    assert!(review.has("SAFE_GAS_REFUND"));
    assert_eq!(review.highest_severity(), Some(Severity::Critical));
}

/// Vector D: EIP-1559 unlimited approval, RLP built by `cast to-rlp`.
#[test]
fn evm_vector_d_unlimited_approval() {
    let bytes = hex::decode("0x02f86e0107843b9aca008506fc23ac00830186a094a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4880b844095ea7b30000000000000000000000003fc91a3afd70395cd496c647d5a6cc9d4b2b7fadffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0").unwrap();
    let tx = parse_unsigned_transaction(&bytes).unwrap();
    assert_eq!(tx.tx_type, TxType::Eip1559);
    assert_eq!(
        h(&tx.signing_hash),
        "0x09178b7896b44cc0af679dd664e3fd99f3cc46316779d3fd773366f49652d431"
    );
    assert_eq!(tx.chain_id, Some(U256::from_u64(1)));
    assert_eq!(tx.nonce, U256::from_u64(7));
    assert_eq!(
        tx.max_priority_fee_per_gas,
        Some(U256::from_u64(1_000_000_000))
    );
    assert_eq!(tx.max_fee_per_gas, Some(U256::from_u64(30_000_000_000)));
    assert_eq!(tx.gas_limit, U256::from_u64(100_000));
    assert_eq!(tx.to, Some(addr(USDC)));
    assert!(tx.value.is_zero());

    let review = review_evm_transaction(&tx);
    assert!(review.has("UNLIMITED_APPROVAL"));
    assert_eq!(review.highest_severity(), Some(Severity::Critical));
}

/// Vector E: EIP-1559 on chain 137 submitting a Safe execTransaction with two
/// 65-byte signatures and a one-entry access list.
#[test]
fn evm_vector_e_exec_transaction_submission() {
    // Exact output of `cast to-rlp`, prefixed with 0x02. Do not hand-edit.
    let bytes = hex::decode("0x02f902e9818980843b9aca008506fc23ac0083030d40941db92e2eebc8e0c075a02bea49a2935bcd2dfcf480b902846a761202000000000000000000000000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb480000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000014000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001c00000000000000000000000000000000000000000000000000000000000000044a9059cbb00000000000000000000000070997970c51812dc3a010c7d01b50e0d17dc79c8000000000000000000000000000000000000000000000000000000003b9aca00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000082abababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababababab000000000000000000000000000000000000000000000000000000000000f838f794a0b86991c6218b36c1d19d4a2e9eb0ce3606eb48e1a00000000000000000000000000000000000000000000000000000000000000003").unwrap();
    let tx = parse_unsigned_transaction(&bytes).unwrap();
    assert_eq!(
        h(&tx.signing_hash),
        "0x9816b5f2df72ba29ffd01cd9dd7705db3c47cc1b79112b5221c6cac54f46ef46"
    );
    assert_eq!(tx.chain_id, Some(U256::from_u64(137)));
    assert!(tx.nonce.is_zero());
    assert_eq!(tx.access_list_entries, 1);

    let review = review_evm_transaction(&tx);
    let text = review.render();
    assert!(review.has("SAFE_EXEC_SUBMISSION"), "{text}");
    assert!(text.contains("Execute a Safe transaction"));
    assert!(
        text.contains("ERC-20 transfer"),
        "inner CALL should be decoded:\n{text}"
    );
    assert!(text.contains("130 bytes (2 owner signatures"));
}
