//! What an agent is and is not allowed to talk this device into.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::indexing_slicing)]

use authority::{Action, Sensitivity, fingerprint, review_plan};
use authority_agent::{AdapterError, Policy, plan_from_json};
use clearsign::Severity;

const INJECTED: &str = include_str!("../examples/injected-proposal.json");

fn plan(json: &str) -> Result<authority::Plan, AdapterError> {
    plan_from_json(json, &Policy::default())
}

#[test]
fn the_planner_does_not_get_to_say_how_sensitive_data_is() {
    // The proposal claims the notes are "public". Local policy says otherwise,
    // and local policy wins. An agent that could classify its own inputs could
    // label a credential public and walk it straight past the flow tracing.
    let p = plan(INJECTED).unwrap();
    match &p.steps[0].action {
        Action::ReadFile { path, sensitivity } => {
            assert_eq!(path, "/home/user/notes/week");
            assert_eq!(*sensitivity, Sensitivity::Personal);
        }
        other => panic!("expected a file read, got {other:?}"),
    }
    match &p.steps[3].action {
        Action::ReadFile { sensitivity, .. } => assert_eq!(*sensitivity, Sensitivity::Secret),
        other => panic!("expected a file read, got {other:?}"),
    }
}

#[test]
fn the_injected_plan_is_critical_and_names_the_destination() {
    let p = plan(INJECTED).unwrap();
    let review = review_plan(&p).unwrap();
    assert_eq!(review.highest_severity(), Some(Severity::Critical));
    let text = review.render();
    assert!(text.contains("SECRET_EGRESS"), "{text}");
    assert!(text.contains("notes-backup.example"), "{text}");
    // The label the planner chose is shown, but never used to describe the step.
    assert!(text.contains("\"Back up the summary\" (unverified)"), "{text}");
    assert!(text.contains("Sends data to \"notes-backup.example\""), "{text}");
}

#[test]
fn a_tool_this_device_does_not_have_is_refused() {
    let json = r#"{"goal":"g","steps":[{"id":1,"label":"x","tool":"exfiltrate","arguments":{}}]}"#;
    match plan(json) {
        Err(AdapterError::UnknownTool(t)) => assert_eq!(t, "exfiltrate"),
        other => panic!("expected a refusal, got {other:?}"),
    }
}

#[test]
fn a_known_tool_with_nonsense_arguments_is_refused() {
    let json = r#"{"goal":"g","steps":[{"id":1,"label":"x","tool":"payment",
        "arguments":{"amount_minor":"lots","currency":"USD","payee":"someone"}}]}"#;
    assert!(matches!(plan(json), Err(AdapterError::BadArguments { .. })));

    let json = r#"{"goal":"g","steps":[{"id":1,"label":"x","tool":"http_request",
        "arguments":{"method":"TRACE","host":"h"}}]}"#;
    assert!(matches!(plan(json), Err(AdapterError::BadArguments { .. })));
}

#[test]
fn a_step_that_uses_an_unproposed_step_is_refused() {
    let json = r#"{"goal":"g","steps":[{"id":1,"label":"x","tool":"transform",
        "arguments":{"description":"d"},"uses":[9]}]}"#;
    assert!(matches!(plan(json), Err(AdapterError::UnknownStep(9))));
}

#[test]
fn a_proposal_that_is_not_a_plan_is_refused() {
    assert!(matches!(plan("not json at all"), Err(AdapterError::NotJson(_))));
    assert!(matches!(plan(r#"{"steps":[]}"#), Err(AdapterError::Shape(_))));
    assert!(matches!(plan(r#"{"goal":"g","steps":[]}"#), Err(AdapterError::Shape(_))));
}

#[test]
fn an_oversized_proposal_is_refused_before_review() {
    let mut steps = Vec::new();
    for id in 1..=(authority::MAX_STEPS + 1) {
        steps.push(format!(
            r#"{{"id":{id},"label":"s","tool":"transform","arguments":{{"description":"d"}}}}"#
        ));
    }
    let json = format!(r#"{{"goal":"g","steps":[{}]}}"#, steps.join(","));
    assert!(matches!(plan(&json), Err(AdapterError::TooLarge(_))));
}

#[test]
fn a_path_that_walks_upwards_is_outside_the_allowed_roots() {
    let policy = Policy::default();
    assert!(policy.allows("/home/user/notes/week"));
    assert!(!policy.allows("/home/user/../etc/shadow"));
    assert!(!policy.allows("/etc/shadow"));
    // And a traversal cannot launder a secret path into a public one.
    assert_eq!(
        policy.classify("/home/user/.ssh/../.ssh/id_ed25519"),
        Sensitivity::Public,
        "a traversing path must not be classified by where it appears to end up"
    );
    assert_eq!(policy.classify("/home/user/.ssh/id_ed25519"), Sensitivity::Secret);
}

#[test]
fn changing_the_proposal_changes_what_approval_would_cover() {
    let original = plan(INJECTED).unwrap();
    let edited_json = INJECTED.replace("notes-backup.example", "notes-backup.example.evil");
    let edited = plan(&edited_json).unwrap();
    assert_ne!(
        fingerprint(&original),
        fingerprint(&edited),
        "a changed destination must change the fingerprint an approval binds to"
    );
}

#[test]
fn the_runner_will_not_read_a_secret_file_for_an_agent() {
    use authority::{StepRunner, Step, StepId, Action, Sensitivity};
    use authority_agent::{LocalRunner, RunnerLimits};

    let mut runner = LocalRunner::new(Policy::default(), RunnerLimits::default());
    let step = Step {
        id: StepId(1),
        title: String::from("Load sync settings"),
        action: Action::ReadFile {
            path: String::from("/home/user/.config/keys/notes-sync-token"),
            sensitivity: Sensitivity::Secret,
        },
        inputs: vec![],
    };
    let err = runner.run(&step, &[]).unwrap_err();
    assert!(err.message.contains("credential"), "{}", err.message);
}
