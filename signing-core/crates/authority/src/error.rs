use core::fmt;

use crate::StepId;

/// Why a plan was rejected before anyone saw it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlanError {
    Empty,
    TooManySteps {
        max: usize,
        found: usize,
    },
    DuplicateStepId(StepId),
    UnknownInput {
        step: StepId,
        input: StepId,
    },
    SelfInput(StepId),
    DuplicateInput {
        step: StepId,
        input: StepId,
    },
    TooManyInputs {
        step: StepId,
        max: usize,
    },
    /// The dependency graph contains a cycle, so there is no valid order to run it.
    Cycle,
    TextTooLong {
        step: Option<StepId>,
        max: usize,
    },
    EmptyText {
        step: Option<StepId>,
    },
    TransactionTooLarge {
        step: StepId,
        max: usize,
    },
}

impl fmt::Display for PlanError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PlanError::Empty => f.write_str("the plan has no steps"),
            PlanError::TooManySteps { max, found } => {
                write!(f, "the plan has {found} steps; the limit is {max}")
            }
            PlanError::DuplicateStepId(id) => write!(f, "step #{} appears more than once", id.0),
            PlanError::UnknownInput { step, input } => {
                write!(
                    f,
                    "step #{} uses step #{}, which does not exist",
                    step.0, input.0
                )
            }
            PlanError::SelfInput(id) => write!(f, "step #{} uses its own output", id.0),
            PlanError::DuplicateInput { step, input } => {
                write!(
                    f,
                    "step #{} lists step #{} as an input twice",
                    step.0, input.0
                )
            }
            PlanError::TooManyInputs { step, max } => {
                write!(f, "step #{} has more than {max} inputs", step.0)
            }
            PlanError::Cycle => f.write_str("the steps depend on each other in a loop"),
            PlanError::TextTooLong { step, max } => match step {
                Some(id) => write!(f, "text in step #{} is longer than {max} bytes", id.0),
                None => write!(f, "the goal is longer than {max} bytes"),
            },
            PlanError::EmptyText { step } => match step {
                Some(id) => write!(f, "step #{} has an empty required field", id.0),
                None => f.write_str("the goal is empty"),
            },
            PlanError::TransactionTooLarge { step, max } => {
                write!(
                    f,
                    "the transaction in step #{} is larger than {max} bytes",
                    step.0
                )
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ApprovalError {
    /// A BLIND or CRITICAL finding was not acknowledged. Identified by the
    /// number the review displays, because a code alone is not unique within a
    /// step.
    Unacknowledged { number: u32, code: &'static str },
    /// Acknowledgements must match exactly the findings that need them.
    UnexpectedAcknowledgement,
    /// More findings than can be acknowledged one by one; see
    /// `clearsign::MAX_ACKNOWLEDGEABLE_FINDINGS`.
    TooManyFindings,
}

impl fmt::Display for ApprovalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ApprovalError::Unacknowledged { number, code } => {
                write!(f, "risk {number}:{code} must be acknowledged")
            }
            ApprovalError::TooManyFindings => f.write_str(
                "this plan has more findings than can be acknowledged one by one; it cannot be approved",
            ),
            ApprovalError::UnexpectedAcknowledgement => {
                f.write_str("acknowledgements must match exactly the risks that require them")
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecError {
    /// The plan presented for execution is invalid.
    Invalid(PlanError),
    /// The plan presented for execution is not the plan that was approved.
    PlanChangedAfterApproval,
    /// A step failed; steps after it were not run.
    StepFailed { step: StepId, completed: usize },
    /// A runner reported success for a step it was not asked to run.
    RunnerMisbehaved,
}

impl fmt::Display for ExecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecError::Invalid(e) => write!(f, "invalid plan: {e}"),
            ExecError::PlanChangedAfterApproval => {
                f.write_str("refused: this plan is not the one that was approved")
            }
            ExecError::StepFailed { step, completed } => write!(
                f,
                "step #{} failed after {completed} steps completed; nothing further was run",
                step.0
            ),
            ExecError::RunnerMisbehaved => {
                f.write_str("the step runner returned an inconsistent result")
            }
        }
    }
}
