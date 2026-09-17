//! Information-flow tracing across a validated plan.
//!
//! Each step's output carries the highest sensitivity of anything that flowed
//! into it, plus the set of source steps responsible. This is what catches a
//! plan where every step looks harmless alone, but secret data from one step
//! reaches a network request several steps later.

use alloc::collections::BTreeSet;
use alloc::vec::Vec;

use crate::plan::{Action, Plan, Sensitivity, StepId, Validated};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Flow {
    /// Highest sensitivity reaching this step's output.
    pub sensitivity: Sensitivity,
    /// Steps that introduced personal or secret data into this output.
    pub sources: BTreeSet<StepId>,
}

impl Flow {
    fn public() -> Flow {
        Flow {
            sensitivity: Sensitivity::Public,
            sources: BTreeSet::new(),
        }
    }
}

/// Sensitivity a step introduces by itself, before considering its inputs.
fn introduced(action: &Action) -> Sensitivity {
    match action {
        Action::ReadFile { sensitivity, .. } => *sensitivity,
        Action::ReadCredential { .. } => Sensitivity::Secret,
        _ => Sensitivity::Public,
    }
}

/// Flow for every step, indexed like `plan.steps`.
pub(crate) fn trace(plan: &Plan, v: &Validated) -> Vec<Flow> {
    let mut flows: Vec<Flow> = plan.steps.iter().map(|_| Flow::public()).collect();
    for &i in &v.order {
        let Some(step) = plan.steps.get(i) else {
            continue;
        };
        let mut flow = Flow::public();
        let own = introduced(&step.action);
        if own > Sensitivity::Public {
            flow.sensitivity = own;
            flow.sources.insert(step.id);
        }
        for input in &step.inputs {
            if let Some(src) = v.index_of.get(input).and_then(|&j| flows.get(j)) {
                if src.sensitivity > flow.sensitivity {
                    flow.sensitivity = src.sensitivity;
                }
                flow.sources.extend(src.sources.iter().copied());
            }
        }
        if let Some(slot) = flows.get_mut(i) {
            *slot = flow;
        }
    }
    flows
}
