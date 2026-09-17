//! The tool registry: the complete list of things an agent may propose.
//!
//! Adding a tool here widens what an agent can ask for, so the list is short,
//! explicit, and each entry states exactly which typed action it becomes.

use authority::{Action, HttpMethod, Plan, Step, StepId};
use serde_json::Value;

use crate::policy::Policy;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AdapterError {
    NotJson(String),
    /// The proposal is not shaped like a list of tool calls.
    Shape(&'static str),
    /// A tool this device does not have. Refused, never approximated.
    UnknownTool(String),
    /// A known tool, called with arguments that do not make sense for it.
    BadArguments {
        tool: String,
        why: &'static str,
    },
    /// A step referred to an earlier step that was never proposed.
    UnknownStep(u16),
    /// The proposal is larger than this device will review.
    TooLarge(&'static str),
}

impl core::fmt::Display for AdapterError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            AdapterError::NotJson(e) => write!(f, "the proposal is not valid JSON: {e}"),
            AdapterError::Shape(w) => write!(f, "the proposal is not shaped like a plan: {w}"),
            AdapterError::UnknownTool(t) => write!(
                f,
                "this device has no tool called {t:?}, so the plan is refused rather than guessed at"
            ),
            AdapterError::BadArguments { tool, why } => {
                write!(f, "tool {tool:?} was called with {why}")
            }
            AdapterError::UnknownStep(id) => {
                write!(
                    f,
                    "a step uses the output of step {id}, which was never proposed"
                )
            }
            AdapterError::TooLarge(w) => write!(f, "{w}"),
        }
    }
}

impl std::error::Error for AdapterError {}

/// Every tool an agent may call on this device, and what it becomes.
pub const TOOLS: &[(&str, &str)] = &[
    (
        "read_file",
        "reads a file; its sensitivity is decided here, not by the agent",
    ),
    ("write_file", "writes a file"),
    ("delete_file", "deletes a file — irreversible"),
    ("read_credential", "retrieves a stored credential"),
    ("http_request", "makes a web request"),
    ("send_message", "sends a message to someone"),
    ("payment", "moves money"),
    (
        "sign_transaction",
        "signs a blockchain transaction, reviewed by clearsign",
    ),
    (
        "run_program",
        "runs a program — what it does cannot be read from the request",
    ),
    ("install_app", "installs software"),
    ("change_setting", "changes a system setting"),
    (
        "transform",
        "computation over earlier outputs, with no side effects",
    ),
];

const MAX_STEPS_PROPOSED: usize = authority::MAX_STEPS;

/// Build a plan from an agent's proposal.
///
/// The expected shape is what tool-calling agents already emit:
///
/// ```json
/// { "goal": "…", "steps": [ { "id": 1, "label": "…", "tool": "read_file",
///                             "arguments": { "path": "…" }, "uses": [] } ] }
/// ```
pub fn plan_from_json(json: &str, policy: &Policy) -> Result<Plan, AdapterError> {
    let root: Value =
        serde_json::from_str(json).map_err(|e| AdapterError::NotJson(e.to_string()))?;
    let goal = root
        .get("goal")
        .and_then(Value::as_str)
        .ok_or(AdapterError::Shape("no goal"))?;
    let steps_json = root
        .get("steps")
        .and_then(Value::as_array)
        .ok_or(AdapterError::Shape("no steps array"))?;
    if steps_json.is_empty() {
        return Err(AdapterError::Shape("the plan has no steps"));
    }
    if steps_json.len() > MAX_STEPS_PROPOSED {
        return Err(AdapterError::TooLarge(
            "the proposal has more steps than this device will review",
        ));
    }

    let mut ids: Vec<u16> = Vec::with_capacity(steps_json.len());
    let mut steps: Vec<Step> = Vec::with_capacity(steps_json.len());
    for item in steps_json {
        let id_raw = item
            .get("id")
            .and_then(Value::as_u64)
            .ok_or(AdapterError::Shape("a step has no id"))?;
        let id =
            u16::try_from(id_raw).map_err(|_| AdapterError::Shape("a step id is too large"))?;
        let label = item.get("label").and_then(Value::as_str).unwrap_or("");
        let tool = item
            .get("tool")
            .and_then(Value::as_str)
            .ok_or(AdapterError::Shape("a step names no tool"))?;
        let args = item.get("arguments").unwrap_or(&Value::Null);
        let action = action_for(tool, args, policy)?;

        let mut inputs = Vec::new();
        if let Some(uses) = item.get("uses") {
            let list = uses
                .as_array()
                .ok_or(AdapterError::Shape("uses is not an array"))?;
            for u in list {
                let n =
                    u.as_u64()
                        .and_then(|n| u16::try_from(n).ok())
                        .ok_or(AdapterError::Shape(
                            "uses contains something that is not a step id",
                        ))?;
                // Only steps proposed earlier: a forward reference would be a
                // cycle the engine would reject anyway, but this says why.
                if !ids.contains(&n) {
                    return Err(AdapterError::UnknownStep(n));
                }
                inputs.push(StepId(n));
            }
        }
        ids.push(id);
        steps.push(Step {
            id: StepId(id),
            title: String::from(label),
            action,
            inputs,
        });
    }
    Ok(Plan {
        goal: String::from(goal),
        steps,
    })
}

fn string_arg<'a>(
    args: &'a Value,
    name: &'static str,
    tool: &str,
) -> Result<&'a str, AdapterError> {
    args.get(name)
        .and_then(Value::as_str)
        .ok_or_else(|| AdapterError::BadArguments {
            tool: String::from(tool),
            why: "a missing or non-string argument",
        })
}

fn action_for(tool: &str, args: &Value, policy: &Policy) -> Result<Action, AdapterError> {
    let action = match tool {
        "read_file" => {
            let path = string_arg(args, "path", tool)?;
            Action::ReadFile {
                path: String::from(path),
                // Local policy decides this. Anything the proposal says about
                // sensitivity is ignored: an agent that could label a credential
                // "public" could walk it straight past the flow tracing.
                sensitivity: policy.classify(path),
            }
        }
        "write_file" => Action::WriteFile {
            path: String::from(string_arg(args, "path", tool)?),
        },
        "delete_file" => Action::DeleteFile {
            path: String::from(string_arg(args, "path", tool)?),
        },
        "read_credential" => Action::ReadCredential {
            name: String::from(string_arg(args, "name", tool)?),
        },
        "http_request" => {
            let method = match string_arg(args, "method", tool)?
                .to_ascii_uppercase()
                .as_str()
            {
                "GET" => HttpMethod::Get,
                "POST" => HttpMethod::Post,
                "PUT" => HttpMethod::Put,
                "DELETE" => HttpMethod::Delete,
                _ => {
                    return Err(AdapterError::BadArguments {
                        tool: String::from(tool),
                        why: "an HTTP method this device does not use",
                    });
                }
            };
            Action::HttpRequest {
                method,
                host: String::from(string_arg(args, "host", tool)?),
            }
        }
        "send_message" => Action::SendMessage {
            channel: String::from(string_arg(args, "channel", tool)?),
            recipient: String::from(string_arg(args, "recipient", tool)?),
        },
        "payment" => {
            let amount = args.get("amount_minor").and_then(Value::as_u64).ok_or(
                AdapterError::BadArguments {
                    tool: String::from(tool),
                    why: "an amount that is not a whole number of minor units",
                },
            )?;
            Action::Payment {
                amount_minor: amount,
                currency: String::from(string_arg(args, "currency", tool)?),
                payee: String::from(string_arg(args, "payee", tool)?),
            }
        }
        "sign_transaction" => {
            let hex = string_arg(args, "unsigned_tx", tool)?;
            let bytes = clearsign::hex::decode(hex).map_err(|_| AdapterError::BadArguments {
                tool: String::from(tool),
                why: "a transaction that is not hexadecimal",
            })?;
            Action::SignTransaction { unsigned_tx: bytes }
        }
        "run_program" => Action::RunProgram {
            program: String::from(string_arg(args, "program", tool)?),
        },
        "install_app" => Action::InstallApp {
            package: String::from(string_arg(args, "package", tool)?),
        },
        "change_setting" => Action::ChangeSetting {
            key: String::from(string_arg(args, "key", tool)?),
            value: String::from(string_arg(args, "value", tool)?),
        },
        "transform" => Action::Transform {
            description: String::from(string_arg(args, "description", tool)?),
        },
        other => return Err(AdapterError::UnknownTool(String::from(other))),
    };
    Ok(action)
}
