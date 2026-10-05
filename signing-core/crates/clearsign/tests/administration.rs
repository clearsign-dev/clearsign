//! Calls that take control of a contract, permissions over tokens beyond
//! approve(), and calls that carry other calls.
//!
//! Added 5 Oct 2026 after the benchmark against real attacks: the closed v1
//! selector set reported Radiant's October 2024 transaction — a Safe calling
//! transferOwnership on the protocol's address provider — as BLIND, which is the
//! same verdict a routine staking call gets. A reviewer that cannot tell those
//! apart has told the signer nothing. Each test here names what it defends.

#![allow(
    clippy::unwrap_used,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects,
    clippy::panic
)]

use clearsign::calls::*;
use clearsign::multisend::MAX_BATCH_CALLS_PARSED;
use clearsign::{DomainVersion, Review, SafeTransaction, Severity, U256};
use clearsign::{hex, review_safe_transaction};

const SAFE: [u8; 20] = [0x11; 20];
const PROTOCOL: [u8; 20] = [0x22; 20];
const ATTACKER: [u8; 20] = [0x66; 20];
const TIMELOCK: [u8; 20] = [0x77; 20];

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
        nonce: U256::from_u64(3),
    }
}

fn review(to: [u8; 20], data: Vec<u8>) -> Review {
    review_safe_transaction(&safe_call(to, data, 0), DomainVersion::V1_3Plus)
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

/// ABI tail for one `bytes` value: length word, data, zero padding.
fn bytes_tail(data: &[u8]) -> Vec<u8> {
    let mut t = word_u64(data.len() as u64);
    t.extend_from_slice(data);
    while t.len() % 32 != 0 {
        t.push(0);
    }
    t
}

/// multicall(bytes[]) encoded the way solc and every library encode it.
fn multicall(elements: &[Vec<u8>]) -> Vec<u8> {
    let mut d = SEL_MULTICALL.to_vec();
    d.extend(word_u64(32));
    d.extend(word_u64(elements.len() as u64));
    let mut offset = 32 * elements.len();
    let mut tails = Vec::new();
    for e in elements {
        d.extend(word_u64(offset as u64));
        let t = bytes_tail(e);
        offset += t.len();
        tails.push(t);
    }
    for t in tails {
        d.extend(t);
    }
    d
}

fn transfer_ownership(to: [u8; 20]) -> Vec<u8> {
    call(SEL_TRANSFER_OWNERSHIP, &[word_addr(to)])
}

fn codes(r: &Review) -> Vec<&'static str> {
    r.findings().iter().map(|f| f.code).collect()
}

fn severity_of(r: &Review, code: &str) -> Severity {
    r.findings()
        .iter()
        .find(|f| f.code == code)
        .unwrap_or_else(|| panic!("{code} not found in {:?}", codes(r)))
        .severity
}

// ------------------------------------------------------------ ownership

#[test]
fn a_disguised_ownership_transfer_is_named_not_blind() {
    // The shape of Radiant Capital, 16 Oct 2024: the Safe calls
    // transferOwnership(attacker) on the contract that controls the protocol.
    let r = review(PROTOCOL, transfer_ownership(ATTACKER));
    assert_eq!(severity_of(&r, "OWNERSHIP_TRANSFER"), Severity::Critical);
    assert!(!r.has("UNKNOWN_SELECTOR"), "must be decoded, not BLIND");
    let text = r.render();
    assert!(
        text.contains("0x6666 6666 6666 6666 6666 6666 6666 6666 6666 6666"),
        "{text}"
    );
    assert!(text.contains("DO NOT SIGN"));
}

#[test]
fn renouncing_ownership_is_critical_and_accepting_is_a_warning() {
    let r = review(PROTOCOL, SEL_RENOUNCE_OWNERSHIP.to_vec());
    assert_eq!(severity_of(&r, "OWNERSHIP_RENOUNCE"), Severity::Critical);
    let r = review(PROTOCOL, SEL_ACCEPT_OWNERSHIP.to_vec());
    assert_eq!(severity_of(&r, "OWNERSHIP_ACCEPT"), Severity::Warning);
    assert_eq!(r.highest_severity(), Some(Severity::Warning));
}

#[test]
fn ownership_calls_with_extra_bytes_are_refused() {
    let mut data = transfer_ownership(ATTACKER);
    data.extend_from_slice(&[0u8; 32]);
    assert!(review(PROTOCOL, data).has("MALFORMED_ARGUMENTS"));
    let mut data = SEL_RENOUNCE_OWNERSHIP.to_vec();
    data.push(0);
    assert!(review(PROTOCOL, data).has("MALFORMED_ARGUMENTS"));
}

// ------------------------------------------------------------ roles

#[test]
fn granting_the_default_admin_role_is_critical_and_named() {
    let data = call(SEL_GRANT_ROLE, &[vec![0u8; 32], word_addr(ATTACKER)]);
    let r = review(PROTOCOL, data);
    assert_eq!(severity_of(&r, "ROLE_GRANT"), Severity::Critical);
    let text = r.render();
    assert!(text.contains("DEFAULT_ADMIN_ROLE"), "{text}");
    assert!(text.contains("can grant itself all of them"), "{text}");
}

#[test]
fn a_known_role_is_recognised_by_its_hash_and_an_unknown_one_is_shown_raw() {
    let minter = clearsign::keccak::keccak256(b"MINTER_ROLE").to_vec();
    let r = review(
        PROTOCOL,
        call(SEL_GRANT_ROLE, &[minter, word_addr(ATTACKER)]),
    );
    assert!(r.render().contains(
        "MINTER_ROLE (0x9f2df0fed2c77648de5860a4cc508cd0818c85b8b8a1ab4ceeef8d981c8956a6)"
    ));

    let r = review(
        PROTOCOL,
        call(SEL_GRANT_ROLE, &[vec![0xab; 32], word_addr(ATTACKER)]),
    );
    assert!(
        r.render()
            .contains("not a role name this reviewer recognises")
    );
}

#[test]
fn revoking_and_renouncing_roles_are_warnings() {
    let r = review(
        PROTOCOL,
        call(SEL_REVOKE_ROLE, &[vec![1; 32], word_addr(ATTACKER)]),
    );
    assert_eq!(severity_of(&r, "ROLE_REVOKE"), Severity::Warning);
    let r = review(
        PROTOCOL,
        call(SEL_RENOUNCE_ROLE, &[vec![1; 32], word_addr(SAFE)]),
    );
    assert_eq!(severity_of(&r, "ROLE_RENOUNCE"), Severity::Warning);
}

#[test]
fn a_role_grant_with_dirty_address_bits_is_refused() {
    let mut account = word_addr(ATTACKER);
    account[0] = 1;
    let r = review(PROTOCOL, call(SEL_GRANT_ROLE, &[vec![0; 32], account]));
    assert!(r.has("MALFORMED_ARGUMENTS"));
    assert!(!r.has("ROLE_GRANT"));
}

// ------------------------------------------------------------ upgrades

#[test]
fn every_upgrade_shape_is_critical() {
    let r = review(PROTOCOL, call(SEL_UPGRADE_TO, &[word_addr(ATTACKER)]));
    assert_eq!(severity_of(&r, "PROXY_UPGRADE"), Severity::Critical);

    let r = review(
        TIMELOCK,
        call(
            SEL_PROXY_ADMIN_UPGRADE,
            &[word_addr(PROTOCOL), word_addr(ATTACKER)],
        ),
    );
    assert_eq!(severity_of(&r, "PROXY_UPGRADE"), Severity::Critical);
    assert!(
        r.render().contains("0x2222 2222"),
        "the proxy, not the admin, is the one upgraded"
    );

    let r = review(PROTOCOL, call(SEL_CHANGE_ADMIN, &[word_addr(ATTACKER)]));
    assert_eq!(severity_of(&r, "PROXY_ADMIN_CHANGE"), Severity::Critical);

    let r = review(
        TIMELOCK,
        call(
            SEL_CHANGE_PROXY_ADMIN,
            &[word_addr(PROTOCOL), word_addr(ATTACKER)],
        ),
    );
    assert_eq!(severity_of(&r, "PROXY_ADMIN_CHANGE"), Severity::Critical);
}

#[test]
fn the_call_an_upgrade_runs_is_blind_like_a_delegatecall() {
    // upgradeToAndCall(impl, data): data runs as the NEW code, so a familiar
    // function name in it describes nothing.
    let init = transfer_ownership(SAFE);
    let mut data = call(
        SEL_UPGRADE_TO_AND_CALL,
        &[word_addr(ATTACKER), word_u64(64)],
    );
    data.extend(bytes_tail(&init));
    let r = review(PROTOCOL, data);
    assert_eq!(severity_of(&r, "PROXY_UPGRADE"), Severity::Critical);
    assert_eq!(severity_of(&r, "UPGRADE_CALL_NOT_DECODED"), Severity::Blind);
    assert!(
        !r.has("OWNERSHIP_TRANSFER"),
        "the init call must not be decoded"
    );

    // Same through ProxyAdmin.upgradeAndCall.
    let mut data = call(
        SEL_PROXY_ADMIN_UPGRADE_AND_CALL,
        &[word_addr(PROTOCOL), word_addr(ATTACKER), word_u64(96)],
    );
    data.extend(bytes_tail(&init));
    let r = review(TIMELOCK, data);
    assert!(r.has("UPGRADE_CALL_NOT_DECODED"));
    assert!(!r.has("OWNERSHIP_TRANSFER"));

    // An empty init call is not a blind spot.
    let mut data = call(
        SEL_UPGRADE_TO_AND_CALL,
        &[word_addr(ATTACKER), word_u64(64)],
    );
    data.extend(bytes_tail(&[]));
    let r = review(PROTOCOL, data);
    assert!(r.has("PROXY_UPGRADE"));
    assert!(!r.has("UPGRADE_CALL_NOT_DECODED"));
}

#[test]
fn an_upgrade_call_at_a_non_standard_offset_is_refused() {
    let mut data = call(
        SEL_UPGRADE_TO_AND_CALL,
        &[word_addr(ATTACKER), word_u64(96)],
    );
    data.extend(vec![0u8; 32]);
    data.extend(bytes_tail(&[1, 2, 3, 4]));
    let r = review(PROTOCOL, data);
    assert!(r.has("MALFORMED_ARGUMENTS"));
    assert!(!r.has("PROXY_UPGRADE"));
}

// ------------------------------------------------------------ token permissions

#[test]
fn an_unlimited_increase_allowance_is_as_critical_as_an_unlimited_approve() {
    // Badger DAO, December 2021: the injected script asked for increaseAllowance.
    let r = review(
        PROTOCOL,
        call(
            SEL_INCREASE_ALLOWANCE,
            &[word_addr(ATTACKER), vec![0xff; 32]],
        ),
    );
    assert_eq!(severity_of(&r, "UNLIMITED_APPROVAL"), Severity::Critical);

    let r = review(
        PROTOCOL,
        call(SEL_INCREASE_ALLOWANCE, &[word_addr(ATTACKER), word_u64(5)]),
    );
    assert_eq!(severity_of(&r, "TOKEN_APPROVAL"), Severity::Warning);
    assert!(r.render().contains("MORE of the token"));

    let r = review(
        PROTOCOL,
        call(SEL_DECREASE_ALLOWANCE, &[word_addr(ATTACKER), word_u64(5)]),
    );
    assert_eq!(r.highest_severity(), Some(Severity::Info));
}

#[test]
fn approval_for_all_is_critical_and_its_revocation_is_not() {
    let r = review(
        PROTOCOL,
        call(
            SEL_SET_APPROVAL_FOR_ALL,
            &[word_addr(ATTACKER), word_u64(1)],
        ),
    );
    assert_eq!(severity_of(&r, "APPROVAL_FOR_ALL"), Severity::Critical);
    let r = review(
        PROTOCOL,
        call(
            SEL_SET_APPROVAL_FOR_ALL,
            &[word_addr(ATTACKER), word_u64(0)],
        ),
    );
    assert!(!r.has("APPROVAL_FOR_ALL"));
    assert_eq!(r.highest_severity(), Some(Severity::Info));
}

#[test]
fn a_bool_that_is_not_zero_or_one_is_refused() {
    let r = review(
        PROTOCOL,
        call(
            SEL_SET_APPROVAL_FOR_ALL,
            &[word_addr(ATTACKER), word_u64(2)],
        ),
    );
    assert!(r.has("MALFORMED_ARGUMENTS"));
    assert!(!r.has("APPROVAL_FOR_ALL"));
}

fn permit2_approve(token: [u8; 20], spender: [u8; 20], amount: Vec<u8>, expiry: u64) -> Vec<u8> {
    call(
        SEL_PERMIT2_APPROVE,
        &[
            word_addr(token),
            word_addr(spender),
            amount,
            word_u64(expiry),
        ],
    )
}

fn uint160_max() -> Vec<u8> {
    let mut w = vec![0u8; 12];
    w.extend_from_slice(&[0xff; 20]);
    w
}

#[test]
fn permit2_unlimited_allowance_is_critical_with_its_expiry_as_a_date() {
    let r = review(
        PERMIT2,
        permit2_approve(PROTOCOL, ATTACKER, uint160_max(), 1_798_761_600),
    );
    assert_eq!(severity_of(&r, "UNLIMITED_APPROVAL"), Severity::Critical);
    assert!(!r.has("NOT_PERMIT2_ADDRESS"));
    let text = r.render();
    assert!(text.contains("1798761600 (2027-01-01 00:00 UTC)"), "{text}");
    assert!(
        text.contains("UNLIMITED (2^160 - 1, Permit2's maximum)"),
        "{text}"
    );
}

#[test]
fn permit2_dates_are_right_at_the_edges() {
    // Permit2 replaces an expiration of 0 with the block's own time, so 0 is
    // not 1970 and not "already expired": the allowance works in that block.
    assert_eq!(unix_time(&U256::ZERO), "0 (1970-01-01 00:00 UTC)");
    for (secs, expected) in [
        (
            0u64,
            "0 — Permit2 replaces this with the current block's time: usable in that block",
        ),
        (951_782_400, "951782400 (2000-02-29 00:00 UTC)"),
        (4_102_444_800, "4102444800 (2100-01-01 00:00 UTC)"),
        (1_709_164_799, "1709164799 (2024-02-28 23:59 UTC)"),
        // The last moment a four-digit year can show.
        (253_402_300_799, "253402300799 (9999-12-31 23:59 UTC)"),
        // One second later, and uint48's maximum, a common "never expires":
        // past 9999, so no date at all.
        (253_402_300_800, "253402300800"),
        (281_474_976_710_655, "281474976710655"),
    ] {
        let r = review(
            PERMIT2,
            permit2_approve(PROTOCOL, ATTACKER, word_u64(1), secs),
        );
        let text = r.render();
        assert!(
            text.contains("Expires .") && text.contains(expected),
            "{secs}: {text}"
        );
        if !expected.contains('(') {
            assert!(!text.contains(&format!("{secs} (")), "{secs}: {text}");
        }
    }
}

#[test]
fn a_permit2_allowance_just_below_the_maximum_is_unlimited_too() {
    // One below uint160's maximum spends exactly like it. ERC-20 approve was
    // given a threshold for this; Permit2 must not reopen the gap.
    let mut below = uint160_max();
    *below.last_mut().unwrap() = 0xfe;
    let r = review(
        PERMIT2,
        permit2_approve(PROTOCOL, ATTACKER, below, 1_798_761_600),
    );
    assert_eq!(severity_of(&r, "UNLIMITED_APPROVAL"), Severity::Critical);
    assert!(
        r.render().contains("EFFECTIVELY UNLIMITED"),
        "{}",
        r.render()
    );

    // 2^144 is the threshold; one below it is still an amount.
    let mut at = vec![0u8; 32];
    at[13] = 1;
    let r = review(PERMIT2, permit2_approve(PROTOCOL, ATTACKER, at, 1));
    assert_eq!(severity_of(&r, "UNLIMITED_APPROVAL"), Severity::Critical);
    let mut under = vec![0u8; 32];
    for b in &mut under[14..] {
        *b = 0xff;
    }
    let r = review(PERMIT2, permit2_approve(PROTOCOL, ATTACKER, under, 1));
    assert!(!r.has("UNLIMITED_APPROVAL"), "{}", r.render());
    assert_eq!(severity_of(&r, "TOKEN_APPROVAL"), Severity::Warning);
}

#[test]
fn permit2_shape_on_another_address_is_called_out() {
    let r = review(
        PROTOCOL,
        permit2_approve(PROTOCOL, ATTACKER, word_u64(1), 1),
    );
    assert_eq!(severity_of(&r, "NOT_PERMIT2_ADDRESS"), Severity::Warning);
}

#[test]
fn permit2_values_wider_than_their_type_are_refused() {
    let r = review(
        PERMIT2,
        permit2_approve(PROTOCOL, ATTACKER, vec![0xff; 32], 1),
    );
    assert!(r.has("MALFORMED_ARGUMENTS"), "amount wider than uint160");
    let mut expiry = vec![0u8; 32];
    expiry[25] = 1; // bit 48 set: wider than uint48
    let data = call(
        SEL_PERMIT2_APPROVE,
        &[
            word_addr(PROTOCOL),
            word_addr(ATTACKER),
            word_u64(1),
            expiry,
        ],
    );
    assert!(
        review(PERMIT2, data).has("MALFORMED_ARGUMENTS"),
        "expiry wider than uint48"
    );
}

// ------------------------------------------------------------ carried calls

#[test]
fn an_ownership_transfer_inside_a_multicall_is_still_found() {
    let inner = vec![
        call(SEL_ACCEPT_OWNERSHIP, &[]),
        transfer_ownership(ATTACKER),
    ];
    let r = review(PROTOCOL, multicall(&inner));
    assert_eq!(severity_of(&r, "OWNERSHIP_TRANSFER"), Severity::Critical);
    assert!(r.has("OWNERSHIP_ACCEPT"));
    let text = r.render();
    assert!(text.contains("Call bundled by multicall 2 of 2"), "{text}");
}

#[test]
fn a_dangerous_call_past_the_display_limit_of_a_multicall_is_named() {
    let mut inner: Vec<Vec<u8>> = (0..40).map(|_| call(SEL_ACCEPT_OWNERSHIP, &[])).collect();
    inner[36] = call(SEL_GRANT_ROLE, &[vec![0; 32], word_addr(ATTACKER)]);
    let r = review(PROTOCOL, multicall(&inner));
    let grant = r
        .findings()
        .iter()
        .find(|f| f.code == "ROLE_GRANT")
        .unwrap();
    assert_eq!(grant.severity, Severity::Critical);
    assert!(grant.message.contains("37"), "{}", grant.message);
    assert!(r.has("CARRIED_CALLS_NOT_SHOWN"));
}

#[test]
fn a_multicall_too_long_to_review_is_not_decoded_and_is_critical() {
    let inner: Vec<Vec<u8>> = (0..1025).map(|_| call(SEL_ACCEPT_OWNERSHIP, &[])).collect();
    let r = review(PROTOCOL, multicall(&inner));
    assert!(r.has("TOO_MANY_CARRIED_CALLS"));
    assert!(!r.has("OWNERSHIP_ACCEPT"));
    // Padding a list past what is read must not earn a milder verdict than a
    // malformed list gets: what was not read could be anything.
    assert_eq!(severity_of(&r, "UNDECODED_CALLS"), Severity::Critical);
    assert!(r.render().contains("DO NOT SIGN"));
}

#[test]
fn a_multicall_with_its_elements_out_of_order_is_refused() {
    let mut data = multicall(&[transfer_ownership(ATTACKER), transfer_ownership(SAFE)]);
    // Swap the two element offsets: same bytes, non-canonical layout.
    let (a, b) = (4 + 64, 4 + 96);
    let first: Vec<u8> = data[a..a + 32].to_vec();
    let second: Vec<u8> = data[b..b + 32].to_vec();
    data[a..a + 32].copy_from_slice(&second);
    data[b..b + 32].copy_from_slice(&first);
    let r = review(PROTOCOL, data);
    assert!(r.has("MALFORMED_ARGUMENTS"));
    assert!(!r.has("OWNERSHIP_TRANSFER"));
}

#[test]
fn a_multicall_declaring_a_huge_length_is_refused_before_reading() {
    let mut data = SEL_MULTICALL.to_vec();
    data.extend(word_u64(32));
    data.extend(vec![0xff; 32]);
    let r = review(PROTOCOL, data);
    assert!(r.has("MALFORMED_ARGUMENTS") || r.has("TOO_MANY_CARRIED_CALLS"));
}

fn schedule(target: [u8; 20], payload: &[u8], delay: u64) -> Vec<u8> {
    let mut d = call(
        SEL_TIMELOCK_SCHEDULE,
        &[
            word_addr(target),
            word_u64(0),
            word_u64(192),
            vec![0; 32],
            vec![7; 32],
            word_u64(delay),
        ],
    );
    d.extend(bytes_tail(payload));
    d
}

#[test]
fn a_call_scheduled_on_a_timelock_is_reviewed_as_though_it_ran_now() {
    let r = review(
        TIMELOCK,
        schedule(
            PROTOCOL,
            &call(SEL_UPGRADE_TO, &[word_addr(ATTACKER)]),
            172_800,
        ),
    );
    assert_eq!(severity_of(&r, "TIMELOCK_SCHEDULE"), Severity::Warning);
    assert_eq!(severity_of(&r, "PROXY_UPGRADE"), Severity::Critical);
    let text = r.render();
    assert!(
        text.contains("Delay (seconds) .") && text.contains("172_800"),
        "{text}"
    );
    assert!(
        text.contains("Call the timelock will be able to run"),
        "{text}"
    );
}

#[test]
fn a_call_a_timelock_executes_is_reviewed() {
    let payload = transfer_ownership(ATTACKER);
    let mut d = call(
        SEL_TIMELOCK_EXECUTE,
        &[
            word_addr(PROTOCOL),
            word_u64(0),
            word_u64(160),
            vec![0; 32],
            vec![7; 32],
        ],
    );
    d.extend(bytes_tail(&payload));
    let r = review(TIMELOCK, d);
    assert_eq!(severity_of(&r, "OWNERSHIP_TRANSFER"), Severity::Critical);
    assert!(!r.has("TIMELOCK_SCHEDULE"));
}

#[test]
fn the_timelock_is_the_caller_of_what_it_runs_not_the_safe() {
    // A Safe administration function scheduled on a timelock is called by the
    // timelock, so it is not this Safe changing itself.
    let inner = call(SEL_CHANGE_THRESHOLD, &[word_u64(1)]);
    let r = review(TIMELOCK, schedule(SAFE, &inner, 1));
    assert!(r.has("SAFE_ADMIN_SELECTOR_ON_OTHER_CONTRACT"));
    assert!(!r.has("SAFE_THRESHOLD_CHANGE"));
}

#[test]
fn carried_calls_nest_exactly_to_the_limit() {
    // The Safe's call is depth 1 and each multicall opens one more level, so
    // Safe -> multicall -> multicall -> transferOwnership is still read.
    let mut data = transfer_ownership(ATTACKER);
    for _ in 0..(MAX_NESTING - 1) {
        data = multicall(&[data]);
    }
    let r = review(PROTOCOL, data.clone());
    assert!(!r.has("NESTING_LIMIT"), "{}", r.render());
    assert_eq!(severity_of(&r, "OWNERSHIP_TRANSFER"), Severity::Critical);
    let r = review(PROTOCOL, multicall(&[data]));
    assert!(r.has("NESTING_LIMIT"));
    assert!(!r.has("OWNERSHIP_TRANSFER"));
}

#[test]
fn nesting_past_the_limit_is_blind() {
    // Safe -> multicall -> multicall -> multicall -> transferOwnership.
    let mut data = transfer_ownership(ATTACKER);
    for _ in 0..3 {
        data = multicall(&[data]);
    }
    let r = review(PROTOCOL, data);
    assert!(r.has("NESTING_LIMIT"));
    assert_eq!(severity_of(&r, "UNDECODED_CALLS"), Severity::Critical);
    assert!(r.render().contains("DO NOT SIGN"));
}

#[test]
fn transfer_from_no_longer_claims_to_be_erc20() {
    let data = call(
        SEL_TRANSFER_FROM,
        &[word_addr(SAFE), word_addr(ATTACKER), word_u64(7)],
    );
    let text = review(PROTOCOL, data).render();
    assert!(text.contains("Matches transferFrom(address,address,uint256)"));
    assert!(text.contains("Amount, or token ID for an NFT"));
    assert!(!text.contains("ERC-20 transferFrom"));
}

#[test]
fn on_a_legacy_safe_the_transaction_finding_is_numbered_before_the_safe_wide_one() {
    // Bybit's Safe was v1.1.x. The DELEGATECALL must be finding 1, ahead of
    // the chain-binding notice every v1.1.x transaction carries.
    let tx = safe_call(ATTACKER, hex::decode("a9059cbb").unwrap(), 1);
    let r = review_safe_transaction(&tx, DomainVersion::Legacy);
    let required = r.required_acknowledgements();
    assert_eq!(required[0], (1, "SAFE_DELEGATECALL"), "{required:?}");
    assert!(
        required.contains(&(2, "SIGNATURE_NOT_CHAIN_BOUND")),
        "{required:?}"
    );
    assert!(r.render().contains("About this Safe, not this transaction"));
}

// ------------------------------------------------------------ added after the landscape study

// Found by mutation testing: arms that share code chose their labels by
// selector, and no test read the label. Swapped, the finding stays right while
// the line a signer reads first names the wrong function.
#[test]
fn each_call_is_labelled_with_the_function_it_matches() {
    let text = review(PROTOCOL, transfer_ownership(ATTACKER)).render();
    assert!(
        text.contains("Matches Ownable transferOwnership(address)"),
        "{text}"
    );
    let text = review(PROTOCOL, call(SEL_SET_OWNER, &[word_addr(ATTACKER)])).render();
    assert!(text.contains("Matches setOwner(address) (DSAuth"), "{text}");
    for (selector, label) in [
        (
            SEL_DECREASE_ALLOWANCE,
            "Matches decreaseAllowance(address,uint256)",
        ),
        (
            SEL_DECREASE_APPROVAL,
            "Matches decreaseApproval(address,uint256)",
        ),
    ] {
        let text = review(
            PROTOCOL,
            call(selector, &[word_addr(ATTACKER), word_u64(1)]),
        )
        .render();
        assert!(text.contains(label), "{text}");
    }
    for (selector, signature, who) in [
        (
            SEL_GRANT_ROLE,
            "grantRole(bytes32,address)",
            "Account receiving it",
        ),
        (
            SEL_REVOKE_ROLE,
            "revokeRole(bytes32,address)",
            "Account losing it",
        ),
        (
            SEL_RENOUNCE_ROLE,
            "renounceRole(bytes32,address)",
            "Account giving it up",
        ),
    ] {
        let text = review(
            PROTOCOL,
            call(selector, &[vec![0u8; 32], word_addr(ATTACKER)]),
        )
        .render();
        assert!(
            text.contains(&format!("Matches AccessControl {signature}")) && text.contains(who),
            "{text}"
        );
    }
}

// Found by mutation testing: the finding names the new owner, or says there
// will be none, and no test read which.
#[test]
fn an_ownership_transfer_names_the_new_owner_or_says_there_will_be_none() {
    let message = |new_owner: [u8; 20]| {
        let r = review(PROTOCOL, transfer_ownership(new_owner));
        let f = r
            .findings()
            .iter()
            .find(|f| f.code == "OWNERSHIP_TRANSFER")
            .unwrap();
        f.message.clone()
    };
    let to_attacker = message(ATTACKER);
    assert!(
        to_attacker.contains(&clearsign::address::checksummed(&ATTACKER)),
        "{to_attacker}"
    );
    assert!(!to_attacker.contains("no owner"), "{to_attacker}");
    let to_nobody = message([0; 20]);
    assert!(
        to_nobody.contains("the zero address, which leaves it with no owner"),
        "{to_nobody}"
    );
}

#[test]
fn a_dsproxy_owner_change_is_an_ownership_transfer() {
    // Scam Sniffer, 2024: one setOwner signature on a Maker DSProxy cost about
    // $55 million, 31.9% of that year's large wallet-drainer losses.
    let r = review(PROTOCOL, call(SEL_SET_OWNER, &[word_addr(ATTACKER)]));
    assert_eq!(severity_of(&r, "OWNERSHIP_TRANSFER"), Severity::Critical);
    assert!(r.render().contains("setOwner(address)"));
}

#[test]
fn increase_approval_is_read_like_increase_allowance() {
    let r = review(
        PROTOCOL,
        call(
            SEL_INCREASE_APPROVAL,
            &[word_addr(ATTACKER), vec![0xff; 32]],
        ),
    );
    assert_eq!(severity_of(&r, "UNLIMITED_APPROVAL"), Severity::Critical);
    assert!(r.render().contains("increaseApproval(address,uint256)"));
    let r = review(
        PROTOCOL,
        call(SEL_DECREASE_APPROVAL, &[word_addr(ATTACKER), word_u64(1)]),
    );
    assert_eq!(r.highest_severity(), Some(Severity::Info));
}

/// ERC-7579 / ERC-7821 batch: abi.encode((address,uint256,bytes)[]).
fn encode_calls(calls: &[([u8; 20], u64, Vec<u8>)]) -> Vec<u8> {
    let mut d = word_u64(32);
    d.extend(word_u64(calls.len() as u64));
    let mut offset = 32 * calls.len();
    let mut tails = Vec::new();
    for (to, value, data) in calls {
        d.extend(word_u64(offset as u64));
        let mut t = word_addr(*to);
        t.extend(word_u64(*value));
        t.extend(word_u64(96));
        t.extend(bytes_tail(data));
        offset += t.len();
        tails.push(t);
    }
    for t in tails {
        d.extend(t);
    }
    d
}

fn account_execute(mode: [u8; 32], exec_data: &[u8]) -> Vec<u8> {
    let mut d = SEL_ACCOUNT_EXECUTE.to_vec();
    d.extend_from_slice(&mode);
    d.extend(word_u64(64));
    d.extend(bytes_tail(exec_data));
    d
}

fn mode(call_type: u8, exec_type: u8, selector: [u8; 4]) -> [u8; 32] {
    let mut m = [0u8; 32];
    m[0] = call_type;
    m[1] = exec_type;
    m[6..10].copy_from_slice(&selector);
    m
}

#[test]
fn an_eip7702_batch_is_opened_and_every_call_judged() {
    // The shape of the 2025 EIP-7702 drains: the victim's own account runs a
    // batch whose calls are unlimited approvals to the drainer.
    let batch = encode_calls(&[
        (
            PROTOCOL,
            0,
            call(SEL_APPROVE, &[word_addr(ATTACKER), vec![0xff; 32]]),
        ),
        (
            TIMELOCK,
            0,
            call(
                SEL_SET_APPROVAL_FOR_ALL,
                &[word_addr(ATTACKER), word_u64(1)],
            ),
        ),
        (ATTACKER, 5, vec![]),
    ]);
    let r = review(SAFE, account_execute(mode(1, 0, [0; 4]), &batch));
    assert!(r.has("SMART_ACCOUNT_BATCH"));
    assert_eq!(severity_of(&r, "UNLIMITED_APPROVAL"), Severity::Critical);
    assert_eq!(severity_of(&r, "APPROVAL_FOR_ALL"), Severity::Critical);
    let text = r.render();
    assert!(text.contains("Call made by the account 3 of 3"), "{text}");
    assert!(text.contains("the whole batch is undone"));
}

#[test]
fn try_mode_single_calls_and_erc7821_auth_data_are_read() {
    let batch = encode_calls(&[(PROTOCOL, 0, transfer_ownership(ATTACKER))]);
    let r = review(SAFE, account_execute(mode(1, 1, [0; 4]), &batch));
    assert!(r.has("OWNERSHIP_TRANSFER"));
    assert!(r.render().contains("it is skipped and the rest still run"));

    let mut single = PROTOCOL.to_vec();
    single.extend(word_u64(0));
    single.extend(transfer_ownership(ATTACKER));
    let r = review(SAFE, account_execute(mode(0, 0, [0; 4]), &single));
    assert!(r.has("OWNERSHIP_TRANSFER"));

    // ERC-7821 with opData: abi.encode(calls, bytes).
    let calls_part = encode_calls(&[(PROTOCOL, 0, transfer_ownership(ATTACKER))]);
    let mut with_op = word_u64(64);
    // The array starts at 0x40 and opData follows it.
    let array_body = &calls_part[32..];
    with_op.extend(word_u64((64 + array_body.len()) as u64));
    with_op.extend_from_slice(array_body);
    with_op.extend(bytes_tail(&[9u8; 65]));
    let r = review(
        SAFE,
        account_execute(mode(1, 0, [0x78, 0x21, 0x00, 0x01]), &with_op),
    );
    assert!(r.has("OWNERSHIP_TRANSFER"), "{}", r.render());
    assert!(r.render().contains("65 bytes, checked by the account"));
}

#[test]
fn an_account_batch_at_the_limit_is_judged_call_by_call_and_one_more_is_not_read() {
    let approve = call(SEL_APPROVE, &[word_addr(ATTACKER), vec![0xff; 32]]);
    let calls: Vec<_> = (0..MAX_BATCH_CALLS_PARSED)
        .map(|_| (PROTOCOL, 0, approve.clone()))
        .collect();
    let r = review(
        SAFE,
        account_execute(mode(1, 0, [0; 4]), &encode_calls(&calls)),
    );
    assert!(!r.has("TOO_MANY_CARRIED_CALLS"), "{}", r.render());
    assert_eq!(severity_of(&r, "UNLIMITED_APPROVAL"), Severity::Critical);

    let mut over = calls;
    over.push((PROTOCOL, 0, approve));
    let r = review(
        SAFE,
        account_execute(mode(1, 0, [0; 4]), &encode_calls(&over)),
    );
    assert!(r.has("TOO_MANY_CARRIED_CALLS"));
    assert!(!r.has("UNLIMITED_APPROVAL"));
    assert_eq!(severity_of(&r, "UNDECODED_CALLS"), Severity::Critical);
}

#[test]
fn a_multicall_at_the_limit_is_decoded() {
    let inner: Vec<Vec<u8>> = (0..MAX_BATCH_CALLS_PARSED)
        .map(|_| transfer_ownership(ATTACKER))
        .collect();
    let r = review(PROTOCOL, multicall(&inner));
    assert!(!r.has("TOO_MANY_CARRIED_CALLS"), "{}", r.render());
    assert_eq!(severity_of(&r, "OWNERSHIP_TRANSFER"), Severity::Critical);
}

#[test]
fn an_account_delegatecall_is_critical_and_not_decoded() {
    let mut packed = ATTACKER.to_vec();
    packed.extend(call(SEL_TRANSFER, &[word_addr(SAFE), word_u64(0)]));
    let r = review(SAFE, account_execute(mode(0xff, 0, [0; 4]), &packed));
    assert_eq!(severity_of(&r, "ACCOUNT_DELEGATECALL"), Severity::Critical);
    assert!(!r.render().contains("Matches ERC-20 transfer"));
}

#[test]
fn modes_the_standards_do_not_define_are_blind() {
    for m in [
        mode(0xfe, 0, [0; 4]),
        mode(1, 2, [0; 4]),
        mode(1, 0, [1, 2, 3, 4]),
    ] {
        let r = review(SAFE, account_execute(m, &encode_calls(&[])));
        assert_eq!(severity_of(&r, "UNKNOWN_EXECUTION_MODE"), Severity::Blind);
    }
}

#[test]
fn a_batch_tuple_with_its_call_data_out_of_place_is_refused() {
    let mut batch = encode_calls(&[(PROTOCOL, 0, transfer_ownership(ATTACKER))]);
    // The tuple's bytes offset word sits at 32 (array offset) + 32 (length)
    // + 32 (element offset) + 64 (address, value) into the execution data.
    let pos = 32 + 32 + 32 + 64;
    batch[pos + 31] = 0x80;
    let r = review(SAFE, account_execute(mode(1, 0, [0; 4]), &batch));
    assert!(r.has("MALFORMED_ARGUMENTS"));
    assert!(!r.has("OWNERSHIP_TRANSFER"));
}
