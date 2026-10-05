#![no_main]
//! Calldata as a Safe would execute it, aimed at the decoders added in
//! October 2026: ownership, roles, upgrades, permissions, multicall, timelock
//! and smart-account batches. The first byte picks a known selector so the
//! fuzzer spends its time inside argument parsing rather than finding one.
//!
//! Properties: never panic; deterministic; anything decoded as a call that
//! carries other calls never exceeds the parse limit; BLIND or CRITICAL means
//! DO NOT SIGN.
use clearsign::calls::*;
use clearsign::{DomainVersion, SafeTransaction, Severity, U256};
use libfuzzer_sys::fuzz_target;

const SELECTORS: &[[u8; 4]] = &[
    SEL_TRANSFER_OWNERSHIP, SEL_SET_OWNER, SEL_GRANT_ROLE, SEL_REVOKE_ROLE, SEL_UPGRADE_TO,
    SEL_UPGRADE_TO_AND_CALL, SEL_PROXY_ADMIN_UPGRADE_AND_CALL, SEL_CHANGE_PROXY_ADMIN,
    SEL_INCREASE_ALLOWANCE, SEL_SET_APPROVAL_FOR_ALL, SEL_PERMIT2_APPROVE, SEL_MULTICALL,
    SEL_TIMELOCK_SCHEDULE, SEL_TIMELOCK_EXECUTE, SEL_ACCOUNT_EXECUTE, SEL_APPROVE,
];

fuzz_target!(|input: &[u8]| {
    let Some((&pick, rest)) = input.split_first() else { return };
    let sel = SELECTORS[usize::from(pick) % SELECTORS.len()];
    let mut data = sel.to_vec();
    data.extend_from_slice(rest);
    let tx = SafeTransaction {
        chain_id: U256::from_u64(1),
        safe: [0x11; 20],
        to: [0x22; 20],
        value: U256::ZERO,
        data,
        operation: 0,
        safe_tx_gas: U256::ZERO,
        base_gas: U256::ZERO,
        gas_price: U256::ZERO,
        gas_token: [0; 20],
        refund_receiver: [0; 20],
        nonce: U256::ZERO,
    };
    let review = clearsign::review_safe_transaction(&tx, DomainVersion::V1_3Plus);
    let text = review.render();
    assert_eq!(text, review.render());
    if review.highest_severity() >= Some(Severity::Blind) {
        assert!(text.contains("DO NOT SIGN"));
    }
    let carried = review.sections.iter().filter(|s| s.title.contains(" of ")).count();
    assert!(carried <= 4 * clearsign::multisend::MAX_BATCH_CALLS, "displayed more carried calls than the display limits allow");
});
