//! The transactions real attacks got people to sign, shown to the reviewer.
//!
//! Each payload is rebuilt from the chain and proven before it is used:
//!
//! - A Safe attack is rebuilt from the `execTransaction` calldata the chain
//!   recorded, decoded by alloy (not by clearsign). Its nonce and contract
//!   version are not in the calldata, so they are found by search: the right
//!   ones are those whose Safe transaction hash equals the hash the Safe
//!   contract itself emitted in `ExecutionSuccess`. The owners' signatures must
//!   then recover over that hash, which shows it is what they signed.
//! - A transaction signed by a person is rebuilt by alloy and must recover to
//!   its sender, as in `verify-evm`.
//!
//! Only proven payloads are reviewed. What the reviewer says is recorded with
//! the finding that decided it, and whether that finding names the mechanism
//! of the attack or merely refuses to read it.
//!
//! Writes `results/detect.json`.

use std::collections::BTreeMap;

use alloy_consensus::transaction::SignerRecoverable;
use alloy_consensus::{SignableTransaction, TxEnvelope};
use alloy_primitives::{Address as AAddress, B256, Signature, eip191_hash_message, keccak256};
use alloy_rpc_types_eth::Transaction as RpcTransaction;
use alloy_sol_types::{SolCall, sol};
use clearsign::{DomainVersion, Review, SafeTransaction, Severity, U256};
use clearsign_bench::{bench_dir, read_jsonl};
use serde::Serialize;
use serde_json::Value;

sol! {
    function execTransaction(
        address to,
        uint256 value,
        bytes data,
        uint8 operation,
        uint256 safeTxGas,
        uint256 baseGas,
        uint256 gasPrice,
        address gasToken,
        address refundReceiver,
        bytes signatures
    );
}

const EXECUTION_SUCCESS: &str = "0x442e715f626346e8c54381002da614f62bee8d27386535b2521ec8540898556e";

/// Findings that name what an attack does, as opposed to refusing to read it.
const SPECIFIC: &[&str] = &[
    "SAFE_DELEGATECALL",
    "SAFE_IMPLEMENTATION_CHANGE",
    "SAFE_OWNER_CHANGE",
    "SAFE_THRESHOLD_CHANGE",
    "SAFE_MODULE_CHANGE",
    "SAFE_GUARD_CHANGE",
    "SAFE_FALLBACK_HANDLER_CHANGE",
    "UNLIMITED_APPROVAL",
    "APPROVAL_FOR_ALL",
    "OWNERSHIP_TRANSFER",
    "OWNERSHIP_RENOUNCE",
    "ROLE_GRANT",
    "PROXY_UPGRADE",
    "PROXY_ADMIN_CHANGE",
    "ACCOUNT_DELEGATION",
    "MULTISEND_WRONG_CHAIN",
];

#[derive(Serialize)]
struct Outcome {
    case: String,
    incident: String,
    vector: String,
    chain_id: u64,
    tx: String,
    loss_usd: Option<f64>,
    proof: String,
    safe_version: Option<String>,
    signers_recovered: usize,
    verdict: String,
    decisive: Vec<String>,
    specific: bool,
    all_codes: Vec<String>,
    key_lines: Vec<String>,
}

fn main() {
    let rows = read_jsonl(&bench_dir().join("corpus/attacks/attacks.jsonl.gz"));
    let mut outcomes = Vec::new();
    for row in &rows {
        let case = &row["case"];
        let kind = case["kind"].as_str().unwrap_or("");
        let chain_id = row["chainId"].as_u64().unwrap_or(0);
        let mut o = Outcome {
            case: case["id"].as_str().unwrap_or("").into(),
            incident: case["incident"].as_str().unwrap_or("").into(),
            vector: case["vector"].as_str().unwrap_or("").into(),
            chain_id,
            tx: row["hash"].as_str().unwrap_or("").into(),
            loss_usd: case["loss_usd"].as_f64(),
            proof: String::new(),
            safe_version: None,
            signers_recovered: 0,
            verdict: String::new(),
            decisive: Vec::new(),
            specific: false,
            all_codes: Vec::new(),
            key_lines: Vec::new(),
        };
        let review = match kind {
            "safe_exec" => safe_case(row, chain_id, &mut o),
            "evm_tx" => evm_case(row, &mut o),
            _ => None,
        };
        if let Some(r) = review {
            judge(&r, &mut o);
        } else {
            o.verdict = "not reviewed".into();
        }
        outcomes.push(o);
    }

    print_table(&outcomes);
    let dir = bench_dir().join("results");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(
        dir.join("detect.json"),
        serde_json::to_string_pretty(&serde_json::json!({ "outcomes": outcomes })).unwrap() + "\n",
    )
    .unwrap();
}

fn word(hex: &str) -> Option<B256> {
    hex.parse::<B256>().ok()
}

fn safe_case(row: &Value, chain_id: u64, o: &mut Outcome) -> Option<Review> {
    // The Safe's own record of what it executed.
    let log = row["receipt"]["logs"].as_array()?.iter().find(|l| {
        l["topics"].as_array().and_then(|t| t.first()).and_then(|t| t.as_str()) == Some(EXECUTION_SUCCESS)
    })?;
    let safe: AAddress = log["address"].as_str()?.parse().ok()?;
    // Up to v1.3.0 the hash is the first data word; from v1.4.0 it is indexed,
    // so it moves to the second topic and the data holds only the payment.
    let topics = log["topics"].as_array()?;
    let logged = match topics.get(1).and_then(|t| t.as_str()) {
        Some(t) => word(t)?,
        None => word(log["data"].as_str()?.get(..66)?)?,
    };

    // The calldata the chain recorded, decoded by alloy. If the Safe was
    // reached through a relayer, find the execTransaction inside its input.
    let input = clearsign::hex::decode(row["tx"]["input"].as_str()?).ok()?;
    let selector = execTransactionCall::SELECTOR;
    let call = if input.starts_with(&selector) {
        execTransactionCall::abi_decode(&input).ok()?
    } else {
        let at = input.windows(4).position(|w| w == selector)?;
        execTransactionCall::abi_decode(&input[at..]).ok()?
    };

    let mut tx = SafeTransaction {
        chain_id: U256::from_u64(chain_id),
        safe: safe.into_array(),
        to: call.to.into_array(),
        value: U256(call.value.to_be_bytes::<32>()),
        data: call.data.to_vec(),
        operation: call.operation,
        safe_tx_gas: U256(call.safeTxGas.to_be_bytes::<32>()),
        base_gas: U256(call.baseGas.to_be_bytes::<32>()),
        gas_price: U256(call.gasPrice.to_be_bytes::<32>()),
        gas_token: call.gasToken.into_array(),
        refund_receiver: call.refundReceiver.into_array(),
        nonce: U256::ZERO,
    };
    // The nonce and the contract version are chain state, not calldata. The
    // right ones are those that reproduce the hash the contract logged.
    let mut found = None;
    'search: for nonce in 0u64..(1 << 22) {
        tx.nonce = U256::from_u64(nonce);
        for domain in [DomainVersion::V1_3Plus, DomainVersion::Legacy] {
            if clearsign::safe_transaction_hash(&tx, domain) == logged.0 {
                found = Some(domain);
                break 'search;
            }
        }
    }
    let Some(domain) = found else {
        o.proof = "no nonce reproduces the logged Safe transaction hash".into();
        return None;
    };
    o.safe_version = Some(match domain {
        DomainVersion::V1_3Plus => "1.3.0+".into(),
        DomainVersion::Legacy => "1.1.x".into(),
    });

    // The owners' signatures must recover over that hash.
    let sigs = call.signatures.to_vec();
    let mut i = 0;
    let mut signers = Vec::new();
    while i + 65 <= sigs.len() {
        let part = &sigs[i..i + 65];
        let v = part[64];
        let recovered = match v {
            27 | 28 | 31 | 32 => {
                let prehash = if v > 30 { eip191_hash_message(logged) } else { logged };
                let mut raw = [0u8; 65];
                raw.copy_from_slice(part);
                raw[64] = if v > 30 { v - 4 } else { v };
                Signature::try_from(&raw[..]).ok().and_then(|s| s.recover_address_from_prehash(&prehash).ok())
            }
            _ => None,
        };
        if let Some(a) = recovered {
            signers.push(a);
        }
        if v == 0 {
            break; // contract signatures point into a dynamic tail
        }
        i += 65;
    }
    o.signers_recovered = signers.len();
    o.proof = format!(
        "nonce {} under the {} domain reproduces the hash the Safe logged; {} owner signature(s) recover over it",
        tx.nonce.to_decimal(),
        o.safe_version.as_deref().unwrap_or("?"),
        signers.len()
    );
    Some(clearsign::review_safe_transaction(&tx, domain))
}

fn evm_case(row: &Value, o: &mut Outcome) -> Option<Review> {
    let mut raw = row["tx"].clone();
    if raw["type"].as_str() != Some("0x0") && raw.get("accessList").is_none() {
        raw["accessList"] = serde_json::json!([]);
    }
    let rpc: RpcTransaction = serde_json::from_value(raw).ok()?;
    let env: &TxEnvelope = rpc.inner.inner();
    let mut payload = Vec::new();
    match env {
        TxEnvelope::Legacy(s) => s.tx().encode_for_signing(&mut payload),
        TxEnvelope::Eip2930(s) => s.tx().encode_for_signing(&mut payload),
        TxEnvelope::Eip1559(s) => s.tx().encode_for_signing(&mut payload),
        TxEnvelope::Eip4844(s) => s.tx().encode_for_signing(&mut payload),
        TxEnvelope::Eip7702(s) => s.tx().encode_for_signing(&mut payload),
    }
    let ok = env.recover_signer().ok() == Some(rpc.inner.signer()) && keccak256(&payload) == env.signature_hash();
    if !ok {
        o.proof = "rebuilt payload does not recover to the sender".into();
        return None;
    }
    o.proof = "rebuilt payload recovers to the victim".into();
    o.signers_recovered = 1;
    match clearsign::review_transaction_bytes(&payload) {
        Ok(r) => Some(r),
        Err(e) => {
            o.verdict = format!("refused: {e}");
            None
        }
    }
}

fn judge(r: &Review, o: &mut Outcome) {
    let top = r.highest_severity();
    o.verdict = match top {
        Some(Severity::Critical) | Some(Severity::Blind) => "DO NOT SIGN".into(),
        Some(Severity::Warning) => "review warnings".into(),
        _ => "nothing flagged".into(),
    };
    let mut by_sev: BTreeMap<std::cmp::Reverse<Severity>, Vec<&str>> = BTreeMap::new();
    for f in r.findings() {
        by_sev.entry(std::cmp::Reverse(f.severity)).or_default().push(f.code);
        if !o.all_codes.iter().any(|c| c == f.code) {
            o.all_codes.push(f.code.into());
        }
    }
    // The decisive findings are the most severe ones that are about this
    // transaction rather than about the Safe's standing configuration.
    for codes in by_sev.values() {
        let about_tx: Vec<&&str> = codes.iter().filter(|c| **c != "SIGNATURE_NOT_CHAIN_BOUND").collect();
        if !about_tx.is_empty() {
            o.decisive = about_tx.iter().map(|c| c.to_string()).collect();
            o.decisive.dedup();
            break;
        }
    }
    o.specific = o.decisive.iter().any(|c| SPECIFIC.contains(&c.as_str()));
    for s in &r.sections {
        for (k, v) in &s.fields {
            if matches!(
                k.as_str(),
                "Action" | "Operation" | "Code that will run as the Safe" | "New owner" | "Operator"
                    | "New implementation" | "Recipient, if it is a token" | "To" | "Delegates to code at"
            ) {
                o.key_lines.push(format!("{} :: {k}: {v}", s.title));
            }
        }
    }
}

fn print_table(outcomes: &[Outcome]) {
    println!(
        "{:<28} {:<27} {:<14} {:<16} {:<9} {}",
        "case", "vector", "verdict", "proof", "specific", "decisive finding"
    );
    for o in outcomes {
        println!(
            "{:<28} {:<27} {:<14} {:<16} {:<9} {}",
            o.case,
            o.vector,
            o.verdict,
            if o.proof.starts_with("nonce") || o.proof.starts_with("rebuilt payload recovers") {
                "proven"
            } else {
                "NOT PROVEN"
            },
            if o.specific { "yes" } else { "no" },
            o.decisive.join(",")
        );
    }
    for o in outcomes {
        println!("\n{} ({}): {}", o.case, o.tx, o.proof);
        for l in &o.key_lines {
            println!("    {l}");
        }
    }
}
