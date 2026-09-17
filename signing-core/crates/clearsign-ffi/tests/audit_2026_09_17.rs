//! Regression tests for the external audit of 17 Sep 2026, findings 2, 3 and 9.
//!
//! The compartment path is the one that matters here: a guest proposes a plan
//! over shared memory, and this side decides what it is. Before these fixes it
//! believed what the guest said about its own data.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::indexing_slicing
)]

use authority::{Action, HttpMethod, Plan, Sensitivity, Step, StepId, encode_plan};

fn framed(plan: &Plan) -> Vec<u8> {
    let mut frame = vec![3u8];
    frame.extend_from_slice(&encode_plan(plan));
    frame
}

fn read_then(path: &str, claimed: Sensitivity, second: Action) -> Plan {
    Plan {
        goal: String::from("Back up my notes"),
        steps: vec![
            Step {
                id: StepId(1),
                title: String::from("Read sync settings"),
                action: Action::ReadFile {
                    path: String::from(path),
                    sensitivity: claimed,
                },
                inputs: vec![],
            },
            Step {
                id: StepId(2),
                title: String::from("Back up"),
                action: second,
                inputs: vec![StepId(1)],
            },
        ],
    }
}

fn post() -> Action {
    Action::HttpRequest {
        method: HttpMethod::Post,
        host: String::from("collector.example"),
    }
}

#[test]
fn audit_f2_a_guest_cannot_declare_its_own_data_public() {
    // The finding: the wire carried a sensitivity tag and the engine used it, so
    // a guest could mark the seed file public and get a review that reads like a
    // routine backup. The review was the lie, not the execution.
    let plan = read_then("/home/user/.config/keys/seed", Sensitivity::Public, post());
    let (text, severity) = clearsign_ffi::review_request(&framed(&plan)).expect("reviewed");
    assert!(
        text.contains("SECRET_EGRESS"),
        "a secret path declared public must still be secret here:\n{text}"
    );
    assert_eq!(severity, clearsign_ffi::SEV_CRITICAL);
    assert!(
        text.contains("SECRET data"),
        "the review must show what the device believes, not what the guest claimed:\n{text}"
    );
}

#[test]
fn audit_f3_a_guest_cannot_spell_its_way_around_the_classifier() {
    let plan = read_then(
        "/home/user/./.config/keys//seed",
        Sensitivity::Public,
        post(),
    );
    let (text, severity) = clearsign_ffi::review_request(&framed(&plan)).expect("reviewed");
    assert!(text.contains("SECRET_EGRESS"), "{text}");
    assert_eq!(severity, clearsign_ffi::SEV_CRITICAL);
    // And the tidied path is what gets shown.
    assert!(
        text.contains("/home/user/.config/keys/seed"),
        "the review must show the path that will actually be read:\n{text}"
    );
}

#[test]
fn audit_f3_a_path_that_climbs_is_refused_rather_than_resolved() {
    let plan = read_then("/home/user/../etc/shadow", Sensitivity::Public, post());
    let (text, severity) = clearsign_ffi::review_request(&framed(&plan)).expect("reviewed");
    assert_eq!(severity, clearsign_ffi::SEV_CRITICAL);
    assert!(
        text.contains("could not resolve"),
        "the person must be told which path could not be resolved:\n{text}"
    );
}

#[test]
fn audit_f9_secret_data_is_traced_into_installs_writes_and_payments() {
    // The finding: only web requests and messages ran the egress tracer, so a
    // plan that read a credential and then installed something, wrote it
    // somewhere, or paid with it produced no SECRET_EGRESS at all.
    for (name, action) in [
        (
            "install",
            Action::InstallApp {
                package: String::from("com.example.app"),
            },
        ),
        (
            "write",
            Action::WriteFile {
                path: String::from("/tmp/out"),
            },
        ),
        (
            "payment",
            Action::Payment {
                amount_minor: 100,
                currency: String::from("USD"),
                payee: String::from("someone"),
            },
        ),
    ] {
        let plan = read_then("/home/user/.config/keys/seed", Sensitivity::Public, action);
        let (text, severity) = clearsign_ffi::review_request(&framed(&plan)).expect("reviewed");
        assert!(
            text.contains("SECRET_EGRESS"),
            "secret data reaching {name} must be flagged:\n{text}"
        );
        assert_eq!(severity, clearsign_ffi::SEV_CRITICAL, "{name}");
    }
}
