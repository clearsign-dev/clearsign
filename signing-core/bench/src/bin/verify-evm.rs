//! Real transactions from many chains, through the plain-transaction reviewer.
//!
//! For every transaction in `corpus/evm/`, alloy rebuilds the unsigned payload
//! the sender's wallet signed, and the claim is checked before anything else:
//! the signature on the chain must recover, over the keccak of that payload,
//! to the sender. Only then is the payload a known-good input, and only then
//! is it handed to clearsign, which must:
//!
//! - accept it, or refuse it by name (types 0x01, 0x03 and 0x04 are refused by
//!   design today, and that refusal rate on real traffic is part of the result);
//! - when it accepts, commit its signature to exactly the digest the sender
//!   signed;
//! - say something about it, and what it says is tallied by chain.
//!
//! Chain-specific transaction types (OP Stack deposits, Arbitrum's internal
//! transactions, Celo's fee-currency type, zkSync's EIP-712 type and so on)
//! cannot be rebuilt by an Ethereum library, and are counted rather than
//! guessed at. Writes `results/evm.json`.

use std::collections::{BTreeMap, BTreeSet};

use alloy_consensus::transaction::SignerRecoverable;
use alloy_consensus::{SignableTransaction, TxEnvelope};
use alloy_primitives::keccak256;
use alloy_rpc_types_eth::Transaction as RpcTransaction;
use clearsign::Severity;
use clearsign_bench::{Tally, bench_dir, corpus_files, pct, read_jsonl};
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Serialize)]
struct ChainReport {
    chain_id: u64,
    name: String,
    transactions: u64,
    by_type: Tally,
    /// Types an Ethereum library cannot rebuild; mostly system transactions.
    chain_specific: Tally,
    rebuild_failed: Tally,
    /// The rebuilt payload's signature recovered to the sender.
    payload_proven: u64,
    payload_not_proven: u64,
    clearsign_accepted: u64,
    clearsign_refused: Tally,
    digest_equal: u64,
    digest_differs: u64,
    outcome: Tally,
    findings: Tally,
    unknown_selectors: Tally,
    contract_creations: u64,
    native_transfers: u64,
    failures: Vec<String>,
}

fn main() {
    let dir = bench_dir().join("corpus/evm");
    let manifest: Value = serde_json::from_str(
        &std::fs::read_to_string(dir.join("manifest.json")).expect("corpus/evm/manifest.json"),
    )
    .expect("manifest");
    let names: BTreeMap<u64, String> = manifest["chains"]
        .as_array()
        .into_iter()
        .flatten()
        .filter_map(|c| Some((c["chainId"].as_u64()?, c["name"].as_str()?.to_owned())))
        .collect();

    let mut reports: BTreeMap<u64, ChainReport> = BTreeMap::new();
    for file in corpus_files(&dir) {
        for tx in read_jsonl(&file) {
            let chain_id = tx["_chainId"].as_u64().expect("_chainId");
            let r = reports.entry(chain_id).or_insert_with(|| ChainReport {
                chain_id,
                name: names.get(&chain_id).cloned().unwrap_or_default(),
                ..Default::default()
            });
            check(r, &tx);
        }
    }

    print_summary(&reports);
    let out = bench_dir().join("results");
    std::fs::create_dir_all(&out).expect("results dir");
    let json = serde_json::json!({
        "corpus_collected": manifest["collected"],
        "chains": reports.values().collect::<Vec<_>>(),
    });
    std::fs::write(out.join("evm.json"), serde_json::to_string_pretty(&json).unwrap() + "\n")
        .expect("write results/evm.json");

    let bad: u64 = reports.values().map(|r| r.digest_differs).sum();
    if bad > 0 {
        eprintln!("\nFAIL: clearsign committed to a different digest {bad} times.");
        std::process::exit(1);
    }
}

fn check(r: &mut ChainReport, raw: &Value) {
    r.transactions += 1;
    let ty = raw["type"].as_str().unwrap_or("none").to_owned();
    r.by_type.add(ty.clone());
    if !matches!(ty.as_str(), "0x0" | "0x1" | "0x2" | "0x3" | "0x4") {
        r.chain_specific.add(ty);
        return;
    }

    // Some endpoints omit an empty access list on typed transactions. Filling
    // in the empty list is safe to try: if the sender signed anything else,
    // the recovery check below fails and the transaction is not used.
    let mut raw = raw.clone();
    if ty != "0x0" && raw.get("accessList").is_none() {
        raw["accessList"] = serde_json::json!([]);
    }
    let raw = &raw;
    let rpc: RpcTransaction = match serde_json::from_value(raw.clone()) {
        Ok(t) => t,
        Err(e) => {
            let reason: String = format!("type {ty}: {e}").chars().take(80).collect();
            r.rebuild_failed.add(reason);
            return;
        }
    };
    let sender = rpc.inner.signer();
    let envelope: &TxEnvelope = rpc.inner.inner();
    let mut payload = Vec::new();
    match envelope {
        TxEnvelope::Legacy(s) => s.tx().encode_for_signing(&mut payload),
        TxEnvelope::Eip2930(s) => s.tx().encode_for_signing(&mut payload),
        TxEnvelope::Eip1559(s) => s.tx().encode_for_signing(&mut payload),
        TxEnvelope::Eip4844(s) => s.tx().encode_for_signing(&mut payload),
        TxEnvelope::Eip7702(s) => s.tx().encode_for_signing(&mut payload),
    }
    let sighash = keccak256(&payload);
    match envelope.recover_signer() {
        Ok(who) if who == sender && sighash == envelope.signature_hash() => r.payload_proven += 1,
        _ => {
            // A payload we cannot prove the sender signed is not a fair input.
            r.payload_not_proven += 1;
            return;
        }
    }

    let review = match clearsign::review_transaction_bytes(&payload) {
        Ok(rv) => rv,
        Err(e) => {
            r.clearsign_refused.add(e.to_string());
            return;
        }
    };
    r.clearsign_accepted += 1;
    match review.signing_target() {
        Some(t) if t.digest == sighash.0 => r.digest_equal += 1,
        _ => {
            r.digest_differs += 1;
            r.failures.push(format!("digest differs: {}", raw["hash"]));
        }
    }
    let outcome = match review.highest_severity() {
        Some(Severity::Critical) => "critical",
        Some(Severity::Blind) => "blind",
        Some(Severity::Warning) => "warning",
        _ => "clear",
    };
    r.outcome.add(outcome);
    let mut codes = BTreeSet::new();
    let dump = std::env::var("DUMP_CODE").ok();
    for f in review.findings() {
        codes.insert(f.code);
        if dump.as_deref() == Some(f.code) {
            let input = raw["input"].as_str().unwrap_or("");
            eprintln!("{}\t{}\t{}\t{}\t{}", r.chain_id, raw["hash"].as_str().unwrap_or(""),
                input.get(..10).unwrap_or(""), (input.len().saturating_sub(2)) / 2, f.message.chars().take(140).collect::<String>());
        }
        if f.code == "UNKNOWN_SELECTOR" {
            if let Some(sel) = f.message.strip_prefix("Function ").and_then(|m| m.get(..10)) {
                r.unknown_selectors.add(sel);
            }
        }
    }
    if codes.contains("CONTRACT_CREATION") {
        r.contract_creations += 1;
    }
    if review.sections.iter().any(|s| {
        s.fields.iter().any(|(k, v)| k == "Action" && v == "Native value transfer")
    }) {
        r.native_transfers += 1;
    }
    for c in codes {
        r.findings.add(c);
    }
}

fn print_summary(reports: &BTreeMap<u64, ChainReport>) {
    println!(
        "{:<26} {:>6} {:>8} {:>9} {:>9} {:>8} {:>7} {:>7} {:>7} {:>7} {:>9}",
        "chain", "txs", "special", "proven", "accepted", "digest=", "clear", "warn", "blind", "crit", "refused"
    );
    let mut t = ChainReport::default();
    for r in reports.values() {
        println!(
            "{:<26} {:>6} {:>8} {:>9} {:>9} {:>8} {:>7} {:>7} {:>7} {:>7} {:>9}",
            format!("{} ({})", r.name, r.chain_id).chars().take(26).collect::<String>(),
            r.transactions,
            r.chain_specific.total(),
            r.payload_proven,
            r.clearsign_accepted,
            r.digest_equal,
            r.outcome.get("clear"),
            r.outcome.get("warning"),
            r.outcome.get("blind"),
            r.outcome.get("critical"),
            r.clearsign_refused.total(),
        );
        t.transactions += r.transactions;
        t.by_type.merge(&r.by_type);
        t.chain_specific.merge(&r.chain_specific);
        t.rebuild_failed.merge(&r.rebuild_failed);
        t.payload_proven += r.payload_proven;
        t.payload_not_proven += r.payload_not_proven;
        t.clearsign_accepted += r.clearsign_accepted;
        t.clearsign_refused.merge(&r.clearsign_refused);
        t.digest_equal += r.digest_equal;
        t.digest_differs += r.digest_differs;
        t.outcome.merge(&r.outcome);
        t.findings.merge(&r.findings);
        t.unknown_selectors.merge(&r.unknown_selectors);
        t.contract_creations += r.contract_creations;
        t.native_transfers += r.native_transfers;
    }
    println!("\n{} transactions from {} chains", t.transactions, reports.len());
    println!("by type: {:?}", t.by_type.sorted());
    println!("chain-specific types, not rebuildable by an Ethereum library: {:?}", t.chain_specific.sorted());
    println!("rebuild failed: {:?}", t.rebuild_failed.sorted());
    println!("payload proven (signature recovers to sender over keccak of the rebuilt payload): {} (not proven: {})",
        t.payload_proven, t.payload_not_proven);
    println!("clearsign accepted {} of {} proven ({}); refused: {:?}",
        t.clearsign_accepted, t.payload_proven, pct(t.clearsign_accepted, t.payload_proven), t.clearsign_refused.sorted());
    println!("digest equal to what the sender signed: {}/{}", t.digest_equal, t.digest_equal + t.digest_differs);
    println!("outcomes: {:?}", t.outcome.sorted());
    println!("native transfers {}  contract creations {}", t.native_transfers, t.contract_creations);
    println!("finding codes (once per review): {:?}", t.findings.sorted());
    println!("top unknown selectors: {:?}", t.unknown_selectors.sorted().into_iter().take(30).collect::<Vec<_>>());
    for r in reports.values() {
        for f in r.failures.iter().take(5) {
            println!("FAILURE [{}]: {f}", r.name);
        }
    }
}
