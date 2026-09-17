//! `authority` — review, approve and run an agent's proposed plan.
//!
//! The point of the tool is the order it enforces: an agent proposes, a person
//! reads what the proposal actually does, acknowledges each serious finding by
//! name, and only then does anything run — and only that exact plan.
//!
//! | code | meaning |
//! |---|---|
//! | 0 | done, or reviewed with nothing above WARNING |
//! | 1 | input error, or the plan was refused |
//! | 2 | the review contains a BLIND finding |
//! | 3 | the review contains a CRITICAL finding |

use std::process::ExitCode;

use authority::{approve_plan, execute, fingerprint, review_plan};
use clearsign::Severity;
use authority_agent::{LocalRunner, Policy, RunnerLimits, plan_from_json};

const USAGE: &str = "\
authority — an agent proposes, you approve, then it runs.

  authority review <PROPOSAL.json>
      Read an agent's proposed tool calls, show what they actually do, and stop.

  authority run <PROPOSAL.json> [--ack <CODE>]... [--perform-file-actions]
      The same review, then run the plan only if every BLIND and CRITICAL
      finding is acknowledged exactly, by code, as `#<step>:<CODE>`.

  authority tools
      List every tool an agent may propose on this device.

Without --perform-file-actions nothing touches the filesystem: steps are
described and the ordering is enforced, but no file is read or written.
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rest: Vec<String> = args.get(1..).unwrap_or(&[]).to_vec();
    match args.first().map(String::as_str) {
        Some("review") => run_command(&rest, false),
        Some("run") => run_command(&rest, true),
        Some("tools") => {
            println!("Tools an agent may propose on this device:\n");
            for (name, what) in authority_agent::tools::TOOLS {
                println!("  {name:<18} {what}");
            }
            println!("\nAnything else is refused, not approximated.");
            ExitCode::SUCCESS
        }
        _ => {
            print!("{USAGE}");
            ExitCode::SUCCESS
        }
    }
}

fn run_command(args: &[String], execute_it: bool) -> ExitCode {
    let mut path = None;
    let mut acks: Vec<String> = Vec::new();
    let mut perform = false;
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--ack" => match it.next() {
                Some(code) => acks.push(code.clone()),
                None => return fail("--ack needs a finding code, as #<step>:<CODE>"),
            },
            "--perform-file-actions" => perform = true,
            other if path.is_none() => path = Some(String::from(other)),
            other => return fail(&format!("unexpected argument {other}")),
        }
    }
    let Some(path) = path else {
        return fail("name the proposal file to read");
    };
    let json = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(e) => return fail(&format!("cannot read {path}: {e}")),
    };

    let policy = Policy::default();
    let plan = match plan_from_json(&json, &policy) {
        Ok(plan) => plan,
        Err(e) => return fail(&format!("{e}")),
    };
    let review = match review_plan(&plan) {
        Ok(review) => review,
        Err(e) => return fail(&format!("the plan is not valid: {e:?}")),
    };
    print!("{}", review.render());

    let worst = review.highest_severity();
    if !execute_it {
        return match worst {
            Some(Severity::Critical) => ExitCode::from(3),
            Some(Severity::Blind) => ExitCode::from(2),
            _ => ExitCode::SUCCESS,
        };
    }

    let ack_refs: Vec<(authority::StepId, &str)> = match parse_acks(&acks) {
        Ok(v) => v,
        Err(message) => return fail(&message),
    };
    let approval = match approve_plan(&review, &ack_refs) {
        Ok(a) => a,
        Err(e) => return fail(&format!("not approved: {e:?}")),
    };
    println!("\n-- Approved --");
    println!(
        "Plan fingerprint ................. 0x{}",
        hex(&fingerprint(&plan))
    );

    let limits = RunnerLimits {
        perform_file_actions: perform,
        ..RunnerLimits::default()
    };
    let mut runner = LocalRunner::new(policy, limits);
    let outcome = execute(&plan, &approval, &mut runner);
    println!("\n-- What ran --");
    for line in &runner.transcript {
        println!("  {line}");
    }
    match outcome {
        Ok(done) => {
            println!("\n{} step(s) completed.", done.len());
            ExitCode::SUCCESS
        }
        Err(e) => {
            println!("\nStopped: {e:?}");
            println!("Nothing after the failing step ran.");
            ExitCode::from(1)
        }
    }
}

/// `--ack #5:SECRET_EGRESS` names the step as well as the finding, so an
/// acknowledgement cannot drift onto a different step of a changed plan.
fn parse_acks(acks: &[String]) -> Result<Vec<(authority::StepId, &str)>, String> {
    let mut out = Vec::with_capacity(acks.len());
    for ack in acks {
        let body = ack
            .strip_prefix('#')
            .ok_or_else(|| format!("acknowledgement {ack:?} should look like #5:SECRET_EGRESS"))?;
        let (step, code) = body
            .split_once(':')
            .ok_or_else(|| format!("acknowledgement {ack:?} should look like #5:SECRET_EGRESS"))?;
        let id: u16 = step
            .parse()
            .map_err(|_| format!("acknowledgement {ack:?} does not name a step number"))?;
        out.push((authority::StepId(id), code));
    }
    Ok(out)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn fail(message: &str) -> ExitCode {
    eprintln!("error: {message}\n\nRun `authority` with no arguments for usage.");
    ExitCode::from(1)
}
