//! Exact EIP-7702 authorization vectors and review-boundary regressions.
//!
//! The signatures and unsigned transaction below were produced independently
//! with Foundry `cast` 1.7.1 using private key 1. The key is a public test key.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use clearsign::{
    Authorization, AuthorizationSignatureError, Error, EvmTransaction, Severity, TxType, U256, hex,
    parse_unsigned_transaction, review_evm_transaction, review_transaction_bytes,
};

const AUTHORITY: [u8; 20] = [
    0x7e, 0x5f, 0x45, 0x52, 0x09, 0x1a, 0x69, 0x12, 0x5d, 0x5d, 0xfc, 0xb7, 0xb8, 0xc2, 0x65, 0x90,
    0x29, 0x39, 0x5b, 0xdf,
];

fn u256(value: &str) -> U256 {
    U256::from_be_slice(&hex::decode(value).unwrap()).unwrap()
}

fn chain_two_clear() -> Authorization {
    Authorization {
        chain_id: U256::from_u64(2),
        address: [0; 20],
        nonce: U256::ZERO,
        y_parity: 1,
        r: u256("cffb1eef5fe6263312c5c1e6dcf262e1abe402b2727b0ff1302666fdfda8195c"),
        s: u256("250ec3901d5dd57e96f43a66a24c2202a7324347fdf993d5c40bce6387970575"),
    }
}

fn chain_one_delegation() -> Authorization {
    Authorization {
        chain_id: U256::from_u64(1),
        address: [0x22; 20],
        nonce: U256::from_u64(7),
        y_parity: 1,
        r: u256("1bf342fc6e92aace8aadd92f190f100a14a2a6678a5698084466cc05932020a8"),
        s: u256("0759faae1d7028f9abbe2f1506308b25964d233ef1b98f8aab714cea4c8f2594"),
    }
}

fn transaction(chain_id: u64, authorizations: Vec<Authorization>) -> EvmTransaction {
    EvmTransaction {
        tx_type: TxType::Eip7702,
        chain_id: Some(U256::from_u64(chain_id)),
        nonce: U256::ZERO,
        gas_price: None,
        max_priority_fee_per_gas: Some(U256::ZERO),
        max_fee_per_gas: Some(U256::ZERO),
        gas_limit: U256::from_u64(21_000),
        to: Some([0x11; 20]),
        value: U256::ZERO,
        data: Vec::new(),
        access_list_entries: 0,
        max_fee_per_blob_gas: None,
        blob_versioned_hashes: Vec::new(),
        authorizations,
        signing_hash: [0; 32],
    }
}

#[test]
fn chain_one_transaction_skips_chain_two_zero_address_before_interpreting_it() {
    // cast wallet sign-auth 0x00..00 --chain 2 --nonce 0 --private-key 0x00..01
    // cast mktx 0x11..11 --chain 1 --nonce 0 --gas-limit 21000
    //   --gas-price 0 --priority-gas-price 0 --value 0 --auth <above> --raw-unsigned
    let raw = hex::decode(
        "0x04f87d018080808252089411111111111111111111111111111111111111118080c0f85cf85a029400000000000000000000000000000000000000008001a0cffb1eef5fe6263312c5c1e6dcf262e1abe402b2727b0ff1302666fdfda8195ca0250ec3901d5dd57e96f43a66a24c2202a7324347fdf993d5c40bce6387970575",
    )
    .unwrap();
    let parsed = parse_unsigned_transaction(&raw).unwrap();
    let authorization = parsed.authorizations.first().unwrap();
    assert_eq!(authorization.recover_authority(), Ok(AUTHORITY));
    assert_eq!(
        hex::encode_prefixed(&authorization.signature_hash()),
        "0xf9e9a22de26800043a7bbba3e623d8fe0dd2b35717226896bc3b12f1e39e64da"
    );

    let review = review_transaction_bytes(&raw).unwrap();
    assert_eq!(review.highest_severity(), Some(Severity::Warning));
    assert!(review.has("AUTHORIZATION_CHAIN_MISMATCH"));
    assert!(!review.has("DELEGATION_CLEARED"));
    assert!(!review.has("DELEGATION_CLEAR_REQUESTED"));
    assert!(!review.has("ACCOUNT_DELEGATION"));
    let rendered = review.render();
    assert!(rendered.contains("Skipped account authorization 1 of 1"));
    assert!(rendered.contains("Stateless signature"));
    assert!(rendered.contains("Recovered authority"));
    assert!(rendered.contains("State-dependent effectiveness"));
    assert!(rendered.contains("UNKNOWN on its authorized chain"));
    assert!(!rendered.contains("No risks detected"));
}

#[test]
fn valid_authority_is_displayed_without_claiming_state_effectiveness_or_sender_identity() {
    let authorization = chain_one_delegation();
    assert_eq!(authorization.recover_authority(), Ok(AUTHORITY));
    let review = review_evm_transaction(&transaction(1, vec![authorization]));

    assert!(review.has("ACCOUNT_DELEGATION"));
    assert!(review.has("AUTHORIZATION_STATE_UNKNOWN"));
    assert_eq!(review.highest_severity(), Some(Severity::Critical));
    let rendered = review.render();
    assert!(rendered.contains("0x7E5F 4552 091A 6912 5d5D fCb7 b8C2 6590 2939 5Bdf"));
    assert!(rendered.contains("UNKNOWN: the unsigned transaction may be sponsored"));
    assert!(rendered.contains("current authority nonce and code are not in the signed bytes"));
    assert!(!rendered.contains("usually you"));
}

#[test]
fn valid_clear_is_only_a_request_and_never_a_clean_verdict() {
    let review = review_evm_transaction(&transaction(2, vec![chain_two_clear()]));
    assert!(review.has("DELEGATION_CLEAR_REQUESTED"));
    assert!(review.has("AUTHORIZATION_STATE_UNKNOWN"));
    assert!(!review.has("DELEGATION_CLEARED"));
    assert_eq!(review.highest_severity(), Some(Severity::Blind));
    assert!(!review.render().contains("No risks detected"));
}

#[test]
fn invalid_parity_is_a_skipped_tuple_not_a_transaction_parse_failure() {
    let raw = hex::decode(
        "0x04f87d028080808252089411111111111111111111111111111111111111118080c0f85cf85a029400000000000000000000000000000000000000008002a0cffb1eef5fe6263312c5c1e6dcf262e1abe402b2727b0ff1302666fdfda8195ca0250ec3901d5dd57e96f43a66a24c2202a7324347fdf993d5c40bce6387970575",
    )
    .unwrap();
    let parsed = parse_unsigned_transaction(&raw).unwrap();
    assert_eq!(parsed.authorizations[0].y_parity, 2);
    assert_eq!(
        parsed.authorizations[0].recover_authority(),
        Err(AuthorizationSignatureError::InvalidYParity)
    );
    let review = review_transaction_bytes(&raw).unwrap();
    assert!(review.has("AUTHORIZATION_INVALID_SIGNATURE"));
    assert!(!review.has("DELEGATION_CLEAR_REQUESTED"));
    assert!(review.render().contains("yParity is not 0 or 1"));
}

#[test]
fn parity_that_does_not_fit_uint8_invalidates_the_transaction() {
    // The same tuple with yParity = 256. EIP-7702 bounds the encoded field to uint8.
    let raw = hex::decode(
        "0x04f87f018080808252089411111111111111111111111111111111111111118080c0f85ef85c0294000000000000000000000000000000000000000080820100a0cffb1eef5fe6263312c5c1e6dcf262e1abe402b2727b0ff1302666fdfda8195ca0250ec3901d5dd57e96f43a66a24c2202a7324347fdf993d5c40bce6387970575",
    )
    .unwrap();
    assert_eq!(
        parse_unsigned_transaction(&raw).map(|_| ()),
        Err(Error::IntegerOverflow)
    );
}

#[test]
fn zero_out_of_range_and_high_s_signatures_are_rejected_exactly() {
    let curve_order = u256("fffffffffffffffffffffffffffffffebaaedce6af48a03bbfd25e8cd0364141");
    let high_s = u256("daf13c6fe2a22a81690bc5995db3ddfc137c999eb14f0c65fbc69029489f3bcc");
    let base = chain_two_clear();
    let cases = [
        (
            Authorization {
                r: U256::ZERO,
                ..base
            },
            AuthorizationSignatureError::InvalidScalar,
        ),
        (
            Authorization {
                r: curve_order,
                ..base
            },
            AuthorizationSignatureError::InvalidScalar,
        ),
        (
            Authorization {
                s: U256::ZERO,
                ..base
            },
            AuthorizationSignatureError::InvalidScalar,
        ),
        (
            Authorization {
                s: curve_order,
                ..base
            },
            AuthorizationSignatureError::InvalidScalar,
        ),
        (
            Authorization { s: high_s, ..base },
            AuthorizationSignatureError::HighS,
        ),
    ];

    for (authorization, expected) in cases {
        assert_eq!(authorization.recover_authority(), Err(expected));
        let review = review_evm_transaction(&transaction(2, vec![authorization]));
        assert!(review.has("AUTHORIZATION_INVALID_SIGNATURE"));
        assert!(!review.has("DELEGATION_CLEAR_REQUESTED"));
        assert!(!review.has("ACCOUNT_DELEGATION"));
        assert!(!review.render().contains("No risks detected"));
    }
}

#[test]
fn duplicate_authorities_are_reported_with_ordered_last_valid_semantics() {
    let authorization = chain_one_delegation();
    let review = review_evm_transaction(&transaction(1, vec![authorization, authorization]));
    assert!(review.has("DUPLICATE_AUTHORITY"));
    let rendered = review.render();
    assert!(rendered.contains("also authorization 1"));
    assert!(rendered.contains("the last valid occurrence wins"));
    assert!(rendered.contains("these bytes do not prove which occurrences are effective"));
}
