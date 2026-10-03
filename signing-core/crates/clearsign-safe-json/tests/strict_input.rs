#![allow(clippy::unwrap_used)]

use clearsign_safe_json::parse;
use serde_json::{Value, json};

fn record() -> Value {
    json!({
        "chainId": 1,
        "safe": "0x1111111111111111111111111111111111111111",
        "to": "0x2222222222222222222222222222222222222222",
        "value": "0", "data": "0x", "operation": 0,
        "safeTxGas": "0", "baseGas": "0", "gasPrice": "0",
        "gasToken": "0x0000000000000000000000000000000000000000",
        "refundReceiver": "0x0000000000000000000000000000000000000000",
        "nonce": 1
    })
}

#[test]
fn malformed_chain_id_is_not_replaced_by_the_flag() {
    for invalid in [json!("garbage"), json!(-1), json!(1.5), json!(true)] {
        let mut value = record();
        value["chainId"] = invalid;
        assert!(parse(&value.to_string(), Some(1)).is_err());
    }
}

#[test]
fn signed_numeric_fields_must_be_explicit() {
    for field in [
        "value",
        "operation",
        "safeTxGas",
        "baseGas",
        "gasPrice",
        "nonce",
    ] {
        let mut value = record();
        value.as_object_mut().unwrap().remove(field);
        assert!(parse(&value.to_string(), None).is_err(), "missing {field}");
        value[field] = Value::Null;
        assert!(parse(&value.to_string(), None).is_err(), "null {field}");
    }
}

#[test]
fn duplicate_fields_are_refused_in_records_and_listings() {
    let original = record().to_string();
    for key in ["nonce", "no\\u006ece"] {
        let duplicate = format!(r#"{{"{key}":99,{}"#, &original[1..]);
        for input in [duplicate.clone(), format!(r#"{{"results":[{duplicate}]}}"#)] {
            assert!(parse(&input, None).is_err(), "duplicate key accepted");
        }
    }
}

#[test]
fn valid_records_and_single_result_listings_still_parse() {
    for input in [record(), json!({"results": [record()]})] {
        assert!(parse(&input.to_string(), None).is_ok());
    }
    let mut without_chain = record();
    without_chain.as_object_mut().unwrap().remove("chainId");
    assert!(parse(&without_chain.to_string(), Some(1)).is_ok());
}

#[test]
fn malformed_or_deep_metadata_is_not_ignored_by_the_parser() {
    let original = record().to_string();
    assert!(parse(&format!("{original} null"), None).is_err());
    let nested = format!("{}0{}", "[".repeat(200), "]".repeat(200));
    let input = format!(r#"{{"metadata":{nested},{}"#, &original[1..]);
    assert!(parse(&input, None).is_err());
}
