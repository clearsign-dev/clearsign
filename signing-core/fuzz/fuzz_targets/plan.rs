#![no_main]
//! Plans come from an untrusted planner. Build arbitrary plans from fuzzer bytes and
//! require: no panic; deterministic rendering; approval with exactly the required
//! acknowledgements always succeeds; the approved plan executes fully; any
//! single-byte change to a text field changes the fingerprint.

use authority::*;
use clearsign::Severity;
use std::collections::BTreeSet;

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

    // The requirements come from the review, but they are then checked against
    // properties stated here. An oracle that only asks the implementation what
    // it wants and hands it back agrees with whatever the implementation
    // believes — which is how a previous version of this file agreed with a bug
    // for about 6.5 million executions.
    let acks = review.required_acknowledgements();

    // 1. One requirement per finding that needs one. Not fewer: two findings
    //    that look alike are still two findings.
    let expected = review
        .findings()
        .iter()
        .filter(|f| f.severity >= Severity::Blind)
        .count();
    assert_eq!(
        acks.len(),
        expected,
        "requirements were lost between the findings and the list to acknowledge"
    );

    // 2. No two requirements share an identifier. An identifier that names two
    //    findings lets confirming one confirm the other.
    let distinct: BTreeSet<_> = acks.iter().collect();
    assert_eq!(distinct.len(), acks.len(), "two requirements share an identifier");

    // 3. Leaving any single requirement out must refuse. Checked for every one
    //    of them, not just the first.
    for skip in 0..acks.len() {
        let short: Vec<_> = acks
            .iter()
            .enumerate()
            .filter(|(i, _)| *i != skip)
            .map(|(_, a)| *a)
            .collect();
        assert!(
            approve_plan(&review, &short).is_err(),
            "approval accepted a list missing requirement {skip}"
        );
    }

    let approval = approve_plan(&review, &acks).expect("exact acknowledgements must be accepted");
    let out = execute(&plan, &approval, &mut Echo).expect("approved valid plan must run");
    assert_eq!(out.len(), plan.steps.len());

    let mut changed = plan.clone();
    changed.goal.push('x');
    assert_ne!(fingerprint(&changed), fp);
    assert!(matches!(execute(&changed, &approval, &mut Echo), Err(ExecError::PlanChangedAfterApproval)));
});
