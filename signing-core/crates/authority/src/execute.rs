//! Executing an approved plan, fail-closed.

use alloc::string::String;
use alloc::vec::Vec;

use crate::fingerprint::fingerprint;
use crate::plan::{self, Plan, Step, StepId};
use crate::{ApprovedPlan, ExecError};

/// Opaque data a step produces. The engine never interprets it; it only routes
/// it to the steps that declared it as an input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Output(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RunError {
    pub message: String,
}

/// Performs a single step. On a real system this is the compartment or service
/// that holds the capability for that kind of action.
pub trait StepRunner {
    /// `inputs` contains exactly the outputs of the steps listed in `step.inputs`,
    /// in that order, and nothing else.
    fn run(&mut self, step: &Step, inputs: &[(StepId, &Output)]) -> Result<Output, RunError>;
}

/// Run `plan` only if it is the plan `approval` was granted for.
///
/// The plan is validated again and its fingerprint recomputed here, so a plan
/// edited after review, or a different plan object altogether, is refused before
/// any step runs. Steps run in dependency order; the first failure stops everything.
pub fn execute(
    plan: &Plan,
    approval: &ApprovedPlan<'_>,
    runner: &mut dyn StepRunner,
) -> Result<Vec<(StepId, Output)>, ExecError> {
    let v = plan::validate(plan).map_err(ExecError::Invalid)?;
    if fingerprint(plan) != approval.fingerprint {
        return Err(ExecError::PlanChangedAfterApproval);
    }

    let mut outputs: Vec<Option<Output>> = plan.steps.iter().map(|_| None).collect();
    let mut completed: Vec<(StepId, Output)> = Vec::with_capacity(plan.steps.len());

    for &i in &v.order {
        let step = plan.steps.get(i).ok_or(ExecError::RunnerMisbehaved)?;
        let mut inputs: Vec<(StepId, &Output)> = Vec::with_capacity(step.inputs.len());
        for id in &step.inputs {
            let j = *v.index_of.get(id).ok_or(ExecError::RunnerMisbehaved)?;
            let out = outputs
                .get(j)
                .and_then(Option::as_ref)
                .ok_or(ExecError::RunnerMisbehaved)?;
            inputs.push((*id, out));
        }
        let result = runner.run(step, &inputs);
        match result {
            Ok(output) => {
                if let Some(slot) = outputs.get_mut(i) {
                    *slot = Some(output.clone());
                }
                completed.push((step.id, output));
            }
            Err(_) => {
                return Err(ExecError::StepFailed {
                    step: step.id,
                    completed: completed.len(),
                });
            }
        }
    }
    Ok(completed)
}
