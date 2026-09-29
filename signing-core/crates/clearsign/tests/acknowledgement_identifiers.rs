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
fn a_review_too_long_to_read_cannot_be_approved() {
    // The other half of the rule. Numbers stay unique however many findings
    // there are, and a list nobody could work through is refused rather than
    // approved from whatever fits.
    assert!(!review_with(MAX_ACKNOWLEDGEABLE_FINDINGS).too_many_to_acknowledge());
    assert!(review_with(MAX_ACKNOWLEDGEABLE_FINDINGS + 1).too_many_to_acknowledge());
}
