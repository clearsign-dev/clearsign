//! Decoding a single contract call, and the risk rules that apply to it.
//!
//! The rules here are the product. Each finding code is stable and documented,
//! because auditors and other wallets may key automation off them.

use alloc::format;
use alloc::string::String;

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
    // inner DELEGATECALL they were never shown.
    if let Some(hidden) = calls.get(shown..) {
        let hidden_delegatecalls = hidden
            .iter()
            .filter(|c| matches!(c.operation, Operation::DelegateCall))
            .count();
        let hidden_invalid = hidden
            .iter()
            .filter(|c| matches!(c.operation, Operation::Invalid(_)))
            .count();
        if hidden_delegatecalls > 0 {
            review.find(
                Severity::Critical,
                "SAFE_DELEGATECALL",
                format!(
                    "{hidden_delegatecalls} of the {} calls in this batch are DELEGATECALLs that run \
                     code with full control over this Safe, and they are past the {shown} shown \
                     below. This is the pattern that emptied Bybit.",
                    calls.len()
                ),
            );
        }
        if hidden_invalid > 0 {
            review.find(
                Severity::Critical,
                "SAFE_INVALID_OPERATION",
                format!("{hidden_invalid} calls past the ones shown have an operation value that is neither CALL nor DELEGATECALL."),
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
            s.field("Action", String::from("ERC-20 transfer"));
            s.field("Token contract", address::display(&to));
            s.field("Recipient", address::display(&recipient));
            s.field("Amount (raw integer units)", amount.to_grouped_decimal());
            note_raw_units(review);
            Ok(Decoded::Done)
        }
        SEL_APPROVE => {
            args.expect_static_len(2)?;
            let spender = args.address(0)?;
            let amount = args.uint256(1)?;
            s.field("Action", String::from("ERC-20 approve"));
            s.field("Token contract", address::display(&to));
            s.field("Spender", address::display(&spender));
            if amount.is_max() {
                s.field("Allowance", String::from("UNLIMITED (2^256 - 1)"));
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
            Ok(Decoded::Done)
        }
        SEL_TRANSFER_FROM => {
            args.expect_static_len(3)?;
            let from = args.address(0)?;
            let recipient = args.address(1)?;
            let amount = args.uint256(2)?;
            s.field("Action", String::from("ERC-20 transferFrom"));
            s.field("Token contract", address::display(&to));
            s.field("From", address::display(&from));
            s.field("Recipient", address::display(&recipient));
            s.field("Amount (raw integer units)", amount.to_grouped_decimal());
            note_raw_units(review);
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
        _ => Ok(Decoded::Unknown),
    }
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
