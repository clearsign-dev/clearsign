//! Turning a review into permission to sign.

use alloc::collections::BTreeSet;
use core::marker::PhantomData;

use clearsign::{Review, Severity, TargetKind};

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
/// `acknowledged` must contain **exactly** the codes of every BLIND and CRITICAL
/// finding in the review: no fewer, and no extras. Requiring an exact match stops
/// an interface from passing a blanket "acknowledge everything" list; the person
/// has to be shown, and confirm, each specific risk. WARNING and INFO findings
/// need no acknowledgement.
pub fn approve<'r>(review: &'r Review, acknowledged: &[&str]) -> Result<Approval<'r>, KeyError> {
    let target = review.signing_target().ok_or(KeyError::NotSignable)?;

    let required: BTreeSet<&'static str> = review
        .findings
        .iter()
        .filter(|f| f.severity >= Severity::Blind)
        .map(|f| f.code)
        .collect();
    let given: BTreeSet<&str> = acknowledged.iter().copied().collect();

    if let Some(missing) = required.iter().find(|code| !given.contains(**code)) {
        return Err(KeyError::UnacknowledgedFinding(missing));
    }
    if given.len() != required.len() || acknowledged.len() != given.len() {
        return Err(KeyError::UnexpectedAcknowledgement);
    }

    Ok(Approval {
        digest: target.digest,
        kind: target.kind,
        _review: PhantomData,
    })
}
