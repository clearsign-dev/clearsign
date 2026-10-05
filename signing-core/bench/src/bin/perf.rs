//! How fast the reviewer is, and how much memory one review costs.
//!
//! Each scenario is a complete review as a signer would get it: parse the
//! bytes, decode, judge, compute the digest and render the text. Times are
//! wall-clock per review on one core, measured over many repetitions after a
//! warm-up; memory is counted by a global allocator wrapper, as total bytes
//! requested and the peak held at once during a single review.
//!
//! The last part times the real corpora: every Safe record and every
//! transaction clearsign accepts, end to end, to give a throughput figure that
//! no hand-built scenario can flatter.
//!
//! Writes `results/perf.json`.

use std::alloc::{GlobalAlloc, Layout, System};
use std::hint::black_box;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use alloy_consensus::{SignableTransaction, TxEip1559, TxEnvelope};
use alloy_primitives::{Address as AAddress, Bytes, TxKind, U256 as AU256};
use clearsign::{DomainVersion, SafeTransaction, U256};
use clearsign_bench::{bench_dir, corpus_files, percentile, read_jsonl};
use serde::Serialize;

struct Counting;
static CURRENT: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);
static TOTAL: AtomicUsize = AtomicUsize::new(0);

unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let p = unsafe { System.alloc(layout) };
        if !p.is_null() {
            let now = CURRENT.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
            PEAK.fetch_max(now, Ordering::Relaxed);
            TOTAL.fetch_add(layout.size(), Ordering::Relaxed);
        }
        p
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) };
        CURRENT.fetch_sub(layout.size(), Ordering::Relaxed);
    }
}

#[global_allocator]
static ALLOC: Counting = Counting;

#[derive(Serialize)]
struct Scenario {
    name: &'static str,
    what: &'static str,
    input_bytes: usize,
    iterations: usize,
    p50_us: f64,
    p95_us: f64,
    p99_us: f64,
    max_us: f64,
    reviews_per_second: f64,
    bytes_allocated: usize,
    peak_bytes_held: usize,
}

fn measure<F: FnMut() -> usize>(name: &'static str, what: &'static str, input: usize, mut f: F) -> Scenario {
    // Memory for one review, measured on its own.
    let base = CURRENT.load(Ordering::Relaxed);
    PEAK.store(base, Ordering::Relaxed);
    let before = TOTAL.load(Ordering::Relaxed);
    black_box(f());
    let bytes_allocated = TOTAL.load(Ordering::Relaxed) - before;
    let peak_bytes_held = PEAK.load(Ordering::Relaxed).saturating_sub(base);

    // Warm up, then time until two seconds or 200,000 runs, whichever first.
    let warm = Instant::now();
    while warm.elapsed() < Duration::from_millis(300) {
        black_box(f());
    }
    let mut samples = Vec::new();
    let start = Instant::now();
    while start.elapsed() < Duration::from_secs(2) && samples.len() < 200_000 {
        let t = Instant::now();
        black_box(f());
        samples.push(t.elapsed().as_secs_f64() * 1e6);
    }
    samples.sort_by(|a, b| a.partial_cmp(b).unwrap());
    let mean = samples.iter().sum::<f64>() / samples.len() as f64;
    let s = Scenario {
        name,
        what,
        input_bytes: input,
        iterations: samples.len(),
        p50_us: percentile(&samples, 50.0),
        p95_us: percentile(&samples, 95.0),
        p99_us: percentile(&samples, 99.0),
        max_us: *samples.last().unwrap(),
        reviews_per_second: 1e6 / mean,
        bytes_allocated,
        peak_bytes_held,
    };
    println!(
        "{:<26} {:>9} B  p50 {:>9.2} us  p95 {:>9.2}  p99 {:>9.2}  max {:>9.1}  {:>11.0}/s  alloc {:>9} B  peak {:>9} B",
        s.name, s.input_bytes, s.p50_us, s.p95_us, s.p99_us, s.max_us, s.reviews_per_second,
        s.bytes_allocated, s.peak_bytes_held
    );
    s
}

fn eip1559(to: [u8; 20], value: u64, input: Vec<u8>) -> Vec<u8> {
    let tx = TxEip1559 {
        chain_id: 1,
        nonce: 42,
        gas_limit: 120_000,
        max_fee_per_gas: 30_000_000_000,
        max_priority_fee_per_gas: 1_000_000_000,
        to: TxKind::Call(AAddress::from(to)),
        value: AU256::from(value),
        access_list: Default::default(),
        input: Bytes::from(input),
    };
    let mut out = Vec::new();
    tx.encode_for_signing(&mut out);
    out
}

fn word_addr(a: [u8; 20]) -> [u8; 32] {
    let mut w = [0u8; 32];
    w[12..].copy_from_slice(&a);
    w
}

fn erc20(selector: [u8; 4], who: [u8; 20], amount: [u8; 32]) -> Vec<u8> {
    let mut d = selector.to_vec();
    d.extend_from_slice(&word_addr(who));
    d.extend_from_slice(&amount);
    d
}

fn multisend(n: usize) -> Vec<u8> {
    let mut packed = Vec::new();
    for i in 0..n {
        let mut amount = [0u8; 32];
        amount[24..].copy_from_slice(&(1_000_000u64 + i as u64).to_be_bytes());
        let inner = erc20([0xa9, 0x05, 0x9c, 0xbb], [0x22; 20], amount);
        packed.push(0u8);
        packed.extend_from_slice(&[0x11; 20]);
        packed.extend_from_slice(&[0u8; 32]);
        let mut len = [0u8; 32];
        len[24..].copy_from_slice(&(inner.len() as u64).to_be_bytes());
        packed.extend_from_slice(&len);
        packed.extend_from_slice(&inner);
    }
    let mut d = vec![0x8d, 0x80, 0xff, 0x0a];
    let mut off = [0u8; 32];
    off[31] = 0x20;
    d.extend_from_slice(&off);
    let mut len = [0u8; 32];
    len[24..].copy_from_slice(&(packed.len() as u64).to_be_bytes());
    d.extend_from_slice(&len);
    d.extend_from_slice(&packed);
    while (d.len() - 4) % 32 != 0 {
        d.push(0);
    }
    d
}

fn safe_tx(to: [u8; 20], data: Vec<u8>, operation: u8) -> SafeTransaction {
    SafeTransaction {
        chain_id: U256::from_u64(1),
        safe: [0x5a; 20],
        to,
        value: U256::ZERO,
        data,
        operation,
        safe_tx_gas: U256::ZERO,
        base_gas: U256::ZERO,
        gas_price: U256::ZERO,
        gas_token: [0; 20],
        refund_receiver: [0; 20],
        nonce: U256::from_u64(7),
    }
}

fn main() {
    println!("machine: {} logical cores; one core used per scenario\n", std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0));
    let mut out = Vec::new();

    let transfer = eip1559([0x33; 20], 1_000_000_000_000_000_000, vec![]);
    out.push(measure("eth_transfer", "EIP-1559 native transfer", transfer.len(), || {
        clearsign::review_transaction_bytes(&transfer).unwrap().render().len()
    }));

    let mut amount = [0u8; 32];
    amount[24..].copy_from_slice(&5_000_000u64.to_be_bytes());
    let erc20_tx = eip1559([0x44; 20], 0, erc20([0xa9, 0x05, 0x9c, 0xbb], [0x55; 20], amount));
    out.push(measure("erc20_transfer", "EIP-1559 ERC-20 transfer", erc20_tx.len(), || {
        clearsign::review_transaction_bytes(&erc20_tx).unwrap().render().len()
    }));

    let approve_tx = eip1559([0x44; 20], 0, erc20([0x09, 0x5e, 0xa7, 0xb3], [0x66; 20], [0xff; 32]));
    out.push(measure("unlimited_approve", "EIP-1559 unlimited ERC-20 approval", approve_tx.len(), || {
        clearsign::review_transaction_bytes(&approve_tx).unwrap().render().len()
    }));

    let mut big = vec![0xde, 0xad, 0xbe, 0xef];
    big.resize(4 + 128 * 1024, 0x01);
    let big_tx = eip1559([0x77; 20], 0, big);
    out.push(measure("unknown_128k", "EIP-1559 with 128 KiB of calldata to an unknown function", big_tx.len(), || {
        clearsign::review_transaction_bytes(&big_tx).unwrap().render().len()
    }));

    let fixture = std::fs::read_to_string(bench_dir().join("../crates/clearsign-cli/tests/fixtures/bybit-safe-tx.json"))
        .expect("Bybit fixture");
    out.push(measure("safe_json_bybit", "Bybit's Safe record: strict JSON read, domain settled from the hash, review, render", fixture.len(), || {
        clearsign_safe_json::review_json(&fixture, Some(1), None).unwrap().review.render().len()
    }));
    let bybit = clearsign_safe_json::parse(&fixture, Some(1)).unwrap().tx;
    out.push(measure("safe_bybit", "Bybit's Safe transaction: review and render (v1.1.x domain)", bybit.data.len(), || {
        clearsign::review_safe_transaction(&bybit, DomainVersion::Legacy).render().len()
    }));
    out.push(measure("safe_hash_only", "Safe transaction hash alone", bybit.data.len(), || {
        clearsign::safe_transaction_hash(&bybit, DomainVersion::Legacy)[0] as usize
    }));

    // MultiSend 1.3.0 canonical, published on chain 1.
    let ms_addr: [u8; 20] = [0xa2, 0x38, 0xcb, 0xeb, 0x14, 0x2c, 0x10, 0xef, 0x7a, 0xd8, 0x44, 0x2c, 0x6d, 0x1f, 0x9e, 0x89, 0xe0, 0x7e, 0x77, 0x61];
    let ms32 = safe_tx(ms_addr, multisend(32), 1);
    out.push(measure("multisend_32", "Safe MultiSend batch of 32 ERC-20 transfers (all shown)", ms32.data.len(), || {
        clearsign::review_safe_transaction(&ms32, DomainVersion::V1_3Plus).render().len()
    }));
    let ms1024 = safe_tx(ms_addr, multisend(1024), 1);
    out.push(measure("multisend_1024", "Safe MultiSend batch of 1,024 calls, the most it will parse", ms1024.data.len(), || {
        clearsign::review_safe_transaction(&ms1024, DomainVersion::V1_3Plus).render().len()
    }));

    // Throughput over the real corpora.
    let safe_records: Vec<(SafeTransaction, DomainVersion)> = corpus_files(&bench_dir().join("corpus/safe"))
        .iter()
        .flat_map(|f| read_jsonl(f))
        .filter_map(|rec| {
            let chain = rec["_chainId"].as_u64()?;
            let v = rec["_safeVersion"].as_str()?;
            let domain = if v.starts_with("1.1") || v.starts_with("1.2") {
                DomainVersion::Legacy
            } else if v.starts_with("1.0") {
                return None;
            } else {
                DomainVersion::V1_3Plus
            };
            Some((clearsign_safe_json::parse(&rec.to_string(), Some(chain)).ok()?.tx, domain))
        })
        .collect();
    let t = Instant::now();
    let mut chars = 0usize;
    for (tx, d) in &safe_records {
        chars += clearsign::review_safe_transaction(tx, *d).render().len();
    }
    let safe_secs = t.elapsed().as_secs_f64();
    black_box(chars);
    println!("\nreal Safe records: {} reviewed in {:.3} s = {:.0} reviews/s on one core",
        safe_records.len(), safe_secs, safe_records.len() as f64 / safe_secs);

    let payloads: Vec<Vec<u8>> = corpus_files(&bench_dir().join("corpus/evm"))
        .iter()
        .flat_map(|f| read_jsonl(f))
        .filter_map(|raw| {
            let rpc: alloy_rpc_types_eth::Transaction = serde_json::from_value(raw).ok()?;
            let mut p = Vec::new();
            match rpc.inner.inner() {
                TxEnvelope::Legacy(s) => s.tx().encode_for_signing(&mut p),
                TxEnvelope::Eip2930(s) => s.tx().encode_for_signing(&mut p),
                TxEnvelope::Eip1559(s) => s.tx().encode_for_signing(&mut p),
                TxEnvelope::Eip4844(s) => s.tx().encode_for_signing(&mut p),
                TxEnvelope::Eip7702(s) => s.tx().encode_for_signing(&mut p),
            }
            Some(p)
        })
        .collect();
    let t = Instant::now();
    let mut n = 0usize;
    for p in &payloads {
        if let Ok(rv) = clearsign::review_transaction_bytes(p) {
            n += rv.render().len();
        }
    }
    let evm_secs = t.elapsed().as_secs_f64();
    black_box(n);
    println!("real transactions: {} reviewed in {:.3} s = {:.0} reviews/s on one core",
        payloads.len(), evm_secs, payloads.len() as f64 / evm_secs);

    let json = serde_json::json!({
        "scenarios": out,
        "corpus_throughput": {
            "safe_records": safe_records.len(),
            "safe_seconds": safe_secs,
            "evm_transactions": payloads.len(),
            "evm_seconds": evm_secs,
        },
        "machine": {
            "logical_cores": std::thread::available_parallelism().map(|n| n.get()).unwrap_or(0),
            "os": std::env::consts::OS,
            "arch": std::env::consts::ARCH,
        },
    });
    let dir = bench_dir().join("results");
    std::fs::create_dir_all(&dir).unwrap();
    std::fs::write(dir.join("perf.json"), serde_json::to_string_pretty(&json).unwrap() + "\n").unwrap();
}
