//! Reading a Safe transaction from the JSON a signer actually has.
//!
//! The fixture is not synthetic: it is the transaction that took about $1.5
//! billion out of Bybit on 21 February 2025, fetched from Safe's own production
//! Transaction Service. See `tests/fixtures/README.md`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::io::Write;
use std::process::{Command, Output, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_clearsign");
const BYBIT: &str = "crates/clearsign-cli/tests/fixtures/bybit-safe-tx.json";
/// What Safe's own service says the hash of that transaction is.
const SAFE_SERVICE_HASH: &str =
    "0xb3476d061aeb8fc1d605a873c483a2402d88a68a9cdd1a8b47655dd55ba004f8";

fn run(args: &[&str], stdin: &str) -> Output {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(|p| p.parent())
        .unwrap()
        .to_path_buf();
    let mut child = Command::new(BIN)
        .args(args)
        .current_dir(&root)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
    if let Some(mut pipe) = child.stdin.take() {
        let _ = pipe.write_all(stdin.as_bytes());
    }
    child.wait_with_output().unwrap()
}

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

#[test]
fn the_real_bybit_transaction_is_refused_and_hashes_the_same_as_safes_own_service() {
    // The thing this project exists for, on the transaction it exists because of.
    let out = run(&["safe-json", BYBIT, "--chain-id", "1"], "");
    let stdout = text(&out.stdout);

    // Our hash equals the one Safe's Transaction Service published for it. If
    // these ever diverge, either we are wrong or the service is, and a signer
    // comparing hashes would be comparing nothing.
    assert!(
        stdout.contains(SAFE_SERVICE_HASH),
        "hash disagrees with Safe's own service:\n{stdout}"
    );
    // That Safe is on the older domain, which the tool works out rather than
    // assuming — assuming v1.3.0+ here produces a hash nobody will ever see.
    assert!(stdout.contains("v1.1.x"), "{stdout}");
    // And the verdict.
    assert_eq!(
        out.status.code(),
        Some(3),
        "should exit CRITICAL:\n{stdout}"
    );
    assert!(stdout.contains("SAFE_DELEGATECALL"), "{stdout}");
    assert!(stdout.contains("DO NOT SIGN"), "{stdout}");
    // The calldata looks like an ordinary transfer. It must not be described as one.
    assert!(
        !stdout.contains("ERC-20 transfer"),
        "the fake transfer must not be decoded as a transfer:\n{stdout}"
    );
}

#[test]
fn a_file_whose_hash_does_not_match_its_fields_is_refused() {
    // The case that catches a preparing computer that is lying: the fields say
    // one thing, the hash says another.
    let original = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .and_then(|p| p.parent())
            .unwrap()
            .join(BYBIT),
    )
    .unwrap();
    let tampered = original.replace("\"nonce\": 71", "\"nonce\": 72");
    assert_ne!(
        tampered, original,
        "the fixture should have a nonce to change"
    );

    let out = run(&["safe-json", "-", "--chain-id", "1"], &tampered);
    assert_eq!(out.status.code(), Some(1));
    let err = text(&out.stderr);
    assert!(
        err.contains("not the hash of the fields"),
        "should refuse a file whose hash does not match:\n{err}"
    );
}

#[test]
fn a_file_with_no_hash_will_not_guess_the_safe_version() {
    // Without a hash there is nothing to identify the domain, and the domain
    // changes what gets signed. Guessing is the one thing not allowed.
    let json = r#"{"safe":"0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4",
        "to":"0x96221423681A6d52E184D440a8eFCEbB105C7242","value":"0","data":"0x",
        "operation":0,"safeTxGas":0,"baseGas":0,"gasPrice":"0",
        "gasToken":"0x0000000000000000000000000000000000000000",
        "refundReceiver":"0x0000000000000000000000000000000000000000","nonce":1}"#;
    let out = run(&["safe-json", "-", "--chain-id", "1"], json);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        text(&out.stderr).contains("--safe-version"),
        "{}",
        text(&out.stderr)
    );

    // Said explicitly, it works.
    let out = run(
        &[
            "safe-json",
            "-",
            "--chain-id",
            "1",
            "--safe-version",
            "1.3.0+",
        ],
        json,
    );
    assert_eq!(out.status.code(), Some(0), "{}", text(&out.stderr));
}

#[test]
fn the_chain_id_is_never_guessed() {
    let json = r#"{"safe":"0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4",
        "to":"0x96221423681A6d52E184D440a8eFCEbB105C7242","value":"0","data":"0x",
        "operation":0,"safeTxGas":0,"baseGas":0,"gasPrice":"0",
        "gasToken":"0x0000000000000000000000000000000000000000",
        "refundReceiver":"0x0000000000000000000000000000000000000000","nonce":1}"#;
    let out = run(&["safe-json", "-"], json);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        text(&out.stderr).contains("chain ID"),
        "{}",
        text(&out.stderr)
    );
}

#[test]
fn a_chain_id_in_the_file_that_contradicts_the_flag_is_refused() {
    let json = r#"{"chainId":1,"safe":"0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4",
        "to":"0x96221423681A6d52E184D440a8eFCEbB105C7242","value":"0","data":"0x",
        "operation":0,"safeTxGas":0,"baseGas":0,"gasPrice":"0",
        "gasToken":"0x0000000000000000000000000000000000000000",
        "refundReceiver":"0x0000000000000000000000000000000000000000","nonce":1}"#;
    let out = run(&["safe-json", "-", "--chain-id", "137"], json);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        text(&out.stderr).contains("will not be guessed"),
        "{}",
        text(&out.stderr)
    );
}

#[test]
fn numbers_may_arrive_as_numbers_or_as_strings() {
    // Safe's service uses both, sometimes for the same field on different
    // endpoints. The fixture has nonce as a number and value as a string.
    let out = run(&["safe-json", BYBIT, "--chain-id", "1"], "");
    let stdout = text(&out.stdout);
    assert!(
        stdout.contains("Safe nonce ....................... 71"),
        "{stdout}"
    );
}
