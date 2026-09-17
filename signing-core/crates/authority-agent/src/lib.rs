//! Turning an agent's proposed tool calls into a plan a person can approve.
//!
//! Agent frameworks emit tool calls: a name and some JSON arguments. This crate
//! maps those onto the authority engine's typed actions, and it is deliberately
//! narrow about how:
//!
//! * **Only known tools.** A tool name the adapter does not recognise is a
//!   refusal, not a step that runs anyway. An agent cannot widen its own reach
//!   by inventing a tool.
//! * **The device classifies data, not the planner.** How sensitive a file is
//!   follows from local policy about where it lives. If the proposal claims its
//!   own sensitivity, the claim is ignored. A planner that could label a
//!   credential "public" could walk data straight past the flow tracing.
//! * **Descriptions are labels.** Whatever the agent calls a step is carried
//!   through as untrusted text and shown as such; what the step *does* is
//!   always described from the typed action.

pub mod policy;
pub mod runner;
pub mod tools;

pub use policy::{Policy, sensitivity_for_path};
pub use runner::{LocalRunner, RunnerLimits};
pub use tools::{AdapterError, plan_from_json};
