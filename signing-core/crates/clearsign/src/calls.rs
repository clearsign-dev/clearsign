//! Decoding a single contract call, and the risk rules that apply to it.
//!
//! The rules here are the product. Each finding code is stable and documented,
//! because auditors and other wallets may key automation off them.

use alloc::collections::BTreeMap;
use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

use crate::abi::Args;
use crate::address::{self, Address};
use crate::hex;
use crate::multisend::{self, BatchCall};
use crate::review::{Review, Section, Severity};
use crate::{Error, U256};

/// How the call is executed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operation {
    Call,
    DelegateCall,
    /// A Safe operation value that is neither 0 nor 1.
    Invalid(u8),
}

impl Operation {
    pub fn from_u8(v: u8) -> Operation {
        match v {
            0 => Operation::Call,
            1 => Operation::DelegateCall,
            other => Operation::Invalid(other),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct CallContext {
    /// The Safe whose authority executes this call, if any.
    pub safe: Option<Address>,
    pub operation: Operation,
    /// The chain these bytes commit to, when they commit to one. `None` for a
    /// pre-EIP-155 legacy transaction, where no chain can be confirmed.
    pub chain_id: Option<u64>,
    /// Nesting level, bounded by [`MAX_NESTING`].
    pub depth: usize,
}

/// Maximum depth of Safe transactions nested inside Safe transactions.
pub const MAX_NESTING: usize = 3;

// Function selectors. Each is verified against keccak256(signature) in tests.
pub const SEL_TRANSFER: [u8; 4] = [0xa9, 0x05, 0x9c, 0xbb];
pub const SEL_APPROVE: [u8; 4] = [0x09, 0x5e, 0xa7, 0xb3];
pub const SEL_TRANSFER_FROM: [u8; 4] = [0x23, 0xb8, 0x72, 0xdd];
pub const SEL_EXEC_TRANSACTION: [u8; 4] = [0x6a, 0x76, 0x12, 0x02];
pub const SEL_ADD_OWNER_WITH_THRESHOLD: [u8; 4] = [0x0d, 0x58, 0x2f, 0x13];
pub const SEL_REMOVE_OWNER: [u8; 4] = [0xf8, 0xdc, 0x5d, 0xd9];
pub const SEL_SWAP_OWNER: [u8; 4] = [0xe3, 0x18, 0xb5, 0x2b];
pub const SEL_CHANGE_THRESHOLD: [u8; 4] = [0x69, 0x4e, 0x80, 0xc3];
pub const SEL_ENABLE_MODULE: [u8; 4] = [0x61, 0x0b, 0x59, 0x25];
pub const SEL_DISABLE_MODULE: [u8; 4] = [0xe0, 0x09, 0xcf, 0xde];
pub const SEL_SET_GUARD: [u8; 4] = [0xe1, 0x9a, 0x9d, 0xd9];
pub const SEL_SET_FALLBACK_HANDLER: [u8; 4] = [0xf0, 0x8a, 0x03, 0x23];
pub const SEL_CHANGE_MASTER_COPY: [u8; 4] = [0x7d, 0xe7, 0xed, 0xef];

// Permissions over tokens, beyond approve(). Phishing pages ask for these
// because a wallet that renders approve() carefully often renders them as
// nothing at all.
pub const SEL_INCREASE_ALLOWANCE: [u8; 4] = [0x39, 0x50, 0x93, 0x51];
pub const SEL_DECREASE_ALLOWANCE: [u8; 4] = [0xa4, 0x57, 0xc2, 0xd7];
pub const SEL_SET_APPROVAL_FOR_ALL: [u8; 4] = [0xa2, 0x2c, 0xb4, 0x65];
pub const SEL_PERMIT2_APPROVE: [u8; 4] = [0x87, 0x51, 0x7c, 0x45];

// Control of contracts other than the Safe: who owns them, who holds their
// roles, and what code sits behind their proxies. A multisig that administers a
// protocol signs these, and a disguised one is how a protocol is taken.
pub const SEL_TRANSFER_OWNERSHIP: [u8; 4] = [0xf2, 0xfd, 0xe3, 0x8b];
pub const SEL_RENOUNCE_OWNERSHIP: [u8; 4] = [0x71, 0x50, 0x18, 0xa6];
pub const SEL_ACCEPT_OWNERSHIP: [u8; 4] = [0x79, 0xba, 0x50, 0x97];
pub const SEL_GRANT_ROLE: [u8; 4] = [0x2f, 0x2f, 0xf1, 0x5d];
pub const SEL_REVOKE_ROLE: [u8; 4] = [0xd5, 0x47, 0x74, 0x1f];
pub const SEL_RENOUNCE_ROLE: [u8; 4] = [0x36, 0x56, 0x8a, 0xbe];
pub const SEL_UPGRADE_TO: [u8; 4] = [0x36, 0x59, 0xcf, 0xe6];
pub const SEL_UPGRADE_TO_AND_CALL: [u8; 4] = [0x4f, 0x1e, 0xf2, 0x86];
pub const SEL_CHANGE_ADMIN: [u8; 4] = [0x8f, 0x28, 0x39, 0x70];
pub const SEL_PROXY_ADMIN_UPGRADE: [u8; 4] = [0x99, 0xa8, 0x8e, 0xc4];
pub const SEL_PROXY_ADMIN_UPGRADE_AND_CALL: [u8; 4] = [0x96, 0x23, 0x60, 0x9d];
pub const SEL_CHANGE_PROXY_ADMIN: [u8; 4] = [0x7e, 0xff, 0x27, 0x5e];
/// DSAuth's `setOwner`, which Maker's DSProxy and many other contracts use. In
/// 2024 one signature of exactly this shape cost its signer about $55 million
/// in DAI: a third of all large wallet-drainer losses that year.
pub const SEL_SET_OWNER: [u8; 4] = [0x13, 0xaf, 0x40, 0x35];
/// The older OpenZeppelin names for increaseAllowance and decreaseAllowance.
pub const SEL_INCREASE_APPROVAL: [u8; 4] = [0xd7, 0x3d, 0xd6, 0x23];
pub const SEL_DECREASE_APPROVAL: [u8; 4] = [0x66, 0x18, 0x84, 0x63];

// Calls that carry other calls. Each carried call is reviewed on its own terms:
// wrapping a dangerous call in another must not make it disappear.
pub const SEL_MULTICALL: [u8; 4] = [0xac, 0x96, 0x50, 0xd8];
pub const SEL_TIMELOCK_SCHEDULE: [u8; 4] = [0x01, 0xd5, 0x06, 0x2a];
pub const SEL_TIMELOCK_EXECUTE: [u8; 4] = [0x13, 0x40, 0x08, 0xd3];
/// ERC-7579 and ERC-7821 `execute(bytes32 mode, bytes executionData)`: how a
/// smart account, including an EOA delegated under EIP-7702, runs a batch of
/// calls as itself. The 2025 EIP-7702 drains were transactions of this shape
/// sent by victims to their own accounts, with the approvals inside the batch.
pub const SEL_ACCOUNT_EXECUTE: [u8; 4] = [0xe9, 0xae, 0x5c, 0x53];

/// The signatures the selectors above must hash from.
pub const KNOWN_SIGNATURES: &[([u8; 4], &str)] = &[
    (SEL_TRANSFER, "transfer(address,uint256)"),
    (SEL_APPROVE, "approve(address,uint256)"),
    (SEL_TRANSFER_FROM, "transferFrom(address,address,uint256)"),
    (
        SEL_EXEC_TRANSACTION,
        "execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)",
    ),
    (
        SEL_ADD_OWNER_WITH_THRESHOLD,
        "addOwnerWithThreshold(address,uint256)",
    ),
    (SEL_REMOVE_OWNER, "removeOwner(address,address,uint256)"),
    (SEL_SWAP_OWNER, "swapOwner(address,address,address)"),
    (SEL_CHANGE_THRESHOLD, "changeThreshold(uint256)"),
    (SEL_ENABLE_MODULE, "enableModule(address)"),
    (SEL_DISABLE_MODULE, "disableModule(address,address)"),
    (SEL_SET_GUARD, "setGuard(address)"),
    (SEL_SET_FALLBACK_HANDLER, "setFallbackHandler(address)"),
    (SEL_CHANGE_MASTER_COPY, "changeMasterCopy(address)"),
    (crate::multisend::SEL_MULTI_SEND, "multiSend(bytes)"),
    (SEL_INCREASE_ALLOWANCE, "increaseAllowance(address,uint256)"),
    (SEL_DECREASE_ALLOWANCE, "decreaseAllowance(address,uint256)"),
    (SEL_SET_APPROVAL_FOR_ALL, "setApprovalForAll(address,bool)"),
    (
        SEL_PERMIT2_APPROVE,
        "approve(address,address,uint160,uint48)",
    ),
    (SEL_TRANSFER_OWNERSHIP, "transferOwnership(address)"),
    (SEL_RENOUNCE_OWNERSHIP, "renounceOwnership()"),
    (SEL_ACCEPT_OWNERSHIP, "acceptOwnership()"),
    (SEL_GRANT_ROLE, "grantRole(bytes32,address)"),
    (SEL_REVOKE_ROLE, "revokeRole(bytes32,address)"),
    (SEL_RENOUNCE_ROLE, "renounceRole(bytes32,address)"),
    (SEL_UPGRADE_TO, "upgradeTo(address)"),
    (SEL_UPGRADE_TO_AND_CALL, "upgradeToAndCall(address,bytes)"),
    (SEL_CHANGE_ADMIN, "changeAdmin(address)"),
    (SEL_PROXY_ADMIN_UPGRADE, "upgrade(address,address)"),
    (
        SEL_PROXY_ADMIN_UPGRADE_AND_CALL,
        "upgradeAndCall(address,address,bytes)",
    ),
    (SEL_CHANGE_PROXY_ADMIN, "changeProxyAdmin(address,address)"),
    (SEL_MULTICALL, "multicall(bytes[])"),
    (
        SEL_TIMELOCK_SCHEDULE,
        "schedule(address,uint256,bytes,bytes32,bytes32,uint256)",
    ),
    (
        SEL_TIMELOCK_EXECUTE,
        "execute(address,uint256,bytes,bytes32,bytes32)",
    ),
    (SEL_SET_OWNER, "setOwner(address)"),
    (SEL_INCREASE_APPROVAL, "increaseApproval(address,uint256)"),
    (SEL_DECREASE_APPROVAL, "decreaseApproval(address,uint256)"),
    (SEL_ACCOUNT_EXECUTE, "execute(bytes32,bytes)"),
];

/// Permit2's address, the same on every chain it is deployed to (CREATE2).
pub const PERMIT2: Address = [
    0x00, 0x00, 0x00, 0x00, 0x00, 0x22, 0xd4, 0x73, 0x03, 0x0f, 0x11, 0x6d, 0xde, 0xe9, 0xf6, 0xb4,
    0x3a, 0xc7, 0x8b, 0xa3,
];

/// Role names whose hashes are recognised when a role is granted or revoked.
/// A role is a bytes32 in the calldata; naming it is a hash comparison against
/// this fixed list, never a label supplied by whoever prepared the transaction.
pub const KNOWN_ROLE_NAMES: &[&str] = &[
    "MINTER_ROLE",
    "BURNER_ROLE",
    "PAUSER_ROLE",
    "UNPAUSER_ROLE",
    "UPGRADER_ROLE",
    "ADMIN_ROLE",
    "OPERATOR_ROLE",
    "GUARDIAN_ROLE",
    "GOVERNOR_ROLE",
    "GOVERNANCE_ROLE",
    "MANAGER_ROLE",
    "OWNER_ROLE",
    "EXECUTOR_ROLE",
    "PROPOSER_ROLE",
    "CANCELLER_ROLE",
    "TIMELOCK_ADMIN_ROLE",
    "KEEPER_ROLE",
    "BRIDGE_ROLE",
    "ORACLE_ROLE",
    "RELAYER_ROLE",
    "EMERGENCY_ROLE",
    "TREASURY_ROLE",
    "STRATEGIST_ROLE",
    "WITHDRAWER_ROLE",
    "SIGNER_ROLE",
];

/// Decode one call into `review`: pushes a section, then any nested sections.
pub fn review_call(
    review: &mut Review,
    title: &str,
    to: Address,
    value: U256,
    data: &[u8],
    ctx: CallContext,
) {
    let mut s = Section::new(title);

    if ctx.safe.is_some() {
        s.field(
            "Operation",
            String::from(match ctx.operation {
                Operation::Call => "CALL",
                Operation::DelegateCall => "DELEGATECALL",
                Operation::Invalid(_) => "INVALID",
            }),
        );
    }

    match ctx.operation {
        Operation::DelegateCall => {
            // The one delegatecall that can be read from the bytes alone: a batch
            // replayed by a pinned Safe MultiSend deployment. Everything else is
            // refused below.
            if let Some((sel, rest)) = data.split_first_chunk::<4>() {
                if *sel == multisend::SEL_MULTI_SEND {
                    match multisend::lookup(&to, ctx.chain_id) {
                        Ok(dep) => {
                            review_batch(review, s, to, value, dep, rest, ctx);
                            return;
                        }
                        Err(multisend::NotABatch::WrongChain(dep)) => {
                            // Falls through to the CRITICAL delegatecall path below,
                            // with the reason named rather than left as a mystery.
                            review.find(
                                Severity::Critical,
                                "MULTISEND_WRONG_CHAIN",
                                format!(
                                    "{} is the address of {}, but Safe does not publish that deployment \
                                     on this chain, so the code at that address here is unknown. The batch \
                                     is not decoded.",
                                    address::checksummed(&to),
                                    multisend::describe(dep)
                                ),
                            );
                        }
                        Err(multisend::NotABatch::UnknownAddress) => {}
                    }
                }
            }
            // Bybit, February 2025: signers approved a delegatecall whose calldata
            // looked like an ordinary token transfer. Under delegatecall the target's
            // code runs with this Safe's storage, so the function name describes nothing.
            s.field("Code that will run as the Safe", address::display(&to));
            s.field("Native value (wei)", value.to_grouped_decimal());
            s.field("Calldata selector", selector_text(data));
            s.field("Calldata length", format!("{} bytes", data.len()));
            review.sections.push(s);
            review.find(
                Severity::Critical,
                "SAFE_DELEGATECALL",
                format!(
                    "DELEGATECALL runs the code at {} with full control over this Safe's storage, owners, modules and funds. \
                     Any function name the calldata appears to have does NOT describe what that code does, so it is \
                     deliberately not decoded.",
                    address::checksummed(&to)
                ),
            );
            return;
        }
        Operation::Invalid(op) => {
            s.field("Target", address::display(&to));
            review.sections.push(s);
            review.find(
                Severity::Critical,
                "SAFE_INVALID_OPERATION",
                format!("Operation value {op} is neither CALL (0) nor DELEGATECALL (1). Refusing to interpret."),
            );
            return;
        }
        Operation::Call => {}
    }

    s.field("To", address::display(&to));
    s.field("Native value (wei)", value.to_grouped_decimal());

    if data.is_empty() {
        if value.is_zero() {
            s.field("Action", String::from("Empty call: no data and no value"));
            review.sections.push(s);
            review.find(
                Severity::Info,
                "EMPTY_CALL",
                String::from("This call sends nothing and calls no function."),
            );
        } else {
            s.field("Action", String::from("Native value transfer"));
            review.sections.push(s);
        }
        return;
    }

    let (selector, args) = match data.split_first_chunk::<4>() {
        Some((sel, rest)) => (*sel, Args::new(rest)),
        None => {
            s.field("Calldata", hex::encode_prefixed(data));
            review.sections.push(s);
            review.find(
                Severity::Blind,
                "SHORT_CALLDATA",
                String::from("Calldata is shorter than a function selector; the target's fallback logic decides what happens."),
            );
            return;
        }
    };

    if !value.is_zero() {
        review.find(
            Severity::Warning,
            "VALUE_WITH_CALL",
            format!(
                "Sends {} wei of native currency along with a function call.",
                value.to_grouped_decimal()
            ),
        );
    }

    let outcome = decode_known(review, &mut s, to, selector, args, ctx);
    match outcome {
        Ok(Decoded::Done) => review.sections.push(s),
        Ok(Decoded::Unknown) => {
            s.field("Action", String::from("Unknown function"));
            s.field("Selector", hex::encode_prefixed(&selector));
            s.field("Calldata length", format!("{} bytes", data.len()));
            review.sections.push(s);
            review.find(
                Severity::Blind,
                "UNKNOWN_SELECTOR",
                format!(
                    "Function {} on {} is not in the decoder's verified set. What it does cannot be determined from these bytes.",
                    hex::encode_prefixed(&selector),
                    address::checksummed(&to)
                ),
            );
        }
        Ok(Decoded::Nested {
            safe,
            inner_to,
            inner_value,
            inner_data,
            operation,
        }) => {
            review.sections.push(s);
            let depth = ctx.depth.saturating_add(1);
            if depth > MAX_NESTING {
                review.find(
                    Severity::Blind,
                    "NESTING_LIMIT",
                    format!("Safe transactions are nested more than {MAX_NESTING} levels deep; the innermost call is not decoded."),
                );
                undecoded_calls_finding(review);
                return;
            }
            review_call(
                review,
                "Inner call executed by the Safe",
                inner_to,
                inner_value,
                inner_data,
                CallContext {
                    safe: Some(safe),
                    operation,
                    chain_id: ctx.chain_id,
                    depth,
                },
            );
        }
        Ok(Decoded::Carried {
            calls,
            title,
            executor,
        }) => {
            review.sections.push(s);
            let depth = ctx.depth.saturating_add(1);
            if depth > MAX_NESTING {
                review.find(
                    Severity::Blind,
                    "NESTING_LIMIT",
                    format!(
                        "Calls are nested more than {MAX_NESTING} levels deep; the {} carried \
                         here are not decoded.",
                        calls.len()
                    ),
                );
                undecoded_calls_finding(review);
                return;
            }
            let inner = CallContext {
                // Calls a timelock executes are made by the timelock, not by the
                // Safe; calls bundled by multicall are still made by the caller.
                safe: match executor {
                    Executor::SameCaller => ctx.safe,
                    Executor::Contract => None,
                },
                operation: Operation::Call,
                chain_id: ctx.chain_id,
                depth,
            };
            review_inner_calls(review, &calls, title, inner);
        }
        Err(Error::TooDeep) => {
            s.field("Action", String::from("Too many carried calls to review"));
            s.field("Selector", hex::encode_prefixed(&selector));
            review.sections.push(s);
            review.find(
                Severity::Blind,
                "TOO_MANY_CARRIED_CALLS",
                format!(
                    "This call carries more than {} others. They are not decoded, because a list \
                     too long to read is not a list that was reviewed.",
                    multisend::MAX_BATCH_CALLS_PARSED
                ),
            );
            undecoded_calls_finding(review);
        }
        Err(err) => {
            s.field("Action", String::from("Refused to interpret"));
            s.field("Selector", hex::encode_prefixed(&selector));
            review.sections.push(s);
            review.find(
                Severity::Critical,
                "MALFORMED_ARGUMENTS",
                format!(
                    "Selector {} is known, but its arguments are not canonically encoded ({err}). \
                     Non-standard encodings can make different tools disagree about what a call does.",
                    hex::encode_prefixed(&selector)
                ),
            );
        }
    }
}

/// A batch executed by a pinned MultiSend deployment: every inner call is
/// reviewed on its own terms, so ordinary batches stop reading as CRITICAL.
fn review_batch(
    review: &mut Review,
    mut s: Section,
    to: Address,
    value: U256,
    dep: &'static multisend::Deployment,
    args_bytes: &[u8],
    ctx: CallContext,
) {
    s.field("Action", String::from("Batch of calls (Safe MultiSend)"));
    s.field("Batch contract", address::display(&to));
    s.field("Deployment", multisend::describe(dep));

    let calls = Args::new(args_bytes)
        .bytes_at(0, 32)
        .and_then(|(packed, end)| {
            Args::new(args_bytes).expect_end(end)?;
            multisend::decode_batch(packed)
        });
    let calls = match calls {
        Ok(calls) => calls,
        Err(Error::TooDeep) => {
            review.sections.push(s);
            review.find(
                Severity::Blind,
                "MULTISEND_TOO_MANY_CALLS",
                format!(
                    "This batch contains more than {} calls. It is not decoded, because a batch \
                     too long to read is not a batch that was reviewed.",
                    multisend::MAX_BATCH_CALLS_PARSED
                ),
            );
            review.find(
                Severity::Critical,
                "UNDECODED_DELEGATECALL_POSSIBLE",
                String::from(
                    "Part of this transaction was not decoded, and code that runs as this Safe \
                     could be inside it. Nothing here rules out the pattern that emptied Bybit.",
                ),
            );
            return;
        }
        Err(err) => {
            review.sections.push(s);
            review.find(
                Severity::Critical,
                "MULTISEND_MALFORMED",
                format!(
                    "The batch argument is not a well-formed MultiSend payload ({err}). \
                     What these calls do cannot be determined from these bytes."
                ),
            );
            return;
        }
    };

    s.field("Calls in batch", format!("{}", calls.len()));
    // What the batch moves in total, so nobody has to add up 32 lines by eye.
    let mut total = U256::ZERO;
    let mut total_known = true;
    for call in &calls {
        match total.checked_add(&call.value) {
            Some(sum) => total = sum,
            None => total_known = false,
        }
    }
    if !total.is_zero() || !total_known {
        s.field(
            "Native value, whole batch (wei)",
            if total_known {
                total.to_grouped_decimal()
            } else {
                String::from("more than 2^256 - 1: the batch does not add up")
            },
        );
    }
    let shown = calls.len().min(multisend::MAX_BATCH_CALLS);
    if calls.len() > shown {
        s.field(
            "Calls shown below",
            format!("{shown} of {} — the rest are not displayed", calls.len()),
        );
    }
    if !value.is_zero() {
        s.field("Native value (wei)", value.to_grouped_decimal());
    }
    review.sections.push(s);

    review.find(
        Severity::Warning,
        "SAFE_MULTISEND_BATCH",
        format!(
            "DELEGATECALL into {}, a published Safe batching contract, which executes the {} \
             call(s) below one after another as this Safe. Each is reviewed separately. This \
             device cannot read chain state, so it confirms the address, not the code deployed \
             at it on this chain.",
            address::checksummed(&to),
            calls.len()
        ),
    );

    let depth = ctx.depth.saturating_add(1);
    if depth > MAX_NESTING {
        review.find(
            Severity::Blind,
            "NESTING_LIMIT",
            format!("Batches are nested more than {MAX_NESTING} levels deep; the innermost calls are not decoded."),
        );
        review.find(
            Severity::Critical,
            "UNDECODED_DELEGATECALL_POSSIBLE",
            String::from(
                "The innermost calls were not decoded, and code that runs as this Safe could be \
                 among them.",
            ),
        );
        return;
    }

    // Every call is judged, including the ones there is no room to display. An
    // operator who acknowledges "this batch is too long" must not be signing an
    // inner DELEGATECALL, an unlimited approval, or an owner change that they
    // were never shown.
    //
    // Each hidden call goes through exactly the same rules as a displayed one,
    // into a scratch review whose sections are then discarded. Only its findings
    // survive, and they are grouped by code before being reported: a batch may
    // hold a thousand calls, and a thousand separately acknowledgeable findings
    // is not a review anybody reads.
    if let Some(hidden) = calls.get(shown..) {
        let mut by_code: BTreeMap<&'static str, (Severity, Vec<usize>)> = BTreeMap::new();
        for (offset, call) in hidden.iter().enumerate() {
            let position = shown.saturating_add(offset).saturating_add(1);
            let mut scratch = Review::new("hidden batch call");
            review_call(
                &mut scratch,
                "hidden batch call",
                call.to,
                call.value,
                call.data,
                CallContext {
                    safe: ctx.safe,
                    operation: call.operation,
                    chain_id: ctx.chain_id,
                    depth,
                },
            );
            for f in scratch.findings() {
                let entry = by_code.entry(f.code).or_insert((f.severity, Vec::new()));
                if f.severity > entry.0 {
                    entry.0 = f.severity;
                }
                entry.1.push(position);
            }
        }

        for (code, (severity, positions)) in by_code {
            review.find(
                severity,
                code,
                format!(
                    "{} of the {} calls in this batch raise this, and they are past the {shown} \
                     shown below: {}. What they do is judged here but cannot be read from the \
                     screen.",
                    positions.len(),
                    calls.len(),
                    list_positions(&positions),
                ),
            );
        }

        if !hidden.is_empty() {
            review.find(
                Severity::Blind,
                "MULTISEND_CALLS_NOT_SHOWN",
                format!(
                    "{} of the {} calls in this batch are not shown. What they do cannot be read \
                     from this screen.",
                    hidden.len(),
                    calls.len()
                ),
            );
        }
    }

    let total = calls.len();
    for (i, call) in calls.iter().take(shown).enumerate() {
        let BatchCall {
            operation,
            to: inner_to,
            value: inner_value,
            data: inner_data,
        } = *call;
        let title = format!("Batch call {} of {}", i.saturating_add(1), total);
        review_call(
            review,
            &title,
            inner_to,
            inner_value,
            inner_data,
            CallContext {
                safe: ctx.safe,
                operation,
                chain_id: ctx.chain_id,
                depth,
            },
        );
    }
}

/// "33, 47 and 108", or "33, 47, 108 and 12 more". A finding naming a thousand
/// call positions is a finding nobody finishes reading.
fn list_positions(positions: &[usize]) -> String {
    const NAMED: usize = 8;
    let named: Vec<String> = positions
        .iter()
        .take(NAMED)
        .map(|p| format!("{p}"))
        .collect();
    let rest = positions.len().saturating_sub(named.len());
    let head = named.join(", ");
    if rest > 0 {
        format!("{head} and {rest} more")
    } else {
        head
    }
}

/// The companion to a carried list nobody decoded: padding a batch past what
/// will be read must not earn an attacker a milder review than a malformed
/// encoding gets, so it is CRITICAL, as an over-long MultiSend batch already is.
fn undecoded_calls_finding(review: &mut Review) {
    review.find(
        Severity::Critical,
        "UNDECODED_CALLS",
        String::from(
            "The calls that were not decoded still run when this is executed, and nothing here \
             rules out an unlimited approval, an ownership transfer or a move of everything an \
             account holds among them.",
        ),
    );
}

enum Decoded<'a> {
    Done,
    Unknown,
    Nested {
        safe: Address,
        inner_to: Address,
        inner_value: U256,
        inner_data: &'a [u8],
        operation: Operation,
    },
    /// Calls this call carries, each to be reviewed on its own terms.
    Carried {
        calls: Vec<InnerCall<'a>>,
        /// How each carried call is titled: "{title} 3 of 7".
        title: &'static str,
        executor: Executor,
    },
}

/// Who makes a carried call.
#[derive(Clone, Copy)]
enum Executor {
    /// multicall: each call runs on the same contract for the same caller.
    SameCaller,
    /// A timelock: the timelock contract itself makes the call.
    Contract,
}

/// One call inside another: a MultiSend batch element, a multicall element,
/// or the call a timelock schedules or executes.
#[derive(Clone, Copy)]
pub struct InnerCall<'a> {
    pub operation: Operation,
    pub to: Address,
    pub value: U256,
    pub data: &'a [u8],
}

/// Review carried calls the way a batch is reviewed: every one is judged by the
/// same rules, the first [`multisend::MAX_BATCH_CALLS`] are displayed, and the
/// findings of the rest are grouped by code so nothing past the display limit
/// can go unnamed.
fn review_inner_calls(review: &mut Review, calls: &[InnerCall<'_>], title: &str, ctx: CallContext) {
    let shown = calls.len().min(multisend::MAX_BATCH_CALLS);
    if let Some(hidden) = calls.get(shown..) {
        let mut by_code: BTreeMap<&'static str, (Severity, Vec<usize>)> = BTreeMap::new();
        for (offset, call) in hidden.iter().enumerate() {
            let position = shown.saturating_add(offset).saturating_add(1);
            let mut scratch = Review::new("hidden carried call");
            review_call(
                &mut scratch,
                "hidden carried call",
                call.to,
                call.value,
                call.data,
                CallContext {
                    operation: call.operation,
                    ..ctx
                },
            );
            for f in scratch.findings() {
                let entry = by_code.entry(f.code).or_insert((f.severity, Vec::new()));
                if f.severity > entry.0 {
                    entry.0 = f.severity;
                }
                entry.1.push(position);
            }
        }
        for (code, (severity, positions)) in by_code {
            review.find(
                severity,
                code,
                format!(
                    "{} of the {} carried calls raise this, and they are past the {shown} shown \
                     below: {}. What they do is judged here but cannot be read from the screen.",
                    positions.len(),
                    calls.len(),
                    list_positions(&positions),
                ),
            );
        }
        if !hidden.is_empty() {
            review.find(
                Severity::Blind,
                "CARRIED_CALLS_NOT_SHOWN",
                format!(
                    "{} of the {} carried calls are not shown. What they do cannot be read from \
                     this screen.",
                    hidden.len(),
                    calls.len()
                ),
            );
        }
    }
    let total = calls.len();
    for (i, call) in calls.iter().take(shown).enumerate() {
        let heading = if total == 1 {
            String::from(title)
        } else {
            format!("{title} {} of {total}", i.saturating_add(1))
        };
        review_call(
            review,
            &heading,
            call.to,
            call.value,
            call.data,
            CallContext {
                operation: call.operation,
                ..ctx
            },
        );
    }
}

fn decode_known<'a>(
    review: &mut Review,
    s: &mut Section,
    to: Address,
    selector: [u8; 4],
    args: Args<'a>,
    ctx: CallContext,
) -> Result<Decoded<'a>, Error> {
    match selector {
        SEL_TRANSFER => {
            args.expect_static_len(2)?;
            let recipient = args.address(0)?;
            let amount = args.uint256(1)?;
            s.field(
                "Action",
                String::from("Matches ERC-20 transfer(address,uint256)"),
            );
            s.field("Contract called", address::display(&to));
            s.field("Recipient, if it is a token", address::display(&recipient));
            s.field("Amount (raw integer units)", amount.to_grouped_decimal());
            note_raw_units(review);
            note_selector_is_not_behaviour(review, &to);
            Ok(Decoded::Done)
        }
        SEL_APPROVE => {
            args.expect_static_len(2)?;
            let spender = args.address(0)?;
            let amount = args.uint256(1)?;
            s.field(
                "Action",
                String::from("Matches ERC-20 approve(address,uint256)"),
            );
            s.field("Contract called", address::display(&to));
            s.field("Spender, if it is a token", address::display(&spender));
            if amount.is_max() || amount.is_effectively_unlimited() {
                // A number just below 2^256-1 spends exactly like 2^256-1 and
                // used to read as an ordinary WARNING, which is a difference no
                // person could be expected to notice on screen.
                s.field(
                    "Allowance",
                    if amount.is_max() {
                        String::from("UNLIMITED (2^256 - 1)")
                    } else {
                        format!("EFFECTIVELY UNLIMITED ({})", amount.to_grouped_decimal())
                    },
                );
                review.find(
                    Severity::Critical,
                    "UNLIMITED_APPROVAL",
                    format!(
                        "Grants {} permission to move ALL of this token, now and in the future, until revoked.",
                        address::checksummed(&spender)
                    ),
                );
            } else if amount.is_zero() {
                s.field(
                    "Allowance (raw integer units)",
                    String::from("0 (revokes approval)"),
                );
            } else {
                s.field("Allowance (raw integer units)", amount.to_grouped_decimal());
                review.find(
                    Severity::Warning,
                    "TOKEN_APPROVAL",
                    format!(
                        "Grants {} permission to move up to this amount of the token.",
                        address::checksummed(&spender)
                    ),
                );
                note_raw_units(review);
            }
            note_selector_is_not_behaviour(review, &to);
            Ok(Decoded::Done)
        }
        SEL_TRANSFER_FROM => {
            args.expect_static_len(3)?;
            let from = args.address(0)?;
            let recipient = args.address(1)?;
            let amount = args.uint256(2)?;
            // ERC-721 uses the same selector with a token ID where ERC-20 has an
            // amount; the bytes do not say which kind of contract this is.
            s.field(
                "Action",
                String::from("Matches transferFrom(address,address,uint256)"),
            );
            s.field("Contract called", address::display(&to));
            s.field("From", address::display(&from));
            s.field("Recipient, if it is a token", address::display(&recipient));
            s.field(
                "Amount, or token ID for an NFT",
                amount.to_grouped_decimal(),
            );
            note_raw_units(review);
            note_selector_is_not_behaviour(review, &to);
            Ok(Decoded::Done)
        }
        SEL_EXEC_TRANSACTION => {
            let inner_to = args.address(0)?;
            let inner_value = args.uint256(1)?;
            let (inner_data, next) = args.bytes_at(2, 10usize.saturating_mul(32))?;
            let operation_raw = args.uint8(3)?;
            let safe_tx_gas = args.uint256(4)?;
            let base_gas = args.uint256(5)?;
            let gas_price = args.uint256(6)?;
            let gas_token = args.address(7)?;
            let refund_receiver = args.address(8)?;
            let (signatures, end) = args.bytes_at(9, next)?;
            args.expect_end(end)?;

            s.field("Action", String::from("Execute a Safe transaction"));
            s.field("Safe", address::display(&to));
            s.field("safeTxGas", safe_tx_gas.to_grouped_decimal());
            s.field("baseGas", base_gas.to_grouped_decimal());
            s.field("gasPrice", gas_price.to_grouped_decimal());
            s.field("gasToken", address::display(&gas_token));
            s.field("refundReceiver", address::display(&refund_receiver));
            s.field("Signatures", signature_summary(signatures.len()));
            review.find(
                Severity::Info,
                "SAFE_EXEC_SUBMISSION",
                String::from(
                    "This submits a Safe transaction that owners have already signed. The Safe nonce is on-chain state \
                     and is not in these bytes, so the Safe transaction hash cannot be recomputed here.",
                ),
            );
            refund_finding(review, gas_price, gas_token, refund_receiver);
            Ok(Decoded::Nested {
                safe: to,
                inner_to,
                inner_value,
                inner_data,
                operation: Operation::from_u8(operation_raw),
            })
        }
        SEL_CHANGE_MASTER_COPY => {
            args.expect_static_len(1)?;
            let implementation = args.address(0)?;
            s.field(
                "Action",
                String::from("Safe: change implementation (master copy)"),
            );
            s.field("New implementation", address::display(&implementation));
            admin_finding(
                review,
                ctx,
                to,
                "SAFE_IMPLEMENTATION_CHANGE",
                "Replaces the code that runs this Safe. The new implementation can do anything with its funds and owners.",
            );
            Ok(Decoded::Done)
        }
        SEL_ADD_OWNER_WITH_THRESHOLD => {
            args.expect_static_len(2)?;
            let owner = args.address(0)?;
            let threshold = args.uint256(1)?;
            s.field("Action", String::from("Safe: add owner"));
            s.field("New owner", address::display(&owner));
            s.field("New threshold", threshold.to_decimal());
            admin_finding(
                review,
                ctx,
                to,
                "SAFE_OWNER_CHANGE",
                "Adds a signer and sets a new signing threshold.",
            );
            Ok(Decoded::Done)
        }
        SEL_REMOVE_OWNER => {
            args.expect_static_len(3)?;
            let prev = args.address(0)?;
            let owner = args.address(1)?;
            let threshold = args.uint256(2)?;
            s.field("Action", String::from("Safe: remove owner"));
            s.field("Owner removed", address::display(&owner));
            s.field("Previous owner in list", address::display(&prev));
            s.field("New threshold", threshold.to_decimal());
            admin_finding(
                review,
                ctx,
                to,
                "SAFE_OWNER_CHANGE",
                "Removes a signer and sets a new signing threshold.",
            );
            Ok(Decoded::Done)
        }
        SEL_SWAP_OWNER => {
            args.expect_static_len(3)?;
            let prev = args.address(0)?;
            let old = args.address(1)?;
            let new = args.address(2)?;
            s.field("Action", String::from("Safe: replace owner"));
            s.field("Owner removed", address::display(&old));
            s.field("Owner added", address::display(&new));
            s.field("Previous owner in list", address::display(&prev));
            admin_finding(
                review,
                ctx,
                to,
                "SAFE_OWNER_CHANGE",
                "Replaces one signer with another.",
            );
            Ok(Decoded::Done)
        }
        SEL_CHANGE_THRESHOLD => {
            args.expect_static_len(1)?;
            let threshold = args.uint256(0)?;
            s.field("Action", String::from("Safe: change signing threshold"));
            s.field("New threshold", threshold.to_decimal());
            admin_finding(
                review,
                ctx,
                to,
                "SAFE_THRESHOLD_CHANGE",
                "Changes how many owners must sign every future transaction.",
            );
            Ok(Decoded::Done)
        }
        SEL_ENABLE_MODULE => {
            args.expect_static_len(1)?;
            let module = args.address(0)?;
            s.field("Action", String::from("Safe: enable module"));
            s.field("Module", address::display(&module));
            admin_finding(
                review,
                ctx,
                to,
                "SAFE_MODULE_CHANGE",
                "An enabled module can execute ANY transaction from this Safe without owner signatures.",
            );
            Ok(Decoded::Done)
        }
        SEL_DISABLE_MODULE => {
            args.expect_static_len(2)?;
            let prev = args.address(0)?;
            let module = args.address(1)?;
            s.field("Action", String::from("Safe: disable module"));
            s.field("Module", address::display(&module));
            s.field("Previous module in list", address::display(&prev));
            admin_finding(
                review,
                ctx,
                to,
                "SAFE_MODULE_CHANGE",
                "Removes a module. Check it is not a recovery or security module you rely on.",
            );
            Ok(Decoded::Done)
        }
        SEL_SET_GUARD => {
            args.expect_static_len(1)?;
            let guard = args.address(0)?;
            s.field("Action", String::from("Safe: set transaction guard"));
            s.field("Guard", address::display(&guard));
            admin_finding(
                review,
                ctx,
                to,
                "SAFE_GUARD_CHANGE",
                "A guard inspects and can block every future transaction, including attempts to remove it.",
            );
            Ok(Decoded::Done)
        }
        SEL_SET_FALLBACK_HANDLER => {
            args.expect_static_len(1)?;
            let handler = args.address(0)?;
            s.field("Action", String::from("Safe: set fallback handler"));
            s.field("Fallback handler", address::display(&handler));
            admin_finding(
                review,
                ctx,
                to,
                "SAFE_FALLBACK_HANDLER_CHANGE",
                "The fallback handler answers calls the Safe does not implement, including signature validation (EIP-1271).",
            );
            Ok(Decoded::Done)
        }
        SEL_INCREASE_ALLOWANCE | SEL_INCREASE_APPROVAL => {
            args.expect_static_len(2)?;
            let spender = args.address(0)?;
            let amount = args.uint256(1)?;
            s.field(
                "Action",
                String::from(if selector == SEL_INCREASE_ALLOWANCE {
                    "Matches increaseAllowance(address,uint256)"
                } else {
                    "Matches increaseApproval(address,uint256)"
                }),
            );
            s.field("Contract called", address::display(&to));
            s.field("Spender, if it is a token", address::display(&spender));
            if amount.is_max() || amount.is_effectively_unlimited() {
                s.field(
                    "Allowance added",
                    if amount.is_max() {
                        String::from("UNLIMITED (2^256 - 1)")
                    } else {
                        format!("EFFECTIVELY UNLIMITED ({})", amount.to_grouped_decimal())
                    },
                );
                review.find(
                    Severity::Critical,
                    "UNLIMITED_APPROVAL",
                    format!(
                        "Adds so much to {}'s allowance that it can move ALL of this token, now and \
                         in the future, until revoked.",
                        address::checksummed(&spender)
                    ),
                );
            } else {
                s.field(
                    "Allowance added (raw integer units)",
                    amount.to_grouped_decimal(),
                );
                if !amount.is_zero() {
                    review.find(
                        Severity::Warning,
                        "TOKEN_APPROVAL",
                        format!(
                            "Grants {} permission to move up to this much MORE of the token, on top \
                             of whatever allowance it already has. The existing allowance is chain \
                             state and is not in these bytes.",
                            address::checksummed(&spender)
                        ),
                    );
                }
                note_raw_units(review);
            }
            note_selector_is_not_behaviour(review, &to);
            Ok(Decoded::Done)
        }
        SEL_DECREASE_ALLOWANCE | SEL_DECREASE_APPROVAL => {
            args.expect_static_len(2)?;
            let spender = args.address(0)?;
            let amount = args.uint256(1)?;
            s.field(
                "Action",
                String::from(if selector == SEL_DECREASE_ALLOWANCE {
                    "Matches decreaseAllowance(address,uint256)"
                } else {
                    "Matches decreaseApproval(address,uint256)"
                }),
            );
            s.field("Contract called", address::display(&to));
            s.field("Spender, if it is a token", address::display(&spender));
            s.field(
                "Allowance removed (raw integer units)",
                amount.to_grouped_decimal(),
            );
            note_raw_units(review);
            note_selector_is_not_behaviour(review, &to);
            Ok(Decoded::Done)
        }
        SEL_SET_APPROVAL_FOR_ALL => {
            args.expect_static_len(2)?;
            let operator = args.address(0)?;
            let approved = args.boolean(1)?;
            s.field(
                "Action",
                String::from("Matches setApprovalForAll(address,bool)"),
            );
            s.field("Collection contract", address::display(&to));
            s.field("Operator", address::display(&operator));
            if approved {
                s.field(
                    "Approval",
                    String::from("EVERY token this account holds in the collection"),
                );
                review.find(
                    Severity::Critical,
                    "APPROVAL_FOR_ALL",
                    format!(
                        "Grants {} permission to transfer EVERY token this account holds in the \
                         collection at {}, including any it receives later, until revoked.",
                        address::checksummed(&operator),
                        address::checksummed(&to)
                    ),
                );
            } else {
                s.field("Approval", String::from("revoked"));
            }
            note_selector_is_not_behaviour(review, &to);
            Ok(Decoded::Done)
        }
        SEL_PERMIT2_APPROVE => {
            args.expect_static_len(4)?;
            let token = args.address(0)?;
            let spender = args.address(1)?;
            let amount = args.uint_bits(2, 160)?;
            let expiration = args.uint_bits(3, 48)?;
            s.field(
                "Action",
                String::from("Matches Permit2 approve(address,address,uint160,uint48)"),
            );
            s.field("Contract called", address::display(&to));
            s.field("Token", address::display(&token));
            s.field("Spender", address::display(&spender));
            s.field("Expires", permit2_expiry(&expiration));
            if to != PERMIT2 {
                review.find(
                    Severity::Warning,
                    "NOT_PERMIT2_ADDRESS",
                    format!(
                        "This is the shape of Permit2's approve, but {} is not Permit2's address \
                         ({}). A contract imitating Permit2 can do anything with this call.",
                        address::checksummed(&to),
                        address::checksummed(&PERMIT2)
                    ),
                );
            }
            if is_unlimited_permit2_amount(&amount) {
                s.field(
                    "Allowance",
                    if is_max_bits(&amount, 160) {
                        String::from("UNLIMITED (2^160 - 1, Permit2's maximum)")
                    } else {
                        format!("EFFECTIVELY UNLIMITED ({})", amount.to_grouped_decimal())
                    },
                );
                review.find(
                    Severity::Critical,
                    "UNLIMITED_APPROVAL",
                    format!(
                        "Lets {} move ALL of token {} out of this account through Permit2 until \
                         the expiry above.",
                        address::checksummed(&spender),
                        address::checksummed(&token)
                    ),
                );
            } else if amount.is_zero() {
                s.field(
                    "Allowance (raw integer units)",
                    String::from("0 (revokes approval)"),
                );
            } else {
                s.field("Allowance (raw integer units)", amount.to_grouped_decimal());
                review.find(
                    Severity::Warning,
                    "TOKEN_APPROVAL",
                    format!(
                        "Lets {} move up to this amount of token {} through Permit2 until the \
                         expiry above.",
                        address::checksummed(&spender),
                        address::checksummed(&token)
                    ),
                );
                note_raw_units(review);
            }
            Ok(Decoded::Done)
        }
        SEL_TRANSFER_OWNERSHIP | SEL_SET_OWNER => {
            args.expect_static_len(1)?;
            let new_owner = args.address(0)?;
            s.field(
                "Action",
                String::from(if selector == SEL_TRANSFER_OWNERSHIP {
                    "Matches Ownable transferOwnership(address)"
                } else {
                    "Matches setOwner(address) (DSAuth, as Maker's DSProxy uses)"
                }),
            );
            s.field("Contract", address::display(&to));
            s.field("New owner", address::display(&new_owner));
            let who = if new_owner == address::ZERO {
                String::from("the zero address, which leaves it with no owner")
            } else {
                address::checksummed(&new_owner)
            };
            review.find(
                Severity::Critical,
                "OWNERSHIP_TRANSFER",
                format!(
                    "If {} is a standard Ownable contract, this hands its ownership to {who} (with \
                     two-step ownership, it nominates them and they take over when they accept). \
                     An owner can typically change its settings, upgrade it, pause it or move what \
                     it holds. Confirm the new owner is an address you control.",
                    address::checksummed(&to)
                ),
            );
            Ok(Decoded::Done)
        }
        SEL_RENOUNCE_OWNERSHIP => {
            args.expect_static_len(0)?;
            s.field(
                "Action",
                String::from("Matches Ownable renounceOwnership()"),
            );
            s.field("Contract", address::display(&to));
            review.find(
                Severity::Critical,
                "OWNERSHIP_RENOUNCE",
                format!(
                    "If {} is a standard Ownable contract, this leaves it with no owner, \
                     permanently: nothing that needs the owner can ever be done again.",
                    address::checksummed(&to)
                ),
            );
            Ok(Decoded::Done)
        }
        SEL_ACCEPT_OWNERSHIP => {
            args.expect_static_len(0)?;
            s.field("Action", String::from("Matches acceptOwnership()"));
            s.field("Contract", address::display(&to));
            review.find(
                Severity::Warning,
                "OWNERSHIP_ACCEPT",
                format!(
                    "Completes a two-step ownership transfer: the account sending this becomes the \
                     owner of {}.",
                    address::checksummed(&to)
                ),
            );
            Ok(Decoded::Done)
        }
        SEL_GRANT_ROLE | SEL_REVOKE_ROLE | SEL_RENOUNCE_ROLE => {
            args.expect_static_len(2)?;
            let role = args.word(0)?;
            let account = args.address(1)?;
            let (signature, verb) = match selector {
                SEL_GRANT_ROLE => ("grantRole(bytes32,address)", "Account receiving it"),
                SEL_REVOKE_ROLE => ("revokeRole(bytes32,address)", "Account losing it"),
                _ => ("renounceRole(bytes32,address)", "Account giving it up"),
            };
            s.field("Action", format!("Matches AccessControl {signature}"));
            s.field("Contract", address::display(&to));
            let named = role_text(role);
            s.field("Role", named.clone());
            s.field(verb, address::display(&account));
            match selector {
                SEL_GRANT_ROLE => review.find(
                    Severity::Critical,
                    "ROLE_GRANT",
                    format!(
                        "Gives {} the role {named} on {}. What a role permits is decided by that \
                         contract's code, which these bytes do not show.{}",
                        address::checksummed(&account),
                        address::checksummed(&to),
                        if role.iter().all(|b| *b == 0) {
                            " The default admin role administers every other role, so its holder \
                             can grant itself all of them."
                        } else {
                            ""
                        }
                    ),
                ),
                SEL_REVOKE_ROLE => review.find(
                    Severity::Warning,
                    "ROLE_REVOKE",
                    format!(
                        "Removes the role {named} from {} on {}. Check it is not a guardian, pauser \
                         or recovery role you rely on.",
                        address::checksummed(&account),
                        address::checksummed(&to)
                    ),
                ),
                _ => review.find(
                    Severity::Warning,
                    "ROLE_RENOUNCE",
                    format!(
                        "{} gives up the role {named} on {}.",
                        address::checksummed(&account),
                        address::checksummed(&to)
                    ),
                ),
            }
            Ok(Decoded::Done)
        }
        SEL_UPGRADE_TO => {
            args.expect_static_len(1)?;
            let implementation = args.address(0)?;
            s.field("Action", String::from("Matches upgradeTo(address)"));
            s.field("Proxy", address::display(&to));
            s.field("New implementation", address::display(&implementation));
            upgrade_finding(review, &to, &implementation, &[]);
            Ok(Decoded::Done)
        }
        SEL_UPGRADE_TO_AND_CALL => {
            let implementation = args.address(0)?;
            let (init, end) = args.bytes_at(1, 64)?;
            args.expect_end(end)?;
            s.field(
                "Action",
                String::from("Matches upgradeToAndCall(address,bytes)"),
            );
            s.field("Proxy", address::display(&to));
            s.field("New implementation", address::display(&implementation));
            s.field("Then runs as the new code", selector_text(init));
            s.field("Length of that call", format!("{} bytes", init.len()));
            upgrade_finding(review, &to, &implementation, init);
            Ok(Decoded::Done)
        }
        SEL_PROXY_ADMIN_UPGRADE => {
            args.expect_static_len(2)?;
            let proxy = args.address(0)?;
            let implementation = args.address(1)?;
            s.field(
                "Action",
                String::from("Matches ProxyAdmin upgrade(address,address)"),
            );
            s.field("Proxy admin", address::display(&to));
            s.field("Proxy", address::display(&proxy));
            s.field("New implementation", address::display(&implementation));
            upgrade_finding(review, &proxy, &implementation, &[]);
            Ok(Decoded::Done)
        }
        SEL_PROXY_ADMIN_UPGRADE_AND_CALL => {
            let proxy = args.address(0)?;
            let implementation = args.address(1)?;
            let (init, end) = args.bytes_at(2, 96)?;
            args.expect_end(end)?;
            s.field(
                "Action",
                String::from("Matches ProxyAdmin upgradeAndCall(address,address,bytes)"),
            );
            s.field("Proxy admin", address::display(&to));
            s.field("Proxy", address::display(&proxy));
            s.field("New implementation", address::display(&implementation));
            s.field("Then runs as the new code", selector_text(init));
            s.field("Length of that call", format!("{} bytes", init.len()));
            upgrade_finding(review, &proxy, &implementation, init);
            Ok(Decoded::Done)
        }
        SEL_CHANGE_ADMIN | SEL_CHANGE_PROXY_ADMIN => {
            let (proxy, new_admin) = if selector == SEL_CHANGE_ADMIN {
                args.expect_static_len(1)?;
                s.field("Action", String::from("Matches changeAdmin(address)"));
                (to, args.address(0)?)
            } else {
                args.expect_static_len(2)?;
                s.field(
                    "Action",
                    String::from("Matches ProxyAdmin changeProxyAdmin(address,address)"),
                );
                s.field("Proxy admin", address::display(&to));
                (args.address(0)?, args.address(1)?)
            };
            s.field("Proxy", address::display(&proxy));
            s.field("New admin", address::display(&new_admin));
            review.find(
                Severity::Critical,
                "PROXY_ADMIN_CHANGE",
                format!(
                    "If {} is a proxy, this makes {} its admin, and the admin decides what code \
                     runs behind it from then on.",
                    address::checksummed(&proxy),
                    address::checksummed(&new_admin)
                ),
            );
            Ok(Decoded::Done)
        }
        SEL_MULTICALL => {
            let (elements, end) = args.bytes_array_at(0, 32, multisend::MAX_BATCH_CALLS_PARSED)?;
            args.expect_end(end)?;
            s.field("Action", String::from("Matches multicall(bytes[])"));
            s.field("Contract", address::display(&to));
            s.field("Calls bundled", format!("{}", elements.len()));
            carrier_semantics_unverified(review, &to, "multicall(bytes[])");
            if elements.is_empty() {
                review.find(
                    Severity::Info,
                    "EMPTY_CALL",
                    String::from("This multicall carries no calls."),
                );
                return Ok(Decoded::Done);
            }
            Ok(Decoded::Carried {
                calls: elements
                    .into_iter()
                    .map(|data| InnerCall {
                        operation: Operation::Call,
                        to,
                        value: U256::ZERO,
                        data,
                    })
                    .collect(),
                title: "Call bundled by multicall",
                executor: Executor::SameCaller,
            })
        }
        SEL_TIMELOCK_SCHEDULE | SEL_TIMELOCK_EXECUTE => {
            let target = args.address(0)?;
            let value = args.uint256(1)?;
            let scheduling = selector == SEL_TIMELOCK_SCHEDULE;
            let heads = if scheduling { 6usize } else { 5 };
            let (payload, end) = args.bytes_at(2, heads.saturating_mul(32))?;
            let predecessor = args.word(3)?;
            let salt = args.word(4)?;
            s.field(
                "Action",
                String::from(if scheduling {
                    "Matches TimelockController schedule(address,uint256,bytes,bytes32,bytes32,uint256)"
                } else {
                    "Matches TimelockController execute(address,uint256,bytes,bytes32,bytes32)"
                }),
            );
            s.field("Timelock", address::display(&to));
            if scheduling {
                let delay = args.uint256(5)?;
                s.field("Delay (seconds)", delay.to_grouped_decimal());
            }
            s.field("Predecessor", hex::encode_prefixed(predecessor));
            s.field("Salt", hex::encode_prefixed(salt));
            args.expect_end(end)?;
            carrier_semantics_unverified(
                review,
                &to,
                if scheduling {
                    "TimelockController schedule"
                } else {
                    "TimelockController execute"
                },
            );
            if scheduling {
                review.find(
                    Severity::Warning,
                    "TIMELOCK_SCHEDULE",
                    format!(
                        "Schedules the call below on the timelock at {}. After the delay, whoever \
                         holds the executor role can run it, so it is reviewed here as though it \
                         were running now.",
                        address::checksummed(&to)
                    ),
                );
            }
            Ok(Decoded::Carried {
                calls: alloc::vec![InnerCall {
                    operation: Operation::Call,
                    to: target,
                    value,
                    data: payload,
                }],
                title: if scheduling {
                    "Call the timelock will be able to run"
                } else {
                    "Call the timelock runs now"
                },
                executor: Executor::Contract,
            })
        }
        SEL_ACCOUNT_EXECUTE => account_execute(review, s, to, args),
        _ => Ok(Decoded::Unknown),
    }
}

/// ERC-7579 / ERC-7821 `execute(bytes32 mode, bytes executionData)`.
///
/// The mode's first byte is the call type, the second the execution type, and
/// bytes 6 to 10 a mode selector. Only the forms whose meaning is fixed by the
/// standards are read; any other mode is BLIND, because an account may define
/// it to mean anything.
fn account_execute<'a>(
    review: &mut Review,
    s: &mut Section,
    to: Address,
    args: Args<'a>,
) -> Result<Decoded<'a>, Error> {
    let mode = args.word(0)?;
    let (exec_data, end) = args.bytes_at(1, 64)?;
    args.expect_end(end)?;
    let call_type = mode.first().copied().unwrap_or(0xfe);
    let exec_type = mode.get(1).copied().unwrap_or(0xfe);
    let mode_selector = mode.get(6..10).unwrap_or(&[0xff; 4]);
    s.field(
        "Action",
        String::from("Matches smart-account execute(bytes32,bytes) (ERC-7579 / ERC-7821)"),
    );
    s.field("Account", address::display(&to));
    s.field("Mode", hex::encode_prefixed(mode));
    let blind_mode = |review: &mut Review, s: &mut Section| {
        s.field("Execution data", format!("{} bytes", exec_data.len()));
        review.find(
            Severity::Blind,
            "UNKNOWN_EXECUTION_MODE",
            format!(
                "Mode {} is not one the standards define, so what the account at {} does with \
                 this call is up to its code, which these bytes do not contain.",
                hex::encode_prefixed(mode),
                address::checksummed(&to)
            ),
        );
    };
    match exec_type {
        0x00 => s.field("If a call fails", String::from("the whole batch is undone")),
        0x01 => s.field(
            "If a call fails",
            String::from("it is skipped and the rest still run"),
        ),
        _ => {
            blind_mode(review, s);
            return Ok(Decoded::Done);
        }
    }
    let calls: Vec<InnerCall<'a>> = match (call_type, mode_selector) {
        // Batch: abi.encode((address,uint256,bytes)[]).
        (0x01, [0, 0, 0, 0]) => {
            let inner = Args::new(exec_data);
            let (list, end) =
                inner.call_tuple_array_at(0, 32, multisend::MAX_BATCH_CALLS_PARSED)?;
            inner.expect_end(end)?;
            list.into_iter()
                .map(|(to, value, data)| InnerCall {
                    operation: Operation::Call,
                    to,
                    value,
                    data,
                })
                .collect()
        }
        // ERC-7821 batch with authorisation data: abi.encode(calls, bytes opData).
        (0x01, [0x78, 0x21, 0x00, 0x01]) => {
            let inner = Args::new(exec_data);
            let (list, after) =
                inner.call_tuple_array_at(0, 64, multisend::MAX_BATCH_CALLS_PARSED)?;
            let (op_data, end) = inner.bytes_at(1, after)?;
            inner.expect_end(end)?;
            s.field(
                "Authorisation data",
                format!(
                    "{} bytes, checked by the account and not interpreted here",
                    op_data.len()
                ),
            );
            list.into_iter()
                .map(|(to, value, data)| InnerCall {
                    operation: Operation::Call,
                    to,
                    value,
                    data,
                })
                .collect()
        }
        // Single: abi.encodePacked(target, value, callData).
        (0x00, [0, 0, 0, 0]) => {
            let target: Address = exec_data
                .get(..20)
                .ok_or(Error::Truncated)?
                .try_into()
                .map_err(|_| Error::InvalidAddress)?;
            let value = U256::from_be_slice(exec_data.get(20..52).ok_or(Error::Truncated)?)?;
            let data = exec_data.get(52..).unwrap_or(&[]);
            alloc::vec![InnerCall {
                operation: Operation::Call,
                to: target,
                value,
                data
            }]
        }
        // Delegatecall: abi.encodePacked(target, callData). Code at the target
        // runs as the account, so its calldata describes nothing.
        (0xff, [0, 0, 0, 0]) => {
            let target: Address = exec_data
                .get(..20)
                .ok_or(Error::Truncated)?
                .try_into()
                .map_err(|_| Error::InvalidAddress)?;
            let data = exec_data.get(20..).unwrap_or(&[]);
            s.field(
                "Code that will run as the account",
                address::display(&target),
            );
            s.field("Calldata selector", selector_text(data));
            s.field("Calldata length", format!("{} bytes", data.len()));
            carrier_semantics_unverified(review, &to, "smart-account execute(bytes32,bytes)");
            review.find(
                Severity::Critical,
                "ACCOUNT_DELEGATECALL",
                format!(
                    "The account at {} runs the code at {} as itself, with full control over \
                     everything it holds. Any function name the calldata appears to have does NOT \
                     describe what that code does, so it is deliberately not decoded.",
                    address::checksummed(&to),
                    address::checksummed(&target)
                ),
            );
            return Ok(Decoded::Done);
        }
        _ => {
            blind_mode(review, s);
            return Ok(Decoded::Done);
        }
    };
    s.field("Calls in batch", format!("{}", calls.len()));
    carrier_semantics_unverified(review, &to, "smart-account execute(bytes32,bytes)");
    if calls.is_empty() {
        review.find(
            Severity::Info,
            "EMPTY_CALL",
            String::from("This batch carries no calls."),
        );
        return Ok(Decoded::Done);
    }
    review.find(
        Severity::Info,
        "SMART_ACCOUNT_BATCH",
        format!(
            "The account at {} makes the {} call(s) below itself, one after another. Each is \
             reviewed separately.",
            address::checksummed(&to),
            calls.len()
        ),
    );
    Ok(Decoded::Carried {
        calls,
        title: "Call made by the account",
        executor: Executor::Contract,
    })
}

fn carrier_semantics_unverified(review: &mut Review, target: &Address, claimed: &str) {
    review.find(
        Severity::Blind,
        "CARRIED_CALL_SEMANTICS_UNVERIFIED",
        format!(
            "The calldata matches {claimed}, but a selector and ABI layout do not prove that the contract at {} carries out those calls. Its code was not verified; the inner-call review is best-effort.",
            address::checksummed(target)
        ),
    );
}

fn upgrade_finding(review: &mut Review, proxy: &Address, implementation: &Address, init: &[u8]) {
    review.find(
        Severity::Critical,
        "PROXY_UPGRADE",
        format!(
            "If {} is a proxy, this replaces the code behind it with the code at {}. That code \
             will run with everything the proxy holds and controls, and nothing in these bytes \
             says what it does.",
            address::checksummed(proxy),
            address::checksummed(implementation)
        ),
    );
    if !init.is_empty() {
        // The same reasoning as a DELEGATECALL: the call runs as code these
        // bytes do not contain, so its function name would describe nothing.
        review.find(
            Severity::Blind,
            "UPGRADE_CALL_NOT_DECODED",
            format!(
                "The upgrade also runs {} ({} bytes) as the new implementation. What it does \
                 depends on code these bytes do not contain, so it is deliberately not decoded.",
                selector_text(init),
                init.len()
            ),
        );
    }
}

/// A role as it appears in AccessControl: the default admin role by its value,
/// a role from [`KNOWN_ROLE_NAMES`] by hash, anything else as raw hex.
fn role_text(role: &[u8]) -> String {
    if role.iter().all(|b| *b == 0) {
        return String::from("DEFAULT_ADMIN_ROLE (0x00…00)");
    }
    for name in KNOWN_ROLE_NAMES {
        if crate::keccak::keccak256(name.as_bytes()).as_slice() == role {
            return format!("{name} ({})", hex::encode_prefixed(role));
        }
    }
    format!(
        "{} (not a role name this reviewer recognises)",
        hex::encode_prefixed(role)
    )
}

/// True when `v` is 2^bits - 1, the maximum of an unsigned `bits`-bit integer.
fn is_max_bits(v: &U256, bits: usize) -> bool {
    let full = bits / 8;
    let zero = 32usize.saturating_sub(full);
    v.0.iter().take(zero).all(|b| *b == 0) && v.0.iter().skip(zero).all(|b| *b == 0xff)
}

/// Whether a Permit2 allowance (a `uint160`) is unlimited in every practical
/// sense: its maximum, or anything from 2^144 up. A trillion trillion tokens
/// with 18 decimals is about 2^140, so nothing above 2^144 is an amount anyone
/// chose for its value — and a number one below the maximum spends exactly like
/// the maximum, which is the gap `U256::is_effectively_unlimited` closed for
/// ERC-20 approvals.
pub fn is_unlimited_permit2_amount(v: &U256) -> bool {
    is_max_bits(v, 160)
        || v.0
            .get(..14)
            .is_some_and(|high| high.iter().any(|b| *b != 0))
}

/// A Permit2 expiry, which reads 0 as "this block" rather than as 1970.
pub fn permit2_expiry(t: &U256) -> String {
    if t.is_zero() {
        String::from(
            "0 — Permit2 replaces this with the current block's time: usable in that block, then \
             expired",
        )
    } else {
        unix_time(t)
    }
}

/// A Unix timestamp as the integer and as a UTC calendar date, so nobody has
/// to convert 1798761600 in their head. The date is arithmetic on the signed
/// value; the device has no clock and does not say whether it has passed.
pub fn unix_time(t: &U256) -> String {
    let Some(secs) = t.to_u64() else {
        return t.to_grouped_decimal();
    };
    match civil_date(secs) {
        Some((y, m, d, hh, mm)) => {
            format!("{secs} ({y:04}-{m:02}-{d:02} {hh:02}:{mm:02} UTC)")
        }
        None => format!("{secs}"),
    }
}

/// Days-to-civil conversion (Howard Hinnant's algorithm), checked throughout.
/// Returns None past the year 9999 rather than printing a date nobody can read.
fn civil_date(secs: u64) -> Option<(u64, u64, u64, u64, u64)> {
    let days = secs.checked_div(86_400)?;
    let rem = secs.checked_rem(86_400)?;
    let (hh, mm) = (
        rem.checked_div(3600)?,
        rem.checked_rem(3600)?.checked_div(60)?,
    );
    let z = days.checked_add(719_468)?;
    let era = z.checked_div(146_097)?;
    let doe = z.checked_sub(era.checked_mul(146_097)?)?;
    let yoe = doe
        .checked_sub(doe.checked_div(1460)?)?
        .checked_add(doe.checked_div(36_524)?)?
        .checked_sub(doe.checked_div(146_096)?)?
        .checked_div(365)?;
    let y = yoe.checked_add(era.checked_mul(400)?)?;
    let doy = doe.checked_sub(
        yoe.checked_mul(365)?
            .checked_add(yoe.checked_div(4)?)?
            .checked_sub(yoe.checked_div(100)?)?,
    )?;
    let mp = doy.checked_mul(5)?.checked_add(2)?.checked_div(153)?;
    let d = doy
        .checked_sub(mp.checked_mul(153)?.checked_add(2)?.checked_div(5)?)?
        .checked_add(1)?;
    let m = if mp < 10 {
        mp.checked_add(3)?
    } else {
        mp.checked_sub(9)?
    };
    let y = if m <= 2 { y.checked_add(1)? } else { y };
    if y > 9999 {
        return None;
    }
    Some((y, m, d, hh, mm))
}

fn admin_finding(
    review: &mut Review,
    ctx: CallContext,
    to: Address,
    code: &'static str,
    what: &str,
) {
    if ctx.safe == Some(to) {
        review.find(Severity::Critical, code, String::from(what));
    } else {
        review.find(
            Severity::Warning,
            "SAFE_ADMIN_SELECTOR_ON_OTHER_CONTRACT",
            format!(
                "A Safe administration function is being called on {}, which is not the Safe executing this call. \
                 It will only succeed if that contract grants this caller authority.",
                address::checksummed(&to)
            ),
        );
    }
}

pub fn refund_finding(
    review: &mut Review,
    gas_price: U256,
    gas_token: Address,
    refund_receiver: Address,
) {
    if gas_price.is_zero() {
        return;
    }
    let token = if gas_token == address::ZERO {
        String::from("native currency")
    } else {
        format!("token {}", address::checksummed(&gas_token))
    };
    let receiver = if refund_receiver == address::ZERO {
        String::from("whoever executes the transaction")
    } else {
        address::checksummed(&refund_receiver)
    };
    review.find(
        Severity::Warning,
        "SAFE_GAS_REFUND",
        format!(
            "The Safe will pay a gas refund in {token} at gasPrice {} to {receiver}.",
            gas_price.to_grouped_decimal()
        ),
    );
}

/// A four-byte selector says what shape the call has, not what the code at the
/// other end will do with it.
///
/// `0xa9059cbb` with two well-formed arguments is what an ERC-20 `transfer`
/// looks like. It is also what a contract that does something else entirely
/// looks like if it chooses that selector, and what a proxy looks like the day
/// after its implementation changes. Nothing in the signed bytes says which.
/// The decoder reads bytes and cannot read deployed code, so it reports the
/// shape as a shape.
fn note_selector_is_not_behaviour(review: &mut Review, to: &Address) {
    // One notice per destination, not one per review. Deduplicating on the code
    // alone meant a batch touching four contracts carried a single notice
    // naming the first of them, which reads as though the limitation applies to
    // that address and not to the other three.
    let named = address::checksummed(to);
    let already = review
        .findings()
        .iter()
        .any(|f| f.code == "SELECTOR_IS_NOT_BEHAVIOUR" && f.message.contains(named.as_str()));
    if already {
        return;
    }
    review.find(
        Severity::Info,
        "SELECTOR_IS_NOT_BEHAVIOUR",
        format!(
            "The call to {named} is shaped like a standard token function, and that is all these \
             bytes establish. Whether that address is a token, and what its code does when called \
             this way, is not in the signed bytes and was not checked. A contract can answer to a \
             familiar selector however it likes, and a proxy can be pointed somewhere new between \
             one transaction and the next."
        ),
    );
}

fn note_raw_units(review: &mut Review) {
    if !review.has("TOKEN_UNITS_RAW") {
        review.find(
            Severity::Info,
            "TOKEN_UNITS_RAW",
            String::from(
                "Token amounts are raw integers. Symbols and decimals are not shown because they cannot be derived \
                 from the signed bytes.",
            ),
        );
    }
}

fn selector_text(data: &[u8]) -> String {
    match data.get(..4) {
        Some(sel) => hex::encode_prefixed(sel),
        None if data.is_empty() => String::from("(none)"),
        None => format!("(short: {})", hex::encode_prefixed(data)),
    }
}

fn signature_summary(len: usize) -> String {
    if len % 65 == 0 {
        format!(
            "{len} bytes ({} owner signatures if all are 65-byte ECDSA)",
            len / 65
        )
    } else {
        format!("{len} bytes (includes contract or non-standard signatures)")
    }
}
