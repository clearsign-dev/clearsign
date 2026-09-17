//! End-to-end tests of the built binary. Reference signatures come from Foundry `cast`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::io::Write;
use std::process::{Command, Output, Stdio};

const BIN: &str = env!("CARGO_BIN_EXE_clearsign");
const MNEMONIC: &str = "test test test test test test test test test test test junk";
const GUARD: (&str, &str) = (
    "CLEARSIGN_DEV_ONLY",
    "I_UNDERSTAND_THIS_COMPUTER_IS_NOT_A_SIGNING_DEVICE",
);

fn safe_args() -> Vec<&'static str> {
    vec![
        "--chain-id",
        "1",
        "--safe",
        "0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4",
        "--to",
        "0x00000000000000000000000000000000DeaDBeef",
        "--nonce",
        "71",
        "--operation",
        "1",
        "--safe-tx-gas",
        "45746",
        "--data",
        "0xa9059cbb000000000000000000000000000000000000000000000000000000000000dead0000000000000000000000000000000000000000000000000000000000000000",
    ]
}

fn run(args: &[&str], stdin: &str, guard: bool) -> Output {
    let mut cmd = Command::new(BIN);
    cmd.args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped());
    cmd.env_remove(GUARD.0);
    if guard {
        cmd.env(GUARD.0, GUARD.1);
    }
    let mut child = cmd.spawn().unwrap();
    // A broken pipe here is not a failure: it means the binary exited before it
    // read standard input. That is exactly what should happen when signing is
    // refused — the recovery phrase is never read at all.
    if let Some(mut pipe) = child.stdin.take() {
        match pipe.write_all(stdin.as_bytes()) {
            Ok(()) => {}
            Err(e) if e.kind() == std::io::ErrorKind::BrokenPipe => {}
            Err(e) => panic!("writing to the binary's stdin failed: {e}"),
        }
    }
    child.wait_with_output().unwrap()
}

fn text(b: &[u8]) -> String {
    String::from_utf8_lossy(b).into_owned()
}

#[test]
fn review_exit_code_reflects_critical() {
    let mut args = vec!["safe-tx"];
    args.extend(safe_args());
    let out = run(&args, "", false);
    assert_eq!(out.status.code(), Some(3));
    assert!(text(&out.stdout).contains("SAFE_DELEGATECALL"));
}

#[test]
fn signing_requires_dev_guard() {
    let mut args = vec!["sign-safe-tx"];
    args.extend(safe_args());
    args.extend(["--ack", "1:SAFE_DELEGATECALL"]);
    let out = run(&args, MNEMONIC, false);
    assert_eq!(out.status.code(), Some(1));
    assert!(text(&out.stderr).contains("development-only"));
}

#[test]
fn signing_refused_without_exact_acknowledgement() {
    let mut args = vec!["sign-safe-tx"];
    args.extend(safe_args());
    let out = run(&args, MNEMONIC, true);
    assert_eq!(out.status.code(), Some(1));
    assert!(text(&out.stderr).contains("SAFE_DELEGATECALL must be explicitly acknowledged"));
    assert!(!text(&out.stdout).contains("-- Signature --"));

    // An extra acknowledgement that names a finding the review does not have.
    args.extend([
        "--ack",
        "1:SAFE_DELEGATECALL",
        "--ack",
        "2:UNLIMITED_APPROVAL",
    ]);
    let out = run(&args, MNEMONIC, true);
    assert_eq!(out.status.code(), Some(1));
    assert!(text(&out.stderr).contains("must match exactly"));
}

#[test]
fn signing_with_acknowledgement_matches_cast() {
    let mut args = vec!["sign-safe-tx"];
    args.extend(safe_args());
    args.extend(["--ack", "1:SAFE_DELEGATECALL"]);
    let out = run(&args, MNEMONIC, true);
    assert_eq!(out.status.code(), Some(0), "{}", text(&out.stderr));
    // cast wallet sign --no-hash --mnemonic "test ... junk" 0xa62b640d...df2d
    assert!(text(&out.stdout).contains(
        "0xe9c8072f00bd0fa236bc009817d280e719dd5b1b8e646004297738f73a17a76b1ec4afe9354f59b9fca477e40ab8b8a9c9e6937b6bda612cb63208a72eace3a21b"
    ));
}

#[test]
fn seed_from_dice_matches_cast() {
    let rolls = "314153265352313323246264332321356222413116333331516522631434453236121646622626233262263422534211166";
    let out = run(&["seed-from-dice"], rolls, true);
    assert_eq!(out.status.code(), Some(0));
    assert_eq!(
        text(&out.stdout).trim(),
        "goat lyrics ripple sunset gasp pair nothing advance shadow follow cave chief fade useful park manual whisper brick face medal cannon fragile feature unknown"
    );
}

// ---------------------------------------------------------------------------
// QR transport. The ur: strings below were produced by Keystone's reference
// implementation under Node; the signature is the one Foundry `cast wallet sign
// --no-hash` produces for the same digest. See tests/vectors/README.md.
// ---------------------------------------------------------------------------

/// The whole request in one QR code, for account 0 of the test phrase.
const QR_SINGLE: &str = "ur:eth-sign-request/osadtpdagdndcawmgtfrkigrpmndutdnbtkgfssbjnaohdjoaoyajnaddrlrfrnysgaelpamztcnpsaelfgmaymwnbroinmeswclluensettntgedmnnpftoenamwmfdlarofyptahnsrkaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeuepmrnwsaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaebsfwfzrtaxaaaaadahtaaddyoeadlecsdwykcsfnykaeykaewkaewkaocywzfhnetdamghwfnetbvwcypmloynwktoimrolfjpkktkzmrhcpiyatisjnihjyhsjnhsjkjevtkbpttk";

/// The same request animated across five QR codes.
const QR_PARTS: [&str; 5] = [
    "ur:eth-sign-request/1-5/lpadahcsskcyvtkbpttkhddeosadtpdagdndcawmgtfrkigrpmndutdnbtkgfssbjnaohdjoaoyajnaddrlrfrnysgaelpamztcnpsaehsclhykg",
    "ur:eth-sign-request/2-5/lpaoahcsskcyvtkbpttkhddelfgmaymwnbroinmeswclluensettntgedmnnpftoenamwmfdlarofyptahnsrkaeaeaeaeaeaeaeaeaentrodnaa",
    "ur:eth-sign-request/3-5/lpaxahcsskcyvtkbpttkhddeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeuepmrnwsaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaeaecehplfrs",
    "ur:eth-sign-request/4-5/lpaaahcsskcyvtkbpttkhddeaeaeaeaeaeaeaeaeaeaeaeaebsfwfzrtaxaaaaadahtaaddyoeadlecsdwykcsfnykaeykaewkaewkaojywshyko",
    "ur:eth-sign-request/5-5/lpahahcsskcyvtkbpttkhddecywzfhnetdamghwfnetbvwcypmloynwktoimrolfjpkktkzmrhcpiyatisjnihjyhsjnhsjkjeaeaeaezeztcyeo",
];

/// `cast wallet sign --no-hash` over the transaction's signing hash.
const EXPECTED_SIGNATURE: &str = "0x6c41afa9f38028749bb734ae6817c158ae298a636cf9ca6344664a196524895c0095065b752486f9d282f810a4abca5973b944ce29aff5fa164ce8cd115da0bd1b";

fn write_codes(lines: &[&str]) -> std::path::PathBuf {
    // Tests run in parallel in one process, so the file name must be unique per
    // call. Naming it after the number of codes meant three tests shared one
    // path and raced: the file was being rewritten while another test's binary
    // was reading it.
    static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let n = NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let dir = std::env::temp_dir().join(format!("clearsign-qr-test-{}", std::process::id()));
    std::fs::create_dir_all(&dir).unwrap();
    let path = dir.join(format!("codes-{n}.txt"));
    std::fs::write(&path, lines.join("\n")).unwrap();
    path
}

#[test]
fn qr_review_reads_an_animated_request() {
    let path = write_codes(&QR_PARTS);
    let out = run(&["qr-review", path.to_str().unwrap()], "", false);
    let stdout = text(&out.stdout);
    assert_eq!(out.status.code(), Some(0), "{}", text(&out.stderr));
    assert!(
        stdout.contains("Requested by ..................... metamask"),
        "{stdout}"
    );
    assert!(stdout.contains("m/44'/60'/0'/0/0"), "{stdout}");
    assert!(stdout.contains("ERC-20 transfer"), "{stdout}");
}

#[test]
fn qr_review_refuses_an_incomplete_request() {
    // Two frames of a five-frame animation: not enough to reconstruct anything.
    let path = write_codes(&QR_PARTS[..2]);
    let out = run(&["qr-review", path.to_str().unwrap()], "", false);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        text(&out.stderr).contains("incomplete"),
        "{}",
        text(&out.stderr)
    );
}

#[test]
fn qr_sign_matches_cast_and_answers_with_an_eth_signature() {
    let path = write_codes(&[QR_SINGLE]);
    let out = run(&["qr-sign", path.to_str().unwrap()], MNEMONIC, true);
    let stdout = text(&out.stdout);
    assert_eq!(out.status.code(), Some(0), "{}", text(&out.stderr));
    assert!(stdout.contains("ur:eth-signature/"), "{stdout}");
    // The reply carries exactly the signature cast produces for this digest.
    let body = stdout
        .lines()
        .find(|l| l.starts_with("ur:eth-signature/"))
        .unwrap();
    let message = clearsign_qr_decode(body);
    let hex: String = message.iter().map(|b| format!("{b:02x}")).collect();
    assert!(
        hex.contains(EXPECTED_SIGNATURE.trim_start_matches("0x")),
        "signature in the reply does not match cast: {hex}"
    );
}

#[test]
fn qr_sign_refuses_when_the_request_names_a_different_signer() {
    // The request expects account 0 of the test phrase; sign with a different
    // phrase and the device must refuse rather than hand back a useless signature.
    let path = write_codes(&[QR_SINGLE]);
    let other = "legal winner thank year wave sausage worth useful legal winner thank yellow";
    let out = run(&["qr-sign", path.to_str().unwrap()], other, true);
    assert_eq!(out.status.code(), Some(1));
    assert!(
        text(&out.stderr).contains("expects signer"),
        "{}",
        text(&out.stderr)
    );
}

#[test]
fn qr_sign_requires_the_dev_guard() {
    let path = write_codes(&[QR_SINGLE]);
    let out = run(&["qr-sign", path.to_str().unwrap()], MNEMONIC, false);
    assert_eq!(out.status.code(), Some(1));
    assert!(text(&out.stderr).contains("development-only"));
}

/// Decode a `ur:` string with the crate under test, for assertions only.
fn clearsign_qr_decode(s: &str) -> Vec<u8> {
    let mut d = clearsign_qr::Decoder::new();
    d.receive(s).unwrap().unwrap().to_vec()
}
