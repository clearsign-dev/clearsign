//! Regression tests for the external audit of 17 Sep 2026, findings 7 and 10.

#![allow(
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]

use clearsign::{DomainVersion, SafeTransaction, Severity, U256, hex, review_safe_transaction};

fn addr(s: &str) -> [u8; 20] {
    hex::decode(s).unwrap().try_into().unwrap()
}

fn element(op: u8, to: [u8; 20], data: &[u8]) -> Vec<u8> {
    let mut out = vec![op];
    out.extend_from_slice(&to);
    out.extend_from_slice(&[0u8; 32]);
    let mut len = [0u8; 32];
    len[28..].copy_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(&len);
    out.extend_from_slice(data);
    out
}

fn transfer_calldata() -> Vec<u8> {
    let mut d = hex::decode("a9059cbb").unwrap();
    d.extend_from_slice(&[0u8; 12]);
    d.extend_from_slice(&addr("1111111111111111111111111111111111111111"));
    d.extend_from_slice(&[0u8; 31]);
    d.push(1);
    d
}

/// A pinned MultiSend batch of `plain` transfers with one DELEGATECALL last.
fn batch(plain: usize) -> SafeTransaction {
    let mut packed = Vec::new();
    for _ in 0..plain {
        packed.extend_from_slice(&element(
            0,
            addr("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
            &transfer_calldata(),
        ));
    }
    packed.extend_from_slice(&element(
        1,
        addr("00000000000000000000000000000000DeaDBeef"),
        &transfer_calldata(),
    ));

    let mut calldata = hex::decode("8d80ff0a").unwrap();
    let mut off = [0u8; 32];
    off[31] = 32;
    calldata.extend_from_slice(&off);
    let mut blen = [0u8; 32];
    blen[28..].copy_from_slice(&(packed.len() as u32).to_be_bytes());
    calldata.extend_from_slice(&blen);
    calldata.extend_from_slice(&packed);
    while calldata.len() % 32 != 4 {
        calldata.push(0);
    }
    SafeTransaction {
        chain_id: U256::from_u64(1),
        safe: addr("1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4"),
        to: addr("40A2aCCbd92BCA938b02010E17A5b8929b49130D"),
        value: U256::from_u64(0),
        data: calldata,
        operation: 1,
        safe_tx_gas: U256::from_u64(0),
        base_gas: U256::from_u64(0),
        gas_price: U256::from_u64(0),
        gas_token: [0u8; 20],
        refund_receiver: [0u8; 20],
        nonce: U256::from_u64(1),
    }
}

fn ack_codes(tx: &SafeTransaction) -> Vec<&'static str> {
    review_safe_transaction(tx, DomainVersion::V1_3Plus)
        .required_acknowledgements()
        .into_iter()
        .map(|(_, code)| code)
        .collect()
}

#[test]
fn audit_f7_a_delegatecall_past_the_display_limit_is_still_named() {
    // The finding: a batch of 33 elements exceeded the display limit, so the
    // whole batch went undecoded and the only thing to acknowledge was
    // "MULTISEND_TOO_MANY_CALLS". A hurried operator acknowledges "batch too
    // long" and signs the Bybit pattern.
    let codes = ack_codes(&batch(32));
    assert!(
        codes.contains(&"SAFE_DELEGATECALL"),
        "a delegatecall past the shown calls must still be named: {codes:?}"
    );
    assert!(
        codes.contains(&"MULTISEND_CALLS_NOT_SHOWN"),
        "the person must also be told that calls are not shown: {codes:?}"
    );

    // A small batch is unchanged: the delegatecall is shown and named.
    let small = ack_codes(&batch(2));
    assert!(small.contains(&"SAFE_DELEGATECALL"), "{small:?}");
    assert!(!small.contains(&"MULTISEND_CALLS_NOT_SHOWN"), "{small:?}");
}

#[test]
fn audit_f7_a_batch_too_large_to_parse_says_a_delegatecall_may_be_hidden() {
    // Past the parsing limit nothing can be ruled out, and the acknowledgement
    // has to say so rather than reading as a length complaint.
    let codes = ack_codes(&batch(2000));
    assert!(
        codes.contains(&"UNDECODED_DELEGATECALL_POSSIBLE"),
        "{codes:?}"
    );
}

#[test]
fn audit_f10_a_legacy_safe_signature_says_it_is_not_chain_bound() {
    // The finding: under the v1.1.x domain the chain ID is not hashed, so the
    // same signature is valid on every chain that Safe exists on — while the
    // review printed "Network chain ID" as though it were signed.
    let mut a = batch(1);
    a.operation = 0;
    a.data = transfer_calldata();
    a.to = addr("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48");
    let mut b = a.clone();
    b.chain_id = U256::from_u64(137);
    assert_eq!(
        clearsign::safe_transaction_hash(&a, DomainVersion::Legacy),
        clearsign::safe_transaction_hash(&b, DomainVersion::Legacy),
        "the legacy domain does not bind a chain; if this ever changes, so does the warning"
    );

    let review = review_safe_transaction(&b, DomainVersion::Legacy);
    let text = review.render();
    assert!(
        text.contains("NOT part of this signature"),
        "the chain ID must not be shown as though it were signed:\n{text}"
    );
    assert_eq!(review.highest_severity(), Some(Severity::Critical));
    assert!(
        review
            .required_acknowledgements()
            .iter()
            .any(|(_, c)| *c == "SIGNATURE_NOT_CHAIN_BOUND"),
        "replayability across chains must be acknowledged deliberately"
    );

    // Under the current domain the chain ID is signed, and nothing is added.
    let modern = review_safe_transaction(&b, DomainVersion::V1_3Plus);
    assert!(!modern.render().contains("NOT part of this signature"));
    assert!(!modern.has("SIGNATURE_NOT_CHAIN_BOUND"));
}
