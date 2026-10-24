//! EIP-712 typed data, read as a wallet receives it.
//!
//! The hashes are checked against values published elsewhere: the example in
//! the EIP itself, Permit2's mainnet domain separator, and the Bybit Safe
//! transaction's hash. The findings are checked for the structures drainers
//! ask for, and the refusals for every way a request can be ambiguous.

#![allow(clippy::unwrap_used, clippy::panic, clippy::indexing_slicing)]

use clearsign::Severity;
use clearsign_safe_json::typed_data::{parse, review};
use serde_json::{Value, json};

fn mail() -> Value {
    json!({
        "types": {
            "EIP712Domain": [
                {"name": "name", "type": "string"}, {"name": "version", "type": "string"},
                {"name": "chainId", "type": "uint256"}, {"name": "verifyingContract", "type": "address"}
            ],
            "Person": [{"name": "name", "type": "string"}, {"name": "wallet", "type": "address"}],
            "Mail": [{"name": "from", "type": "Person"}, {"name": "to", "type": "Person"}, {"name": "contents", "type": "string"}]
        },
        "primaryType": "Mail",
        "domain": {"name": "Ether Mail", "version": "1", "chainId": 1, "verifyingContract": "0xCcCCccccCCCCcCCCCCCcCcCccCcCCCcCcccccccC"},
        "message": {
            "from": {"name": "Cow", "wallet": "0xCD2a3d9F938E13CD947Ec05AbC7FE734Df8DD826"},
            "to": {"name": "Bob", "wallet": "0xbBbBBBBbbBBBbbbBbbBbbbbBBbBbbbbBbBbbBBbB"},
            "contents": "Hello, Bob!"
        }
    })
}

fn permit(value: &str) -> Value {
    json!({
        "types": {
            "EIP712Domain": [
                {"name": "name", "type": "string"}, {"name": "version", "type": "string"},
                {"name": "chainId", "type": "uint256"}, {"name": "verifyingContract", "type": "address"}
            ],
            "Permit": [
                {"name": "owner", "type": "address"}, {"name": "spender", "type": "address"},
                {"name": "value", "type": "uint256"}, {"name": "nonce", "type": "uint256"}, {"name": "deadline", "type": "uint256"}
            ]
        },
        "primaryType": "Permit",
        "domain": {"name": "USD Coin", "version": "2", "chainId": 1, "verifyingContract": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"},
        "message": {
            "owner": "0x1111111111111111111111111111111111111111",
            "spender": "0x6666666666666666666666666666666666666666",
            "value": value, "nonce": 0, "deadline": "1798761600"
        }
    })
}

fn permit2_domain() -> Value {
    json!({"name": "Permit2", "chainId": 1, "verifyingContract": "0x000000000022D473030F116dDEE9F6B43aC78BA3"})
}

fn permit2_domain_types() -> Value {
    json!([{"name": "name", "type": "string"}, {"name": "chainId", "type": "uint256"}, {"name": "verifyingContract", "type": "address"}])
}

fn permit_single(amount: &str) -> Value {
    json!({
        "types": {
            "EIP712Domain": permit2_domain_types(),
            "PermitSingle": [{"name": "details", "type": "PermitDetails"}, {"name": "spender", "type": "address"}, {"name": "sigDeadline", "type": "uint256"}],
            "PermitDetails": [{"name": "token", "type": "address"}, {"name": "amount", "type": "uint160"}, {"name": "expiration", "type": "uint48"}, {"name": "nonce", "type": "uint48"}]
        },
        "primaryType": "PermitSingle",
        "domain": permit2_domain(),
        "message": {
            "details": {"token": "0xdAC17F958D2ee523a2206206994597C13D831ec7", "amount": amount, "expiration": "1798761600", "nonce": "0"},
            "spender": "0x6666666666666666666666666666666666666666", "sigDeadline": "1798761600"
        }
    })
}

fn codes(json: &Value) -> Vec<(Severity, &'static str)> {
    let (r, _) = review(&json.to_string()).unwrap_or_else(|e| panic!("{e}"));
    r.findings().iter().map(|f| (f.severity, f.code)).collect()
}

fn hex32(h: [u8; 32]) -> String {
    clearsign::hex::encode_prefixed(&h)
}

// ------------------------------------------------------------ hashes

#[test]
fn the_example_in_eip_712_hashes_to_the_published_values() {
    let (_, h) = review(&mail().to_string()).unwrap();
    assert_eq!(
        hex32(h.domain_separator),
        "0xf2cee375fa42b42143804025fc449deafd50cc031ca257e0b194a650a912090f"
    );
    assert_eq!(
        hex32(h.message_hash.unwrap()),
        "0xc52c0ee5d84264471806290a3f2c4cecfc5490626bf912d01f240d7a274b371e"
    );
    assert_eq!(
        hex32(h.signing_hash),
        "0xbe609aee343fb3c4b28e1df9e632fca64fcfaede20f02e86244efddf30957bd2"
    );
}

#[test]
fn permit2_mainnet_domain_separator_matches_the_deployed_contract() {
    // Permit2.DOMAIN_SEPARATOR() on Ethereum mainnet.
    let (_, h) = review(&permit_single("1").to_string()).unwrap();
    assert_eq!(
        hex32(h.domain_separator),
        "0x866a5aba21966af95d6c7ab78eb2b2fc913915c28be3b9aa07cc04ff903e3f28"
    );
}

/// The Bybit transaction, as Safe{Wallet} would ask a wallet to sign it.
fn bybit() -> Value {
    json!({
        "types": {
            "EIP712Domain": [{"name": "verifyingContract", "type": "address"}],
            "SafeTx": [
                {"name": "to", "type": "address"}, {"name": "value", "type": "uint256"}, {"name": "data", "type": "bytes"},
                {"name": "operation", "type": "uint8"}, {"name": "safeTxGas", "type": "uint256"}, {"name": "baseGas", "type": "uint256"},
                {"name": "gasPrice", "type": "uint256"}, {"name": "gasToken", "type": "address"}, {"name": "refundReceiver", "type": "address"},
                {"name": "nonce", "type": "uint256"}
            ]
        },
        "primaryType": "SafeTx",
        "domain": {"verifyingContract": "0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4"},
        "message": {
            "to": "0x96221423681A6d52E184D440a8eFCEbB105C7242", "value": "0",
            "data": "0xa9059cbb000000000000000000000000bdd077f651ebe7f7b3ce16fe5f2b025be29695160000000000000000000000000000000000000000000000000000000000000000",
            "operation": 1, "safeTxGas": "45746", "baseGas": "0", "gasPrice": "0",
            "gasToken": "0x0000000000000000000000000000000000000000", "refundReceiver": "0x0000000000000000000000000000000000000000",
            "nonce": "71"
        }
    })
}

#[test]
fn a_safe_transaction_as_typed_data_gets_the_safe_review_and_the_safe_hash() {
    let (r, h) = review(&bybit().to_string()).unwrap();
    assert_eq!(
        hex32(h.signing_hash),
        "0xb3476d061aeb8fc1d605a873c483a2402d88a68a9cdd1a8b47655dd55ba004f8"
    );
    assert!(r.has("SAFE_DELEGATECALL"));
    assert!(r.has("TYPED_SCHEMA_NOT_BEHAVIOUR"));
    assert_eq!(r.required_acknowledgements()[0], (1, "SAFE_DELEGATECALL"));
}

#[test]
fn a_safe_transaction_as_typed_data_is_for_display_and_shows_its_domain() {
    let (r, h) = review(&bybit().to_string()).unwrap();
    // Every typed-data review is display-only: the request's signature is the
    // wallet's to make, and this review must not become an approval to sign.
    assert!(r.signing_target().is_none());
    assert_eq!(r.sections[0].title, "Signing domain");
    assert!(r.has("SIGNATURE_NEEDS_NO_TRANSACTION"));
    assert!(
        r.digests
            .iter()
            .any(|(k, v)| k == "Safe transaction hash" && *v == hex32(h.signing_hash))
    );
    // The Safe review's own chain-binding finding covers the legacy domain;
    // it is not reported twice.
    assert!(r.has("SIGNATURE_NOT_CHAIN_BOUND"));
    assert!(!r.has("TYPED_DATA_NOT_CHAIN_BOUND"));
}

#[test]
fn a_safe_transaction_with_a_domain_no_safe_uses_is_refused() {
    let mut m = mail();
    m["primaryType"] = json!("SafeTx");
    m["types"]["SafeTx"] = json!([
        {"name": "to", "type": "address"}, {"name": "value", "type": "uint256"}, {"name": "data", "type": "bytes"},
        {"name": "operation", "type": "uint8"}, {"name": "safeTxGas", "type": "uint256"}, {"name": "baseGas", "type": "uint256"},
        {"name": "gasPrice", "type": "uint256"}, {"name": "gasToken", "type": "address"}, {"name": "refundReceiver", "type": "address"},
        {"name": "nonce", "type": "uint256"}
    ]);
    m["message"] = json!({
        "to": "0x96221423681A6d52E184D440a8eFCEbB105C7242", "value": "0", "data": "0x", "operation": 0,
        "safeTxGas": "0", "baseGas": "0", "gasPrice": "0",
        "gasToken": "0x0000000000000000000000000000000000000000", "refundReceiver": "0x0000000000000000000000000000000000000000", "nonce": "1"
    });
    // The domain here has a name and version: not what a Safe hashes.
    let e = review(&m.to_string()).err().unwrap();
    assert!(
        e.contains("does not hash to the Safe transaction hash"),
        "{e}"
    );
}

// ------------------------------------------------------------ findings

#[test]
fn an_unlimited_permit_is_critical_a_bounded_one_a_warning() {
    let max = "115792089237316195423570985008687907853269984665640564039457584007913129639935";
    assert!(codes(&permit(max)).contains(&(Severity::Critical, "UNLIMITED_APPROVAL")));
    assert!(
        codes(&permit(
            "0xffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffffff"
        ))
        .contains(&(Severity::Critical, "UNLIMITED_APPROVAL"))
    );
    let bounded = codes(&permit("5000000"));
    assert!(bounded.contains(&(Severity::Warning, "TOKEN_APPROVAL")));
    assert!(bounded.contains(&(Severity::Blind, "TYPED_SCHEMA_NOT_BEHAVIOUR")));
    assert!(
        !codes(&permit("0"))
            .iter()
            .any(|(_, c)| *c == "TOKEN_APPROVAL")
    );
}

#[test]
fn a_zero_permit_at_an_unverified_contract_is_still_blind() {
    let findings = codes(&permit("0"));
    assert!(findings.contains(&(Severity::Blind, "TYPED_SCHEMA_NOT_BEHAVIOUR")));
    assert!(!findings.iter().any(|(_, code)| *code == "TOKEN_APPROVAL"));
}

// Found by mutation testing, 5 Oct 2026: a permit's finding names the token,
// which is the contract that checks the signature, or says the domain names
// none. No test read which.
#[test]
fn a_permit_finding_names_its_token() {
    let max = "115792089237316195423570985008687907853269984665640564039457584007913129639935";
    let message = |doc: &Value| {
        let (r, _) = review(&doc.to_string()).unwrap();
        let f = r
            .findings()
            .iter()
            .find(|f| f.code == "UNLIMITED_APPROVAL")
            .unwrap();
        f.message.clone()
    };
    let with_token = message(&permit(max));
    assert!(
        with_token.contains("0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48"),
        "{with_token}"
    );
    let mut no_contract = permit(max);
    no_contract["types"]["EIP712Domain"] = json!([
        {"name": "name", "type": "string"}, {"name": "version", "type": "string"},
        {"name": "chainId", "type": "uint256"}
    ]);
    no_contract["domain"]
        .as_object_mut()
        .unwrap()
        .remove("verifyingContract");
    let without = message(&no_contract);
    assert!(without.contains("the token this domain names"), "{without}");
}

// Found by mutation testing, 5 Oct 2026: an ERC-2612 amount near the maximum
// is unlimited too, as it is for an on-chain approve; only the exact maximum
// was tested.
#[test]
fn a_permit_for_an_amount_near_the_maximum_is_unlimited() {
    let two_to_192 = "6277101735386680763835789423207666416102355444464034512896";
    assert!(codes(&permit(two_to_192)).contains(&(Severity::Critical, "UNLIMITED_APPROVAL")));
}

#[test]
fn a_dai_permit_with_allowed_true_is_unlimited() {
    let dai = json!({
        "types": {
            "EIP712Domain": [{"name": "name", "type": "string"}, {"name": "version", "type": "string"}, {"name": "chainId", "type": "uint256"}, {"name": "verifyingContract", "type": "address"}],
            "Permit": [{"name": "holder", "type": "address"}, {"name": "spender", "type": "address"}, {"name": "nonce", "type": "uint256"}, {"name": "expiry", "type": "uint256"}, {"name": "allowed", "type": "bool"}]
        },
        "primaryType": "Permit",
        "domain": {"name": "Dai Stablecoin", "version": "1", "chainId": 1, "verifyingContract": "0x6B175474E89094C44Da98b954EedeAC495271d0F"},
        "message": {"holder": "0x1111111111111111111111111111111111111111", "spender": "0x6666666666666666666666666666666666666666", "nonce": 3, "expiry": 0, "allowed": true}
    });
    assert!(codes(&dai).contains(&(Severity::Critical, "UNLIMITED_APPROVAL")));
}

#[test]
fn permit2_unlimited_is_uint160_max_and_a_fake_permit2_is_called_out() {
    let max160 = "1461501637330902918203684832716283019655932542975";
    assert!(codes(&permit_single(max160)).contains(&(Severity::Critical, "UNLIMITED_APPROVAL")));
    assert!(codes(&permit_single("5")).contains(&(Severity::Warning, "TOKEN_APPROVAL")));
    let mut fake = permit_single("5");
    fake["domain"]["verifyingContract"] = json!("0x6666666666666666666666666666666666666666");
    assert!(codes(&fake).contains(&(Severity::Blind, "NOT_PERMIT2_ADDRESS")));
}

#[test]
fn permit2_amounts_near_the_maximum_are_unlimited_too() {
    // One below uint160's maximum spends exactly like the maximum.
    let below = "1461501637330902918203684832716283019655932542974";
    assert!(codes(&permit_single(below)).contains(&(Severity::Critical, "UNLIMITED_APPROVAL")));
    // 2^144 and up is no amount anyone chose for its value; one below is.
    let at = "22300745198530623141535718272648361505980416";
    assert!(codes(&permit_single(at)).contains(&(Severity::Critical, "UNLIMITED_APPROVAL")));
    let under = codes(&permit_single(
        "22300745198530623141535718272648361505980415",
    ));
    assert!(under.contains(&(Severity::Warning, "TOKEN_APPROVAL")));
    assert!(!under.iter().any(|(_, c)| *c == "UNLIMITED_APPROVAL"));
}

#[test]
fn a_permit2_expiration_of_zero_is_not_shown_as_a_date_long_past() {
    // Permit2 replaces 0 with the block's time: usable at once, not dead.
    let mut p = permit_single("5");
    p["message"]["details"]["expiration"] = json!("0");
    let (r, _) = review(&p.to_string()).unwrap();
    let text = r.render();
    assert!(!text.contains("1970"), "{text}");
    assert!(text.contains("usable in that block"), "{text}");
}

// Found by mutation testing, 5 Oct 2026: Permit2's batch of allowances and
// its single one-time transfer were never put in front of the reviewer, so a
// broken match on either would have fallen back to BLIND unnoticed.
#[test]
fn permit2_allowance_batches_and_single_transfers_are_read() {
    let no_blind =
        |r: &clearsign::Review| !r.findings().iter().any(|f| f.severity == Severity::Blind);
    let batch = json!({
        "types": {
            "EIP712Domain": permit2_domain_types(),
            "PermitBatch": [{"name": "details", "type": "PermitDetails[]"}, {"name": "spender", "type": "address"}, {"name": "sigDeadline", "type": "uint256"}],
            "PermitDetails": [{"name": "token", "type": "address"}, {"name": "amount", "type": "uint160"}, {"name": "expiration", "type": "uint48"}, {"name": "nonce", "type": "uint48"}]
        },
        "primaryType": "PermitBatch",
        "domain": permit2_domain(),
        "message": {
            "details": [
                {"token": "0xdAC17F958D2ee523a2206206994597C13D831ec7", "amount": "1461501637330902918203684832716283019655932542975", "expiration": "1798761600", "nonce": "0"},
                {"token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "amount": "5", "expiration": "1798761600", "nonce": "0"}
            ],
            "spender": "0x6666666666666666666666666666666666666666", "sigDeadline": "1798761600"
        }
    });
    let (r, _) = review(&batch.to_string()).unwrap();
    let text = r.render();
    assert!(
        text.contains("Permit2 allowance 1 of 2") && text.contains("Permit2 allowance 2 of 2"),
        "{text}"
    );
    assert!(r.has("UNLIMITED_APPROVAL") && r.has("TOKEN_APPROVAL"));
    assert!(no_blind(&r), "{text}");

    let single = json!({
        "types": {
            "EIP712Domain": permit2_domain_types(),
            "PermitTransferFrom": [{"name": "permitted", "type": "TokenPermissions"}, {"name": "spender", "type": "address"}, {"name": "nonce", "type": "uint256"}, {"name": "deadline", "type": "uint256"}],
            "TokenPermissions": [{"name": "token", "type": "address"}, {"name": "amount", "type": "uint256"}]
        },
        "primaryType": "PermitTransferFrom",
        "domain": permit2_domain(),
        "message": {
            "permitted": {"token": "0xdAC17F958D2ee523a2206206994597C13D831ec7", "amount": "1000000"},
            "spender": "0x6666666666666666666666666666666666666666", "nonce": "7", "deadline": "1798761600"
        }
    });
    let (r, _) = review(&single.to_string()).unwrap();
    let text = r.render();
    assert!(
        text.contains("Permit2 one-time transfer") && !text.contains("one-time transfer 1 of"),
        "{text}"
    );
    assert!(r.has("TOKEN_APPROVAL"));
    assert!(no_blind(&r), "{text}");
}

#[test]
fn permit2_one_time_transfers_name_every_token() {
    let batch = json!({
        "types": {
            "EIP712Domain": permit2_domain_types(),
            "PermitBatchTransferFrom": [{"name": "permitted", "type": "TokenPermissions[]"}, {"name": "spender", "type": "address"}, {"name": "nonce", "type": "uint256"}, {"name": "deadline", "type": "uint256"}],
            "TokenPermissions": [{"name": "token", "type": "address"}, {"name": "amount", "type": "uint256"}]
        },
        "primaryType": "PermitBatchTransferFrom",
        "domain": permit2_domain(),
        "message": {
            "permitted": [
                {"token": "0xdAC17F958D2ee523a2206206994597C13D831ec7", "amount": "115792089237316195423570985008687907853269984665640564039457584007913129639935"},
                {"token": "0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48", "amount": "1"}
            ],
            "spender": "0x6666666666666666666666666666666666666666", "nonce": "7", "deadline": "1798761600"
        }
    });
    let (r, _) = review(&batch.to_string()).unwrap();
    let text = r.render();
    assert!(
        text.contains("Permit2 one-time transfer 1 of 2")
            && text.contains("Permit2 one-time transfer 2 of 2"),
        "{text}"
    );
    assert!(r.has("UNLIMITED_APPROVAL") && r.has("TOKEN_APPROVAL"));
}

#[test]
fn an_unknown_structure_is_shown_field_by_field_and_blind() {
    let (r, _) = review(&mail().to_string()).unwrap();
    assert_eq!(r.highest_severity(), Some(Severity::Blind));
    let text = r.render();
    assert!(
        text.contains("from.wallet") && text.contains("\"Hello, Bob!\""),
        "{text}"
    );
}

// Found by mutation testing, 5 Oct 2026: how much of an unknown structure is
// shown, at the limit, and how its negative numbers read.
#[test]
fn an_unknown_structure_shows_up_to_its_limit_and_signs_its_numbers() {
    let generic = |xs: usize| {
        json!({
            "types": {
                "EIP712Domain": [{"name": "chainId", "type": "uint256"}],
                "T": [{"name": "a", "type": "int8"}, {"name": "b", "type": "int8"}, {"name": "xs", "type": "uint8[]"}]
            },
            "primaryType": "T",
            "domain": {"chainId": 1},
            "message": {"a": -5, "b": 5, "xs": vec![1; xs]}
        })
        .to_string()
    };
    let lines = clearsign::typed::MAX_GENERIC_LINES;
    let (r, _) = review(&generic(lines - 2)).unwrap();
    let text = r.render();
    assert!(!text.contains("Not shown"), "{text}");
    let field = |name: &str| {
        text.lines()
            .find(|l| l.starts_with(&format!("{name} .")))
            .map(|l| l.rsplit(' ').next().unwrap_or("").to_owned())
    };
    assert_eq!(field("a").as_deref(), Some("-5"));
    assert_eq!(field("b").as_deref(), Some("5"));

    let (r, _) = review(&generic(lines - 1)).unwrap();
    let text = r.render();
    assert!(text.contains("1 more values"), "{text}");

    // Bytes are shown in full up to 64; longer ones by length and hash.
    let blob = |n: usize| {
        json!({
            "types": {
                "EIP712Domain": [{"name": "chainId", "type": "uint256"}],
                "T": [{"name": "d", "type": "bytes"}]
            },
            "primaryType": "T",
            "domain": {"chainId": 1},
            "message": {"d": format!("0x{}", "ab".repeat(n))}
        })
        .to_string()
    };
    let (r, _) = review(&blob(64)).unwrap();
    assert!(r.render().contains(&format!("0x{}", "ab".repeat(64))));
    let (r, _) = review(&blob(65)).unwrap();
    let text = r.render();
    assert!(text.contains("65 bytes, keccak256 0x"), "{text}");
    assert!(!text.contains(&"ab".repeat(65)), "{text}");
}

#[test]
fn control_characters_in_signed_text_are_escaped() {
    let mut m = mail();
    m["message"]["contents"] = json!("ok\u{1b}[2J\u{202e}");
    let (r, _) = review(&m.to_string()).unwrap();
    let text = r.render();
    assert!(!text.contains('\u{1b}') && !text.contains('\u{202e}'));
    assert!(
        text.contains("\\u{001b}") && text.contains("\\u{202e}"),
        "{text}"
    );
}

#[test]
fn a_domain_without_a_chain_is_a_warning() {
    let mut m = permit("1");
    m["types"]["EIP712Domain"] = json!([{"name": "name", "type": "string"}, {"name": "version", "type": "string"}, {"name": "verifyingContract", "type": "address"}]);
    m["domain"].as_object_mut().unwrap().remove("chainId");
    assert!(codes(&m).contains(&(Severity::Warning, "TYPED_DATA_NOT_CHAIN_BOUND")));
}

#[test]
fn the_rpc_forms_are_accepted() {
    // [address, typedData] and typed data as a JSON string.
    let p = permit("1");
    let wrapped = json!(["0x1111111111111111111111111111111111111111", p.to_string()]);
    let a = review(&p.to_string()).unwrap().1;
    let b = review(&wrapped.to_string()).unwrap().1;
    assert_eq!(a, b);
}

// ------------------------------------------------------------ refusals

fn refused(json: &str, why: &str) {
    match review(json) {
        Ok((r, _)) => panic!("accepted ({why}):\n{}", r.render()),
        Err(e) => assert!(!e.is_empty(), "{why}"),
    }
}

#[test]
fn every_ambiguous_request_is_refused() {
    let p = permit("1").to_string();
    refused(
        &p.replacen(
            "\"primaryType\"",
            "\"primaryType\":\"Permit\",\"primaryType\"",
            1,
        ),
        "duplicate key",
    );

    let mut extra = permit("1");
    extra["message"]["spender2"] = json!("0x6666666666666666666666666666666666666666");
    refused(&extra.to_string(), "undeclared field");

    let mut missing = permit("1");
    missing["message"]
        .as_object_mut()
        .unwrap()
        .remove("deadline");
    refused(&missing.to_string(), "missing field");

    let mut alias = permit("1");
    alias["types"]["Permit"][2]["type"] = json!("uint");
    refused(&alias.to_string(), "non-canonical type name");

    let mut negative = permit("1");
    negative["message"]["nonce"] = json!(-1);
    refused(&negative.to_string(), "negative unsigned");

    let mut too_big = permit_single("1461501637330902918203684832716283019655932542976");
    refused(&too_big.to_string(), "uint160 overflow");
    too_big = permit_single("1");
    too_big["message"]["details"]["expiration"] = json!("281474976710656");
    refused(&too_big.to_string(), "uint48 overflow");

    // A number JSON cannot carry exactly.
    let lossy = permit("1").to_string().replace(
        "\"value\":\"1\"",
        "\"value\":115792089237316195423570985008687907853269984665640564039457584007913129639935",
    );
    refused(&lossy, "number too large for exact JSON");

    let mut addr = permit("1");
    addr["message"]["owner"] = json!("0x1111");
    refused(&addr.to_string(), "short address");

    let mut undeclared = mail();
    undeclared["types"]["Mail"][0]["type"] = json!("Sender");
    refused(&undeclared.to_string(), "undeclared struct type");

    let mut twice = mail();
    twice["types"]["Person"] =
        json!([{"name": "name", "type": "string"}, {"name": "name", "type": "address"}]);
    refused(&twice.to_string(), "field declared twice");

    let mut no_domain_type = permit("1");
    no_domain_type["types"]
        .as_object_mut()
        .unwrap()
        .remove("EIP712Domain");
    refused(&no_domain_type.to_string(), "no EIP712Domain");

    refused(&"x".repeat(1024 * 1024 + 1), "oversized");
}

// Found by mutation testing, 5 Oct 2026: type and field names must be
// identifiers, because they are spliced into the type string that is hashed.
// A name holding a comma, a space or a bracket could make two different
// structures hash alike, and nothing tested the rule.
#[test]
fn type_and_field_names_must_be_identifiers() {
    let renamed_type = |name: &str| {
        let mut m = mail();
        let fields = m["types"]["Mail"].take();
        m["types"].as_object_mut().unwrap().remove("Mail");
        m["types"][name] = fields;
        m["primaryType"] = json!(name);
        m.to_string()
    };
    let renamed_field = |name: &str| {
        let mut m = mail();
        m["types"]["Mail"][2]["name"] = json!(name);
        let contents = m["message"]["contents"].take();
        m["message"].as_object_mut().unwrap().remove("contents");
        m["message"][name] = contents;
        m.to_string()
    };
    for bad in ["1Mail", "Ma il", "Mail,x", "Mail(", "Mäil", "Mail-x"] {
        refused(&renamed_type(bad), &format!("type name {bad:?}"));
    }
    for bad in ["1x", "to x", "to,uint256 y", "a(b", "-x", "é"] {
        refused(&renamed_field(bad), &format!("field name {bad:?}"));
    }
    for good in ["_Mail", "$Mail", "Mail2", "M_$9"] {
        assert!(review(&renamed_type(good)).is_ok(), "type name {good:?}");
    }
    for good in ["_x", "$x", "x9", "a_$1"] {
        assert!(review(&renamed_field(good)).is_ok(), "field name {good:?}");
    }
}

// Found by mutation testing, 5 Oct 2026: only a request whose primary type is
// the domain itself may leave out its message.
#[test]
fn only_a_domain_only_request_may_leave_out_its_message() {
    let mut domain_only = permit("1");
    domain_only["primaryType"] = json!("EIP712Domain");
    domain_only.as_object_mut().unwrap().remove("message");
    let (r, h) = review(&domain_only.to_string()).unwrap();
    assert!(r.has("DOMAIN_ONLY_SIGNATURE"));
    assert_eq!(h.message_hash, None);

    let mut no_message = permit("1");
    no_message.as_object_mut().unwrap().remove("message");
    let e = review(&no_message.to_string()).err().unwrap();
    assert!(e.contains("has no message"), "{e}");
}

// Found by mutation testing, 5 Oct 2026: the reader's nesting limit, at its
// boundary. Values may nest to twice the type depth limit, and no deeper.
#[test]
fn values_nest_exactly_to_the_readers_limit() {
    let limit = 2 * clearsign::typed::MAX_DEPTH;
    let nested = |arrays: usize| {
        let mut m = mail();
        let mut v = json!(1);
        for _ in 0..arrays {
            v = json!([v]);
        }
        m["message"]["deep"] = v;
        m.to_string()
    };
    // The message is depth 0 and each array one more, so the innermost value
    // of `limit - 1` arrays sits exactly at the limit.
    assert!(parse(&nested(limit - 1)).is_ok());
    let e = parse(&nested(limit)).err().unwrap();
    assert!(e.contains("nested too deeply"), "{e}");
}

// Found by mutation testing, 5 Oct 2026: the size limit, at its boundary,
// with a request that is otherwise valid so only the limit can refuse it.
#[test]
fn a_request_of_exactly_the_size_limit_is_read_and_one_byte_more_is_not() {
    use clearsign_safe_json::MAX_RECORD_BYTES;
    let base = permit("1").to_string();
    let at_limit = format!("{base}{}", " ".repeat(MAX_RECORD_BYTES - base.len()));
    assert_eq!(at_limit.len(), MAX_RECORD_BYTES);
    assert!(review(&at_limit).is_ok());
    let e = review(&format!("{at_limit} ")).err().unwrap();
    assert!(e.contains("the limit is"), "{e}");
}

#[test]
fn hex_is_read_only_in_the_one_form_the_standard_writes() {
    // A second prefix, surrounding space, or nothing after the prefix: a
    // lenient reader hashes these as numbers another tool would refuse or
    // read differently, and the signer is shown a hash the wallet won't sign.
    for bad in ["0x0x01", "0x01 ", " 0x01", "0x", "0x+1", "0x-1", "0x 1"] {
        let mut p = permit("1");
        p["message"]["value"] = json!(bad);
        refused(&p.to_string(), &format!("uint {bad:?}"));
    }
    for bad in [
        "0x1111111111111111111111111111111111111111 ",
        " 0x1111111111111111111111111111111111111111",
        "0x0x11111111111111111111111111111111111111",
        "1111111111111111111111111111111111111111",
        "0x111111111111111111111111111111111111111g",
    ] {
        let mut p = permit("1");
        p["message"]["owner"] = json!(bad);
        refused(&p.to_string(), &format!("address {bad:?}"));
    }
    let blob = |data: &str| {
        json!({
            "types": {
                "EIP712Domain": [{"name": "chainId", "type": "uint256"}],
                "Blob": [{"name": "data", "type": "bytes"}, {"name": "tag", "type": "bytes4"}]
            },
            "primaryType": "Blob",
            "domain": {"chainId": 1},
            "message": {"data": data, "tag": "0x01020304"}
        })
        .to_string()
    };
    for bad in ["0x0x12", "0x12 ", " 0x12", "0x123", "12", "0x1g"] {
        refused(&blob(bad), &format!("bytes {bad:?}"));
    }
    // What the standard does write is still read: either case of prefix,
    // empty bytes, and an odd number of digits in a number.
    assert!(review(&blob("0x")).is_ok());
    assert!(review(&blob("0XaBcD")).is_ok());
    let mut p = permit("1");
    p["message"]["value"] = json!("0X00a");
    let (_, odd) = review(&p.to_string()).unwrap();
    p["message"]["value"] = json!("10");
    let (_, decimal) = review(&p.to_string()).unwrap();
    assert_eq!(odd, decimal);
}

// Found by mutation testing, 5 Oct 2026: each rule of the type grammar, one
// at a time. A type string is hashed as written, so a reader that accepts a
// spelling another tool reads differently is signing a different structure.
#[test]
fn every_rule_of_the_type_grammar_is_enforced() {
    let with_type = |ty: &str, value: Value| {
        let mut m = json!({
            "types": {"EIP712Domain": [{"name": "chainId", "type": "uint256"}]},
            "primaryType": "T",
            "domain": {"chainId": 1},
            "message": {"x": value}
        });
        m["types"]["T"] = json!([{"name": "x", "type": ty}]);
        m.to_string()
    };
    // Sizes: written without leading zeros, and only sizes that exist.
    for bad in [
        "uint08", "int08", "bytes01", "uint7", "int7", "uint264", "int264", "bytes33", "uint0",
        "int0", "bytes0", "uint", "int", "byte",
    ] {
        refused(&with_type(bad, json!("1")), bad);
    }
    for (good, value) in [
        ("uint8", json!("1")),
        ("uint256", json!("1")),
        ("int8", json!("-1")),
        ("int256", json!("-1")),
        ("bytes1", json!("0x01")),
        ("bytes32", json!(format!("0x{}", "00".repeat(32)))),
    ] {
        assert!(review(&with_type(good, value)).is_ok(), "{good}");
    }
    // Array lengths: digits only, no leading zero, none of length zero.
    for bad in [
        "uint8[0]",
        "uint8[01]",
        "uint8[1a]",
        "uint8[ 1]",
        "uint8[-1]",
        "uint8[",
        "uint8[1",
    ] {
        refused(&with_type(bad, json!(["1"])), bad);
    }
    // At most MAX_ARRAY elements, exactly.
    let max = clearsign::typed::MAX_ARRAY;
    assert!(review(&with_type(&format!("uint8[{max}]"), json!(vec!["1"; max]))).is_ok());
    refused(
        &with_type(&format!("uint8[{}]", max + 1), json!(vec!["1"; max + 1])),
        "array longer than the limit",
    );
    // A struct may not take the name of a built-in type, or of an alias, or
    // `uint value` would read like an integer in the string that is hashed.
    for name in ["uint", "int", "byte", "uint8", "address", "bytes32"] {
        let mut m = json!({
            "types": {"EIP712Domain": [{"name": "chainId", "type": "uint256"}]},
            "primaryType": "T",
            "domain": {"chainId": 1},
            "message": {"y": {"x": "1"}}
        });
        m["types"][name] = json!([{"name": "x", "type": "uint8"}]);
        m["types"]["T"] = json!([{"name": "y", "type": name}]);
        refused(&m.to_string(), &format!("a struct named {name}"));
    }
}

// Found by mutation testing, 5 Oct 2026: the reader's three structural limits,
// each at its boundary — types per request, fields per type, and how deep
// one struct may refer to the next.
#[test]
fn structural_limits_hold_exactly_at_their_boundaries() {
    use clearsign::typed::{MAX_ARRAY, MAX_DEPTH, MAX_FIELDS, MAX_TYPES};
    let base = || {
        json!({
            "types": {"EIP712Domain": [{"name": "chainId", "type": "uint256"}]},
            "primaryType": "T",
            "domain": {"chainId": 1},
            "message": {"v": "1"}
        })
    };

    // Types per request, counting EIP712Domain.
    let types = |n: usize| {
        let mut m = base();
        m["types"]["T"] = json!([{"name": "v", "type": "uint8"}]);
        for i in 0..n - 2 {
            m["types"][format!("U{i}")] = json!([{"name": "v", "type": "uint8"}]);
        }
        m.to_string()
    };
    assert!(review(&types(MAX_TYPES)).is_ok());
    refused(&types(MAX_TYPES + 1), "one type too many");

    // Fields per type.
    let fields = |n: usize| {
        let mut m = base();
        let list: Vec<Value> = (0..n)
            .map(|i| json!({"name": format!("f{i}"), "type": "uint8"}))
            .collect();
        m["types"]["T"] = Value::Array(list);
        m["message"] = Value::Object((0..n).map(|i| (format!("f{i}"), json!("1"))).collect());
        m.to_string()
    };
    assert!(review(&fields(MAX_FIELDS)).is_ok());
    refused(&fields(MAX_FIELDS + 1), "one field too many");

    // Elements of a dynamic array. (A fixed length over the limit is refused
    // earlier, when its type is read.)
    let dynamic = |n: usize| {
        let mut m = base();
        m["types"]["T"] = json!([{"name": "v", "type": "uint8[]"}]);
        m["message"] = json!({ "v": vec!["1"; n] });
        m.to_string()
    };
    assert!(review(&dynamic(MAX_ARRAY)).is_ok());
    let e = review(&dynamic(MAX_ARRAY + 1)).err().unwrap();
    assert!(e.contains("more than"), "{e}");

    // A chain S0 -> S1 -> ... of struct references, `links` long, declared
    // beside T. Unused, it meets only the limit on how deep types refer to
    // each other; used as the message, the limit on nested values comes first.
    let chain = |links: usize, used: bool| {
        let mut m = base();
        let name = |i: usize| format!("S{i}");
        for i in 0..links {
            m["types"][name(i)] = json!([{"name": "n", "type": name(i + 1)}]);
        }
        m["types"][name(links)] = json!([{"name": "v", "type": "uint8"}]);
        if used {
            m["primaryType"] = json!("S0");
            let mut v = json!({"v": "1"});
            for _ in 0..links {
                v = json!({ "n": v });
            }
            m["message"] = v;
        } else {
            m["types"]["T"] = json!([{"name": "v", "type": "uint8"}]);
        }
        m.to_string()
    };
    assert!(review(&chain(MAX_DEPTH, false)).is_ok());
    let e = review(&chain(MAX_DEPTH + 1, false)).err().unwrap();
    assert!(e.contains("types are nested more than"), "{e}");
    assert!(review(&chain(MAX_DEPTH - 1, true)).is_ok());
    let e = review(&chain(MAX_DEPTH, true)).err().unwrap();
    assert!(e.contains("values are nested more than"), "{e}");

    // The same limit as the hash counts it: each struct inside an array is two
    // levels down. Only a domain field reaches it on its own, because a domain
    // field is hashed in full but shown as "(structure)", while a message is
    // also walked for display, which stops first.
    let in_domain = |links: usize| {
        let mut m = base();
        m["types"]["T"] = json!([{"name": "v", "type": "uint8"}]);
        m["types"]["EIP712Domain"] = json!([
            {"name": "chainId", "type": "uint256"}, {"name": "x", "type": "S1[]"}
        ]);
        for i in 1..links {
            m["types"][format!("S{i}")] = json!([{"name": "n", "type": format!("S{}[]", i + 1)}]);
        }
        m["types"][format!("S{links}")] = json!([{"name": "v", "type": "uint8"}]);
        let mut v = json!({"v": "1"});
        for _ in 1..links {
            v = json!({ "n": [v] });
        }
        m["domain"]["x"] = json!([v]);
        m.to_string()
    };
    assert!(review(&in_domain(MAX_DEPTH / 2)).is_ok());
    let e = review(&in_domain(MAX_DEPTH / 2 + 1)).err().unwrap();
    assert!(e.contains("values are nested more than"), "{e}");
}

// Found by mutation testing, 5 Oct 2026: signed integers given as JSON
// numbers, which wallets receive as often as strings, were never tested; nor
// was the reason a negative number is refused for an unsigned type.
#[test]
fn signed_integers_may_be_json_numbers_and_unsigned_ones_may_not_be_negative() {
    let doc = |xs: Value| {
        json!({
            "types": {
                "EIP712Domain": [{"name": "chainId", "type": "uint256"}],
                "T": [{"name": "xs", "type": "int8[2]"}]
            },
            "primaryType": "T",
            "domain": {"chainId": 1},
            "message": {"xs": xs}
        })
        .to_string()
    };
    let (_, as_numbers) = review(&doc(json!([-128, 5]))).unwrap();
    let (_, as_strings) = review(&doc(json!(["-128", "5"]))).unwrap();
    assert_eq!(as_numbers, as_strings);
    refused(&doc(json!([-129, 5])), "int8 underflow as a number");

    let mut negative = permit("1");
    negative["message"]["nonce"] = json!(-1);
    let e = review(&negative.to_string()).err().unwrap();
    assert!(
        e.contains("negative number was given for an unsigned type"),
        "{e}"
    );
}

#[test]
fn fixed_arrays_and_signed_integers_are_checked() {
    let doc = |len_ok: bool, i: &str| -> String {
        json!({
            "types": {
                "EIP712Domain": [{"name": "chainId", "type": "uint256"}],
                "T": [{"name": "xs", "type": "int8[2]"}]
            },
            "primaryType": "T",
            "domain": {"chainId": 1},
            "message": {"xs": if len_ok { json!([i, "-1"]) } else { json!([i]) }}
        })
        .to_string()
    };
    assert!(review(&doc(true, "-128")).is_ok());
    assert!(review(&doc(true, "127")).is_ok());
    refused(&doc(true, "128"), "int8 overflow");
    refused(&doc(true, "-129"), "int8 underflow");
    refused(&doc(false, "1"), "fixed array of wrong length");
}

#[test]
fn deep_and_self_referential_types_are_bounded() {
    // A type that refers to itself through an array is legal EIP-712; data is
    // finite, so hashing ends. Nesting past the limit is refused, not followed.
    let mut node = json!({"kids": []});
    for _ in 0..40 {
        node = json!({"kids": [node]});
    }
    let doc = json!({
        "types": {"EIP712Domain": [{"name": "chainId", "type": "uint256"}], "Node": [{"name": "kids", "type": "Node[]"}]},
        "primaryType": "Node", "domain": {"chainId": 1}, "message": node
    });
    refused(&doc.to_string(), "nesting past the limit");
    let shallow = json!({
        "types": {"EIP712Domain": [{"name": "chainId", "type": "uint256"}], "Node": [{"name": "kids", "type": "Node[]"}]},
        "primaryType": "Node", "domain": {"chainId": 1}, "message": {"kids": [{"kids": []}]}
    });
    assert!(parse(&shallow.to_string()).is_ok());
    assert!(review(&shallow.to_string()).is_ok());
}

/// Found by reading the parser, not by fuzzing (whose inputs stop at 4 KiB): a
/// type with 200,000 array dimensions overflowed the stack and aborted the
/// process. It is refused now, before any of it is parsed.
#[test]
fn a_type_with_too_many_array_dimensions_is_refused_not_a_crash() {
    let deep = format!("uint8{}", "[]".repeat(200_000));
    let doc = json!({
        "types": {"EIP712Domain": [{"name": "chainId", "type": "uint256"}], "T": [{"name": "x", "type": deep}]},
        "primaryType": "T", "domain": {"chainId": 1}, "message": {"x": []}
    });
    let e = review(&doc.to_string()).err().unwrap();
    assert!(e.contains("array dimensions"), "{e}");

    let ok = json!({
        "types": {"EIP712Domain": [{"name": "chainId", "type": "uint256"}], "T": [{"name": "x", "type": "uint8[][][][][][][][]"}]},
        "primaryType": "T", "domain": {"chainId": 1}, "message": {"x": []}
    });
    assert!(
        review(&ok.to_string()).is_ok(),
        "eight dimensions are still accepted"
    );
}
