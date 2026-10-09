//! Safe MultiSend batches.
//!
//! The calldata vectors here came out of Foundry's `cast` verbatim
//! (`cast calldata "multiSend(bytes)" 0x<packed>`); the packing commands are in
//! `tests/vectors/README.md`. The deployment addresses came from
//! `safe-global/safe-deployments`. Nothing in this file was hand-assembled.

#![allow(
    clippy::expect_used,
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use clearsign::address::checksummed;
use clearsign::multisend::{self, MAX_BATCH_CALLS};
use clearsign::{DomainVersion, SafeTransaction, Severity, U256, hex, review_safe_transaction};

/// MultiSendCallOnly 1.3.0, canonical.
const MULTISEND_1_3_0: &str = "0x40A2aCCbd92BCA938b02010E17A5b8929b49130D";
const NOT_MULTISEND: &str = "0x00000000000000000000000000000000DeaDBeef";
const SAFE: &str = "0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4";

/// Two plain ERC-20 calls: `transfer(0x…DeaDBeef, 1500000)` then
/// `approve(0x1111…, 0)`, both on USDC, packed and ABI-encoded by `cast`.
const BATCH_TWO_CALLS: &str = "0x8d80ff0a0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000013200a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000044a9059cbb00000000000000000000000000000000000000000000000000000000deadbeef000000000000000000000000000000000000000000000000000000000016e36000a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000044095ea7b3000000000000000000000000111111111111111111111111111111111111111100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

/// The same batch, except the second element's operation byte is 1: a
/// DELEGATECALL smuggled inside an otherwise ordinary-looking batch.
const BATCH_WITH_INNER_DELEGATECALL: &str = "0x8d80ff0a0000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000013200a0b86991c6218b36c1d19d4a2e9eb0ce3606eb4800000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000044a9059cbb00000000000000000000000000000000000000000000000000000000deadbeef000000000000000000000000000000000000000000000000000000000016e3600100000000000000000000000000000000deadbeef00000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000044095ea7b3000000000000000000000000111111111111111111111111111111111111111100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000";

fn addr(s: &str) -> [u8; 20] {
    hex::decode(s).unwrap().try_into().unwrap()
}

/// A Safe transaction that DELEGATECALLs `to` with `data`.
fn batch_tx(to: &str, data: &str) -> SafeTransaction {
    SafeTransaction {
        chain_id: U256::from_u64(1),
        safe: addr(SAFE),
        to: addr(to),
        value: U256::from_u64(0),
        data: hex::decode(data).unwrap(),
        operation: 1,
        safe_tx_gas: U256::from_u64(0),
        base_gas: U256::from_u64(0),
        gas_price: U256::from_u64(0),
        gas_token: [0u8; 20],
        refund_receiver: [0u8; 20],
        nonce: U256::from_u64(7),
    }
}

fn codes(review: &clearsign::Review) -> Vec<&'static str> {
    review.findings().iter().map(|f| f.code).collect()
}

fn max_severity(review: &clearsign::Review) -> Severity {
    review
        .findings()
        .iter()
        .map(|f| f.severity)
        .max()
        .unwrap_or(Severity::Info)
}

#[test]
fn every_pinned_address_matches_its_published_checksum() {
    // Guards against a transcription error in the deployment table: the bytes and
    // the address as Safe publishes it must be the same address.
    for d in multisend::DEPLOYMENTS {
        assert_eq!(
            checksummed(&d.address),
            d.checksummed,
            "{} {} {}",
            d.contract,
            d.version,
            d.variant
        );
    }
    assert_eq!(multisend::DEPLOYMENTS.len(), 11);
}

#[test]
fn pinned_addresses_are_unique() {
    for (i, a) in multisend::DEPLOYMENTS.iter().enumerate() {
        for b in &multisend::DEPLOYMENTS[i + 1..] {
            assert_ne!(a.address, b.address, "duplicate deployment address");
        }
    }
}

#[test]
fn ordinary_batch_is_readable_and_is_not_critical() {
    // The point of the whole feature: a routine batch must not read CRITICAL,
    // because a warning that always fires is a warning nobody reads.
    let review = review_safe_transaction(
        &batch_tx(MULTISEND_1_3_0, BATCH_TWO_CALLS),
        DomainVersion::V1_3Plus,
    );
    assert_eq!(max_severity(&review), Severity::Warning);
    assert!(codes(&review).contains(&"SAFE_MULTISEND_BATCH"));
    assert!(!codes(&review).contains(&"SAFE_DELEGATECALL"));
    assert!(!codes(&review).contains(&"CARRIED_CALL_SEMANTICS_UNVERIFIED"));

    let text = review.render();
    assert!(text.contains("Batch call 1 of 2"), "{text}");
    assert!(text.contains("Batch call 2 of 2"), "{text}");
    assert!(text.contains("ERC-20 transfer"), "{text}");
    assert!(text.contains("ERC-20 approve"), "{text}");
    // The recipient and amount of each inner call are shown, not summarised away.
    assert!(text.contains("1_500_000"), "{text}");
    assert!(text.contains("MultiSendCallOnly 1.3.0"), "{text}");
}

#[test]
fn a_delegatecall_inside_a_batch_is_still_critical() {
    let review = review_safe_transaction(
        &batch_tx(MULTISEND_1_3_0, BATCH_WITH_INNER_DELEGATECALL),
        DomainVersion::V1_3Plus,
    );
    assert_eq!(max_severity(&review), Severity::Critical);
    assert!(codes(&review).contains(&"SAFE_DELEGATECALL"));
}

#[test]
fn the_same_calldata_at_an_unpinned_address_stays_critical() {
    // Same bytes, different batching contract. Being called `multiSend` earns
    // nothing; only a published deployment address is looked inside.
    let review = review_safe_transaction(
        &batch_tx(NOT_MULTISEND, BATCH_TWO_CALLS),
        DomainVersion::V1_3Plus,
    );
    assert_eq!(max_severity(&review), Severity::Critical);
    assert!(codes(&review).contains(&"SAFE_DELEGATECALL"));
    assert!(!codes(&review).contains(&"SAFE_MULTISEND_BATCH"));
}

#[test]
fn a_truncated_batch_is_refused_not_half_decoded() {
    let mut data = hex::decode(BATCH_TWO_CALLS).unwrap();
    data.truncate(data.len() - 1);
    // Fix up nothing: the ABI length now overruns the payload.
    let tx = SafeTransaction {
        data,
        ..batch_tx(MULTISEND_1_3_0, BATCH_TWO_CALLS)
    };
    let review = review_safe_transaction(&tx, DomainVersion::V1_3Plus);
    assert_eq!(max_severity(&review), Severity::Critical);
    assert!(codes(&review).contains(&"MULTISEND_MALFORMED"));
    assert!(!review.render().contains("Batch call 1"));
}

#[test]
fn a_batch_element_that_overruns_the_payload_is_refused() {
    // Declare a data length one byte longer than what follows.
    let mut packed = Vec::new();
    packed.push(0u8);
    packed.extend_from_slice(&[0x11u8; 20]);
    packed.extend_from_slice(&[0u8; 32]);
    let mut len = [0u8; 32];
    len[31] = 5;
    packed.extend_from_slice(&len);
    packed.extend_from_slice(&[0xaa; 4]);
    assert!(multisend::decode_batch(&packed).is_err());
}

#[test]
fn trailing_bytes_after_the_last_element_are_refused() {
    let mut packed = Vec::new();
    packed.push(0u8);
    packed.extend_from_slice(&[0x11u8; 20]);
    packed.extend_from_slice(&[0u8; 32]);
    packed.extend_from_slice(&[0u8; 32]);
    assert_eq!(multisend::decode_batch(&packed).unwrap().len(), 1);
    packed.push(0x00); // one stray byte: not a complete element
    assert!(multisend::decode_batch(&packed).is_err());
}

#[test]
fn an_empty_batch_is_refused() {
    assert!(multisend::decode_batch(&[]).is_err());
}

#[test]
fn a_batch_is_parsed_further_than_it_is_displayed() {
    // The decoder reads past the display limit so it can answer the question
    // that matters about a long batch — is there a DELEGATECALL in the part you
    // are not being shown? — and stops at a hard limit beyond which nothing can
    // be ruled out. See the audit regression tests for what the review says.
    let element = |out: &mut Vec<u8>| {
        out.push(0u8);
        out.extend_from_slice(&[0x11u8; 20]);
        out.extend_from_slice(&[0u8; 32]);
        out.extend_from_slice(&[0u8; 32]);
    };
    let mut past_display = Vec::new();
    for _ in 0..MAX_BATCH_CALLS + 1 {
        element(&mut past_display);
    }
    let parsed = multisend::decode_batch(&past_display).expect("parsed past the display limit");
    assert_eq!(parsed.len(), MAX_BATCH_CALLS + 1);

    let mut past_parsing = Vec::new();
    for _ in 0..multisend::MAX_BATCH_CALLS_PARSED + 1 {
        element(&mut past_parsing);
    }
    assert!(multisend::decode_batch(&past_parsing).is_err());
}

#[test]
fn every_deployment_publishes_at_least_one_chain_and_lists_them_sorted() {
    for d in multisend::DEPLOYMENTS {
        assert!(
            !d.chains.is_empty(),
            "{} {} has no chains",
            d.contract,
            d.version
        );
        assert!(
            d.chains.windows(2).all(|w| w[0] < w[1]),
            "{} {} {} chain list is not sorted and unique",
            d.contract,
            d.version,
            d.variant
        );
    }
    // Mainnet must be present for the canonical deployments, or the lookup below
    // would be vacuous.
    let mainnet = multisend::DEPLOYMENTS
        .iter()
        .filter(|d| d.variant == "canonical" && d.chains.contains(&1))
        .count();
    assert!(mainnet >= 3, "canonical deployments on mainnet: {mainnet}");
}

#[test]
fn a_published_address_on_an_unpublished_chain_is_not_decoded() {
    // Same address, same calldata, a chain Safe does not publish this deployment
    // on. The 20 bytes alone are not evidence about the code that will run.
    let mut tx = batch_tx(MULTISEND_1_3_0, BATCH_TWO_CALLS);
    tx.chain_id = U256::from_u64(31_337); // a local development chain
    let review = review_safe_transaction(&tx, DomainVersion::V1_3Plus);
    assert_eq!(max_severity(&review), Severity::Critical);
    assert!(codes(&review).contains(&"MULTISEND_WRONG_CHAIN"));
    assert!(codes(&review).contains(&"SAFE_DELEGATECALL"));
    assert!(!codes(&review).contains(&"SAFE_MULTISEND_BATCH"));
    assert!(!review.render().contains("Batch call 1"));
}

#[test]
fn the_same_batch_is_decoded_on_a_chain_that_publishes_it() {
    for chain in [1u64, 10, 137, 42161] {
        let mut tx = batch_tx(MULTISEND_1_3_0, BATCH_TWO_CALLS);
        tx.chain_id = U256::from_u64(chain);
        let review = review_safe_transaction(&tx, DomainVersion::V1_3Plus);
        assert_eq!(max_severity(&review), Severity::Warning, "chain {chain}");
        assert!(
            codes(&review).contains(&"SAFE_MULTISEND_BATCH"),
            "chain {chain}"
        );
    }
}

#[test]
fn lookup_requires_both_the_address_and_the_chain() {
    let good: [u8; 20] = addr(MULTISEND_1_3_0);
    assert!(multisend::lookup(&good, Some(1)).is_ok());
    assert_eq!(
        multisend::lookup(&good, None).unwrap_err(),
        multisend::NotABatch::WrongChain(multisend::lookup(&good, Some(1)).unwrap())
    );
    assert_eq!(
        multisend::lookup(&addr(NOT_MULTISEND), Some(1)).unwrap_err(),
        multisend::NotABatch::UnknownAddress
    );
}

/// Pack one MultiSend element: operation, to, value, data length, data.
fn element(out: &mut Vec<u8>, operation: u8, to: [u8; 20], data: &[u8]) {
    out.push(operation);
    out.extend_from_slice(&to);
    out.extend_from_slice(&[0u8; 32]); // value 0
    let mut len = [0u8; 32];
    len[24..].copy_from_slice(&(data.len() as u64).to_be_bytes());
    out.extend_from_slice(&len);
    out.extend_from_slice(data);
}

/// Wrap packed elements in `multiSend(bytes)` the way the ABI encodes it.
fn multi_send_calldata(packed: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    out.extend_from_slice(&[0x8d, 0x80, 0xff, 0x0a]);
    let mut offset = [0u8; 32];
    offset[31] = 0x20;
    out.extend_from_slice(&offset);
    let mut len = [0u8; 32];
    len[24..].copy_from_slice(&(packed.len() as u64).to_be_bytes());
    out.extend_from_slice(&len);
    out.extend_from_slice(packed);
    while out.len() % 32 != 4 {
        out.push(0);
    }
    out
}

/// `approve(spender, 2^256-1)`.
fn approve_max(spender: [u8; 20]) -> Vec<u8> {
    let mut d = vec![0x09, 0x5e, 0xa7, 0xb3];
    d.extend_from_slice(&[0u8; 12]);
    d.extend_from_slice(&spender);
    d.extend_from_slice(&[0xffu8; 32]);
    d
}

#[test]
fn a_dangerous_call_past_the_display_limit_is_still_named() {
    // A batch long enough that the last call is never displayed. The hidden
    // call grants an unlimited approval — the thing INV-6 says must be
    // CRITICAL. Reporting only "some calls are not shown" is not the same as
    // saying what they do: an operator who acknowledges a long batch must not
    // be signing away every token they hold without being told.
    const USDC: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
    let mut packed = Vec::new();
    for _ in 0..MAX_BATCH_CALLS {
        element(&mut packed, 0, addr(NOT_MULTISEND), &[]);
    }
    element(
        &mut packed,
        0,
        addr(USDC),
        &approve_max(addr(NOT_MULTISEND)),
    );

    let calldata = multi_send_calldata(&packed);
    let tx = batch_tx(MULTISEND_1_3_0, &format!("0x{}", hex::encode(&calldata)));
    let review = review_safe_transaction(&tx, DomainVersion::V1_3Plus);
    let found = codes(&review);

    assert!(
        found.contains(&"MULTISEND_CALLS_NOT_SHOWN"),
        "the batch should say some calls are not shown, got {found:?}"
    );
    assert!(
        found.contains(&"UNLIMITED_APPROVAL"),
        "a hidden call granting an unlimited approval must still be named CRITICAL, got {found:?}"
    );
    assert_eq!(max_severity(&review), Severity::Critical);
}

#[test]
fn every_destination_that_matched_a_selector_is_named() {
    // Deduplicating the qualification on its code alone meant a batch touching
    // several contracts carried one notice naming the first, which reads as if
    // the limitation applied to that address and not to the others.
    const USDC: &str = "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48";
    const OTHER: &str = "0x6B175474E89094C44Da98b954EedeAC495271d0F";
    let mut packed = Vec::new();
    for token in [USDC, OTHER] {
        element(
            &mut packed,
            0,
            addr(token),
            &approve_max(addr(NOT_MULTISEND)),
        );
    }
    let calldata = multi_send_calldata(&packed);
    let tx = batch_tx(MULTISEND_1_3_0, &format!("0x{}", hex::encode(&calldata)));
    let review = review_safe_transaction(&tx, DomainVersion::V1_3Plus);

    let notices: Vec<&str> = review
        .findings()
        .iter()
        .filter(|f| f.code == "SELECTOR_IS_NOT_BEHAVIOUR")
        .map(|f| f.message.as_str())
        .collect();
    assert_eq!(
        notices.len(),
        2,
        "each destination should carry its own notice, got {notices:?}"
    );
    for token in [USDC, OTHER] {
        let bare = token.trim_start_matches("0x");
        assert!(
            notices
                .iter()
                .any(|m| m.to_lowercase().contains(&bare.to_lowercase())),
            "no notice names {token}"
        );
    }
}
