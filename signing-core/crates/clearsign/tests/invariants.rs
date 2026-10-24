//! One or more named tests per threat-model invariant (docs/01-threat-model.md).

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use clearsign::rlp;
use clearsign::{DomainVersion, Error, SafeTransaction, Severity, TxType, U256};
use clearsign::{hex, parse_unsigned_transaction, review_evm_transaction, review_safe_transaction};

const SAFE: [u8; 20] = [0x11; 20];
const TOKEN: [u8; 20] = [0x22; 20];

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

fn call(sel: [u8; 4], words: &[Vec<u8>]) -> Vec<u8> {
    let mut d = sel.to_vec();
    for w in words {
        d.extend_from_slice(w);
    }
    d
}

// ---------------------------------------------------------------- INV-2

#[test]
fn inv2_unknown_selector_is_blind() {
    let tx = safe_call(TOKEN, vec![0xde, 0xad, 0xbe, 0xef, 0, 0, 0, 1], 0);
    let r = review_safe_transaction(&tx, DomainVersion::V1_3Plus);
    assert!(r.has("UNKNOWN_SELECTOR"));
    assert!(r.highest_severity() >= Some(Severity::Blind));
    assert!(r.render().contains("DO NOT SIGN"));
}

#[test]
fn inv2_short_calldata_is_blind() {
    let r = review_safe_transaction(
        &safe_call(TOKEN, vec![0x01, 0x02], 0),
        DomainVersion::V1_3Plus,
    );
    assert!(r.has("SHORT_CALLDATA"));
}

#[test]
fn inv2_contract_creation_is_blind() {
    // EIP-1559, chain 1, nonce 0, fees 1, gas 21000, to = empty, value 0, data 0x6000, access list []
    let bytes = hex::decode(
        "0x02cd0180010182520880808260 00c0"
            .replace(' ', "")
            .as_str(),
    )
    .unwrap();
    let tx = parse_unsigned_transaction(&bytes).unwrap();
    assert_eq!(tx.to, None);
    let r = review_evm_transaction(&tx);
    assert!(r.has("CONTRACT_CREATION"));
}

// ---------------------------------------------------------------- INV-3

#[test]
fn inv3_rendering_is_deterministic() {
    let data = call(
        clearsign::calls::SEL_APPROVE,
        &[word_addr([0x33; 20]), vec![0xff; 32]],
    );
    let tx = safe_call(TOKEN, data, 0);
    let a = review_safe_transaction(&tx, DomainVersion::V1_3Plus).render();
    let b = review_safe_transaction(&tx.clone(), DomainVersion::V1_3Plus).render();
    assert_eq!(a, b);
}

#[test]
fn inv3_golden_render_unlimited_approval() {
    let data = call(
        clearsign::calls::SEL_APPROVE,
        &[word_addr([0x33; 20]), vec![0xff; 32]],
    );
    let text =
        review_safe_transaction(&safe_call(TOKEN, data, 0), DomainVersion::V1_3Plus).render();
    let golden = include_str!("golden/safe_unlimited_approval.txt");
    assert_eq!(
        text, golden,
        "rendering changed; if intended, update the golden file after review"
    );
}

// ---------------------------------------------------------------- INV-4

#[test]
fn inv4_rlp_rejects_single_byte_wrapped_in_string() {
    assert!(matches!(
        rlp::decode(&[0x81, 0x05]),
        Err(Error::NonCanonical(_))
    ));
    assert!(rlp::decode(&[0x81, 0x80]).is_ok());
}

#[test]
fn inv4_rlp_rejects_long_form_for_short_payload() {
    let mut v = vec![0xb8, 0x02];
    v.extend_from_slice(&[0xaa, 0xbb]);
    assert!(matches!(rlp::decode(&v), Err(Error::NonCanonical(_))));
}

#[test]
fn inv4_rlp_rejects_length_with_leading_zero() {
    let mut v = vec![0xb9, 0x00, 0x38];
    v.extend(std::iter::repeat_n(0xaa, 56));
    assert!(matches!(rlp::decode(&v), Err(Error::NonCanonical(_))));
}

#[test]
fn inv4_rlp_rejects_trailing_bytes_and_truncation() {
    assert_eq!(rlp::decode(&[0x80, 0x00]), Err(Error::TrailingBytes));
    assert_eq!(rlp::decode(&[0x83, 0x01]), Err(Error::Truncated));
}

#[test]
fn inv4_rlp_depth_is_bounded() {
    let mut v = vec![0xc1; 40];
    v.push(0x80);
    // Inner lengths are wrong on purpose for most levels; either way it must be an error, not a stack overflow.
    assert!(rlp::decode(&v).is_err());

    let mut nested = vec![0xc0];
    for _ in 0..(rlp::MAX_DEPTH + 2) {
        let mut outer = vec![0xc0 + nested.len() as u8];
        outer.extend_from_slice(&nested);
        nested = outer;
    }
    assert_eq!(rlp::decode(&nested), Err(Error::TooDeep));
}

#[test]
fn inv4_transaction_integer_with_leading_zero_is_rejected() {
    // Nonce encoded as the two-byte string 0x0007: valid RLP, but not a canonical integer.
    let bytes = hex::decode("0x02cd018200070101825208808080c0").unwrap();
    assert!(matches!(
        parse_unsigned_transaction(&bytes),
        Err(Error::NonCanonical(_))
    ));
}

#[test]
fn inv4_dirty_address_bits_refuse_to_interpret() {
    let mut spender = word_addr([0x33; 20]);
    spender[0] = 0x01; // dirty high bit: an ABI v2 decoder would revert
    let data = call(clearsign::calls::SEL_APPROVE, &[spender, word_u64(5)]);
    let r = review_safe_transaction(&safe_call(TOKEN, data, 0), DomainVersion::V1_3Plus);
    assert!(r.has("MALFORMED_ARGUMENTS"));
    assert!(!r.render().contains("ERC-20 approve"));
}

#[test]
fn inv4_trailing_calldata_refuses_to_interpret() {
    let mut data = call(
        clearsign::calls::SEL_TRANSFER,
        &[word_addr([0x33; 20]), word_u64(5)],
    );
    data.push(0x00);
    let r = review_safe_transaction(&safe_call(TOKEN, data, 0), DomainVersion::V1_3Plus);
    assert!(r.has("MALFORMED_ARGUMENTS"));
}

#[test]
fn inv4_exec_transaction_noncanonical_offset_refuses_to_interpret() {
    // execTransaction with the bytes offset pointing 32 bytes later than standard.
    let mut d = clearsign::calls::SEL_EXEC_TRANSACTION.to_vec();
    let head = [
        word_addr(TOKEN),
        word_u64(0),
        word_u64(0x160), // standard would be 0x140
        word_u64(0),
        word_u64(0),
        word_u64(0),
        word_u64(0),
        word_addr([0; 20]),
        word_addr([0; 20]),
        word_u64(0x1a0),
    ];
    for w in head {
        d.extend_from_slice(&w);
    }
    d.extend_from_slice(&word_u64(0)); // filler
    d.extend_from_slice(&word_u64(0)); // data length 0
    d.extend_from_slice(&word_u64(0)); // signatures length 0
    let r = review_evm_transaction(&parse_evm_call(SAFE, d));
    assert!(r.has("MALFORMED_ARGUMENTS"), "{}", r.render());
}

fn parse_evm_call(to: [u8; 20], data: Vec<u8>) -> clearsign::EvmTransaction {
    clearsign::EvmTransaction {
        tx_type: TxType::Eip1559,
        chain_id: Some(U256::from_u64(1)),
        nonce: U256::ZERO,
        gas_price: None,
        max_priority_fee_per_gas: Some(U256::ZERO),
        max_fee_per_gas: Some(U256::ZERO),
        gas_limit: U256::from_u64(21_000),
        to: Some(to),
        value: U256::ZERO,
        data,
        access_list_entries: 0,
        max_fee_per_blob_gas: None,
        blob_versioned_hashes: Vec::new(),
        authorizations: Vec::new(),
        signing_hash: [0; 32],
    }
}

// ---------------------------------------------------------------- INV-5

#[test]
fn inv5_safe_review_always_carries_the_safe_tx_hash() {
    let r = review_safe_transaction(&safe_call(TOKEN, vec![], 0), DomainVersion::V1_3Plus);
    assert!(
        r.digests
            .iter()
            .any(|(label, value)| label == "Safe transaction hash" && value.len() == 66)
    );
}

// ---------------------------------------------------------------- INV-6

#[test]
fn inv6_admin_changes_on_the_safe_itself_are_critical() {
    use clearsign::calls::*;
    let cases: Vec<(Vec<u8>, &str)> = vec![
        (
            call(SEL_ENABLE_MODULE, &[word_addr([0x44; 20])]),
            "SAFE_MODULE_CHANGE",
        ),
        (
            call(
                SEL_DISABLE_MODULE,
                &[word_addr([0x01; 20]), word_addr([0x44; 20])],
            ),
            "SAFE_MODULE_CHANGE",
        ),
        (
            call(SEL_SET_GUARD, &[word_addr([0x44; 20])]),
            "SAFE_GUARD_CHANGE",
        ),
        (
            call(SEL_SET_FALLBACK_HANDLER, &[word_addr([0x44; 20])]),
            "SAFE_FALLBACK_HANDLER_CHANGE",
        ),
        (
            call(SEL_CHANGE_THRESHOLD, &[word_u64(1)]),
            "SAFE_THRESHOLD_CHANGE",
        ),
        (
            call(
                SEL_SWAP_OWNER,
                &[word_addr([1; 20]), word_addr([2; 20]), word_addr([3; 20])],
            ),
            "SAFE_OWNER_CHANGE",
        ),
        (
            call(
                SEL_REMOVE_OWNER,
                &[word_addr([1; 20]), word_addr([2; 20]), word_u64(1)],
            ),
            "SAFE_OWNER_CHANGE",
        ),
        (
            call(SEL_CHANGE_MASTER_COPY, &[word_addr([0x44; 20])]),
            "SAFE_IMPLEMENTATION_CHANGE",
        ),
    ];
    for (data, code) in cases {
        let r = review_safe_transaction(&safe_call(SAFE, data, 0), DomainVersion::V1_3Plus);
        assert!(r.has(code), "expected {code}:\n{}", r.render());
        assert_eq!(r.highest_severity(), Some(Severity::Critical));
    }
}

#[test]
fn inv6_admin_selector_on_another_contract_is_a_warning_not_critical() {
    let data = call(
        clearsign::calls::SEL_ENABLE_MODULE,
        &[word_addr([0x44; 20])],
    );
    let r = review_safe_transaction(&safe_call(TOKEN, data, 0), DomainVersion::V1_3Plus);
    assert!(r.has("SAFE_ADMIN_SELECTOR_ON_OTHER_CONTRACT"));
    assert!(!r.has("SAFE_MODULE_CHANGE"));
}

#[test]
fn inv6_invalid_operation_is_critical() {
    let r = review_safe_transaction(&safe_call(TOKEN, vec![], 2), DomainVersion::V1_3Plus);
    assert!(r.has("SAFE_INVALID_OPERATION"));
}

#[test]
fn inv6_nested_delegatecall_inside_exec_transaction_is_critical() {
    let inner = call(
        clearsign::calls::SEL_TRANSFER,
        &[word_addr([0x33; 20]), word_u64(0)],
    );
    let mut d = clearsign::calls::SEL_EXEC_TRANSACTION.to_vec();
    let padded = inner.len().div_ceil(32) * 32;
    for w in [
        word_addr(TOKEN),
        word_u64(0),
        word_u64(0x140),
        word_u64(1), // DELEGATECALL
        word_u64(0),
        word_u64(0),
        word_u64(0),
        word_addr([0; 20]),
        word_addr([0; 20]),
        word_u64((0x140 + 32 + padded) as u64),
    ] {
        d.extend_from_slice(&w);
    }
    d.extend_from_slice(&word_u64(inner.len() as u64));
    d.extend_from_slice(&inner);
    d.extend(std::iter::repeat_n(0u8, padded - inner.len()));
    d.extend_from_slice(&word_u64(0));
    let r = review_evm_transaction(&parse_evm_call(SAFE, d));
    assert!(r.has("SAFE_DELEGATECALL"), "{}", r.render());
}

// ---------------------------------------------------------------- INV-7

/// Deterministic xorshift so failures are reproducible without a fuzzing dependency.
struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn bytes(&mut self, max_len: usize) -> Vec<u8> {
        let len = (self.next() as usize) % max_len;
        (0..len).map(|_| self.next() as u8).collect()
    }
}

#[test]
fn inv7_random_inputs_never_panic() {
    let mut rng = Rng(0x9e37_79b9_7f4a_7c15);
    for i in 0..60_000u32 {
        let mut b = rng.bytes(700);
        // Bias a third of inputs towards structurally plausible transactions.
        if i % 3 == 0 {
            b.insert(0, 0x02);
        }
        if let Ok(tx) = parse_unsigned_transaction(&b) {
            let _ = review_evm_transaction(&tx).render();
        }
        let _ = rlp::decode(&b);

        let op = (rng.next() % 3) as u8;
        let mut data = b.clone();
        if i % 2 == 0 && data.len() >= 4 {
            let sels = clearsign::calls::KNOWN_SIGNATURES;
            let sel = sels[(rng.next() as usize) % sels.len()].0;
            data[..4].copy_from_slice(&sel);
        }
        let _ =
            review_safe_transaction(&safe_call(TOKEN, data, op), DomainVersion::V1_3Plus).render();
    }
}

#[test]
fn inv7_mutated_real_transactions_never_panic() {
    let seed = hex::decode("0x02f86e0107843b9aca008506fc23ac00830186a094a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4880b844095ea7b30000000000000000000000003fc91a3afd70395cd496c647d5a6cc9d4b2b7fadffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffc0").unwrap();
    let mut rng = Rng(42);
    for _ in 0..60_000 {
        let mut b = seed.clone();
        for _ in 0..(1 + rng.next() % 4) {
            let idx = (rng.next() as usize) % b.len();
            b[idx] = rng.next() as u8;
        }
        if rng.next() % 5 == 0 {
            let cut = (rng.next() as usize) % b.len();
            b.truncate(cut);
        }
        if let Ok(tx) = parse_unsigned_transaction(&b) {
            let _ = review_evm_transaction(&tx).render();
        }
    }
}

// ---------------------------------------------------------------- parsing basics

#[test]
fn signed_transactions_are_rejected() {
    // 12 fields (y_parity, r, s appended), built with `cast to-rlp`.
    let bytes = hex::decode("0x02ce01800101825208808080c0010101").unwrap();
    assert!(matches!(
        parse_unsigned_transaction(&bytes),
        Err(Error::WrongFieldCount {
            expected: 9,
            found: 12
        })
    ));
}

#[test]
fn legacy_without_chain_id_warns_about_replay() {
    // [nonce 0, gasPrice 1, gas 21000, to 0x22..22, value 1, data empty]
    let mut payload = vec![0x80, 0x01, 0x82, 0x52, 0x08, 0x94];
    payload.extend_from_slice(&TOKEN);
    payload.extend_from_slice(&[0x01, 0x80]);
    let mut bytes = vec![0xc0 + payload.len() as u8];
    bytes.extend_from_slice(&payload);
    let tx = parse_unsigned_transaction(&bytes).unwrap();
    assert_eq!(tx.tx_type, TxType::Legacy);
    let r = review_evm_transaction(&tx);
    assert!(r.has("REPLAYABLE_NO_CHAIN_ID"));
}

#[test]
fn u256_decimal_round_trip() {
    let max = "115792089237316195423570985008687907853269984665640564039457584007913129639935";
    assert_eq!(U256::from_decimal(max).unwrap(), U256::MAX);
    assert_eq!(U256::MAX.to_decimal(), max);
    assert!(
        U256::from_decimal(
            "115792089237316195423570985008687907853269984665640564039457584007913129639936"
        )
        .is_err()
    );
    assert_eq!(U256::from_decimal("0").unwrap().to_decimal(), "0");
    assert_eq!(U256::from_u64(1_000_000).to_grouped_decimal(), "1_000_000");
    assert!(U256::from_decimal("-1").is_err());
    assert!(U256::from_decimal("").is_err());
}
