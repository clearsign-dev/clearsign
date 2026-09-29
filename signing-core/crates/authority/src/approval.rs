//! Binding a person's approval to one exact plan.

use alloc::collections::BTreeSet;
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
    acknowledged: &[(u16, &str)],
) -> Result<ApprovedPlan<'r>, ApprovalError> {
    let required: BTreeSet<(u16, &'static str)> =
        review.required_acknowledgements().into_iter().collect();
    let given: BTreeSet<(u16, &str)> = acknowledged.iter().copied().collect();

    if let Some((number, code)) = required.iter().find(|(n, c)| !given.contains(&(*n, *c))) {
        return Err(ApprovalError::Unacknowledged {
            number: *number,
            code,
        });
    }
    if given.len() != required.len() || acknowledged.len() != given.len() {
        return Err(ApprovalError::UnexpectedAcknowledgement);
    }
    Ok(ApprovedPlan {
        fingerprint: review.fingerprint,
        _review: PhantomData,
    })
}
