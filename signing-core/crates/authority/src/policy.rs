//! Risk classification. Each finding is tied to a step and has a stable code.

use alloc::collections::BTreeSet;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use clearsign::Severity;

use crate::flow::{self, Flow};
use crate::plan::{self, Action, HttpMethod, Plan, Sensitivity, StepId};
use crate::text::display;
use crate::{PlanError, fingerprint};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanFinding {
    pub step: StepId,
    pub severity: Severity,
    pub code: &'static str,
    pub message: String,
}

/// A validated, traced and classified plan, ready to show a person.
pub struct PlanReview<'p> {
    pub(crate) plan: &'p Plan,
    pub(crate) order: Vec<usize>,
    pub(crate) flows: Vec<Flow>,
    pub(crate) findings: Vec<PlanFinding>,
    pub(crate) fingerprint: [u8; 32],
    pub(crate) transactions: Vec<(StepId, clearsign::Review)>,
}

impl<'p> PlanReview<'p> {
    pub fn plan(&self) -> &'p Plan {
        self.plan
    }

    pub fn fingerprint(&self) -> [u8; 32] {
        self.fingerprint
    }

    pub fn findings(&self) -> &[PlanFinding] {
        &self.findings
    }

    pub fn highest_severity(&self) -> Option<Severity> {
        self.findings.iter().map(|f| f.severity).max()
    }

    pub fn has(&self, step: StepId, code: &str) -> bool {
        self.findings()
            .iter()
            .any(|f| f.step == step && f.code == code)
    }

    /// Information flow reaching a step's output.
    pub fn flow_of(&self, step: StepId) -> Option<&Flow> {
        let i = self.plan.steps.iter().position(|s| s.id == step)?;
        self.flows.get(i)
    }

    /// Step ids in the order they will run.
    pub fn execution_order(&self) -> Vec<StepId> {
        self.order
            .iter()
            .filter_map(|&i| self.plan.steps.get(i).map(|s| s.id))
            .collect()
    }
}

const SYSTEM_PREFIXES: &[&str] = &[
    "/system", "/boot", "/efi", "/etc", "/usr", "/bin", "/sbin", "/lib",
];
const SECURITY_SETTING_PREFIXES: &[&str] = &[
    "security.",
    "network.",
    "boot.",
    "update.",
    "privacy.",
    "accounts.",
];

pub fn review_plan(plan: &Plan) -> Result<PlanReview<'_>, PlanError> {
    let v = plan::validate(plan)?;
    let flows = flow::trace(plan, &v);
    let mut findings = Vec::new();
    let mut transactions = Vec::new();

    for &i in &v.order {
        let (Some(step), Some(flow)) = (plan.steps.get(i), flows.get(i)) else {
            continue;
        };
        let mut add = |severity, code, message: String| {
            findings.push(PlanFinding {
                step: step.id,
                severity,
                code,
                message,
            });
        };

        match &step.action {
            Action::Transform { .. } | Action::ReadFile { .. } => {}
            Action::WriteFile { path } => {
                if is_system_path(path) {
                    add(
                        Severity::Critical,
                        "SYSTEM_MODIFICATION",
                        format!("Writes to the system location {}.", display(path)),
                    );
                }
                if has_traversal(path) {
                    add(
                        Severity::Warning,
                        "PATH_TRAVERSAL",
                        format!(
                            "The path {} contains '..' and may resolve somewhere unexpected.",
                            display(path)
                        ),
                    );
                }
                if flow.sensitivity == Sensitivity::Secret {
                    add(
                        Severity::Warning,
                        "SECRET_WRITTEN_TO_FILE",
                        format!(
                            "Writes secret data from {} into a file.",
                            ids(&flow.sources)
                        ),
                    );
                }
            }
            Action::DeleteFile { path } => {
                add(
                    Severity::Critical,
                    "DELETE_IRREVERSIBLE",
                    format!("Permanently deletes {}.", display(path)),
                );
                if is_system_path(path) {
                    add(
                        Severity::Critical,
                        "SYSTEM_MODIFICATION",
                        format!("Deletes from the system location {}.", display(path)),
                    );
                }
                if has_traversal(path) {
                    add(
                        Severity::Warning,
                        "PATH_TRAVERSAL",
                        format!(
                            "The path {} contains '..' and may resolve somewhere unexpected.",
                            display(path)
                        ),
                    );
                }
            }
            Action::ReadCredential { name } => {
                add(
                    Severity::Warning,
                    "CREDENTIAL_ACCESS",
                    format!("Retrieves the stored credential {}.", display(name)),
                );
            }
            Action::HttpRequest { method, host } => {
                if !matches!(method, HttpMethod::Get) {
                    add(
                        Severity::Warning,
                        "REMOTE_CHANGE",
                        format!("May create, change or delete data on {}.", display(host)),
                    );
                }
                egress(
                    &mut add,
                    flow,
                    &format!("a web request to {}", display(host)),
                );
            }
            Action::SendMessage { channel, recipient } => {
                add(
                    Severity::Warning,
                    "SENDS_MESSAGE",
                    format!(
                        "Sends a message in your name via {} to {}. It cannot be unsent.",
                        display(channel),
                        display(recipient)
                    ),
                );
                egress(
                    &mut add,
                    flow,
                    &format!("a message to {}", display(recipient)),
                );
            }
            Action::Payment {
                amount_minor,
                currency,
                payee,
            } => {
                add(
                    Severity::Critical,
                    "PAYMENT",
                    format!(
                        "Pays {amount_minor} minor units of {} to {}. Payments may not be reversible.",
                        display(currency),
                        display(payee)
                    ),
                );
            }
            Action::SignTransaction { unsigned_tx } => {
                add(
                    Severity::Critical,
                    "SIGNS_TRANSACTION",
                    String::from(
                        "Signs a blockchain transaction. Once broadcast it cannot be reversed.",
                    ),
                );
                match clearsign::review_transaction_bytes(unsigned_tx) {
                    Ok(review) => {
                        for f in review
                            .findings()
                            .iter()
                            .filter(|f| f.severity >= Severity::Warning)
                        {
                            add(
                                f.severity,
                                f.code,
                                format!("In the transaction: {}", f.message),
                            );
                        }
                        transactions.push((step.id, review));
                    }
                    Err(e) => {
                        add(
                            Severity::Blind,
                            "TRANSACTION_UNDECODABLE",
                            format!(
                                "The transaction could not be decoded ({e}), so what it does is unknown."
                            ),
                        );
                    }
                }
            }
            Action::RunProgram { program } => {
                add(
                    Severity::Blind,
                    "ARBITRARY_PROGRAM",
                    format!(
                        "Runs {}. What a program does cannot be determined from this request.",
                        display(program)
                    ),
                );
                match flow.sensitivity {
                    Sensitivity::Secret => add(
                        Severity::Critical,
                        "SECRET_TO_PROGRAM",
                        format!(
                            "Passes secret data from {} to a program whose behaviour is unknown.",
                            ids(&flow.sources)
                        ),
                    ),
                    Sensitivity::Personal => add(
                        Severity::Warning,
                        "PERSONAL_DATA_TO_PROGRAM",
                        format!(
                            "Passes personal data from {} to a program whose behaviour is unknown.",
                            ids(&flow.sources)
                        ),
                    ),
                    Sensitivity::Public => {}
                }
            }
            Action::InstallApp { package } => {
                add(
                    Severity::Warning,
                    "APP_INSTALL",
                    format!("Installs the app {}.", display(package)),
                );
            }
            Action::ChangeSetting { key, value } => {
                if SECURITY_SETTING_PREFIXES.iter().any(|p| key.starts_with(p)) {
                    add(
                        Severity::Critical,
                        "SECURITY_SETTING_CHANGE",
                        format!(
                            "Changes the security-relevant setting {} to {}.",
                            display(key),
                            display(value)
                        ),
                    );
                } else {
                    add(
                        Severity::Warning,
                        "SETTING_CHANGE",
                        format!(
                            "Changes the setting {} to {}.",
                            display(key),
                            display(value)
                        ),
                    );
                }
            }
        }
    }

    Ok(PlanReview {
        plan,
        order: v.order,
        flows,
        findings,
        fingerprint: fingerprint(plan),
        transactions,
    })
}

fn egress<F: FnMut(Severity, &'static str, String)>(add: &mut F, flow: &Flow, destination: &str) {
    match flow.sensitivity {
        Sensitivity::Secret => add(
            Severity::Critical,
            "SECRET_EGRESS",
            format!(
                "Secret data from {} would leave this device through {destination}.",
                ids(&flow.sources)
            ),
        ),
        Sensitivity::Personal => add(
            Severity::Warning,
            "PERSONAL_DATA_EGRESS",
            format!(
                "Personal data from {} would leave this device through {destination}.",
                ids(&flow.sources)
            ),
        ),
        Sensitivity::Public => {}
    }
}

fn is_system_path(path: &str) -> bool {
    SYSTEM_PREFIXES.iter().any(|p| {
        path == *p
            || path
                .strip_prefix(p)
                .is_some_and(|rest| rest.starts_with('/'))
    })
}

fn has_traversal(path: &str) -> bool {
    path.split(['/', '\\']).any(|part| part == "..")
}

pub(crate) fn ids(set: &BTreeSet<StepId>) -> String {
    let mut out = String::new();
    for (i, id) in set.iter().enumerate() {
        if i != 0 {
            out.push_str(", ");
        }
        out.push_str(&format!("#{}", id.0));
    }
    if out.is_empty() {
        out.push_str("an earlier step");
    }
    out
}
