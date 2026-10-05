//! Tests written because mutation testing showed they were missing.
//!
//! `cargo mutants -p clearsign` against the v0.1.2 decoder, 5 Oct 2026: of 332
//! mutants, 223 were caught by the existing tests and 42 survived. A surviving
//! mutant is a change to the code that no test notices. Each test here names
//! the mutant it exists to kill.
//!
//! A second run the same day, against the decoder as extended for real attacks,
//! found survivors of the same kind: limits nothing tested exactly at the
//! boundary (array lengths, carried-call depth, the width guard in
//! `uint_bits`) and one assertion that matched the wrong text. Those have tests
//! here too, marked with the line they defend.
//!
//! Some survivors are left deliberately, because they are equivalent: no input
//! can tell the mutant from the original, so no test can kill them.
//!
//! - The severity escalation when hidden batch or carried-call findings are
//!   grouped by code (`if f.severity > entry.0`, three mutants in each of
//!   `review_batch` and `review_inner_calls`). Every finding code is raised
//!   at one fixed severity, so within a group the severity never rises. The
//!   check stays so that a code given a second severity later is still
//!   reported at its worst.
//! - `<` to `<=` in `Args::expect_static_len` and `Args::expect_end`. The arm
//!   before matches equality, so the two operators decide every input alike.
//! - Deleting `operation` from the context `review_inner_calls` passes on.
//!   Every carried call is a CALL, and so is the context it inherits from; a
//!   carried DELEGATECALL is refused before it gets here.
//! - `|` to `^` when `hex::decode` joins two nibbles: the high one is shifted
//!   clear of the low one, so the bits never overlap.
//! - Dropping the even-length check in the EIP-712 reader's `bytes_value`:
//!   `hex::decode` refuses an odd number of digits by itself, so the request
//!   is refused either way, with a different message.
//!
//! In all, the second run tested 718 mutants of the extended decoder: 617
//! caught, 87 that did not compile, 2 that made the suite loop until it timed
//! out, and the 12 equivalent survivors above.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use clearsign::abi::Args;
use clearsign::calls::{MAX_NESTING, SEL_APPROVE, SEL_EXEC_TRANSACTION, SEL_TRANSFER};
use clearsign::{DomainVersion, Error, SafeTransaction, U256};
use clearsign::{
    hex, parse_unsigned_transaction, review_safe_transaction, review_transaction_bytes,
};

const SAFE: [u8; 20] = [0x11; 20];
const TOKEN: [u8; 20] = [0x22; 20];
const OTHER_TOKEN: [u8; 20] = [0x23; 20];
const MULTISEND_130: [u8; 20] = [
    0xa2, 0x38, 0xcb, 0xeb, 0x14, 0x2c, 0x10, 0xef, 0x7a, 0xd8, 0x44, 0x2c, 0x6d, 0x1f, 0x9e, 0x89,
    0xe0, 0x7e, 0x77, 0x61,
];

fn safe_call(to: [u8; 20], data: Vec<u8>, operation: u8) -> SafeTransaction {
    SafeTransaction {
        chain_id: U256::from_u64(1),
        safe: SAFE,
        to,
        value: U256::ZERO,
        data,
        operation,
        safe_tx_gas: U256::ZERO,
        base_gas: U256::ZERO,
        gas_price: U256::ZERO,
        gas_token: [0; 20],
        refund_receiver: [0; 20],
        nonce: U256::ZERO,
    }
}

fn word_addr(a: [u8; 20]) -> Vec<u8> {
    let mut w = vec![0u8; 12];
    w.extend_from_slice(&a);
    w
}

fn word_u64(v: u64) -> Vec<u8> {
    U256::from_u64(v).0.to_vec()
}

fn transfer(to: [u8; 20], amount: u64) -> Vec<u8> {
    let mut d = SEL_TRANSFER.to_vec();
    d.extend(word_addr(to));
    d.extend(word_u64(amount));
    d
}

/// A MultiSend batch of `(operation, to, value, data)` calls.
fn multisend(calls: &[(u8, [u8; 20], u64, Vec<u8>)]) -> Vec<u8> {
    let mut packed = Vec::new();
    for (op, to, value, data) in calls {
        packed.push(*op);
        packed.extend_from_slice(to);
        packed.extend(word_u64(*value));
        packed.extend(word_u64(data.len() as u64));
        packed.extend_from_slice(data);
    }
    let mut d = clearsign::multisend::SEL_MULTI_SEND.to_vec();
    d.extend(word_u64(32));
    d.extend(word_u64(packed.len() as u64));
    d.extend_from_slice(&packed);
    while (d.len() - 4) % 32 != 0 {
        d.push(0);
    }
    d
}

/// execTransaction calldata on `safe`, carrying `inner`.
fn exec_transaction(inner_to: [u8; 20], inner: &[u8]) -> Vec<u8> {
    let mut d = SEL_EXEC_TRANSACTION.to_vec();
    let data_padded = inner.len().div_ceil(32) * 32;
    d.extend(word_addr(inner_to));
    d.extend(word_u64(0));
    d.extend(word_u64(320)); // data at 10 * 32
    d.extend(word_u64(0)); // operation
    for _ in 0..3 {
        d.extend(word_u64(0));
    }
    d.extend(word_addr([0; 20]));
    d.extend(word_addr([0; 20]));
    d.extend(word_u64((320 + 32 + data_padded) as u64)); // signatures after data
    d.extend(word_u64(inner.len() as u64));
    d.extend_from_slice(inner);
    d.extend(vec![0u8; data_padded - inner.len()]);
    d.extend(word_u64(0)); // empty signatures
    d
}

// u256.rs:14 — a U256 debug-prints as its exact decimal value, which is what
// every failing assertion in these tests shows.
#[test]
fn a_u256_debug_prints_as_its_decimal_value() {
    assert_eq!(format!("{:?}", U256::from_u64(1_234_567)), "1234567");
    assert_eq!(format!("{:?}", U256::ZERO), "0");
}

// abi.rs:27 and abi.rs:31 — Args::len and Args::is_empty returning constants.
#[test]
fn args_report_their_length() {
    assert_eq!(Args::new(&[1, 2, 3]).len(), 3);
    assert!(!Args::new(&[1]).is_empty());
    assert!(Args::new(&[]).is_empty());
    assert_eq!(Args::new(&[]).len(), 0);
}

// abi.rs:80 — uint_bits refuses a width that is not a type: zero, wider than
// a word, or not a whole number of bytes. Every caller passes 160 or 48, so
// only a direct test reaches the guard, and each operator in it is pinned by
// one of these widths. Found by the second run, 5 Oct 2026.
#[test]
fn uint_bits_refuses_widths_that_are_not_a_type() {
    let a = Args::new(&[0u8; 32]);
    assert_eq!(a.uint_bits(0, 0), Err(Error::IntegerOverflow));
    assert_eq!(a.uint_bits(0, 12), Err(Error::IntegerOverflow));
    assert_eq!(a.uint_bits(0, 264), Err(Error::IntegerOverflow));
    assert_eq!(a.uint_bits(0, 256), Ok(U256::ZERO));
    assert_eq!(a.uint_bits(0, 8), Ok(U256::ZERO));
}

/// `bytes[]` at offset 32 holding `n` empty elements, encoded as solc would.
fn empty_bytes_array(n: usize) -> Vec<u8> {
    let mut d = word_u64(32);
    d.extend(word_u64(n as u64));
    for i in 0..n {
        d.extend(word_u64((n * 32 + i * 32) as u64));
    }
    for _ in 0..n {
        d.extend(word_u64(0));
    }
    d
}

/// `(address,uint256,bytes)[]` at offset 32 holding `n` calls with no data.
fn empty_call_array(n: usize) -> Vec<u8> {
    let mut d = word_u64(32);
    d.extend(word_u64(n as u64));
    for i in 0..n {
        d.extend(word_u64((n * 32 + i * 128) as u64));
    }
    for _ in 0..n {
        d.extend(word_addr(TOKEN));
        d.extend(word_u64(0));
        d.extend(word_u64(96));
        d.extend(word_u64(0));
    }
    d
}

// abi.rs:124, abi.rs:199 — an array of exactly as many elements as the limit
// is read; one more is refused before any element is. Nothing tested the
// boundary itself, so `>` could become `>=` or `==` unnoticed.
#[test]
fn arrays_of_exactly_the_limit_are_read_and_one_more_is_not() {
    let two = empty_bytes_array(2);
    let (elements, end) = Args::new(&two).bytes_array_at(0, 32, 2).unwrap();
    assert_eq!((elements.len(), end), (2, 32 + 32 + 2 * 64));
    assert_eq!(
        Args::new(&empty_bytes_array(3))
            .bytes_array_at(0, 32, 2)
            .map(|_| ()),
        Err(Error::TooDeep)
    );

    let two = empty_call_array(2);
    let (calls, end) = Args::new(&two).call_tuple_array_at(0, 32, 2).unwrap();
    assert_eq!((calls.len(), end), (2, 32 + 32 + 2 * 160));
    assert_eq!(
        Args::new(&empty_call_array(3))
            .call_tuple_array_at(0, 32, 2)
            .map(|_| ()),
        Err(Error::TooDeep)
    );
}

// rlp.rs:72 — nesting is capped at MAX_DEPTH, and nothing tested the cap at
// its boundary: an item inside exactly MAX_DEPTH lists is read, one list more
// is refused.
#[test]
fn rlp_nests_exactly_to_its_limit() {
    use clearsign::rlp::{MAX_DEPTH, decode};
    let nested = |lists: usize| {
        let mut item = vec![0x01u8];
        for _ in 0..lists {
            let mut list = vec![0xc0 + item.len() as u8];
            list.extend_from_slice(&item);
            item = list;
        }
        item
    };
    assert!(decode(&nested(MAX_DEPTH)).is_ok());
    assert_eq!(
        decode(&nested(MAX_DEPTH + 1)).map(|_| ()),
        Err(Error::TooDeep)
    );
}

// abi.rs:69, abi.rs:117, error.rs:33 — truncated and trailing arguments were
// both refused, but nothing checked that the reason given was the right one.
#[test]
fn a_refusal_says_whether_bytes_were_missing_or_extra() {
    let a = Args::new(&[0u8; 32]);
    assert_eq!(a.expect_static_len(2), Err(Error::Truncated));
    assert_eq!(a.expect_static_len(0), Err(Error::TrailingBytes));
    assert_eq!(a.expect_end(64), Err(Error::Truncated));
    assert_eq!(a.expect_end(0), Err(Error::TrailingBytes));

    let mut short = transfer(TOKEN, 1);
    short.truncate(40);
    let r = review_safe_transaction(&safe_call(TOKEN, short, 0), DomainVersion::V1_3Plus);
    assert!(
        r.render()
            .contains("input ended before the structure was complete"),
        "{}",
        r.render()
    );
    let mut long = transfer(TOKEN, 1);
    long.extend_from_slice(&[0; 32]);
    let r = review_safe_transaction(&safe_call(TOKEN, long, 0), DomainVersion::V1_3Plus);
    assert!(
        r.render()
            .contains("unexpected bytes after the end of the structure"),
        "{}",
        r.render()
    );
}

// evm.rs:103 — `||` to `&&`: an unsigned EIP-155 transaction with only one of
// r and s filled in was accepted by the mutant.
#[test]
fn an_unsigned_legacy_transaction_with_either_r_or_s_set_is_refused() {
    // [nonce, gasPrice, gas, to, value, data, chainId 1, r, s]
    let make = |r: &[u8], s: &[u8]| -> Vec<u8> {
        let mut payload = vec![0x80, 0x01, 0x82, 0x52, 0x08, 0x94];
        payload.extend_from_slice(&TOKEN);
        payload.extend_from_slice(&[0x01, 0x80, 0x01]);
        payload.extend_from_slice(r);
        payload.extend_from_slice(s);
        let mut b = vec![0xc0 + payload.len() as u8];
        b.extend_from_slice(&payload);
        b
    };
    assert!(parse_unsigned_transaction(&make(&[0x80], &[0x80])).is_ok());
    assert!(matches!(
        parse_unsigned_transaction(&make(&[0x01], &[0x80])),
        Err(Error::NonCanonical(_))
    ));
    assert!(matches!(
        parse_unsigned_transaction(&make(&[0x80], &[0x01])),
        Err(Error::NonCanonical(_))
    ));
}

// evm.rs:186 and evm.rs:198 — the access list count and the ACCESS_LIST note.
#[test]
fn typed_transactions_show_their_access_list_and_legacy_ones_do_not() {
    // EIP-1559 with one access-list entry: [0x22.., []]
    let mut fields = vec![0x01, 0x80, 0x01, 0x01, 0x82, 0x52, 0x08, 0x94];
    fields.extend_from_slice(&TOKEN);
    fields.extend_from_slice(&[0x80, 0x80]);
    let mut entry = vec![0x94];
    entry.extend_from_slice(&TOKEN);
    entry.push(0xc0);
    let mut list = vec![0xc0 + entry.len() as u8];
    list.extend_from_slice(&entry);
    let mut access = vec![0xc0 + list.len() as u8];
    access.extend_from_slice(&list);
    fields.extend_from_slice(&access);
    let mut bytes = vec![0x02, 0xc0 + fields.len() as u8];
    bytes.extend_from_slice(&fields);
    let r = review_transaction_bytes(&bytes).unwrap();
    assert!(r.has("ACCESS_LIST"));
    assert!(
        r.render()
            .lines()
            .any(|l| l.starts_with("Access list entries ") && l.ends_with(" 1")),
        "{}",
        r.render()
    );

    let mut legacy = vec![0x80, 0x01, 0x82, 0x52, 0x08, 0x94];
    legacy.extend_from_slice(&TOKEN);
    legacy.extend_from_slice(&[0x01, 0x80]);
    let mut b = vec![0xc0 + legacy.len() as u8];
    b.extend_from_slice(&legacy);
    let r = review_transaction_bytes(&b).unwrap();
    assert!(!r.has("ACCESS_LIST"));
    assert!(!r.render().contains("Access list entries"));
}

// safe.rs:118 — `||` to `&&`: the refund fields must be shown when any one of
// them is set, not only when all are.
#[test]
fn any_one_refund_field_set_shows_all_of_them() {
    let mut a = safe_call(TOKEN, vec![], 0);
    a.value = U256::from_u64(1);
    a.refund_receiver = [0x66; 20];
    assert!(
        review_safe_transaction(&a, DomainVersion::V1_3Plus)
            .render()
            .contains("refundReceiver")
    );
    let mut b = safe_call(TOKEN, vec![], 0);
    b.value = U256::from_u64(1);
    b.gas_token = OTHER_TOKEN;
    assert!(
        review_safe_transaction(&b, DomainVersion::V1_3Plus)
            .render()
            .contains("gasToken")
    );
    let mut c = safe_call(TOKEN, vec![], 0);
    c.value = U256::from_u64(1);
    assert!(
        !review_safe_transaction(&c, DomainVersion::V1_3Plus)
            .render()
            .contains("refundReceiver")
    );
}

// calls.rs:834, calls.rs:839 — the refund finding names what it pays and to whom.
#[test]
fn the_refund_finding_names_the_currency_and_the_receiver() {
    let mut a = safe_call(TOKEN, vec![], 0);
    a.gas_price = U256::from_u64(5);
    let text = review_safe_transaction(&a, DomainVersion::V1_3Plus).render();
    assert!(
        text.contains("native currency") && text.contains("whoever executes the transaction"),
        "{text}"
    );
    a.gas_token = OTHER_TOKEN;
    a.refund_receiver = [0x66; 20];
    let text = review_safe_transaction(&a, DomainVersion::V1_3Plus).render();
    assert!(text.contains("token 0x2323"), "{text}");
    assert!(
        text.contains("0x6666666666666666666666666666666666666666"),
        "{text}"
    );
}

// calls.rs:248 — the Safe-in-Safe nesting limit, at the boundary.
#[test]
fn safe_transactions_nest_exactly_to_the_limit() {
    // The outer Safe call is depth 1; each execTransaction inside adds one.
    let mut data = transfer(TOKEN, 1);
    for _ in 0..(MAX_NESTING - 1) {
        data = exec_transaction(TOKEN, &data);
    }
    let r = review_safe_transaction(&safe_call(SAFE, data.clone(), 0), DomainVersion::V1_3Plus);
    assert!(!r.has("NESTING_LIMIT"), "{}", r.render());
    assert!(r.render().contains("Matches ERC-20 transfer"));
    let deeper = exec_transaction(TOKEN, &data);
    let r = review_safe_transaction(&safe_call(SAFE, deeper, 0), DomainVersion::V1_3Plus);
    assert!(r.has("NESTING_LIMIT"));
    assert!(
        r.findings()
            .iter()
            .any(|f| f.code == "UNDECODED_CALLS" && f.severity == clearsign::Severity::Critical)
    );
}

// calls.rs:391 — batches nested to the limit are decoded, one more is not.
#[test]
fn batches_nest_exactly_to_the_limit() {
    // Safe (1) -> execTransaction (2) -> MultiSend batch call at depth 3.
    let batch = multisend(&[(0, TOKEN, 0, transfer(TOKEN, 1))]);
    let mut data = batch.clone();
    data = exec_transaction(MULTISEND_130, &data);
    // The inner Safe transaction delegatecalls the batch.
    let pos = 4 + 3 * 32 + 31;
    data[pos] = 1;
    let r = review_safe_transaction(&safe_call(SAFE, data, 0), DomainVersion::V1_3Plus);
    assert!(r.has("SAFE_MULTISEND_BATCH"), "{}", r.render());
    assert!(!r.has("NESTING_LIMIT"), "{}", r.render());
}

// calls.rs:355, calls.rs:366, calls.rs:372 — what a batch shows about itself.
#[test]
fn a_batch_shows_its_total_its_overflow_and_its_own_value() {
    let calls: Vec<_> = (0..34).map(|i| (0u8, TOKEN, i as u64, vec![])).collect();
    let mut tx = safe_call(MULTISEND_130, multisend(&calls), 1);
    let text = review_safe_transaction(&tx, DomainVersion::V1_3Plus).render();
    assert!(text.contains("Native value, whole batch (wei)"), "{text}");
    // The field itself: "Batch call 32 of 34" also contains "32 of 34", which
    // let the second run's mutant that hid this field go unnoticed.
    assert!(
        text.contains("32 of 34 — the rest are not displayed"),
        "{text}"
    );
    assert!(!text.contains("\nNative value (wei) ...") || text.contains("Native value (wei)"));

    tx.data = multisend(&[(0, TOKEN, 0, transfer(TOKEN, 1))]);
    let text = review_safe_transaction(&tx, DomainVersion::V1_3Plus).render();
    assert!(!text.contains("whole batch"), "{text}");
    assert!(!text.contains(" of 1 — the rest"), "{text}");

    tx.value = U256::from_u64(7);
    let text = review_safe_transaction(&tx, DomainVersion::V1_3Plus).render();
    let batch_section = text.split("-- Batch call").next().unwrap();
    assert!(
        batch_section.contains("Native value (wei) ") && batch_section.contains(" 7\n"),
        "{text}"
    );
}

// calls.rs:502, calls.rs:510 — the positions of hidden findings are listed.
#[test]
fn hidden_findings_list_where_they_are() {
    let mut calls: Vec<_> = (0..45).map(|_| (0u8, TOKEN, 0u64, vec![])).collect();
    let mut unlimited = SEL_APPROVE.to_vec();
    unlimited.extend(word_addr([0x66; 20]));
    unlimited.extend(vec![0xff; 32]);
    for call in calls.iter_mut().skip(32) {
        call.3 = unlimited.clone();
    }
    let r = review_safe_transaction(
        &safe_call(MULTISEND_130, multisend(&calls), 1),
        DomainVersion::V1_3Plus,
    );
    let f = r
        .findings()
        .iter()
        .find(|f| f.code == "UNLIMITED_APPROVAL")
        .unwrap();
    assert!(
        f.message
            .contains("33, 34, 35, 36, 37, 38, 39, 40 and 5 more"),
        "{}",
        f.message
    );

    calls.truncate(35);
    let r = review_safe_transaction(
        &safe_call(MULTISEND_130, multisend(&calls), 1),
        DomainVersion::V1_3Plus,
    );
    let f = r
        .findings()
        .iter()
        .find(|f| f.code == "UNLIMITED_APPROVAL")
        .unwrap();
    assert!(f.message.contains(": 33, 34, 35."), "{}", f.message);
}

// calls.rs:872 — one SELECTOR_IS_NOT_BEHAVIOUR notice per destination.
// calls.rs:890 — one TOKEN_UNITS_RAW notice per review.
#[test]
fn notices_are_one_per_destination_and_one_per_review() {
    let calls = vec![
        (0u8, TOKEN, 0u64, transfer(SAFE, 1)),
        (0u8, TOKEN, 0u64, transfer(SAFE, 2)),
        (0u8, OTHER_TOKEN, 0u64, transfer(SAFE, 3)),
    ];
    let r = review_safe_transaction(
        &safe_call(MULTISEND_130, multisend(&calls), 1),
        DomainVersion::V1_3Plus,
    );
    let notices: Vec<_> = r
        .findings()
        .iter()
        .filter(|f| f.code == "SELECTOR_IS_NOT_BEHAVIOUR")
        .collect();
    assert_eq!(notices.len(), 2, "{notices:?}");
    assert_eq!(
        r.findings()
            .iter()
            .filter(|f| f.code == "TOKEN_UNITS_RAW")
            .count(),
        1
    );
}

// calls.rs:905 — what a delegatecall's calldata selector reads as when there
// is none, or less than a selector.
#[test]
fn a_delegatecall_with_no_selector_says_so() {
    let r = review_safe_transaction(&safe_call(TOKEN, vec![], 1), DomainVersion::V1_3Plus);
    assert!(
        r.render()
            .lines()
            .any(|l| l.starts_with("Calldata selector ") && l.ends_with(" (none)")),
        "{}",
        r.render()
    );
    let r = review_safe_transaction(
        &safe_call(TOKEN, vec![0xab, 0xcd], 1),
        DomainVersion::V1_3Plus,
    );
    assert!(r.render().contains("(short: 0xabcd)"), "{}", r.render());
    let _ = hex::encode(&[]);
}
