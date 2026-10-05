//! Real Safe transactions from many chains, checked three ways.
//!
//! For every record in `corpus/safe/`:
//!
//! 1. **Hash.** The Safe transaction hash clearsign computes from the signed
//!    fields, under the domain the Safe's version calls for, must equal the
//!    hash Safe's service published *and* the hash alloy computes. Three
//!    implementations, one answer.
//! 2. **Signatures.** Every owner signature that is an ordinary ECDSA
//!    signature must recover, over the hash clearsign computed, to the owner
//!    who gave it. For executed transactions this includes the signatures the
//!    Safe contract itself accepted on-chain: if clearsign's hash were wrong,
//!    they would recover to strangers.
//! 3. **Review.** What the reviewer says about each transaction, by chain:
//!    exit status, finding codes, and which functions it could not read.
//!
//! Writes `results/safe.json` and prints a summary. Exits non-zero if any hash
//! disagrees or any signature recovers to someone other than its owner.

use std::collections::{BTreeMap, BTreeSet};

use alloy_primitives::{Address as AAddress, B256, Bytes, Signature, U256 as AU256, eip191_hash_message};
use alloy_sol_types::{Eip712Domain, SolStruct, sol};
use clearsign::{DomainVersion, Severity};
use clearsign_bench::{Tally, bench_dir, corpus_files, pct, read_jsonl};
use serde::Serialize;
use serde_json::Value;

sol! {
    struct SafeTx {
        address to;
        uint256 value;
        bytes data;
        uint8 operation;
        uint256 safeTxGas;
        uint256 baseGas;
        uint256 gasPrice;
        address gasToken;
        address refundReceiver;
        uint256 nonce;
    }
}

#[derive(Default, Serialize)]
struct ChainReport {
    chain_id: u64,
    name: String,
    safes: BTreeSet<String>,
    multisig_safes: BTreeSet<String>,
    versions: Tally,
    records: u64,
    unsupported_version: u64,
    parse_refused: Tally,
    hash_agree_service: u64,
    hash_disagree_service: u64,
    hash_agree_alloy: u64,
    hash_disagree_alloy: u64,
    auto_domain_agree: u64,
    auto_domain_refused: u64,
    /// Transactions whose owners signed under a different domain than the
    /// Safe's current version uses: signed before the Safe was upgraded.
    signed_before_upgrade: u64,
    domain_from_signatures: u64,
    domain_from_service_hash: u64,
    executed: u64,
    confirmation_sigs_checked: u64,
    confirmation_sigs_recovered_to_owner: u64,
    confirmation_sigs_other_type: u64,
    onchain_sigs_checked: u64,
    onchain_sigs_recovered_to_owner: u64,
    outcome: Tally,
    outcome_multisig_only: Tally,
    findings: Tally,
    critical_excluding_standing: u64,
    unknown_selectors: Tally,
    multisend_decoded: u64,
    multisend_inner_calls: u64,
    failures: Vec<String>,
}

fn domain_for(version: &str) -> Option<DomainVersion> {
    // "1.3.0+L2" and friends: only the numeric part decides the domain.
    let numeric: String = version.chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    let parts: Vec<u32> = numeric.split('.').filter_map(|p| p.parse().ok()).collect();
    match parts.as_slice() {
        [1, 0, ..] => None, // v1.0.0 hashes a different struct
        [1, 1, ..] | [1, 2, ..] => Some(DomainVersion::Legacy),
        [1, minor, ..] if *minor >= 3 => Some(DomainVersion::V1_3Plus),
        _ => None,
    }
}

fn hex_bytes(v: &Value) -> Vec<u8> {
    match v.as_str() {
        Some(s) => clearsign::hex::decode(s).unwrap_or_default(),
        None => Vec::new(),
    }
}

fn addr(s: &str) -> Option<AAddress> {
    s.parse::<AAddress>().ok()
}

fn alloy_hash(tx: &clearsign::SafeTransaction, version: DomainVersion) -> B256 {
    let s = SafeTx {
        to: AAddress::from(tx.to),
        value: AU256::from_be_bytes(tx.value.0),
        data: Bytes::from(tx.data.clone()),
        operation: tx.operation,
        safeTxGas: AU256::from_be_bytes(tx.safe_tx_gas.0),
        baseGas: AU256::from_be_bytes(tx.base_gas.0),
        gasPrice: AU256::from_be_bytes(tx.gas_price.0),
        gasToken: AAddress::from(tx.gas_token),
        refundReceiver: AAddress::from(tx.refund_receiver),
        nonce: AU256::from_be_bytes(tx.nonce.0),
    };
    let domain = Eip712Domain {
        chain_id: match version {
            DomainVersion::V1_3Plus => Some(AU256::from_be_bytes(tx.chain_id.0)),
            DomainVersion::Legacy => None,
        },
        verifying_contract: Some(AAddress::from(tx.safe)),
        ..Default::default()
    };
    s.eip712_signing_hash(&domain)
}

/// Recover the signer of one 65-byte Safe signature over `hash`, when it is
/// an ECDSA signature (v 27/28) or an eth_sign one (v 31/32). Contract
/// signatures (v 0) and pre-approved hashes (v 1) are not recoverable.
fn recover(sig: &[u8], hash: B256) -> Option<AAddress> {
    if sig.len() != 65 {
        return None;
    }
    let v = sig[64];
    let (prehash, v) = match v {
        27 | 28 => (hash, v),
        31 | 32 => (eip191_hash_message(hash), v - 4),
        _ => return None,
    };
    let mut raw = [0u8; 65];
    raw.copy_from_slice(sig);
    raw[64] = v;
    let sig = Signature::try_from(&raw[..]).ok()?;
    sig.recover_address_from_prehash(&prehash).ok()
}

/// The fixed 65-byte parts of a packed Safe signatures blob. A contract
/// signature's `s` points into a dynamic tail, so parsing stops where the
/// first tail begins.
fn static_parts(blob: &[u8]) -> Vec<&[u8]> {
    let mut out = Vec::new();
    let mut boundary = blob.len();
    let mut i = 0;
    while i + 65 <= boundary {
        let part = &blob[i..i + 65];
        if part[64] == 0 {
            let offset = AU256::from_be_slice(&part[32..64]);
            if let Ok(off) = usize::try_from(offset) {
                boundary = boundary.min(off);
            }
        }
        out.push(part);
        i += 65;
    }
    out
}

fn main() {
    let dir = bench_dir().join("corpus/safe");
    let manifest: Value = serde_json::from_str(
        &std::fs::read_to_string(dir.join("manifest.json")).expect("corpus/safe/manifest.json"),
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
        for rec in read_jsonl(&file) {
            let chain_id = rec["_chainId"].as_u64().expect("_chainId");
            let r = reports.entry(chain_id).or_insert_with(|| ChainReport {
                chain_id,
                name: names.get(&chain_id).cloned().unwrap_or_default(),
                ..Default::default()
            });
            check(r, &rec, chain_id);
        }
    }

    print_summary(&reports);
    let out = bench_dir().join("results");
    std::fs::create_dir_all(&out).expect("results dir");
    let json = serde_json::json!({
        "corpus_collected": manifest["collected"],
        "chains": reports.values().collect::<Vec<_>>(),
    });
    std::fs::write(out.join("safe.json"), serde_json::to_string_pretty(&json).unwrap() + "\n")
        .expect("write results/safe.json");

    let bad: u64 = reports
        .values()
        .map(|r| {
            r.hash_disagree_service
                + r.hash_disagree_alloy
                + (r.confirmation_sigs_checked - r.confirmation_sigs_recovered_to_owner)
                + (r.onchain_sigs_checked - r.onchain_sigs_recovered_to_owner)
        })
        .sum();
    if bad > 0 {
        eprintln!("\nFAIL: {bad} disagreements. Each one is a correctness bug until shown otherwise.");
        std::process::exit(1);
    }
}

fn check(r: &mut ChainReport, rec: &Value, chain_id: u64) {
    r.records += 1;
    let safe = rec["safe"].as_str().unwrap_or_default().to_owned();
    let version = rec["_safeVersion"].as_str().unwrap_or("?").to_owned();
    r.versions.add(version.clone());
    r.safes.insert(safe.clone());
    let threshold = rec["_threshold"].as_u64().unwrap_or(0);
    if threshold >= 2 {
        r.multisig_safes.insert(safe.clone());
    }

    let Some(current) = domain_for(&version) else {
        r.unsupported_version += 1;
        return;
    };

    // The same entry point the application and the command-line tool use.
    let text = rec.to_string();
    let parsed = match clearsign_safe_json::parse(&text, Some(chain_id)) {
        Ok(p) => p,
        Err(e) => {
            let reason: String = e.chars().take(60).collect();
            r.parse_refused.add(reason);
            return;
        }
    };
    let tx = parsed.tx;

    // Which domain did the owners actually sign under? A Safe's version is
    // what it runs today; a transaction from before an upgrade was signed
    // under the domain it had then. So the domain is settled per transaction,
    // by the owners' own ECDSA signatures, without asking the service.
    let recovering = |d: DomainVersion| -> usize {
        let h = B256::from(clearsign::safe_transaction_hash(&tx, d));
        rec["confirmations"]
            .as_array()
            .into_iter()
            .flatten()
            .filter(|c| {
                let owner = c["owner"].as_str().and_then(addr);
                owner.is_some() && recover(&hex_bytes(&c["signature"]), h) == owner
            })
            .count()
    };
    let (by_new, by_old) = (recovering(DomainVersion::V1_3Plus), recovering(DomainVersion::Legacy));
    let domain = if by_new > 0 && by_old == 0 {
        r.domain_from_signatures += 1;
        DomainVersion::V1_3Plus
    } else if by_old > 0 && by_new == 0 {
        r.domain_from_signatures += 1;
        DomainVersion::Legacy
    } else {
        // No ordinary signatures to settle it (contract signatures or
        // pre-approved hashes only): fall back to the service's hash, and
        // failing that to the current version.
        r.domain_from_service_hash += 1;
        match parsed.claimed_hash {
            Some(c) if c == clearsign::safe_transaction_hash(&tx, DomainVersion::Legacy) => DomainVersion::Legacy,
            Some(c) if c == clearsign::safe_transaction_hash(&tx, DomainVersion::V1_3Plus) => DomainVersion::V1_3Plus,
            _ => current,
        }
    };
    if domain != current {
        r.signed_before_upgrade += 1;
    }
    let ours = clearsign::safe_transaction_hash(&tx, domain);
    let ours_b = B256::from(ours);

    // 1. Hash, three ways.
    match parsed.claimed_hash {
        Some(claimed) if claimed == ours => r.hash_agree_service += 1,
        Some(claimed) => {
            r.hash_disagree_service += 1;
            r.failures.push(format!(
                "service hash disagrees: {} nonce {} service {} clearsign {}",
                safe,
                tx.nonce.to_decimal(),
                clearsign::hex::encode_prefixed(&claimed),
                clearsign::hex::encode_prefixed(&ours)
            ));
        }
        None => {}
    }
    if alloy_hash(&tx, domain) == ours_b {
        r.hash_agree_alloy += 1;
    } else {
        r.hash_disagree_alloy += 1;
        r.failures.push(format!("alloy hash disagrees: {safe} nonce {}", tx.nonce.to_decimal()));
    }
    // What a user gets without saying which version their Safe is.
    match clearsign_safe_json::review_json(&text, Some(chain_id), None) {
        Ok(rv) if rv.version == domain => r.auto_domain_agree += 1,
        Ok(_) => r.failures.push(format!("auto-detected the wrong domain: {safe}")),
        Err(_) => r.auto_domain_refused += 1,
    }

    // 2. Signatures over our hash.
    for c in rec["confirmations"].as_array().into_iter().flatten() {
        let sig = hex_bytes(&c["signature"]);
        let Some(owner) = c["owner"].as_str().and_then(addr) else { continue };
        match recover(&sig, ours_b) {
            Some(who) => {
                r.confirmation_sigs_checked += 1;
                if who == owner {
                    r.confirmation_sigs_recovered_to_owner += 1;
                } else {
                    r.failures.push(format!("confirmation by {owner} recovers to {who}: {safe}"));
                }
            }
            None => r.confirmation_sigs_other_type += 1,
        }
    }
    if rec["isExecuted"].as_bool() == Some(true) {
        r.executed += 1;
        let owners: BTreeSet<AAddress> = rec["_owners"]
            .as_array()
            .into_iter()
            .flatten()
            .chain(rec["confirmations"].as_array().into_iter().flatten().map(|c| &c["owner"]))
            .filter_map(|o| o.as_str().and_then(addr))
            .collect();
        let blob = hex_bytes(&rec["signatures"]);
        for part in static_parts(&blob) {
            if let Some(who) = recover(part, ours_b) {
                r.onchain_sigs_checked += 1;
                if owners.contains(&who) {
                    r.onchain_sigs_recovered_to_owner += 1;
                } else {
                    r.failures.push(format!("executed signature recovers to non-owner {who}: {safe}"));
                }
            }
        }
    }

    // 3. What the reviewer says.
    let review = clearsign::review_safe_transaction(&tx, domain);
    let outcome = match review.highest_severity() {
        Some(Severity::Critical) => "critical",
        Some(Severity::Blind) => "blind",
        Some(Severity::Warning) => "warning",
        _ => "clear",
    };
    r.outcome.add(outcome);
    if threshold >= 2 {
        r.outcome_multisig_only.add(outcome);
    }
    let mut codes = BTreeSet::new();
    let mut transaction_critical = false;
    for f in review.findings() {
        codes.insert(f.code);
        if f.severity == Severity::Critical && f.code != "SIGNATURE_NOT_CHAIN_BOUND" {
            transaction_critical = true;
        }
        if f.code == "UNKNOWN_SELECTOR" {
            if let Some(sel) = f.message.strip_prefix("Function ").and_then(|m| m.get(..10)) {
                r.unknown_selectors.add(sel);
            }
        }
    }
    if transaction_critical {
        r.critical_excluding_standing += 1;
    }
    for c in codes {
        r.findings.add(c);
    }
    if review.has("SAFE_MULTISEND_BATCH") {
        r.multisend_decoded += 1;
        r.multisend_inner_calls += review
            .sections
            .iter()
            .filter(|s| s.title.starts_with("Batch call "))
            .count() as u64;
    }
}

fn print_summary(reports: &BTreeMap<u64, ChainReport>) {
    println!(
        "{:<24} {:>6} {:>5} {:>8} {:>9} {:>9} {:>10} {:>7} {:>7} {:>7} {:>7}",
        "chain", "recs", "safes", "hash ok", "alloy ok", "sigs ok", "on-chain", "clear", "warn", "blind", "crit"
    );
    let mut t = ChainReport::default();
    for r in reports.values() {
        println!(
            "{:<24} {:>6} {:>5} {:>8} {:>9} {:>9} {:>10} {:>7} {:>7} {:>7} {:>7}",
            format!("{} ({})", r.name, r.chain_id).chars().take(24).collect::<String>(),
            r.records,
            r.safes.len(),
            format!("{}/{}", r.hash_agree_service, r.hash_agree_service + r.hash_disagree_service),
            format!("{}/{}", r.hash_agree_alloy, r.hash_agree_alloy + r.hash_disagree_alloy),
            format!("{}/{}", r.confirmation_sigs_recovered_to_owner, r.confirmation_sigs_checked),
            format!("{}/{}", r.onchain_sigs_recovered_to_owner, r.onchain_sigs_checked),
            r.outcome.get("clear"),
            r.outcome.get("warning"),
            r.outcome.get("blind"),
            r.outcome.get("critical"),
        );
        t.records += r.records;
        t.safes.extend(r.safes.iter().cloned());
        t.multisig_safes.extend(r.multisig_safes.iter().cloned());
        t.unsupported_version += r.unsupported_version;
        t.parse_refused.merge(&r.parse_refused);
        t.hash_agree_service += r.hash_agree_service;
        t.hash_disagree_service += r.hash_disagree_service;
        t.hash_agree_alloy += r.hash_agree_alloy;
        t.hash_disagree_alloy += r.hash_disagree_alloy;
        t.auto_domain_agree += r.auto_domain_agree;
        t.auto_domain_refused += r.auto_domain_refused;
        t.signed_before_upgrade += r.signed_before_upgrade;
        t.domain_from_signatures += r.domain_from_signatures;
        t.domain_from_service_hash += r.domain_from_service_hash;
        t.executed += r.executed;
        t.confirmation_sigs_checked += r.confirmation_sigs_checked;
        t.confirmation_sigs_recovered_to_owner += r.confirmation_sigs_recovered_to_owner;
        t.confirmation_sigs_other_type += r.confirmation_sigs_other_type;
        t.onchain_sigs_checked += r.onchain_sigs_checked;
        t.onchain_sigs_recovered_to_owner += r.onchain_sigs_recovered_to_owner;
        t.outcome.merge(&r.outcome);
        t.outcome_multisig_only.merge(&r.outcome_multisig_only);
        t.findings.merge(&r.findings);
        t.critical_excluding_standing += r.critical_excluding_standing;
        t.unknown_selectors.merge(&r.unknown_selectors);
        t.multisend_decoded += r.multisend_decoded;
        t.multisend_inner_calls += r.multisend_inner_calls;
        t.versions.merge(&r.versions);
    }
    let checked = t.hash_agree_service + t.hash_disagree_service;
    println!("\n{} records, {} chains, {} Safes ({} with threshold of two or more)",
        t.records, reports.len(), t.safes.len(), t.multisig_safes.len());
    println!("versions: {:?}", t.versions.sorted());
    println!("unsupported version (v1.0.0 or unknown): {}", t.unsupported_version);
    println!("refused by the JSON reader: {:?}", t.parse_refused.sorted());
    println!("hash = Safe service: {}/{} ({})", t.hash_agree_service, checked, pct(t.hash_agree_service, checked));
    println!("hash = alloy:        {}/{}", t.hash_agree_alloy, t.hash_agree_alloy + t.hash_disagree_alloy);
    println!("domain auto-detected correctly: {}  refused: {}", t.auto_domain_agree, t.auto_domain_refused);
    println!("domain settled by the owners' signatures: {}; by the service hash: {}", t.domain_from_signatures, t.domain_from_service_hash);
    println!("signed under an earlier domain than the Safe's current version (before an upgrade): {}", t.signed_before_upgrade);
    println!("owner confirmations recovering to their owner over clearsign's hash: {}/{} (+{} contract/approved-hash)",
        t.confirmation_sigs_recovered_to_owner, t.confirmation_sigs_checked, t.confirmation_sigs_other_type);
    println!("signatures accepted on-chain recovering to an owner over clearsign's hash: {}/{} across {} executed",
        t.onchain_sigs_recovered_to_owner, t.onchain_sigs_checked, t.executed);
    println!("outcomes: {:?}", t.outcome.sorted());
    println!("outcomes, threshold >= 2 only: {:?}", t.outcome_multisig_only.sorted());
    println!("CRITICAL about the transaction itself (not the v1.1.x chain-binding property): {}", t.critical_excluding_standing);
    println!("MultiSend batches decoded: {} ({} inner calls shown)", t.multisend_decoded, t.multisend_inner_calls);
    println!("finding codes (once per review): {:?}", t.findings.sorted());
    println!("top unknown selectors: {:?}", t.unknown_selectors.sorted().into_iter().take(25).collect::<Vec<_>>());
    for r in reports.values() {
        for f in r.failures.iter().take(5) {
            println!("FAILURE [{}]: {f}", r.name);
        }
    }
}
