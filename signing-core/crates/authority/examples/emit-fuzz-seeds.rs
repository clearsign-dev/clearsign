//! Seed inputs for the `plan_wire` fuzz target: valid plans covering every
//! action, so the fuzzer starts inside the format instead of guessing its way in.
//!
//! Usage, from signing-core: cargo run -p authority --example emit-fuzz-seeds
#![allow(clippy::unwrap_used, clippy::print_stdout)]

fn main() {
    use authority::*;
    let plans = [
        Plan { goal: "g".into(), steps: vec![Step { id: StepId(1), title: "t".into(), action: Action::Payment { amount_minor: 1000, currency: "USD".into(), payee: "shop".into() }, inputs: vec![] }] },
        Plan { goal: "g".into(), steps: vec![
            Step { id: StepId(1), title: "a".into(), action: Action::ReadCredential { name: "k".into() }, inputs: vec![] },
            Step { id: StepId(2), title: "b".into(), action: Action::SendMessage { channel: "sms".into(), recipient: "+1".into() }, inputs: vec![StepId(1)] }] },
        Plan { goal: "g".into(), steps: vec![Step { id: StepId(1), title: "t".into(), action: Action::SignTransaction { unsigned_tx: vec![2, 0xf8, 0x6d] }, inputs: vec![] }] },
        Plan { goal: "g".into(), steps: vec![Step { id: StepId(1), title: "t".into(), action: Action::DeleteFile { path: "/x".into() }, inputs: vec![] }] },
    ];
    for (i, p) in plans.iter().enumerate() {
        std::fs::write(format!("fuzz/corpus/plan_wire/seed-{i}"), encode_plan(p)).unwrap();
    }
    println!("wrote {} seeds", plans.len());
}
