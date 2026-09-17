//! Authority engine tests: validation, information flow, risk, rendering,
//! approval binding and fail-closed execution.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use authority::*;
use clearsign::Severity;

fn step(id: u16, title: &str, action: Action, inputs: &[u16]) -> Step {
    Step {
        id: StepId(id),
        title: title.into(),
        action,
        inputs: inputs.iter().map(|&i| StepId(i)).collect(),
    }
}

fn plan(goal: &str, steps: Vec<Step>) -> Plan {
    Plan {
        goal: goal.into(),
        steps,
    }
}

/// The attack this engine exists for: a prompt-injected agent asked to "summarise
/// my GitHub notifications" quietly posts the user's token to an attacker's server.
fn injected_exfiltration() -> Plan {
    plan(
        "Summarise my GitHub notifications",
        vec![
            step(
                1,
                "Load GitHub access",
                Action::ReadCredential {
                    name: "github-token".into(),
                },
                &[],
            ),
            step(
                2,
                "Fetch notifications",
                Action::HttpRequest {
                    method: HttpMethod::Get,
                    host: "api.github.com".into(),
                },
                &[1],
            ),
            step(
                3,
                "Summarise",
                Action::Transform {
                    description: "summarise notifications".into(),
                },
                &[2],
            ),
            step(
                4,
                "Sync summary to backup",
                Action::HttpRequest {
                    method: HttpMethod::Post,
                    host: "notes-backup.example".into(),
                },
                &[3],
            ),
        ],
    )
}

// ------------------------------------------------------------------ information flow

#[test]
fn secret_egress_is_caught_across_intermediate_steps() {
    let p = injected_exfiltration();
    let r = review_plan(&p).unwrap();
    // The token flows 1 -> 2 -> 3 -> 4. Step 4 looks like a harmless "sync" but carries it.
    assert!(r.has(StepId(4), "SECRET_EGRESS"), "{}", r.render());
    let flow = r.flow_of(StepId(4)).unwrap();
    assert_eq!(flow.sensitivity, Sensitivity::Secret);
    assert!(flow.sources.contains(&StepId(1)));
    // Step 2 also sends the token (to GitHub itself) and must be flagged too: the
    // engine cannot know which host is legitimate, so the person decides.
    assert!(r.has(StepId(2), "SECRET_EGRESS"));
    assert_eq!(r.highest_severity(), Some(Severity::Critical));
}

#[test]
fn unrelated_branch_is_not_tainted() {
    let p = plan(
        "Check weather and read my password manager export",
        vec![
            step(
                1,
                "Read export",
                Action::ReadFile {
                    path: "/home/me/passwords.csv".into(),
                    sensitivity: Sensitivity::Secret,
                },
                &[],
            ),
            step(
                2,
                "Get weather",
                Action::HttpRequest {
                    method: HttpMethod::Get,
                    host: "weather.example".into(),
                },
                &[],
            ),
            step(
                3,
                "Show weather",
                Action::Transform {
                    description: "format forecast".into(),
                },
                &[2],
            ),
        ],
    );
    let r = review_plan(&p).unwrap();
    assert!(!r.has(StepId(2), "SECRET_EGRESS"));
    assert_eq!(
        r.flow_of(StepId(3)).unwrap().sensitivity,
        Sensitivity::Public
    );
    assert_eq!(
        r.flow_of(StepId(1)).unwrap().sensitivity,
        Sensitivity::Secret
    );
}

#[test]
fn personal_data_egress_is_a_warning_not_critical() {
    let p = plan(
        "Email Bob a summary of my meeting notes",
        vec![
            step(
                1,
                "Read notes",
                Action::ReadFile {
                    path: "/home/me/notes/standup.md".into(),
                    sensitivity: Sensitivity::Personal,
                },
                &[],
            ),
            step(
                2,
                "Summarise",
                Action::Transform {
                    description: "three bullet summary".into(),
                },
                &[1],
            ),
            step(
                3,
                "Email Bob",
                Action::SendMessage {
                    channel: "email".into(),
                    recipient: "bob@example.com".into(),
                },
                &[2],
            ),
        ],
    );
    let r = review_plan(&p).unwrap();
    assert!(r.has(StepId(3), "PERSONAL_DATA_EGRESS"));
    assert!(r.has(StepId(3), "SENDS_MESSAGE"));
    assert_eq!(r.highest_severity(), Some(Severity::Warning));
    assert!(
        approve_plan(&r, &[]).is_ok(),
        "warnings need no acknowledgement"
    );
}

#[test]
fn flow_merges_multiple_sources() {
    let p = plan(
        "Combine",
        vec![
            step(
                1,
                "a",
                Action::ReadFile {
                    path: "/a".into(),
                    sensitivity: Sensitivity::Personal,
                },
                &[],
            ),
            step(2, "b", Action::ReadCredential { name: "k".into() }, &[]),
            step(
                3,
                "c",
                Action::Transform {
                    description: "merge".into(),
                },
                &[1, 2],
            ),
            step(
                4,
                "d",
                Action::SendMessage {
                    channel: "sms".into(),
                    recipient: "+10000000000".into(),
                },
                &[3],
            ),
        ],
    );
    let r = review_plan(&p).unwrap();
    let f = r.flow_of(StepId(4)).unwrap();
    assert_eq!(f.sensitivity, Sensitivity::Secret);
    assert_eq!(
        f.sources.iter().copied().collect::<Vec<_>>(),
        vec![StepId(1), StepId(2)]
    );
}

// ------------------------------------------------------------------ risk rules

#[test]
fn mislabelled_destructive_step_is_described_by_its_action() {
    let p = plan(
        "Tidy my notes",
        vec![step(
            1,
            "Summarise notes",
            Action::DeleteFile {
                path: "/home/me/notes".into(),
            },
            &[],
        )],
    );
    let r = review_plan(&p).unwrap();
    let text = r.render();
    assert!(r.has(StepId(1), "DELETE_IRREVERSIBLE"));
    let does = text
        .lines()
        .find(|l| l.trim_start().starts_with("Does "))
        .unwrap();
    assert!(
        does.ends_with("Permanently deletes \"/home/me/notes\""),
        "{text}"
    );
    assert!(text.contains("\"Summarise notes\" (unverified)"));
}

#[test]
fn untrusted_text_cannot_inject_terminal_or_bidi_control() {
    let p = plan(
        "Hello\u{1b}[2J",
        vec![step(
            1,
            "safe\u{202e}exe.txt",
            Action::Transform {
                description: "x\u{0}y".into(),
            },
            &[],
        )],
    );
    let text = review_plan(&p).unwrap().render();
    assert!(!text.contains('\u{1b}') && !text.contains('\u{202e}') && !text.contains('\u{0}'));
    assert!(text.contains("\\u{001b}") && text.contains("\\u{202e}") && text.contains("\\u{0000}"));
}

#[test]
fn system_paths_traversal_and_settings() {
    let p = plan(
        "Configure",
        vec![
            step(
                1,
                "w",
                Action::WriteFile {
                    path: "/etc/hosts".into(),
                },
                &[],
            ),
            step(
                2,
                "w2",
                Action::WriteFile {
                    path: "/home/me/../../etc/passwd".into(),
                },
                &[],
            ),
            step(
                3,
                "s",
                Action::ChangeSetting {
                    key: "security.firewall".into(),
                    value: "off".into(),
                },
                &[],
            ),
            step(
                4,
                "s2",
                Action::ChangeSetting {
                    key: "display.theme".into(),
                    value: "dark".into(),
                },
                &[],
            ),
            step(
                5,
                "w3",
                Action::WriteFile {
                    path: "/etcetera/notes.txt".into(),
                },
                &[],
            ),
        ],
    );
    let r = review_plan(&p).unwrap();
    assert!(r.has(StepId(1), "SYSTEM_MODIFICATION"));
    assert!(r.has(StepId(2), "PATH_TRAVERSAL"));
    assert!(r.has(StepId(3), "SECURITY_SETTING_CHANGE"));
    assert!(r.has(StepId(4), "SETTING_CHANGE") && !r.has(StepId(4), "SECURITY_SETTING_CHANGE"));
    assert!(
        !r.has(StepId(5), "SYSTEM_MODIFICATION"),
        "prefix match must respect path boundaries"
    );
}

#[test]
fn programs_are_blind_and_secrets_to_programs_are_critical() {
    let p = plan(
        "Run cleanup",
        vec![
            step(
                1,
                "key",
                Action::ReadCredential {
                    name: "ssh-key".into(),
                },
                &[],
            ),
            step(
                2,
                "run",
                Action::RunProgram {
                    program: "cleanup.sh".into(),
                },
                &[1],
            ),
        ],
    );
    let r = review_plan(&p).unwrap();
    assert!(r.has(StepId(2), "ARBITRARY_PROGRAM"));
    assert!(r.has(StepId(2), "SECRET_TO_PROGRAM"));
}

#[test]
fn transaction_steps_are_reviewed_by_clearsign() {
    // Unsigned EIP-1559 unlimited USDC approval (test vector D, built with `cast to-rlp`).
    let tx = clearsign::hex::decode("0x02f86e0107843b9aca008506fc23ac00830186a094a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4880b844095ea7b30000000000000000000000003fc91a3afd70395cd496c647d5a6cc9d4b2b7fadffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0").unwrap();
    let p = plan(
        "Swap on a DEX",
        vec![
            step(
                1,
                "Approve token",
                Action::SignTransaction { unsigned_tx: tx },
                &[],
            ),
            step(
                2,
                "Garbage",
                Action::SignTransaction {
                    unsigned_tx: vec![0x02, 0xff],
                },
                &[],
            ),
        ],
    );
    let r = review_plan(&p).unwrap();
    assert!(r.has(StepId(1), "SIGNS_TRANSACTION"));
    assert!(r.has(StepId(1), "UNLIMITED_APPROVAL"), "{}", r.render());
    assert!(r.has(StepId(2), "TRANSACTION_UNDECODABLE"));
    assert!(r.render().contains("Transaction review:"));
}

#[test]
fn payments_are_critical() {
    let p = plan(
        "Buy the book",
        vec![step(
            1,
            "Checkout",
            Action::Payment {
                amount_minor: 2499,
                currency: "USD".into(),
                payee: "books.example".into(),
            },
            &[],
        )],
    );
    let r = review_plan(&p).unwrap();
    assert!(r.has(StepId(1), "PAYMENT"));
    assert!(r.render().contains("Pays 2499 minor units of \"USD\""));
}

// ------------------------------------------------------------------ validation

#[test]
fn structural_validation() {
    let t = |id, inputs: &[u16]| {
        step(
            id,
            "t",
            Action::Transform {
                description: "x".into(),
            },
            inputs,
        )
    };
    assert_eq!(
        review_plan(&plan("g", vec![])).err(),
        Some(PlanError::Empty)
    );
    assert_eq!(
        review_plan(&plan("g", vec![t(1, &[]), t(1, &[])])).err(),
        Some(PlanError::DuplicateStepId(StepId(1)))
    );
    assert_eq!(
        review_plan(&plan("g", vec![t(1, &[9])])).err(),
        Some(PlanError::UnknownInput {
            step: StepId(1),
            input: StepId(9)
        })
    );
    assert_eq!(
        review_plan(&plan("g", vec![t(1, &[1])])).err(),
        Some(PlanError::SelfInput(StepId(1)))
    );
    assert_eq!(
        review_plan(&plan("g", vec![t(1, &[]), t(2, &[1, 1])])).err(),
        Some(PlanError::DuplicateInput {
            step: StepId(2),
            input: StepId(1)
        })
    );
    assert_eq!(
        review_plan(&plan("g", vec![t(1, &[2]), t(2, &[1])])).err(),
        Some(PlanError::Cycle)
    );
    assert_eq!(
        review_plan(&plan("g", vec![t(1, &[3]), t(2, &[1]), t(3, &[2])])).err(),
        Some(PlanError::Cycle)
    );
    assert_eq!(
        review_plan(&plan("  ", vec![t(1, &[])])).err(),
        Some(PlanError::EmptyText { step: None })
    );
    let too_many: Vec<Step> = (0..=MAX_STEPS as u16).map(|i| t(i, &[])).collect();
    assert!(matches!(
        review_plan(&plan("g", too_many)),
        Err(PlanError::TooManySteps { .. })
    ));
    let long = "x".repeat(MAX_TEXT_BYTES + 1);
    assert!(matches!(
        review_plan(&plan(&long, vec![t(1, &[])])),
        Err(PlanError::TextTooLong { .. })
    ));
}

#[test]
fn execution_order_follows_dependencies_not_listing_order() {
    let t = |id, inputs: &[u16]| {
        step(
            id,
            "t",
            Action::Transform {
                description: "x".into(),
            },
            inputs,
        )
    };
    let p = plan(
        "g",
        vec![t(30, &[20]), t(20, &[10]), t(10, &[]), t(40, &[])],
    );
    let order = review_plan(&p).unwrap().execution_order();
    let pos = |id| order.iter().position(|s| *s == StepId(id)).unwrap();
    assert!(pos(10) < pos(20) && pos(20) < pos(30));
    // Deterministic: repeated reviews give the same order.
    assert_eq!(order, review_plan(&p).unwrap().execution_order());
}

// ------------------------------------------------------------------ approval

#[test]
fn approval_requires_exact_step_scoped_acknowledgements() {
    let p = injected_exfiltration();
    let r = review_plan(&p).unwrap();
    assert!(matches!(
        approve_plan(&r, &[]),
        Err(ApprovalError::Unacknowledged { .. })
    ));
    // Acknowledging only one of the two secret egress steps is not enough.
    assert!(matches!(
        approve_plan(&r, &[(StepId(4), "SECRET_EGRESS")]),
        Err(ApprovalError::Unacknowledged {
            step: StepId(2),
            ..
        })
    ));
    // Right code, wrong step.
    assert!(
        approve_plan(
            &r,
            &[(StepId(2), "SECRET_EGRESS"), (StepId(3), "SECRET_EGRESS")]
        )
        .is_err()
    );
    // Extra acknowledgement.
    assert_eq!(
        approve_plan(
            &r,
            &[
                (StepId(2), "SECRET_EGRESS"),
                (StepId(4), "SECRET_EGRESS"),
                (StepId(1), "PAYMENT")
            ]
        )
        .err(),
        Some(ApprovalError::UnexpectedAcknowledgement)
    );
    // Duplicate acknowledgement.
    assert_eq!(
        approve_plan(
            &r,
            &[
                (StepId(2), "SECRET_EGRESS"),
                (StepId(4), "SECRET_EGRESS"),
                (StepId(4), "SECRET_EGRESS")
            ]
        )
        .err(),
        Some(ApprovalError::UnexpectedAcknowledgement)
    );
    let ok = approve_plan(
        &r,
        &[(StepId(2), "SECRET_EGRESS"), (StepId(4), "SECRET_EGRESS")],
    )
    .unwrap();
    assert_eq!(ok.fingerprint(), r.fingerprint());
}

// ------------------------------------------------------------------ execution

#[derive(Default)]
struct Recorder {
    calls: Vec<(StepId, Vec<StepId>)>,
    fail_on: Option<StepId>,
}

impl StepRunner for Recorder {
    fn run(&mut self, step: &Step, inputs: &[(StepId, &Output)]) -> Result<Output, RunError> {
        self.calls
            .push((step.id, inputs.iter().map(|(id, _)| *id).collect()));
        if self.fail_on == Some(step.id) {
            return Err(RunError {
                message: "simulated failure".into(),
            });
        }
        let mut data = vec![step.id.0 as u8];
        for (_, out) in inputs {
            data.extend_from_slice(&out.0);
        }
        Ok(Output(data))
    }
}

fn approved_personal_plan() -> Plan {
    plan(
        "Email Bob a summary",
        vec![
            step(
                3,
                "Email",
                Action::SendMessage {
                    channel: "email".into(),
                    recipient: "bob@example.com".into(),
                },
                &[2],
            ),
            step(
                1,
                "Read",
                Action::ReadFile {
                    path: "/n.md".into(),
                    sensitivity: Sensitivity::Personal,
                },
                &[],
            ),
            step(
                2,
                "Summarise",
                Action::Transform {
                    description: "summary".into(),
                },
                &[1],
            ),
        ],
    )
}

#[test]
fn executes_in_order_with_only_declared_inputs() {
    let p = approved_personal_plan();
    let r = review_plan(&p).unwrap();
    let a = approve_plan(&r, &[]).unwrap();
    let mut runner = Recorder::default();
    let out = execute(&p, &a, &mut runner).unwrap();
    assert_eq!(
        runner.calls,
        vec![
            (StepId(1), vec![]),
            (StepId(2), vec![StepId(1)]),
            (StepId(3), vec![StepId(2)])
        ]
    );
    assert_eq!(out.last().unwrap(), &(StepId(3), Output(vec![3, 2, 1])));
}

#[test]
fn a_plan_changed_after_approval_never_runs() {
    let p = approved_personal_plan();
    let r = review_plan(&p).unwrap();
    let a = approve_plan(&r, &[]).unwrap();

    // The planner swaps the recipient after the person approved.
    let mut swapped = p.clone();
    swapped.steps[0].action = Action::SendMessage {
        channel: "email".into(),
        recipient: "attacker@example.com".into(),
    };
    let mut runner = Recorder::default();
    assert_eq!(
        execute(&swapped, &a, &mut runner).err(),
        Some(ExecError::PlanChangedAfterApproval)
    );
    assert!(runner.calls.is_empty(), "no step may run");

    // Even a change to an untrusted label invalidates approval.
    let mut relabelled = p.clone();
    relabelled.steps[1].title = "Read (edited)".into();
    assert_eq!(
        execute(&relabelled, &a, &mut Recorder::default()).err(),
        Some(ExecError::PlanChangedAfterApproval)
    );
}

#[test]
fn first_failure_stops_everything() {
    let p = approved_personal_plan();
    let r = review_plan(&p).unwrap();
    let a = approve_plan(&r, &[]).unwrap();
    let mut runner = Recorder {
        fail_on: Some(StepId(2)),
        ..Default::default()
    };
    assert_eq!(
        execute(&p, &a, &mut runner).err(),
        Some(ExecError::StepFailed {
            step: StepId(2),
            completed: 1
        })
    );
    assert_eq!(
        runner.calls.len(),
        2,
        "the send step must not run after a failure"
    );
}

// ------------------------------------------------------------------ fingerprint

#[test]
fn fingerprint_changes_when_anything_changes() {
    let base = injected_exfiltration();
    let fp = fingerprint(&base);
    let mut variants = Vec::new();
    let mut v = base.clone();
    v.goal.push('!');
    variants.push(v);
    let mut v = base.clone();
    v.steps[3].action = Action::HttpRequest {
        method: HttpMethod::Put,
        host: "notes-backup.example".into(),
    };
    variants.push(v);
    let mut v = base.clone();
    v.steps[3].action = Action::HttpRequest {
        method: HttpMethod::Post,
        host: "notes-backup.exampl".into(),
    };
    variants.push(v);
    let mut v = base.clone();
    v.steps[3].inputs = vec![StepId(2)];
    variants.push(v);
    let mut v = base.clone();
    v.steps[0].id = StepId(9);
    v.steps[1].inputs = vec![StepId(9)];
    variants.push(v);
    let mut v = base.clone();
    v.steps.swap(2, 3);
    variants.push(v);
    // Moving bytes between adjacent fields must not collide (length prefixes).
    let a = plan(
        "g",
        vec![step(
            1,
            "ab",
            Action::Transform {
                description: "c".into(),
            },
            &[],
        )],
    );
    let b = plan(
        "g",
        vec![step(
            1,
            "a",
            Action::Transform {
                description: "bc".into(),
            },
            &[],
        )],
    );
    assert_ne!(fingerprint(&a), fingerprint(&b));
    for (i, v) in variants.iter().enumerate() {
        assert_ne!(fingerprint(v), fp, "variant {i} collided");
    }
}

// ------------------------------------------------------------------ robustness

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n.max(1)
    }
    fn text(&mut self) -> String {
        let len = self.below(12);
        (0..len)
            .map(|_| char::from_u32(self.below(0x2100) as u32).unwrap_or('?'))
            .collect()
    }
}

fn random_action(rng: &mut Rng) -> Action {
    let sens = |r: &mut Rng| match r.below(3) {
        0 => Sensitivity::Public,
        1 => Sensitivity::Personal,
        _ => Sensitivity::Secret,
    };
    match rng.below(12) {
        0 => Action::Transform {
            description: rng.text(),
        },
        1 => Action::ReadFile {
            path: rng.text(),
            sensitivity: sens(rng),
        },
        2 => Action::WriteFile { path: rng.text() },
        3 => Action::DeleteFile { path: rng.text() },
        4 => Action::ReadCredential { name: rng.text() },
        5 => Action::HttpRequest {
            method: HttpMethod::Post,
            host: rng.text(),
        },
        6 => Action::SendMessage {
            channel: rng.text(),
            recipient: rng.text(),
        },
        7 => Action::Payment {
            amount_minor: rng.next(),
            currency: rng.text(),
            payee: rng.text(),
        },
        8 => Action::SignTransaction {
            unsigned_tx: (0..rng.below(80)).map(|_| rng.next() as u8).collect(),
        },
        9 => Action::RunProgram {
            program: rng.text(),
        },
        10 => Action::InstallApp {
            package: rng.text(),
        },
        _ => Action::ChangeSetting {
            key: rng.text(),
            value: rng.text(),
        },
    }
}

#[test]
fn random_plans_never_panic_and_approved_plans_execute() {
    let mut rng = Rng(0x0123_4567_89ab_cdef);
    let mut executed = 0;
    for _ in 0..20_000 {
        let n = rng.below(10) as u16;
        let steps: Vec<Step> = (0..n)
            .map(|i| {
                let inputs: Vec<StepId> = (0..rng.below(3))
                    .map(|_| StepId(rng.below(u64::from(n) + 2) as u16))
                    .collect();
                Step {
                    id: StepId(i),
                    title: rng.text(),
                    action: random_action(&mut rng),
                    inputs,
                }
            })
            .collect();
        let p = Plan {
            goal: rng.text(),
            steps,
        };
        let _ = fingerprint(&p);
        let Ok(r) = review_plan(&p) else { continue };
        let text = r.render();
        assert_eq!(text, r.render(), "rendering must be deterministic");
        let acks: Vec<(StepId, &str)> = r
            .findings()
            .iter()
            .filter(|f| f.severity >= Severity::Blind)
            .map(|f| (f.step, f.code))
            .collect::<std::collections::BTreeSet<_>>()
            .into_iter()
            .collect();
        let a = approve_plan(&r, &acks).unwrap();
        let out = execute(&p, &a, &mut Recorder::default()).unwrap();
        assert_eq!(out.len(), p.steps.len());
        executed += 1;
    }
    assert!(
        executed > 1_000,
        "too few valid random plans exercised: {executed}"
    );
}

#[test]
#[ignore = "prints a sample review; run with --ignored --nocapture"]
fn print_sample_review() {
    println!(
        "{}",
        review_plan(&injected_exfiltration()).unwrap().render()
    );
}
