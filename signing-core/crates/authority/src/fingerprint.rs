//! A plan's fingerprint: Keccak-256 over an unambiguous canonical encoding.
//!
//! Every variable-length field is length-prefixed and every action carries a
//! distinct tag, so two different plans can never encode to the same bytes.

use alloc::vec::Vec;

use crate::plan::{Action, HttpMethod, Plan, Sensitivity};

pub(crate) const DOMAIN: &[u8] = b"authority/plan/v1";

pub fn fingerprint(plan: &Plan) -> [u8; 32] {
    clearsign::keccak::keccak256(&encode(plan))
}

pub(crate) fn encode(plan: &Plan) -> Vec<u8> {
    let mut out = Vec::new();
    put_bytes(&mut out, DOMAIN);
    put_bytes(&mut out, plan.goal.as_bytes());
    put_u32(&mut out, plan.steps.len());
    for step in &plan.steps {
        out.extend_from_slice(&step.id.0.to_be_bytes());
        put_bytes(&mut out, step.title.as_bytes());
        out.push(step.action.tag());
        match &step.action {
            Action::Transform { description } => put_bytes(&mut out, description.as_bytes()),
            Action::ReadFile { path, sensitivity } => {
                put_bytes(&mut out, path.as_bytes());
                out.push(match sensitivity {
                    Sensitivity::Public => 0,
                    Sensitivity::Personal => 1,
                    Sensitivity::Secret => 2,
                });
            }
            Action::WriteFile { path } | Action::DeleteFile { path } => {
                put_bytes(&mut out, path.as_bytes());
            }
            Action::ReadCredential { name } => put_bytes(&mut out, name.as_bytes()),
            Action::HttpRequest { method, host } => {
                out.push(match method {
                    HttpMethod::Get => 0,
                    HttpMethod::Post => 1,
                    HttpMethod::Put => 2,
                    HttpMethod::Delete => 3,
                });
                put_bytes(&mut out, host.as_bytes());
            }
            Action::SendMessage { channel, recipient } => {
                put_bytes(&mut out, channel.as_bytes());
                put_bytes(&mut out, recipient.as_bytes());
            }
            Action::Payment {
                amount_minor,
                currency,
                payee,
            } => {
                out.extend_from_slice(&amount_minor.to_be_bytes());
                put_bytes(&mut out, currency.as_bytes());
                put_bytes(&mut out, payee.as_bytes());
            }
            Action::SignTransaction { unsigned_tx } => put_bytes(&mut out, unsigned_tx),
            Action::RunProgram { program } => put_bytes(&mut out, program.as_bytes()),
            Action::InstallApp { package } => put_bytes(&mut out, package.as_bytes()),
            Action::ChangeSetting { key, value } => {
                put_bytes(&mut out, key.as_bytes());
                put_bytes(&mut out, value.as_bytes());
            }
        }
        put_u32(&mut out, step.inputs.len());
        for input in &step.inputs {
            out.extend_from_slice(&input.0.to_be_bytes());
        }
    }
    out
}

fn put_u32(out: &mut Vec<u8>, n: usize) {
    // Lengths are bounded by validation; saturate rather than wrap if called on an invalid plan.
    let n = u32::try_from(n).unwrap_or(u32::MAX);
    out.extend_from_slice(&n.to_be_bytes());
}

fn put_bytes(out: &mut Vec<u8>, b: &[u8]) {
    put_u32(out, b.len());
    out.extend_from_slice(b);
}
