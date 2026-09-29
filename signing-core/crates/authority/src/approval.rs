//! Binding a person's approval to one exact plan.

use alloc::vec::Vec;
use core::marker::PhantomData;

use crate::ApprovalError;
use crate::policy::PlanReview;

/// Permission to run exactly one plan. No public constructor.
pub struct ApprovedPlan<'r> {
    pub(crate) fingerprint: [u8; 32],
    _review: PhantomData<&'r PlanReview<'r>>,
}

impl ApprovedPlan<'_> {
    pub fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }
}

/// Approve a reviewed plan. `acknowledged` must list **exactly** every BLIND and
/// CRITICAL finding as `(number, code)`, using the numbers the review displays:
/// none missing, no extras, no duplicates.
///
/// Numbered rather than keyed by `(step, code)`, because two findings sharing a
/// code within one step are two separate requirements. Keying by code merged
/// them, so acknowledging "the unlimited approval" once approved two of them,
/// to two different spenders.
pub fn approve_plan<'r>(
    review: &'r PlanReview<'_>,
    acknowledged: &[(u32, &str)],
) -> Result<ApprovedPlan<'r>, ApprovalError> {
    // A review too long to read through is refused rather than approved from a
    // partial list. Checked before numbering, so no identifier is ever reused.
    if review.too_many_to_acknowledge() {
        return Err(ApprovalError::TooManyFindings);
    }
    let required: Vec<(u32, &'static str)> = review.required_acknowledgements();

    // Compared as lists, not through a set. A set silently merged two
    // requirements that happened to look alike, which is how confirming one
    // unlimited approval came to confirm two.
    for (n, code) in &required {
        if !acknowledged.iter().any(|(m, c)| m == n && *c == *code) {
            return Err(ApprovalError::Unacknowledged { number: *n, code });
        }
    }
    if acknowledged.len() != required.len() {
        return Err(ApprovalError::UnexpectedAcknowledgement);
    }
    for (i, (n, code)) in acknowledged.iter().enumerate() {
        let repeated = acknowledged
            .iter()
            .skip(i.saturating_add(1))
            .any(|(m, c)| m == n && *c == *code);
        if repeated || !required.iter().any(|(m, c)| m == n && *c == *code) {
            return Err(ApprovalError::UnexpectedAcknowledgement);
        }
    }
    Ok(ApprovedPlan {
        fingerprint: review.fingerprint,
        _review: PhantomData,
    })
}
