#![no_main]
//! Plans come from an untrusted planner. Build arbitrary plans from fuzzer bytes and
//! require: no panic; deterministic rendering; approval with exactly the required
//! acknowledgements always succeeds; the approved plan executes fully; any
//! single-byte change to a text field changes the fingerprint.

use authority::*;
use libfuzzer_sys::fuzz_target;

struct Bytes<'a>(&'a [u8]);
impl Bytes<'_> {
    fn u8(&mut self) -> u8 {
        let (b, rest) = self.0.split_first().map(|(b, r)| (*b, r)).unwrap_or((0, &[]));
        self.0 = rest;
        b
    }
    fn text(&mut self) -> String {
        let n = usize::from(self.u8() % 24);
        let take = n.min(self.0.len());
        let (a, b) = self.0.split_at(take);
        self.0 = b;
        String::from_utf8_lossy(a).into_owned()
    }
    fn bytes(&mut self) -> Vec<u8> {
        let n = usize::from(self.u8());
        let take = n.min(self.0.len());
        let (a, b) = self.0.split_at(take);
        self.0 = b;
        a.to_vec()
    }
}

struct Echo;
impl StepRunner for Echo {
    fn run(&mut self, step: &Step, inputs: &[(StepId, &Output)]) -> Result<Output, RunError> {
        assert_eq!(inputs.len(), step.inputs.len(), "runner received undeclared inputs");
        Ok(Output(step.id.0.to_be_bytes().to_vec()))
    }
}

fuzz_target!(|data: &[u8]| {
    let mut b = Bytes(data);
    let n = b.u8() % 12;
    let mut steps = Vec::new();
    for i in 0..n {
        let sens = match b.u8() % 3 { 0 => Sensitivity::Public, 1 => Sensitivity::Personal, _ => Sensitivity::Secret };
        let action = match b.u8() % 12 {
            0 => Action::Transform { description: b.text() },
            1 => Action::ReadFile { path: b.text(), sensitivity: sens },
            2 => Action::WriteFile { path: b.text() },
            3 => Action::DeleteFile { path: b.text() },
            4 => Action::ReadCredential { name: b.text() },
            5 => Action::HttpRequest { method: match b.u8() % 4 { 0 => HttpMethod::Get, 1 => HttpMethod::Post, 2 => HttpMethod::Put, _ => HttpMethod::Delete }, host: b.text() },
            6 => Action::SendMessage { channel: b.text(), recipient: b.text() },
            7 => Action::Payment { amount_minor: u64::from(b.u8()), currency: b.text(), payee: b.text() },
            8 => Action::SignTransaction { unsigned_tx: b.bytes() },
            9 => Action::RunProgram { program: b.text() },
            10 => Action::InstallApp { package: b.text() },
            _ => Action::ChangeSetting { key: b.text(), value: b.text() },
        };
        let inputs = (0..b.u8() % 4).map(|_| StepId(u16::from(b.u8() % (n + 2)))).collect();
        steps.push(Step { id: StepId(u16::from(i)), title: b.text(), action, inputs });
    }
    let plan = Plan { goal: b.text(), steps };
    let fp = fingerprint(&plan);

    let Ok(review) = review_plan(&plan) else { return };
    let text = review.render();
    assert_eq!(text, review.render());
    assert!(!text.contains('\u{1b}'), "raw escape character reached the review");

    // Ask the review what it requires rather than deriving it here. The previous
    // version deduplicated by (step, code) — the same assumption the engine was
    // making — so the oracle agreed with the bug and could never have found it.
    let acks = review.required_acknowledgements();
    let approval = approve_plan(&review, &acks).expect("exact acknowledgements must be accepted");
    let out = execute(&plan, &approval, &mut Echo).expect("approved valid plan must run");
    assert_eq!(out.len(), plan.steps.len());

    let mut changed = plan.clone();
    changed.goal.push('x');
    assert_ne!(fingerprint(&changed), fp);
    assert!(matches!(execute(&changed, &approval, &mut Echo), Err(ExecError::PlanChangedAfterApproval)));
});
