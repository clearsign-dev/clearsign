//! A deliberately capability-poor executor.
//!
//! The engine guarantees that only an approved plan runs, in order, with only
//! declared inputs. This runner is what actually performs the steps, and it
//! holds almost no capabilities: it reads and writes files under the roots
//! policy allows, and refuses everything else with a reason. A step the device
//! cannot perform is a refusal after approval, not a silent success.

use std::fs;
use std::path::Path;

use authority::{Action, Output, RunError, Step, StepId, StepRunner};

use crate::policy::Policy;

/// What this runner is willing to do.
#[derive(Debug, Clone)]
pub struct RunnerLimits {
    /// Actually touch the filesystem. When false, file steps are described and
    /// counted but nothing is read or written.
    pub perform_file_actions: bool,
    /// The largest file this runner will read into memory.
    pub max_file_bytes: u64,
}

impl Default for RunnerLimits {
    fn default() -> Self {
        RunnerLimits {
            perform_file_actions: false,
            max_file_bytes: 1024 * 1024,
        }
    }
}

pub struct LocalRunner {
    pub policy: Policy,
    pub limits: RunnerLimits,
    /// What each step did, for the transcript printed after a run.
    pub transcript: Vec<String>,
}

impl LocalRunner {
    pub fn new(policy: Policy, limits: RunnerLimits) -> Self {
        LocalRunner {
            policy,
            limits,
            transcript: Vec::new(),
        }
    }

    fn note(&mut self, step: &Step, what: String) {
        self.transcript
            .push(format!("#{} {}", step.id.0, what));
    }

    fn refuse(&mut self, step: &Step, why: &str) -> Result<Output, RunError> {
        self.note(step, format!("refused: {why}"));
        Err(RunError {
            message: String::from(why),
        })
    }
}

impl StepRunner for LocalRunner {
    fn run(&mut self, step: &Step, inputs: &[(StepId, &Output)]) -> Result<Output, RunError> {
        match &step.action {
            Action::Transform { description } => {
                // No model here: the transform concatenates what it was given, so
                // the data flow the engine traced is the data flow that happens.
                let mut out = Vec::new();
                for (_, input) in inputs {
                    out.extend_from_slice(&input.0);
                }
                self.note(
                    step,
                    format!("transformed {} input(s) — {description}", inputs.len()),
                );
                Ok(Output(out))
            }
            Action::ReadFile { path, .. } => {
                if !self.policy.allows(path) {
                    return self.refuse(step, "that path is outside the roots this device allows");
                }
                // Classification is not only for the review. A file this device
                // considers secret is not handed to an agent because the agent
                // asked politely for it.
                if self.policy.classify(path) == authority::Sensitivity::Secret {
                    return self.refuse(
                        step,
                        "this device does not read credential or key files on an agent's behalf",
                    );
                }
                if !self.limits.perform_file_actions {
                    self.note(step, format!("would read {path}"));
                    return Ok(Output(Vec::new()));
                }
                let meta = fs::metadata(Path::new(path))
                    .map_err(|e| RunError { message: format!("cannot read {path}: {e}") })?;
                if meta.len() > self.limits.max_file_bytes {
                    return self.refuse(step, "that file is larger than this device will read");
                }
                let bytes = fs::read(Path::new(path))
                    .map_err(|e| RunError { message: format!("cannot read {path}: {e}") })?;
                self.note(step, format!("read {} bytes from {path}", bytes.len()));
                Ok(Output(bytes))
            }
            Action::WriteFile { path } => {
                if !self.policy.allows(path) {
                    return self.refuse(step, "that path is outside the roots this device allows");
                }
                let bytes: Vec<u8> = inputs.iter().flat_map(|(_, o)| o.0.clone()).collect();
                if !self.limits.perform_file_actions {
                    self.note(step, format!("would write {} bytes to {path}", bytes.len()));
                    return Ok(Output(Vec::new()));
                }
                fs::write(Path::new(path), &bytes)
                    .map_err(|e| RunError { message: format!("cannot write {path}: {e}") })?;
                self.note(step, format!("wrote {} bytes to {path}", bytes.len()));
                Ok(Output(Vec::new()))
            }
            // Everything below needs a capability this device does not hand to an
            // agent. Each refusal names what is missing rather than failing vaguely.
            Action::DeleteFile { .. } => self.refuse(step, "this device does not let an agent delete files"),
            Action::ReadCredential { .. } => {
                self.refuse(step, "credentials are not readable by an agent on this device")
            }
            Action::HttpRequest { .. } => {
                self.refuse(step, "this device has no network capability to give an agent")
            }
            Action::SendMessage { .. } => self.refuse(step, "no messaging capability is configured"),
            Action::Payment { .. } => self.refuse(step, "no payment capability is configured"),
            Action::SignTransaction { .. } => self.refuse(
                step,
                "signing happens on the signing device over QR, not in this process",
            ),
            Action::RunProgram { .. } => self.refuse(step, "this device does not run arbitrary programs for an agent"),
            Action::InstallApp { .. } => self.refuse(step, "installing software is not an agent action"),
            Action::ChangeSetting { .. } => self.refuse(step, "system settings are not agent-writable"),
        }
    }
}
