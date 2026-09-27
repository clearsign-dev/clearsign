//! Reading a Safe transaction the way a signer actually has one.
//!
//! Nobody preparing a transaction has its fields as command-line flags. They
//! have it in Safe{Wallet}, or as JSON from Safe's Transaction Service. Asking
//! them to retype it by hand is both a barrier and a hazard — a mistyped nonce
//! produces a different hash and a confusing failure, and a *correctly* typed
//! field copied from a compromised screen produces a confident review of the
//! wrong thing.
//!
//! So this reads the JSON directly. Two rules make that safe:
//!
//! 1. Only the fields that are actually signed are read. Everything else the
//!    service returns — including `dataDecoded`, its own opinion of what the
//!    calldata means — is ignored. That opinion comes from the same
//!    infrastructure the signer is trying to check.
//! 2. The `safeTxHash` in the file is never believed. It is recomputed from the
//!    fields, and a disagreement is reported loudly, because the only reason
//!    the two would differ is that something in between is not telling the truth.

use clearsign::{SafeTransaction, U256, hex};
use serde_json::Value;

pub struct FromJson {
    pub tx: SafeTransaction,
    /// The hash the file claims, if it carried one.
    pub claimed_hash: Option<[u8; 32]>,
    /// Where the chain ID came from, for the person reading the output.
    pub chain_id_source: &'static str,
}

/// Parse a Safe Transaction Service record, or the equivalent copied out of
/// Safe{Wallet}.
///
/// `chain_id_flag` is used when the JSON does not carry one. The chain ID is
/// part of what gets signed, so it is never guessed: with neither, this fails.
pub fn parse(json: &str, chain_id_flag: Option<u64>) -> Result<FromJson, String> {
    let v: Value = serde_json::from_str(json).map_err(|e| format!("not valid JSON: {e}"))?;
    // A Transaction Service listing wraps the records in "results".
    let v = match v.get("results").and_then(Value::as_array) {
        Some(results) if results.len() == 1 => results.first().unwrap_or(&Value::Null).clone(),
        Some(results) => {
            return Err(format!(
                "this file holds {} transactions; save the one you are signing on its own",
                results.len()
            ));
        }
        None => v,
    };

    let (chain_id, chain_id_source) = match (uint(&v, "chainId").ok().flatten(), chain_id_flag) {
        (Some(from_file), Some(from_flag)) if from_file != from_flag => {
            return Err(format!(
                "the file says chain {from_file} and --chain-id says {from_flag}. \
                 The chain ID is part of what you sign, so this will not be guessed"
            ));
        }
        (Some(from_file), _) => (from_file, "the file"),
        (None, Some(from_flag)) => (from_flag, "--chain-id"),
        (None, None) => {
            return Err(String::from(
                "the file does not say which chain this is, and the chain ID is part of what you \
                 sign. Pass --chain-id (1 for Ethereum mainnet, 137 for Polygon, and so on)",
            ));
        }
    };

    let tx = SafeTransaction {
        chain_id: U256::from_u64(chain_id),
        safe: address(&v, "safe")?,
        to: address(&v, "to")?,
        value: u256(&v, "value")?,
        data: match v.get("data") {
            None | Some(Value::Null) => Vec::new(),
            Some(Value::String(s)) if s.is_empty() => Vec::new(),
            Some(Value::String(s)) => {
                hex::decode(s).map_err(|e| format!("data is not hexadecimal: {e}"))?
            }
            Some(_) => return Err(String::from("data should be a hex string or null")),
        },
        operation: u8::try_from(uint(&v, "operation")?.unwrap_or(0))
            .map_err(|_| "operation should be 0 (call) or 1 (delegatecall)".to_owned())?,
        safe_tx_gas: u256(&v, "safeTxGas")?,
        base_gas: u256(&v, "baseGas")?,
        gas_price: u256(&v, "gasPrice")?,
        gas_token: address(&v, "gasToken")?,
        refund_receiver: address(&v, "refundReceiver")?,
        nonce: u256(&v, "nonce")?,
    };

    let claimed_hash = match v.get("safeTxHash") {
        None | Some(Value::Null) => None,
        Some(Value::String(s)) => {
            let bytes =
                hex::decode(s).map_err(|e| format!("safeTxHash is not hexadecimal: {e}"))?;
            Some(
                <[u8; 32]>::try_from(bytes.as_slice())
                    .map_err(|_| "safeTxHash is not 32 bytes".to_owned())?,
            )
        }
        Some(_) => return Err(String::from("safeTxHash should be a hex string")),
    };

    Ok(FromJson {
        tx,
        claimed_hash,
        chain_id_source,
    })
}

/// A field that may arrive as a JSON number or as a decimal string. The
/// Transaction Service uses both, sometimes for the same field on different
/// endpoints, so neither form is treated as the odd one out.
fn uint(v: &Value, key: &str) -> Result<Option<u64>, String> {
    match v.get(key) {
        None | Some(Value::Null) => Ok(None),
        Some(Value::Number(n)) => n
            .as_u64()
            .ok_or_else(|| format!("{key} is not a whole, non-negative number"))
            .map(Some),
        Some(Value::String(s)) => s
            .parse::<u64>()
            .map_err(|_| format!("{key} is {s:?}, which is not a whole number"))
            .map(Some),
        Some(_) => Err(format!("{key} should be a number")),
    }
}

fn u256(v: &Value, key: &str) -> Result<U256, String> {
    match v.get(key) {
        None | Some(Value::Null) => Ok(U256::from_u64(0)),
        Some(Value::Number(n)) => n
            .as_u64()
            .map(U256::from_u64)
            .ok_or_else(|| format!("{key} is not a whole, non-negative number")),
        // Values here can exceed u64 — a wei amount easily does — so decimal
        // strings are parsed at full width rather than through u64.
        Some(Value::String(s)) => {
            U256::from_decimal(s).map_err(|e| format!("{key} is {s:?}, which is not a number: {e}"))
        }
        Some(_) => Err(format!("{key} should be a number or a decimal string")),
    }
}

fn address(v: &Value, key: &str) -> Result<[u8; 20], String> {
    let s = match v.get(key) {
        Some(Value::String(s)) => s,
        None | Some(Value::Null) => return Err(format!("the file has no {key}")),
        Some(_) => return Err(format!("{key} should be an address")),
    };
    let bytes = hex::decode(s).map_err(|e| format!("{key} is not hexadecimal: {e}"))?;
    <[u8; 20]>::try_from(bytes.as_slice()).map_err(|_| format!("{key} is not a 20-byte address"))
}
