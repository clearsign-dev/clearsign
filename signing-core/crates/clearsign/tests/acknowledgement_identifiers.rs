//! An acknowledgement identifier has one job: to name exactly one finding.
//!
//! It used to be a `u16` produced by `unwrap_or(u16::MAX)`, so every finding
//! past the 65,535th was numbered 65,535. Two findings sharing a number means
//! confirming one confirms the other, which is the whole thing acknowledgement
//! exists to prevent.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::collections::BTreeSet;

use clearsign::{MAX_ACKNOWLEDGEABLE_FINDINGS, Review, Severity};

fn review_with(n: usize) -> Review {
    let mut r = Review::new("many findings");
    for i in 0..n {
        r.find(Severity::Critical, "CODE", format!("finding {i}"));
    }
    r
}

#[test]
fn no_two_findings_ever_share_an_identifier() {
    // Well past the old 16-bit ceiling.
    let r = review_with(70_000);
    let required = r.required_acknowledgements();
    let distinct: BTreeSet<_> = required.iter().collect();
    assert_eq!(
        distinct.len(),
        required.len(),
        "{} of {} identifiers collided",
        required.len() - distinct.len(),
        required.len()
    );
}

#[test]
fn the_cap_predicate_moves_at_the_documented_boundary() {
    // This checks the predicate, not the gate. Deleting the cap check from
    // `approve` would leave this green, which is why the gate has its own test
    // in `clearsign-keys/tests/approval_boundary.rs` — a boundary is only
    // tested where a caller meets it.
    assert!(!review_with(MAX_ACKNOWLEDGEABLE_FINDINGS).too_many_to_acknowledge());
    assert!(review_with(MAX_ACKNOWLEDGEABLE_FINDINGS + 1).too_many_to_acknowledge());
}
