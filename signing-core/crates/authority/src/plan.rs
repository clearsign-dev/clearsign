//! Plans: typed steps and their dependencies, plus structural validation.

use alloc::collections::{BTreeMap, BTreeSet};
use alloc::string::String;
use alloc::vec::Vec;

use crate::PlanError;

pub const MAX_STEPS: usize = 128;
pub const MAX_INPUTS_PER_STEP: usize = 16;
pub const MAX_TEXT_BYTES: usize = 512;
pub const MAX_TRANSACTION_BYTES: usize = 64 * 1024;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct StepId(pub u16);

/// How sensitive a piece of data is. Ordered: combining data takes the maximum.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Sensitivity {
    Public,
    /// Personal information: documents, messages, contacts, location.
    Personal,
    /// Credentials, keys, recovery phrases, anything that grants access.
    Secret,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
}

/// Everything a step can do. Deliberately a closed set: an action the engine
/// cannot name is an action it cannot review.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Pure computation over inputs, such as summarising or sorting. No side effects.
    Transform {
        description: String,
    },
    ReadFile {
        path: String,
        sensitivity: Sensitivity,
    },
    WriteFile {
        path: String,
    },
    DeleteFile {
        path: String,
    },
    /// Retrieve a stored credential such as an API token.
    ReadCredential {
        name: String,
    },
    HttpRequest {
        method: HttpMethod,
        host: String,
    },
    SendMessage {
        channel: String,
        recipient: String,
    },
    /// Pay money through a payment method, in minor units (cents).
    Payment {
        amount_minor: u64,
        currency: String,
        payee: String,
    },
    /// Sign an unsigned EVM transaction; reviewed in full by `clearsign`.
    SignTransaction {
        unsigned_tx: Vec<u8>,
    },
    RunProgram {
        program: String,
    },
    InstallApp {
        package: String,
    },
    ChangeSetting {
        key: String,
        value: String,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Step {
    pub id: StepId,
    /// The planner's own description. Untrusted: shown only as a label.
    pub title: String,
    pub action: Action,
    /// Steps whose outputs this step consumes.
    pub inputs: Vec<StepId>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    /// The user's goal as the planner restated it. Untrusted.
    pub goal: String,
    pub steps: Vec<Step>,
}

/// A structurally valid plan and the order to run it in (indexes into `steps`).
pub(crate) struct Validated {
    pub order: Vec<usize>,
    pub index_of: BTreeMap<StepId, usize>,
}

pub(crate) fn validate(plan: &Plan) -> Result<Validated, PlanError> {
    check_text(&plan.goal, None)?;
    if plan.steps.is_empty() {
        return Err(PlanError::Empty);
    }
    if plan.steps.len() > MAX_STEPS {
        return Err(PlanError::TooManySteps {
            max: MAX_STEPS,
            found: plan.steps.len(),
        });
    }

    let mut index_of = BTreeMap::new();
    for (i, step) in plan.steps.iter().enumerate() {
        if index_of.insert(step.id, i).is_some() {
            return Err(PlanError::DuplicateStepId(step.id));
        }
    }

    for step in &plan.steps {
        check_text(&step.title, Some(step.id))?;
        check_action(step)?;
        if step.inputs.len() > MAX_INPUTS_PER_STEP {
            return Err(PlanError::TooManyInputs {
                step: step.id,
                max: MAX_INPUTS_PER_STEP,
            });
        }
        let mut seen = BTreeSet::new();
        for input in &step.inputs {
            if *input == step.id {
                return Err(PlanError::SelfInput(step.id));
            }
            if !index_of.contains_key(input) {
                return Err(PlanError::UnknownInput {
                    step: step.id,
                    input: *input,
                });
            }
            if !seen.insert(*input) {
                return Err(PlanError::DuplicateInput {
                    step: step.id,
                    input: *input,
                });
            }
        }
    }

    let order = topological_order(plan, &index_of)?;
    Ok(Validated { order, index_of })
}

/// Kahn's algorithm. Deterministic: among ready steps, the earliest listed runs first.
fn topological_order(
    plan: &Plan,
    index_of: &BTreeMap<StepId, usize>,
) -> Result<Vec<usize>, PlanError> {
    let n = plan.steps.len();
    let mut remaining_inputs: Vec<usize> = plan.steps.iter().map(|s| s.inputs.len()).collect();
    let mut dependents: Vec<Vec<usize>> = (0..n).map(|_| Vec::new()).collect();
    for (i, step) in plan.steps.iter().enumerate() {
        for input in &step.inputs {
            let src = *index_of.get(input).ok_or(PlanError::Cycle)?;
            dependents.get_mut(src).ok_or(PlanError::Cycle)?.push(i);
        }
    }

    let mut ready: BTreeSet<usize> = remaining_inputs
        .iter()
        .enumerate()
        .filter(|(_, c)| **c == 0)
        .map(|(i, _)| i)
        .collect();
    let mut order = Vec::with_capacity(n);
    while let Some(i) = ready.pop_first() {
        order.push(i);
        for &d in dependents.get(i).ok_or(PlanError::Cycle)? {
            let count = remaining_inputs.get_mut(d).ok_or(PlanError::Cycle)?;
            *count = count.checked_sub(1).ok_or(PlanError::Cycle)?;
            if *count == 0 {
                ready.insert(d);
            }
        }
    }
    if order.len() != n {
        return Err(PlanError::Cycle);
    }
    Ok(order)
}

fn check_text(s: &str, step: Option<StepId>) -> Result<(), PlanError> {
    if s.trim().is_empty() {
        return Err(PlanError::EmptyText { step });
    }
    if s.len() > MAX_TEXT_BYTES {
        return Err(PlanError::TextTooLong {
            step,
            max: MAX_TEXT_BYTES,
        });
    }
    Ok(())
}

fn check_field(s: &str, step: StepId) -> Result<(), PlanError> {
    check_text(s, Some(step))
}

fn check_action(step: &Step) -> Result<(), PlanError> {
    let id = step.id;
    match &step.action {
        Action::Transform { description } => check_field(description, id),
        Action::ReadFile { path, .. }
        | Action::WriteFile { path }
        | Action::DeleteFile { path } => check_field(path, id),
        Action::ReadCredential { name } => check_field(name, id),
        Action::HttpRequest { host, .. } => check_field(host, id),
        Action::SendMessage { channel, recipient } => {
            check_field(channel, id)?;
            check_field(recipient, id)
        }
        Action::Payment {
            currency, payee, ..
        } => {
            check_field(currency, id)?;
            check_field(payee, id)
        }
        Action::SignTransaction { unsigned_tx } => {
            if unsigned_tx.is_empty() {
                return Err(PlanError::EmptyText { step: Some(id) });
            }
            if unsigned_tx.len() > MAX_TRANSACTION_BYTES {
                return Err(PlanError::TransactionTooLarge {
                    step: id,
                    max: MAX_TRANSACTION_BYTES,
                });
            }
            Ok(())
        }
        Action::RunProgram { program } => check_field(program, id),
        Action::InstallApp { package } => check_field(package, id),
        Action::ChangeSetting { key, value } => {
            check_field(key, id)?;
            // An empty value is a legitimate setting (for example clearing a field).
            if value.len() > MAX_TEXT_BYTES {
                return Err(PlanError::TextTooLong {
                    step: Some(id),
                    max: MAX_TEXT_BYTES,
                });
            }
            Ok(())
        }
    }
}

impl Action {
    /// Whether this action sends data somewhere outside the device.
    pub fn is_egress(&self) -> bool {
        matches!(
            self,
            Action::HttpRequest { .. } | Action::SendMessage { .. }
        )
    }

    /// Whether this action cannot be undone by the system.
    pub fn is_irreversible(&self) -> bool {
        matches!(
            self,
            Action::DeleteFile { .. }
                | Action::Payment { .. }
                | Action::SignTransaction { .. }
                | Action::SendMessage { .. }
        )
    }

    pub(crate) fn tag(&self) -> u8 {
        match self {
            Action::Transform { .. } => 1,
            Action::ReadFile { .. } => 2,
            Action::WriteFile { .. } => 3,
            Action::DeleteFile { .. } => 4,
            Action::ReadCredential { .. } => 5,
            Action::HttpRequest { .. } => 6,
            Action::SendMessage { .. } => 7,
            Action::Payment { .. } => 8,
            Action::SignTransaction { .. } => 9,
            Action::RunProgram { .. } => 10,
            Action::InstallApp { .. } => 11,
            Action::ChangeSetting { .. } => 12,
        }
    }
}
