//! The cap tested at the gate, not at the helper beside it.
//!
//! A first version of this checked `Review::too_many_to_acknowledge()`, which
//! is the predicate `approve` consults — so deleting the check from `approve`
//! itself would have left the test green. A boundary is only tested where a
//! caller meets it.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use clearsign::{
    DomainVersion, MAX_ACKNOWLEDGEABLE_FINDINGS, Review, SafeTransaction, Severity, U256, hex,
    review_safe_transaction,
};
use clearsign_keys::{KeyError, approve};

fn addr(s: &str) -> [u8; 20] {
    hex::decode(s).unwrap().try_into().unwrap()
}

/// A real review, so it carries a signing target and `approve` can be reached.
fn review_with(extra_findings: usize) -> Review {
    let mut data = hex::decode("a9059cbb").unwrap();
    data.extend_from_slice(&[0u8; 12]);
    data.extend_from_slice(&addr("1111111111111111111111111111111111111111"));
    data.extend_from_slice(&[0u8; 32]);
    let tx = SafeTransaction {
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
    };
    let mut review = review_safe_transaction(&tx, DomainVersion::V1_3Plus);
    let already = review.findings().len();
    for i in already..extra_findings {
        review.find(Severity::Critical, "PADDING", format!("padding {i}"));
    }
    review
}

#[test]
fn the_gate_accepts_a_review_exactly_at_the_cap() {
    let review = review_with(MAX_ACKNOWLEDGEABLE_FINDINGS);
    assert_eq!(review.findings().len(), MAX_ACKNOWLEDGEABLE_FINDINGS);
    let required = review.required_acknowledgements();
    assert!(
        approve(&review, &required).is_ok(),
        "a review exactly at the cap, acknowledged exactly, must be approvable"
    );
}

#[test]
fn the_gate_itself_refuses_one_over_the_cap() {
    // The check must live in `approve`. Deleting it there has to fail this,
    // which is what the earlier version of this test did not guarantee.
    let review = review_with(MAX_ACKNOWLEDGEABLE_FINDINGS + 1);
    assert_eq!(review.findings().len(), MAX_ACKNOWLEDGEABLE_FINDINGS + 1);

    // With nothing acknowledged.
    assert!(matches!(
        approve(&review, &[]),
        Err(KeyError::TooManyFindings)
    ));

    // And with a complete, correct list — the cap refuses regardless, rather
    // than letting a long enough acknowledgement through.
    let required = review.required_acknowledgements();
    assert!(
        matches!(approve(&review, &required), Err(KeyError::TooManyFindings)),
        "a complete acknowledgement list must not get past the cap"
    );
}
