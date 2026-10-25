//! ClearSign — the desktop application.
//!
//! The window is a window. Every decision about what a transaction does is made
//! here, in Rust, by the same crates the command-line tool uses and the same
//! crates the security review examined. The front end sends a file across and
//! renders the answer; it has no opinion of its own, because a second opinion
//! about what a transaction does is the situation this whole project exists to
//! remove.
//!
//! The Rust reviewer takes text in and gives a review back without networking
//! or key access. The WebView can fetch records from Safe and persist UI settings.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![forbid(unsafe_code)]

use serde_json::{Value, json};

/// Review a Safe transaction given as JSON.
///
/// `chain_id` of 0 means the file must say. `version` is 0 for "work it out
/// from the hash", 1 for v1.1.x, 3 for v1.3.0 and later.
#[tauri::command]
fn review_transaction(
    json_text: String,
    chain_id: u64,
    version: u32,
) -> clearsign_api::ReviewResponse {
    clearsign_api::review_safe_json(&json_text, chain_id, version)
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
