//! Real typed-data signatures, recovered over clearsign's EIP-712 hash.
//!
//! Each row in `corpus/permits/` is a transaction that submitted somebody's
//! permit. Its calldata holds every field the owner signed except, for
//! ERC-2612, the nonce, which is the token's count of the owner's earlier
//! permits; that one is found by trying each value in turn, the right one
//! being the one the signature recovers under. For DAI and Permit2 every field
//! is in the calldata and there is nothing to search.
//!
//! The typed data is built as a wallet would receive it and hashed by
//! clearsign. If the hash is right, the signature recovers to the owner. If
//! any part of the hashing were wrong, it would recover to a stranger, every
//! time. Then the request is reviewed, to see what the reviewer would have said
//! to the person signing it.
//!
//! Writes `results/permits.json`.

use alloy_primitives::{Address, B256, Signature, U256 as AU256};
use alloy_sol_types::{SolCall, sol};
use clearsign::Severity;
use clearsign::typed::{Field, TypedData, Value, hash_typed_data, review_typed_data};
use clearsign_bench::{Tally, bench_dir, read_jsonl};
use serde_json::Value as Json;

sol! {
    function permit(address owner, address spender, uint256 value, uint256 deadline, uint8 v, bytes32 r, bytes32 s);
    function daiPermit(address holder, address spender, uint256 nonce, uint256 expiry, bool allowed, uint8 v, bytes32 r, bytes32 s);
    struct PermitDetails { address token; uint160 amount; uint48 expiration; uint48 nonce; }
    struct PermitSingle { PermitDetails details; address spender; uint256 sigDeadline; }
    struct PermitBatch { PermitDetails[] details; address spender; uint256 sigDeadline; }
    function permitSingle(address owner, PermitSingle permitSingle, bytes signature);
    function permitBatch(address owner, PermitBatch permitBatch, bytes signature);
}

const PERMIT2: &str = "0x000000000022D473030F116dDEE9F6B43aC78BA3";
const USDC: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
const DAI: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";

fn f(name: &str, ty: &str) -> Field {
    Field { name: name.into(), ty: ty.into() }
}

fn s(v: impl ToString) -> Value {
    Value::Str(v.to_string())
}

fn obj(fields: Vec<(&str, Value)>) -> Value {
    Value::Object(fields.into_iter().map(|(k, v)| (k.to_string(), v)).collect())
}

fn domain_named(name: &str, version: Option<&str>, contract: &str) -> (Vec<Field>, Value) {
    let mut fields = vec![f("name", "string")];
    let mut values = vec![("name", s(name))];
    if let Some(v) = version {
        fields.push(f("version", "string"));
        values.push(("version", s(v)));
    }
    fields.push(f("chainId", "uint256"));
    values.push(("chainId", Value::Int { negative: false, magnitude: 1 }));
    fields.push(f("verifyingContract", "address"));
    values.push(("verifyingContract", s(contract)));
    (fields, obj(values))
}

fn details_fields() -> Vec<Field> {
    vec![f("token", "address"), f("amount", "uint160"), f("expiration", "uint48"), f("nonce", "uint48")]
}

fn details_value(d: &PermitDetails) -> Value {
    obj(vec![
        ("token", s(d.token)),
        ("amount", s(d.amount)),
        ("expiration", s(d.expiration)),
        ("nonce", s(d.nonce)),
    ])
}

fn recover(sig: &[u8], hash: [u8; 32]) -> Option<Address> {
    let parsed = match sig.len() {
        65 => Signature::try_from(sig).ok()?,
        64 => {
            // EIP-2098 compact: the top bit of s carries y parity.
            let r = AU256::from_be_slice(&sig[..32]);
            let vs = AU256::from_be_slice(&sig[32..]);
            let y = vs.bit(255);
            let s = vs & (AU256::MAX >> 1);
            Signature::new(r, s, y)
        }
        _ => return None,
    };
    parsed.recover_address_from_prehash(&B256::from(hash)).ok()
}

fn rsv(r: B256, s: B256, v: u8) -> Vec<u8> {
    let mut out = r.to_vec();
    out.extend_from_slice(s.as_slice());
    out.push(v);
    out
}

#[derive(Default, serde::Serialize)]
struct Report {
    by_kind: Tally,
    proven: Tally,
    not_proven: Tally,
    outcome: Tally,
    findings: Tally,
    nonce_searched_max: u64,
    unlimited: u64,
    bounded: u64,
}

fn main() {
    let dir = bench_dir().join("corpus/permits");
    let manifest: Json = serde_json::from_str(&std::fs::read_to_string(dir.join("manifest.json")).unwrap()).unwrap();
    let usdc_name = manifest["token_domains"][USDC.to_lowercase()]["name"].as_str().unwrap_or("USD Coin").to_owned();
    let usdc_version = manifest["token_domains"][USDC.to_lowercase()]["version"].as_str().unwrap_or("2").to_owned();
    let dai_name = manifest["token_domains"][DAI.to_lowercase()]["name"].as_str().unwrap_or("Dai Stablecoin").to_owned();
    let dai_version = manifest["token_domains"][DAI.to_lowercase()]["version"].as_str().unwrap_or("1").to_owned();

    let mut rep = Report::default();
    for row in read_jsonl(&dir.join("permits.jsonl.gz")) {
        let kind = row["kind"].as_str().unwrap_or("").to_owned();
        rep.by_kind.add(kind.clone());
        let input = clearsign::hex::decode(row["tx"]["input"].as_str().unwrap_or("")).unwrap_or_default();
        let proven: Option<TypedData> = match kind.as_str() {
            "erc2612" => (|| {
                let c = permitCall::abi_decode(&input).ok()?;
                let (dfields, dvalue) = domain_named(&usdc_name, Some(&usdc_version), USDC);
                let sig = rsv(c.r, c.s, c.v);
                for nonce in 0u64..50_000 {
                    let td = TypedData {
                        types: vec![
                            ("EIP712Domain".into(), dfields.clone()),
                            ("Permit".into(), vec![f("owner", "address"), f("spender", "address"), f("value", "uint256"), f("nonce", "uint256"), f("deadline", "uint256")]),
                        ],
                        primary_type: "Permit".into(),
                        domain: dvalue.clone(),
                        message: obj(vec![
                            ("owner", s(c.owner)),
                            ("spender", s(c.spender)),
                            ("value", s(c.value)),
                            ("nonce", Value::Int { negative: false, magnitude: nonce }),
                            ("deadline", s(c.deadline)),
                        ]),
                    };
                    let h = hash_typed_data(&td).ok()?;
                    if recover(&sig, h.signing_hash) == Some(c.owner) {
                        rep.nonce_searched_max = rep.nonce_searched_max.max(nonce);
                        return Some(td);
                    }
                }
                None
            })(),
            "dai" => (|| {
                let c = daiPermitCall::abi_decode(&{
                    let mut v = daiPermitCall::SELECTOR.to_vec();
                    v.extend_from_slice(input.get(4..)?);
                    v
                })
                .ok()?;
                let (dfields, dvalue) = domain_named(&dai_name, Some(&dai_version), DAI);
                let td = TypedData {
                    types: vec![
                        ("EIP712Domain".into(), dfields),
                        ("Permit".into(), vec![f("holder", "address"), f("spender", "address"), f("nonce", "uint256"), f("expiry", "uint256"), f("allowed", "bool")]),
                    ],
                    primary_type: "Permit".into(),
                    domain: dvalue,
                    message: obj(vec![
                        ("holder", s(c.holder)),
                        ("spender", s(c.spender)),
                        ("nonce", s(c.nonce)),
                        ("expiry", s(c.expiry)),
                        ("allowed", Value::Bool(c.allowed)),
                    ]),
                };
                let h = hash_typed_data(&td).ok()?;
                (recover(&rsv(c.r, c.s, c.v), h.signing_hash) == Some(c.holder)).then_some(td)
            })(),
            "permit2_single" => (|| {
                let c = permitSingleCall::abi_decode(&{
                    let mut v = permitSingleCall::SELECTOR.to_vec();
                    v.extend_from_slice(input.get(4..)?);
                    v
                })
                .ok()?;
                let (dfields, dvalue) = domain_named("Permit2", None, PERMIT2);
                let p = &c.permitSingle;
                let td = TypedData {
                    types: vec![
                        ("EIP712Domain".into(), dfields),
                        ("PermitSingle".into(), vec![f("details", "PermitDetails"), f("spender", "address"), f("sigDeadline", "uint256")]),
                        ("PermitDetails".into(), details_fields()),
                    ],
                    primary_type: "PermitSingle".into(),
                    domain: dvalue,
                    message: obj(vec![
                        ("details", details_value(&p.details)),
                        ("spender", s(p.spender)),
                        ("sigDeadline", s(p.sigDeadline)),
                    ]),
                };
                let h = hash_typed_data(&td).ok()?;
                (recover(&c.signature, h.signing_hash) == Some(c.owner)).then_some(td)
            })(),
            "permit2_batch" => (|| {
                let c = permitBatchCall::abi_decode(&{
                    let mut v = permitBatchCall::SELECTOR.to_vec();
                    v.extend_from_slice(input.get(4..)?);
                    v
                })
                .ok()?;
                let (dfields, dvalue) = domain_named("Permit2", None, PERMIT2);
                let p = &c.permitBatch;
                let td = TypedData {
                    types: vec![
                        ("EIP712Domain".into(), dfields),
                        ("PermitBatch".into(), vec![f("details", "PermitDetails[]"), f("spender", "address"), f("sigDeadline", "uint256")]),
                        ("PermitDetails".into(), details_fields()),
                    ],
                    primary_type: "PermitBatch".into(),
                    domain: dvalue,
                    message: obj(vec![
                        ("details", Value::Array(p.details.iter().map(details_value).collect())),
                        ("spender", s(p.spender)),
                        ("sigDeadline", s(p.sigDeadline)),
                    ]),
                };
                let h = hash_typed_data(&td).ok()?;
                (recover(&c.signature, h.signing_hash) == Some(c.owner)).then_some(td)
            })(),
            _ => None,
        };
        let Some(td) = proven else {
            rep.not_proven.add(kind);
            continue;
        };
        rep.proven.add(kind);
        let review = review_typed_data(&td).unwrap();
        rep.outcome.add(match review.highest_severity() {
            Some(Severity::Critical) => "critical",
            Some(Severity::Blind) => "blind",
            Some(Severity::Warning) => "warning",
            _ => "clear",
        });
        for code in review.findings().iter().map(|f| f.code).collect::<std::collections::BTreeSet<_>>() {
            rep.findings.add(code);
        }
        if review.has("UNLIMITED_APPROVAL") {
            rep.unlimited += 1;
        } else {
            rep.bounded += 1;
        }
    }

    println!("permit submissions by kind: {:?}", rep.by_kind.sorted());
    println!("signature recovered to the owner over clearsign's EIP-712 hash: {:?}", rep.proven.sorted());
    println!("not proven: {:?}", rep.not_proven.sorted());
    println!("largest ERC-2612 nonce found by search: {}", rep.nonce_searched_max);
    println!("what the reviewer said: {:?}", rep.outcome.sorted());
    println!("unlimited {}  bounded {}", rep.unlimited, rep.bounded);
    println!("findings: {:?}", rep.findings.sorted());
    let out = bench_dir().join("results");
    std::fs::create_dir_all(&out).unwrap();
    std::fs::write(out.join("permits.json"), serde_json::to_string_pretty(&rep).unwrap() + "\n").unwrap();
    let failed: u64 = rep.not_proven.total();
    if failed > 0 {
        eprintln!("\nNOTE: {failed} submissions could not be proven; see the kinds above.");
    }
}
