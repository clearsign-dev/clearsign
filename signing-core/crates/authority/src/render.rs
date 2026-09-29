//! Plain-language rendering: a complex plan broken into simple chunks.
//!
//! What each step does is written from its typed action. The planner's own
//! wording is shown only as a quoted, escaped, explicitly unverified label.

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use clearsign::Severity;

use crate::plan::{Action, HttpMethod, Sensitivity};
use crate::policy::{PlanReview, ids};
use crate::text::display;

const W: usize = 24;

impl PlanReview<'_> {
    pub fn render(&self) -> String {
        let plan = self.plan;
        let total = self.order.len();
        let mut out = String::new();
        out.push_str("== Plan ==\n");
        out.push_str(&line(
            "Goal (planner's words)",
            &format!("{} (unverified)", display(&plan.goal)),
        ));
        out.push_str(&line("Steps", &format!("{total}")));

        for (pos, &i) in self.order.iter().enumerate() {
            let (Some(step), Some(flow)) = (plan.steps.get(i), self.flows.get(i)) else {
                continue;
            };
            out.push_str(&format!(
                "\n-- Step {} of {total}  (#{}) --\n",
                pos.saturating_add(1),
                step.id.0
            ));
            out.push_str(&line("Does", &describe(&step.action)));
            out.push_str(&line(
                "Planner's label",
                &format!("{} (unverified)", display(&step.title)),
            ));
            if step.inputs.is_empty() {
                out.push_str(&line("Uses", "nothing from other steps"));
            } else {
                let set = step.inputs.iter().copied().collect();
                out.push_str(&line("Uses", &format!("output of {}", ids(&set))));
            }
            match flow.sensitivity {
                Sensitivity::Public => {}
                Sensitivity::Personal => out.push_str(&line(
                    "Carries",
                    &format!("PERSONAL data from {}", ids(&flow.sources)),
                )),
                Sensitivity::Secret => out.push_str(&line(
                    "Carries",
                    &format!("SECRET data from {}", ids(&flow.sources)),
                )),
            }
            if step.action.is_irreversible() {
                out.push_str(&line("Reversible", "no"));
            }
            // Numbered across the whole review, so what a step displays is what
            // the operator quotes back when approving.
            let mut step_findings: Vec<_> = self
                .numbered()
                .into_iter()
                .filter(|(_, f)| f.step == step.id)
                .collect();
            step_findings.sort_by_key(|(_, f)| core::cmp::Reverse(f.severity));
            for (number, f) in step_findings {
                out.push_str(&format!(
                    "  [{}] {number}:{}: {}\n",
                    f.severity.label(),
                    f.code,
                    f.message
                ));
            }
            if let Some((_, tx)) = self.transactions.iter().find(|(id, _)| *id == step.id) {
                out.push_str("  Transaction review:\n");
                for l in tx.render().lines() {
                    out.push_str("    ");
                    out.push_str(l);
                    out.push('\n');
                }
            }
        }

        let steps = || self.order.iter().filter_map(|&i| plan.steps.get(i));
        let irreversible = steps().filter(|s| s.action.is_irreversible()).count();
        let leaving = steps().filter(|s| s.action.is_egress()).count();
        let money = steps()
            .filter(|s| {
                matches!(
                    s.action,
                    Action::Payment { .. } | Action::SignTransaction { .. }
                )
            })
            .count();
        out.push_str("\n-- Summary --\n");
        out.push_str(&line("Irreversible steps", &format!("{irreversible}")));
        out.push_str(&line("Data leaves device", &format!("{leaving} step(s)")));
        out.push_str(&line("Moves money or funds", &format!("{money} step(s)")));
        let needing: Vec<String> = self
            .required_acknowledgements()
            .into_iter()
            .map(|(n, code)| format!("{n}:{code}"))
            .collect();
        if !needing.is_empty() {
            out.push_str(&line("Must acknowledge", &needing.join(", ")));
        }
        out.push_str(&line(
            "Plan fingerprint",
            &clearsign::hex::encode_prefixed(&self.fingerprint),
        ));

        out.push_str("\n-- Verdict --\n");
        out.push_str(match self.highest_severity() {
            Some(Severity::Critical) => {
                "DO NOT APPROVE unless you intend every CRITICAL item above.\n"
            }
            Some(Severity::Blind) => {
                "DO NOT APPROVE: part of this plan cannot be understood from what it declares.\n"
            }
            Some(Severity::Warning) => "Review each WARNING before approving.\n",
            Some(Severity::Info) | None => {
                "No risks detected. Confirm the steps match what you asked for.\n"
            }
        });
        out
    }
}

fn line(label: &str, value: &str) -> String {
    let dots = W.saturating_sub(label.chars().count()).max(2);
    let mut s = String::from("  ");
    s.push_str(label);
    s.push(' ');
    for _ in 0..dots.saturating_sub(1) {
        s.push('.');
    }
    s.push(' ');
    s.push_str(value);
    s.push('\n');
    s
}

/// A plain sentence describing the action, built only from typed fields.
fn describe(action: &Action) -> String {
    match action {
        Action::Transform { description } => format!(
            "Works on data already gathered, with no outside effect: {}",
            display(description)
        ),
        Action::ReadFile { path, sensitivity } => format!(
            "Reads the file {} ({})",
            display(path),
            match sensitivity {
                Sensitivity::Public => "public data",
                Sensitivity::Personal => "personal data",
                Sensitivity::Secret => "SECRET data",
            }
        ),
        Action::WriteFile { path } => format!("Writes to the file {}", display(path)),
        Action::DeleteFile { path } => format!("Permanently deletes {}", display(path)),
        Action::ReadCredential { name } => {
            format!("Retrieves the stored credential {}", display(name))
        }
        Action::HttpRequest { method, host } => format!(
            "{} {}",
            match method {
                HttpMethod::Get => "Fetches data from",
                HttpMethod::Post => "Sends data to",
                HttpMethod::Put => "Uploads or replaces data on",
                HttpMethod::Delete => "Asks to delete data on",
            },
            display(host)
        ),
        Action::SendMessage { channel, recipient } => {
            format!(
                "Sends a message via {} to {}",
                display(channel),
                display(recipient)
            )
        }
        Action::Payment {
            amount_minor,
            currency,
            payee,
        } => format!(
            "Pays {amount_minor} minor units of {} to {}",
            display(currency),
            display(payee)
        ),
        Action::SignTransaction { unsigned_tx } => {
            format!(
                "Signs a blockchain transaction ({} bytes, decoded below)",
                unsigned_tx.len()
            )
        }
        Action::RunProgram { program } => format!("Runs the program {}", display(program)),
        Action::InstallApp { package } => format!("Installs the app {}", display(package)),
        Action::ChangeSetting { key, value } => {
            format!("Changes the setting {} to {}", display(key), display(value))
        }
    }
}
