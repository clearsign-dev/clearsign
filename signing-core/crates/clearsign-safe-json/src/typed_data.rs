//! Reading an `eth_signTypedData_v4` request the way a wallet receives it.
//!
//! The same two rules as the Safe reader: refuse anything ambiguous before it
//! can be displayed, and check size before parsing. Duplicate keys are refused
//! at every level. A JSON number is accepted only when it is a whole number
//! that fits in 64 bits; anything larger must be a string, because a parser
//! that rounds `115792089237316195423570985008687907853269984665640564039457584007913129639935`
//! to a float would show one amount and hash another.

use clearsign::Review;
use clearsign::typed::{Field, TypedData, TypedDataHashes, Value};
use serde_json::Value as Json;

use crate::MAX_RECORD_BYTES;
use crate::strict_json::UniqueValue;

/// Parse a typed-data request. Accepts the bare object, or one wrapped as the
/// second element of `eth_signTypedData_v4` params (`[address, typedData]`),
/// and typed data given as a JSON string, which is how the RPC carries it.
pub fn parse(json: &str) -> Result<TypedData, String> {
    if json.len() > MAX_RECORD_BYTES {
        return Err(format!(
            "this request is {} bytes; the limit is {MAX_RECORD_BYTES}",
            json.len()
        ));
    }
    let UniqueValue(mut v) =
        serde_json::from_str(json).map_err(|e| format!("not valid JSON: {e}"))?;
    // [address, typedData] as an RPC client passes it.
    if let Json::Array(items) = &v {
        match items.as_slice() {
            [Json::String(_), second] => v = second.clone(),
            _ => {
                return Err(String::from(
                    "expected a typed-data object, or [address, typedData]",
                ));
            }
        }
    }
    // The typed data itself may arrive as a JSON string.
    if let Json::String(inner) = &v {
        if inner.len() > MAX_RECORD_BYTES {
            return Err(String::from(
                "the typed data inside this request is too large",
            ));
        }
        let UniqueValue(parsed) = serde_json::from_str(inner)
            .map_err(|e| format!("the typed data string is not valid JSON: {e}"))?;
        v = parsed;
    }
    let obj = v.as_object().ok_or("typed data must be a JSON object")?;
    for key in obj.keys() {
        if !matches!(key.as_str(), "types" | "primaryType" | "domain" | "message") {
            return Err(format!("typed data has an unexpected key {key:?}"));
        }
    }

    let types_json = obj
        .get("types")
        .and_then(Json::as_object)
        .ok_or("typed data has no types object")?;
    let mut types = Vec::new();
    for (name, fields) in types_json {
        let list = fields
            .as_array()
            .ok_or_else(|| format!("type {name:?} is not a list of fields"))?;
        let mut out = Vec::new();
        for f in list {
            let fo = f
                .as_object()
                .ok_or_else(|| format!("a field of {name} is not an object"))?;
            if fo.keys().any(|k| k != "name" && k != "type") {
                return Err(format!(
                    "a field of {name} has keys other than name and type"
                ));
            }
            let fname = fo
                .get("name")
                .and_then(Json::as_str)
                .ok_or_else(|| format!("a field of {name} has no name"))?;
            let ftype = fo
                .get("type")
                .and_then(Json::as_str)
                .ok_or_else(|| format!("field {fname} of {name} has no type"))?;
            out.push(Field {
                name: fname.to_owned(),
                ty: ftype.to_owned(),
            });
        }
        types.push((name.clone(), out));
    }
    let primary_type = obj
        .get("primaryType")
        .and_then(Json::as_str)
        .ok_or("typed data has no primaryType")?
        .to_owned();
    let domain = convert(obj.get("domain").ok_or("typed data has no domain")?, 0)?;
    let message = match obj.get("message") {
        Some(m) => convert(m, 0)?,
        None if primary_type == "EIP712Domain" => Value::Object(Vec::new()),
        None => return Err(String::from("typed data has no message")),
    };
    Ok(TypedData {
        types,
        primary_type,
        domain,
        message,
    })
}

fn convert(v: &Json, depth: usize) -> Result<Value, String> {
    if depth > clearsign::typed::MAX_DEPTH.saturating_mul(2) {
        return Err(String::from("typed data is nested too deeply"));
    }
    Ok(match v {
        Json::Null => Value::Null,
        Json::Bool(b) => Value::Bool(*b),
        Json::Number(n) => {
            if let Some(u) = n.as_u64() {
                Value::Int {
                    negative: false,
                    magnitude: u,
                }
            } else if let Some(i) = n.as_i64() {
                Value::Int {
                    negative: i < 0,
                    magnitude: i.unsigned_abs(),
                }
            } else {
                return Err(format!(
                    "the number {n} cannot be read exactly; large or fractional numbers must be given as strings"
                ));
            }
        }
        Json::String(s) => Value::Str(s.clone()),
        Json::Array(items) => {
            let mut out = Vec::with_capacity(items.len());
            for i in items {
                out.push(convert(i, depth.saturating_add(1))?);
            }
            Value::Array(out)
        }
        Json::Object(map) => {
            let mut out = Vec::with_capacity(map.len());
            for (k, val) in map {
                out.push((k.clone(), convert(val, depth.saturating_add(1))?));
            }
            Value::Object(out)
        }
    })
}

/// Parse, hash and review a typed-data request in one step.
pub fn review(json: &str) -> Result<(Review, TypedDataHashes), String> {
    let td = parse(json)?;
    clearsign::typed::review_and_hash_typed_data(&td).map_err(|e| e.to_string())
}
