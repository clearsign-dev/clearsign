//! Turning a review into permission to sign.

use alloc::vec::Vec;
use core::marker::PhantomData;

use clearsign::{Review, TargetKind};

use crate::KeyError;

/// Permission to sign exactly one digest, derived from exactly one review.
///
/// It has no public constructor, and it borrows the review, so it cannot
/// outlive what the human was shown.
pub struct Approval<'r> {
    pub(crate) digest: [u8; 32],
    pub(crate) kind: TargetKind,
    _review: PhantomData<&'r Review>,
}

impl Approval<'_> {
    pub fn digest(&self) -> [u8; 32] {
        self.digest
    }

    pub fn kind(&self) -> TargetKind {
        self.kind
    }
}

/// Grant permission to sign a review.
///
/// `acknowledged` must name **exactly** the findings that require it: each one
/// by the number shown beside it on screen and its code, no fewer and no extras.
///
/// Naming each finding individually, rather than passing a set of codes, is what
/// makes two unlimited approvals to two different spenders two separate
/// confirmations. A set of codes collapses them into one, which is the ritual
/// looking like it happened rather than happening.
pub fn approve<'r>(
    review: &'r Review,
    acknowledged: &[(u32, &str)],
) -> Result<Approval<'r>, KeyError> {
    let target = review.signing_target().ok_or(KeyError::NotSignable)?;
    // A review too long to read through is refused rather than approved from a
    // partial list. Checked before numbering, so no identifier is ever reused.
    if review.too_many_to_acknowledge() {
        return Err(KeyError::TooManyFindings);
    }
    let required: Vec<(u32, &'static str)> = review.required_acknowledgements();

    for (n, code) in &required {
        if !acknowledged.iter().any(|(m, c)| m == n && c == code) {
            return Err(KeyError::UnacknowledgedFinding(code));
        }
    }
    // No extras, and no repeats standing in for a second finding.
    if acknowledged.len() != required.len() {
        return Err(KeyError::UnexpectedAcknowledgement);
    }
    for (i, (n, code)) in acknowledged.iter().enumerate() {
        let duplicated = acknowledged
            .iter()
            .skip(i.saturating_add(1))
            .any(|(m, c)| m == n && c == code);
        if duplicated || !required.iter().any(|(m, c)| m == n && c == code) {
            return Err(KeyError::UnexpectedAcknowledgement);
        }
    }

    Ok(Approval {
        digest: target.digest,
        kind: target.kind,
        _review: PhantomData,
    })
}
