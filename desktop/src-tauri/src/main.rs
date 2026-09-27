//! ClearSign — the desktop application.
//!
//! The window is a window. Every decision about what a transaction does is made
//! here, in Rust, by the same crates the command-line tool uses and the same
//! crates the security review examined. The front end sends a file across and
//! renders the answer; it has no opinion of its own, because a second opinion
//! about what a transaction does is the situation this whole project exists to
//! remove.
//!
//! Nothing in this binary opens a network connection, reads a key, or writes to
//! disk. It takes text in and gives a review back.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]

use clearsign::{DomainVersion, Severity};
use serde_json::{Value, json};

/// Review a Safe transaction given as JSON.
///
/// `chain_id` of 0 means the file must say. `version` is 0 for "work it out
/// from the hash", 1 for v1.1.x, 3 for v1.3.0 and later.
#[tauri::command]
fn review_transaction(json_text: String, chain_id: u64, version: u32) -> Value {
    let version = match version {
        1 => Some(DomainVersion::Legacy),
        3 => Some(DomainVersion::V1_3Plus),
        _ => None,
    };
    let chain_id = if chain_id == 0 { None } else { Some(chain_id) };

    match clearsign_safe_json::review_json(&json_text, chain_id, version) {
        Ok(r) => {
            let severity = match r.review.highest_severity() {
                Some(Severity::Critical) => "critical",
                Some(Severity::Blind) => "blind",
                Some(Severity::Warning) => "warning",
                Some(Severity::Info) | None => "clear",
            };
            json!({
                "ok": true,
                "severity": severity,
                "hash": clearsign::hex::encode_prefixed(&r.hash),
                "hashConfirmed": r.hash_confirmed,
                "domain": match r.version {
                    DomainVersion::Legacy => "v1.1.x",
                    DomainVersion::V1_3Plus => "v1.3.0 or later",
                },
                "domainSource": r.version_source,
                "chainIdSource": r.chain_id_source,
                "chainBound": !matches!(r.version, DomainVersion::Legacy),
                "sections": r.review.sections.iter().map(|s| json!({
                    "title": s.title,
                    "fields": s.fields.iter().map(|(k, v)| json!([k, v])).collect::<Vec<_>>(),
                })).collect::<Vec<_>>(),
                "findings": r.review.findings().iter().map(|f| json!({
                    "severity": f.severity.label(),
                    "code": f.code,
                    "message": f.message,
                })).collect::<Vec<_>>(),
                "mustAcknowledge": r.review.required_acknowledgements().into_iter()
                    .map(|(n, code)| json!({ "number": n, "code": code }))
                    .collect::<Vec<_>>(),
                "text": r.review.render(),
            })
        }
        Err(message) => json!({ "ok": false, "severity": "refused", "message": message }),
    }
}

/// What the About panel and the footer say, taken from the build rather than
/// typed into the page, so the window cannot claim a version it is not.
#[tauri::command]
fn build_info() -> Value {
    json!({
        "version": env!("CARGO_PKG_VERSION"),
        "name": env!("CARGO_PKG_NAME"),
    })
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![review_transaction, build_info])
        .run(tauri::generate_context!())
        .expect("the application could not start");
}
