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

// Found by mutation testing, 5 Oct 2026: nothing checked that a listing of
// several transactions is refused. Read as its first record instead, a file
// could show one transaction while the signer was asked for another.
#[test]
fn a_listing_of_several_transactions_is_refused_not_read_as_its_first() {
    let mut other = record();
    other["nonce"] = json!(2);
    let two = json!({"results": [record(), other]});
    let e = parse(&two.to_string(), None).err().unwrap();
    assert!(e.contains("holds 2 transactions"), "{e}");
    let none = json!({"results": []});
    let e = parse(&none.to_string(), None).err().unwrap();
    assert!(e.contains("holds 0 transactions"), "{e}");
}

// Found by mutation testing, 5 Oct 2026: the shared flow the desktop app and
// the browser use settles a v1.1.x Safe's domain from the file's own hash.
// Only the command line's copy of this logic had a test.
#[test]
fn the_hash_in_the_file_settles_a_legacy_domain_without_being_told() {
    use clearsign::{DomainVersion, hex};
    let tx = parse(&record().to_string(), None).unwrap().tx;
    for version in [DomainVersion::Legacy, DomainVersion::V1_3Plus] {
        let mut v = record();
        let hash = clearsign::safe_transaction_hash(&tx, version);
        v["safeTxHash"] = json!(hex::encode_prefixed(&hash));
        let reviewed = clearsign_safe_json::review_json(&v.to_string(), None, None).unwrap();
        assert_eq!(reviewed.version, version);
        assert_eq!(reviewed.hash, hash);
        assert!(reviewed.hash_confirmed);
        assert_eq!(
            reviewed.version_source,
            "the hash in the file matches this domain"
        );
    }
}

// Found by mutation testing, 5 Oct 2026: the size limit, at its boundary.
//
// Five survivors in this crate are equivalent, so no test can kill them: the
// special case for an empty `data` string (without it, the string decodes to
// the same empty bytes); the text the JSON visitor gives serde for a value of
// the wrong kind (it accepts every kind, so the text is never used); the size
// check on typed data that arrives as a string, twice (that string is shorter
// than the request already checked); and the sign test on integers read as
// i64 because they are not u64 (only negative ones get there, so `< 0` and
// `<= 0` agree).
#[test]
fn a_record_of_exactly_the_size_limit_is_read_and_one_byte_more_is_not() {
    use clearsign_safe_json::MAX_RECORD_BYTES;
    let base = record().to_string();
    let at_limit = format!("{base}{}", " ".repeat(MAX_RECORD_BYTES - base.len()));
    assert_eq!(at_limit.len(), MAX_RECORD_BYTES);
    assert!(parse(&at_limit, None).is_ok());
    let e = parse(&format!("{at_limit} "), None).err().unwrap();
    assert!(e.contains("the limit is"), "{e}");
}

#[test]
fn malformed_or_deep_metadata_is_not_ignored_by_the_parser() {
    let original = record().to_string();
    assert!(parse(&format!("{original} null"), None).is_err());
    let nested = format!("{}0{}", "[".repeat(200), "]".repeat(200));
    let input = format!(r#"{{"metadata":{nested},{}"#, &original[1..]);
    assert!(parse(&input, None).is_err());
}

/// Safe's service writes `null` for a zero refund address on some executed
/// transactions (one of 10,851 records sampled across 28 chains, Unichain,
/// October 2026). That record is accepted only because its own hash proves
/// what was signed; without a hash, or with one that disagrees, it is refused.
#[test]
fn a_null_refund_address_is_accepted_only_when_the_hash_proves_it() {
    use clearsign::{DomainVersion, hex};
    let mut v = record();
    v["gasToken"] = Value::Null;
    v["refundReceiver"] = Value::Null;
    assert!(
        parse(&v.to_string(), None).is_err(),
        "no hash: nothing proves the reading"
    );

    let zero = record();
    let tx = parse(&zero.to_string(), None).unwrap().tx;
    let hash = clearsign::safe_transaction_hash(&tx, DomainVersion::V1_3Plus);
    v["safeTxHash"] = json!(hex::encode_prefixed(&hash));
    let parsed = parse(&v.to_string(), None).unwrap();
    assert_eq!(parsed.null_read_as_zero, vec!["gasToken", "refundReceiver"]);
    let reviewed = clearsign_safe_json::review_json(&v.to_string(), None, None).unwrap();
    assert_eq!(reviewed.hash, hash);
    // Said in the review itself, so the desktop app and the browser show it,
    // not only the command line.
    let note = reviewed
        .review
        .findings()
        .iter()
        .find(|f| f.code == "NULL_READ_AS_ZERO")
        .unwrap();
    assert_eq!(note.severity, clearsign::Severity::Info);
    assert!(
        note.message.contains("gasToken and refundReceiver"),
        "{}",
        note.message
    );
    let plain =
        clearsign_safe_json::review_json(&zero.to_string(), Some(1), Some(DomainVersion::V1_3Plus))
            .unwrap();
    assert!(!plain.review.has("NULL_READ_AS_ZERO"));

    let mut wrong = v.clone();
    wrong["safeTxHash"] = json!(hex::encode_prefixed(&[7u8; 32]));
    assert!(clearsign_safe_json::review_json(&wrong.to_string(), None, None).is_err());
}
