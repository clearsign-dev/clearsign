//! `#5:CODE` used to mean step 5. Accepting it now, by stripping the `#` and
//! reading 5 as a finding number, turns an acknowledgement of one thing into an
//! acknowledgement of another — silently, which is the worst way.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use std::process::Command;

fn repo_root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../..")
        .canonicalize()
        .expect("repository root")
}

#[test]
fn the_old_syntax_is_refused_rather_than_reinterpreted() {
    let out = Command::new(env!("CARGO_BIN_EXE_authority"))
        .args([
            "run",
            "crates/authority-agent/examples/injected-proposal.json",
            "--ack",
            "#1:SECRET_EGRESS",
        ])
        .current_dir(repo_root())
        .output()
        .expect("the tool should run");
    let text = String::from_utf8_lossy(&out.stderr);
    assert!(
        !out.status.success(),
        "the old syntax must not be accepted. stderr: {text}"
    );
    assert!(
        text.contains("old form"),
        "it should say why, rather than failing obscurely. stderr: {text}"
    );
}

#[test]
fn the_current_syntax_is_understood() {
    // Same tool, a well-formed acknowledgement that is simply incomplete: it
    // must fail for the right reason, not for the shape of the argument.
    let out = Command::new(env!("CARGO_BIN_EXE_authority"))
        .args([
            "run",
            "crates/authority-agent/examples/injected-proposal.json",
            "--ack",
            "1:SECRET_EGRESS",
        ])
        .current_dir(repo_root())
        .output()
        .expect("the tool should run");
    let text = String::from_utf8_lossy(&out.stderr);
    assert!(
        !text.contains("old form"),
        "a current-form acknowledgement must not be mistaken for the old one. stderr: {text}"
    );
}
