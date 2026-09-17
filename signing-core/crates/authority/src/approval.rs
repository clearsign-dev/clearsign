//! Binding a person's approval to one exact plan.

use alloc::collections::BTreeSet;
use core::marker::PhantomData;

use clearsign::Severity;

use crate::policy::PlanReview;
use crate::{ApprovalError, StepId};

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
/// CRITICAL finding as `(step, code)`: none missing, no extras, no duplicates.
pub fn approve_plan<'r>(
    review: &'r PlanReview<'_>,
    acknowledged: &[(StepId, &str)],
) -> Result<ApprovedPlan<'r>, ApprovalError> {
    let required: BTreeSet<(StepId, &'static str)> = review
        .findings
        .iter()
        .filter(|f| f.severity >= Severity::Blind)
        .map(|f| (f.step, f.code))
        .collect();
    let given: BTreeSet<(StepId, &str)> = acknowledged.iter().copied().collect();

    if let Some((step, code)) = required.iter().find(|(s, c)| !given.contains(&(*s, *c))) {
        return Err(ApprovalError::Unacknowledged { step: *step, code });
    }
    if given.len() != required.len() || acknowledged.len() != given.len() {
        return Err(ApprovalError::UnexpectedAcknowledgement);
    }
    Ok(ApprovedPlan {
        fingerprint: review.fingerprint,
        _review: PhantomData,
    })
}
