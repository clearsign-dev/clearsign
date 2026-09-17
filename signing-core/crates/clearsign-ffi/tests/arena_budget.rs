//! How much does one review allocate, worst case?
//!
//! The bare-metal build serves requests from a fixed 4 MiB arena whose `dealloc`
//! is a no-op, so what matters is not the live set but the *sum of every
//! allocation* during one call. If a request the guest can send exceeds that
//! sum, the allocator returns null, Rust's allocation-failure path runs, and the
//! panic handler parks the protection domain for good: one untrusted message
//! would take the signer out of service until reboot.

#![allow(
    clippy::unwrap_used,
    clippy::arithmetic_side_effects,
    clippy::indexing_slicing
)]

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

static TOTAL: AtomicUsize = AtomicUsize::new(0);

struct Counting;

// SAFETY: forwards every call to the system allocator unchanged.
unsafe impl GlobalAlloc for Counting {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        TOTAL.fetch_add(layout.size(), Ordering::Relaxed);
        unsafe { System.alloc(layout) }
    }
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        unsafe { System.dealloc(ptr, layout) }
    }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        // A bump allocator cannot grow in place: a realloc costs the new size.
        TOTAL.fetch_add(new_size, Ordering::Relaxed);
        unsafe { System.realloc(ptr, layout, new_size) }
    }
}

#[global_allocator]
static ALLOC: Counting = Counting;

/// The arena in the bare-metal build.
const ARENA_SIZE: usize = 4 * 1024 * 1024;
/// The largest request the seL4 signer will copy in (region minus its header).
const MAX_REQUEST: usize = 0x10000 - 8;

/// The counter is global, so two measurements running at once would add each
/// other's allocations. Serialise them: a measurement that counts another
/// test's work is not a measurement.
static MEASURING: std::sync::Mutex<()> = std::sync::Mutex::new(());

fn measure(request: &[u8]) -> (usize, bool) {
    let _guard = MEASURING.lock().unwrap_or_else(|e| e.into_inner());
    TOTAL.store(0, Ordering::Relaxed);
    let reviewed = clearsign_ffi::review_request(request).is_some();
    (TOTAL.load(Ordering::Relaxed), reviewed)
}

/// The alarm threshold, not the cliff. Exceeding half the arena means a change
/// has doubled what a request costs, and that should be noticed here rather
/// than by a protection domain that stops answering.
const BUDGET: usize = ARENA_SIZE / 2;

#[test]
fn a_maximal_transaction_fits_in_the_arena() {
    // An EIP-1559 transaction whose calldata fills the request region.
    let mut tx = vec![1u8]; // framing: unsigned EVM transaction
    tx.extend_from_slice(&[0x02, 0xf9]);
    tx.resize(MAX_REQUEST, 0x41);
    let (used, _) = measure(&tx);
    assert!(
        used < BUDGET,
        "a maximal request allocated {used} bytes, over the {BUDGET} budget (arena is {ARENA_SIZE})"
    );
    println!("maximal malformed transaction: {used} bytes allocated");
}

#[test]
fn a_maximal_batch_fits_in_the_arena() {
    // A MultiSend batch is the densest thing the decoder renders: many inner
    // calls, each producing its own section and fields.
    let mut packed = Vec::new();
    for _ in 0..32 {
        packed.push(0u8); // operation: call
        packed.extend_from_slice(&[0x11; 20]); // to
        packed.extend_from_slice(&[0; 32]); // value
        let data = vec![0xab; 1800];
        let mut len = [0u8; 32];
        len[28..].copy_from_slice(&(data.len() as u32).to_be_bytes());
        packed.extend_from_slice(&len);
        packed.extend_from_slice(&data);
    }
    // ABI-encode it as multiSend(bytes) and frame it as a Safe transaction.
    let mut calldata = vec![0x8d, 0x80, 0xff, 0x0a];
    let mut off = [0u8; 32];
    off[31] = 32;
    calldata.extend_from_slice(&off);
    let mut blen = [0u8; 32];
    blen[28..].copy_from_slice(&(packed.len() as u32).to_be_bytes());
    calldata.extend_from_slice(&blen);
    calldata.extend_from_slice(&packed);
    while calldata.len() % 32 != 4 {
        calldata.push(0);
    }

    let mut frame = vec![2u8];
    frame.extend_from_slice(&{
        let mut c = [0u8; 32];
        c[31] = 1;
        c
    }); // chain id 1
    frame.extend_from_slice(&[0x11; 20]); // safe
    frame.extend_from_slice(&hex_addr("40A2aCCbd92BCA938b02010E17A5b8929b49130D")); // MultiSend
    frame.extend_from_slice(&[0; 32]); // value
    frame.push(1); // delegatecall
    frame.extend_from_slice(&[0; 32 * 3]);
    frame.extend_from_slice(&[0; 40]);
    frame.extend_from_slice(&[0; 32]); // nonce
    frame.extend_from_slice(&(calldata.len() as u32).to_be_bytes());
    frame.extend_from_slice(&calldata);
    assert!(
        frame.len() <= MAX_REQUEST,
        "test frame is {} bytes",
        frame.len()
    );

    let (used, reviewed) = measure(&frame);
    println!(
        "maximal MultiSend batch ({} byte request): {used} bytes allocated, reviewed={reviewed}",
        frame.len()
    );
    assert!(
        used < BUDGET,
        "a maximal batch allocated {used} bytes, over the {BUDGET} budget (arena is {ARENA_SIZE})"
    );
}

#[test]
fn a_maximal_plan_fits_in_the_arena() {
    // The largest plan that still fits in the request region: every step full,
    // every step carrying data forward, so the flow tracing has the most to do.
    let text_len = (MAX_REQUEST - 4096) / (authority::MAX_STEPS * 2);
    let mut steps = Vec::new();
    for id in 1..=authority::MAX_STEPS {
        steps.push(authority::Step {
            id: authority::StepId(id as u16),
            title: "t".repeat(text_len),
            action: authority::Action::HttpRequest {
                method: authority::HttpMethod::Post,
                host: "h".repeat(text_len),
            },
            inputs: if id > 1 {
                vec![authority::StepId((id - 1) as u16)]
            } else {
                vec![]
            },
        });
    }
    let plan = authority::Plan {
        goal: "g".repeat(text_len),
        steps,
    };
    let mut frame = vec![3u8];
    frame.extend_from_slice(&authority::encode_plan(&plan));
    assert!(
        frame.len() <= MAX_REQUEST,
        "plan frame is {} bytes",
        frame.len()
    );
    let (used, reviewed) = measure(&frame);
    println!(
        "maximal plan ({} byte request): {used} bytes allocated, reviewed={reviewed}",
        frame.len()
    );
    assert!(
        used < BUDGET,
        "a maximal plan allocated {used} bytes, over the {BUDGET} budget (arena is {ARENA_SIZE})"
    );
}

fn hex_addr(s: &str) -> [u8; 20] {
    let mut out = [0u8; 20];
    for i in 0..20 {
        out[i] = u8::from_str_radix(&s[i * 2..i * 2 + 2], 16).unwrap();
    }
    out
}
