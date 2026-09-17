//! # authority
//!
//! The authority engine: intelligence proposes, a person approves exactly what
//! will happen, and nothing else runs.
//!
//! A planner, typically an AI model, turns a goal into a [`Plan`]: a small graph
//! of typed [`Step`]s. The planner is treated as **untrusted**, exactly like the
//! web interface in the Bybit attack. This crate:
//!
//! 1. **validates** the plan: bounded size, no missing references, no cycles
//! 2. **traces information flow**: which steps carry personal or secret data
//! 3. **classifies risk** with the same levels as the transaction signer
//! 4. **renders** the plan as simple plain-language chunks, describing each step
//!    from its typed action, never from the planner's own words
//! 5. **binds approval** to a fingerprint of the exact plan and an exact set of
//!    acknowledged risks
//! 6. **executes** only an approved plan, in dependency order, giving each step
//!    only the inputs it declared, and stopping at the first failure
//!
//! Architecture and rationale: `docs/04-platform-architecture.md`.

#![no_std]
#![forbid(unsafe_code)]

extern crate alloc;

mod approval;
mod error;
mod execute;
mod fingerprint;
mod flow;
mod plan;
mod policy;
mod render;
mod text;
mod wire;

pub use approval::{ApprovedPlan, approve_plan};
pub use error::{ApprovalError, ExecError, PlanError};
pub use execute::{Output, RunError, StepRunner, execute};
pub use fingerprint::fingerprint;
pub use flow::Flow;
pub use plan::{
    Action, HttpMethod, MAX_INPUTS_PER_STEP, MAX_STEPS, MAX_TEXT_BYTES, MAX_TRANSACTION_BYTES,
    Plan, Sensitivity, Step, StepId,
};
pub use policy::{PlanFinding, PlanReview, review_plan};
pub use wire::{WireError, decode_plan, encode_plan};
