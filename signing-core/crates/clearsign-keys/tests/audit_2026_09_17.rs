//! Regression tests for the external audit of 17 Sep 2026.
//!
//! Findings 1 and 6: a review's findings could be cleared by any caller holding
//! it, and a single acknowledgement code covered every finding sharing that
//! code. Both are fixed; these are the tests that would notice their return.
#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use clearsign::{DomainVersion, SafeTransaction, Severity, U256, hex, review_safe_transaction};
use clearsign_keys::approve;

fn addr(s: &str) -> [u8; 20] {
    hex::decode(s).unwrap().try_into().unwrap()
}

/// A Safe transaction granting an unlimited ERC-20 approval.
fn unlimited_approval_tx(spender: &str) -> SafeTransaction {
    let mut data = hex::decode("095ea7b3").unwrap();
    data.extend_from_slice(&[0u8; 12]);
    data.extend_from_slice(&addr(spender));
    data.extend_from_slice(&[0xff; 32]);
    SafeTransaction {
        chain_id: U256::from_u64(1),
        safe: addr("1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4"),
        to: addr("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
        value: U256::from_u64(0),
        data,
        operation: 0,
        safe_tx_gas: U256::from_u64(0),
        base_gas: U256::from_u64(0),
        gas_price: U256::from_u64(0),
        gas_token: [0u8; 20],
        refund_receiver: [0u8; 20],
        nonce: U256::from_u64(1),
    }
}

#[test]
fn audit_f1_a_copy_of_a_review_cannot_shed_its_findings() {
    // The finding: `findings` was public and `Review` is `Clone`, so any code
    // holding a review could clone it, empty the list, and approve with no
    // acknowledgements at all — for the original digest. That is the Bybit
    // attack rebuilt inside the thing meant to prevent it.
    //
    // `findings` is private now, so the clone-and-clear no longer compiles. What
    // this test pins down is the property underneath: a copy of a review demands
    // exactly what the original demanded.
    let tx = unlimited_approval_tx("00000000000000000000000000000000DeaDBeef");
    let review = review_safe_transaction(&tx, DomainVersion::V1_3Plus);
    assert_eq!(review.highest_severity(), Some(Severity::Critical));

    let copy = review.clone();
    assert_eq!(
        copy.required_acknowledgements(),
        review.required_acknowledgements(),
        "a copy must require what the original required"
    );
    assert!(
        !copy.required_acknowledgements().is_empty(),
        "this review must require an acknowledgement at all"
    );
    assert!(
        approve(&copy, &[]).is_err(),
        "a copy of a CRITICAL review must not be approvable with nothing"
    );
    assert!(approve(&copy, &[(1, "UNLIMITED_APPROVAL")]).is_ok());
}

#[test]
fn audit_f6_one_ack_code_covers_two_distinct_findings() {
    // Two unlimited approvals to different spenders, in one MultiSend batch.
    let mut packed = Vec::new();
    for spender in [
        "00000000000000000000000000000000DeaDBeef",
        "1111111111111111111111111111111111111111",
    ] {
        let mut data = hex::decode("095ea7b3").unwrap();
        data.extend_from_slice(&[0u8; 12]);
        data.extend_from_slice(&addr(spender));
        data.extend_from_slice(&[0xff; 32]);
        packed.push(0u8);
        packed.extend_from_slice(&addr("A0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"));
        packed.extend_from_slice(&[0u8; 32]);
        let mut len = [0u8; 32];
        len[28..].copy_from_slice(&(data.len() as u32).to_be_bytes());
        packed.extend_from_slice(&len);
        packed.extend_from_slice(&data);
    }
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
    let tx = SafeTransaction {
        to: addr("40A2aCCbd92BCA938b02010E17A5b8929b49130D"),
        operation: 1,
        data: calldata,
        ..unlimited_approval_tx("00000000000000000000000000000000DeaDBeef")
    };
    let review = review_safe_transaction(&tx, DomainVersion::V1_3Plus);
    let unlimited = review
        .findings()
        .iter()
        .filter(|f| f.code == "UNLIMITED_APPROVAL")
        .count();
    println!("UNLIMITED_APPROVAL findings in the review: {unlimited}");
    let codes: Vec<&str> = review
        .findings()
        .iter()
        .filter(|f| f.severity >= Severity::Blind)
        .map(|f| f.code)
        .collect();
    println!("codes needing acknowledgement: {codes:?}");
    assert_eq!(unlimited, 2, "the batch must produce two separate findings");

    // The finding: acknowledgements were a set of codes, so one
    // UNLIMITED_APPROVAL covered both spenders. Now each finding is named by the
    // number shown beside it, so two findings take two acknowledgements.
    let required = review.required_acknowledgements();
    let unlimited_acks: Vec<_> = required
        .iter()
        .filter(|(_, c)| *c == "UNLIMITED_APPROVAL")
        .collect();
    assert_eq!(
        unlimited_acks.len(),
        2,
        "two unlimited approvals must require two acknowledgements, not one: {required:?}"
    );

    // Acknowledging one of them twice must not stand in for the other.
    let (first, _) = unlimited_acks[0];
    let doubled: Vec<(u32, &str)> = required
        .iter()
        .map(|(n, c)| {
            if *c == "UNLIMITED_APPROVAL" {
                (*first, *c)
            } else {
                (*n, *c)
            }
        })
        .collect();
    assert!(
        approve(&review, &doubled).is_err(),
        "the same finding acknowledged twice must not cover a different one"
    );

    // The honest set works.
    let honest: Vec<(u32, &str)> = required.iter().map(|(n, c)| (*n, *c)).collect();
    assert!(approve(&review, &honest).is_ok(), "codes: {codes:?}");
}
