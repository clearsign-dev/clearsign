//! A limit applied after the value has been built is not a limit.

#![allow(clippy::expect_used, clippy::unwrap_used, clippy::panic)]

use clearsign_safe_json::{MAX_RECORD_BYTES, parse};

fn record_with_padding(bytes: usize) -> String {
    let padding = "A".repeat(bytes);
    format!(
        r#"{{"safe":"0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4",
            "to":"0x00000000000000000000000000000000DeaDBeef",
            "value":"0","data":"0x","operation":0,"nonce":1,
            "safeTxGas":"0","baseGas":"0","gasPrice":"0",
            "gasToken":"0x0000000000000000000000000000000000000000",
            "refundReceiver":"0x0000000000000000000000000000000000000000",
            "chainId":"1","padding":"{padding}"}}"#
    )
}

#[test]
fn an_oversized_record_is_refused_before_it_is_parsed() {
    // The signed fields are all fine. Two megabytes of padding sit in a field
    // nothing reads, which is exactly the shape of input that gets through a
    // check placed after parsing.
    let json = record_with_padding(2 * 1024 * 1024);
    assert!(json.len() > MAX_RECORD_BYTES);
    let err = match parse(&json, Some(1)) {
        Err(e) => e,
        Ok(_) => panic!("a record over the limit must be refused"),
    };
    assert!(
        err.contains("the limit is"),
        "should say it is too large; said: {err}"
    );
}

#[test]
fn an_ordinary_record_is_not_caught_by_the_limit() {
    let json = record_with_padding(64);
    assert!(json.len() < MAX_RECORD_BYTES);
    assert!(
        parse(&json, Some(1)).is_ok(),
        "an ordinary record must still be read"
    );
}
