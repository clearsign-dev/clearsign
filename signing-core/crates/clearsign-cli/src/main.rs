//! `clearsign` command-line tool.
//!
//! Reads only its arguments and standard input. Opens no files and no network.
//! Exit codes are meant for scripts:
//!
//! | code | meaning |
//! |---|---|
//! | 0 | success; for reviews, highest finding is INFO or WARNING |
//! | 1 | input error, or signing refused |
//! | 2 | review contains a BLIND finding |
//! | 3 | review contains a CRITICAL finding |

use std::io::Read;
use std::process::ExitCode;

use clearsign::address::Address;
use clearsign::{DomainVersion, Review, SafeTransaction, Severity, U256, hex};
use clearsign_keys::{DiceRolls, Wallet, approve, mnemonic_from_dice};
use clearsign_qr::ur::encode_single;
use clearsign_qr::{
    DataType, Decoder as QrDecoder, SignRequest, decode_sign_request, encode_signature,
};
use qrcodegen::{QrCode, QrCodeEcc};
use zeroize::Zeroizing;

const DEV_GUARD_VAR: &str = "CLEARSIGN_DEV_ONLY";
const DEV_GUARD_VALUE: &str = "I_UNDERSTAND_THIS_COMPUTER_IS_NOT_A_SIGNING_DEVICE";

const USAGE: &str = "\
clearsign — decode what you are about to sign, from the signed bytes alone.

REVIEW (safe on any computer):
  clearsign qr-review [FILE | -]
      Read an EIP-4527 eth-sign-request as ur: strings, one per line, scanned
      from a wallet's animated QR code, and review the transaction inside it.

  clearsign tx <HEX | ->
      Review an unsigned EVM transaction (EIP-1559 or legacy). Use - to read hex from stdin.

  clearsign safe-tx --chain-id <N> --safe <ADDR> --to <ADDR> --nonce <N>
                    [--value <N>] [--data <HEX>] [--operation <0|1>]
                    [--safe-tx-gas <N>] [--base-gas <N>] [--gas-price <N>]
                    [--gas-token <ADDR>] [--refund-receiver <ADDR>] [--legacy-domain]
      Review a Safe transaction an owner is asked to sign, and compute its hash.

SIGNING AND SEEDS (development only, see below):
  clearsign sign-tx <HEX> [--account <I>] [--ack <N:CODE>]...
  clearsign sign-safe-tx <safe-tx flags> [--account <I>] [--ack <N:CODE>]...
      Review, then sign only if every BLIND and CRITICAL finding is acknowledged
      individually, by the number and code shown beside it, exactly. Two findings
      that share a code take two acknowledgements. The recovery phrase is read
      from stdin, first line; an optional passphrase from the second line.

  clearsign qr-sign <FILE> [--ack <N:CODE>]...
      Read an EIP-4527 eth-sign-request from FILE (the scanned ur: strings, one
      per line), review it, sign with the key path the request asks for, and
      print the reply as an eth-signature UR and a QR code. The recovery phrase
      is read from stdin, as always.

  clearsign seed-from-dice
      Read at least 99 dice rolls (digits 1-6) from stdin and print a 24-word phrase.

  Signing and seed commands refuse to run unless the environment variable
  CLEARSIGN_DEV_ONLY is set to I_UNDERSTAND_THIS_COMPUTER_IS_NOT_A_SIGNING_DEVICE.
  A recovery phrase typed into an internet-connected computer must be treated as exposed.

EXIT CODES: 0 ok, 1 error or refused, 2 BLIND finding present, 3 CRITICAL finding present.
";

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let rest = args.get(1..).unwrap_or(&[]);
    let outcome = match args.first().map(String::as_str) {
        Some("tx") => review_tx(rest).map(Outcome::Review),
        Some("safe-tx") => parse_safe_tx(rest)
            .map(|(tx, v)| Outcome::Review(clearsign::review_safe_transaction(&tx, v))),
        Some("sign-tx") => dev_guard().and_then(|()| sign_tx(rest)),
        Some("sign-safe-tx") => dev_guard().and_then(|()| sign_safe_tx(rest)),
        Some("seed-from-dice") => dev_guard().and_then(|()| seed_from_dice()),
        Some("qr-review") => qr_review(rest),
        Some("qr-sign") => dev_guard().and_then(|()| qr_sign(rest)),
        Some("-h" | "--help") | None => {
            print!("{USAGE}");
            return ExitCode::SUCCESS;
        }
        Some(other) => Err(format!("unknown command '{other}'")),
    };
    match outcome {
        Ok(Outcome::Review(review)) => {
            print!("{}", review.render());
            match review.highest_severity() {
                Some(Severity::Critical) => ExitCode::from(3),
                Some(Severity::Blind) => ExitCode::from(2),
                _ => ExitCode::SUCCESS,
            }
        }
        Ok(Outcome::Done) => ExitCode::SUCCESS,
        Err(msg) => {
            eprintln!("error: {msg}\n\nRun `clearsign --help` for usage.");
            ExitCode::from(1)
        }
    }
}

enum Outcome {
    Review(Review),
    Done,
}

fn dev_guard() -> Result<(), String> {
    match std::env::var(DEV_GUARD_VAR) {
        Ok(v) if v == DEV_GUARD_VALUE => {
            eprintln!(
                "WARNING: development mode. Anything typed into this computer, including a recovery \
                 phrase, may be exposed to malware on it."
            );
            Ok(())
        }
        _ => Err(format!(
            "signing and seed commands are development-only. Set {DEV_GUARD_VAR}={DEV_GUARD_VALUE} to proceed."
        )),
    }
}

fn review_tx(args: &[String]) -> Result<Review, String> {
    let input: String = match args {
        [one] if one == "-" => read_stdin()?.as_str().to_owned(),
        [one] => one.clone(),
        _ => return Err(String::from("tx takes exactly one argument")),
    };
    let bytes = hex::decode(&input).map_err(|e| e.to_string())?;
    clearsign::review_transaction_bytes(&bytes).map_err(|e| e.to_string())
}

struct SignFlags {
    account: u32,
    acks: Vec<String>,
}

/// Parse `--ack 2:UNLIMITED_APPROVAL` into the pair the library wants.
///
/// The number is the one printed beside the finding, so acknowledging two
/// findings that share a code takes two different acknowledgements.
fn parse_acks(acks: &[String]) -> Result<Vec<(u16, &str)>, String> {
    let mut out = Vec::with_capacity(acks.len());
    for ack in acks {
        let (n, code) = ack.split_once(':').ok_or_else(|| {
            format!("--ack {ack:?} should be the number and code shown in the review, like 2:UNLIMITED_APPROVAL")
        })?;
        let n: u16 = n.trim().parse().map_err(|_| {
            format!("--ack {ack:?} does not start with the number shown beside the finding")
        })?;
        out.push((n, code.trim()));
    }
    Ok(out)
}

/// Pull `--account` and repeated `--ack` out of the arguments; return the rest.
fn split_sign_flags(args: &[String]) -> Result<(SignFlags, Vec<String>), String> {
    let mut flags = SignFlags {
        account: 0,
        acks: Vec::new(),
    };
    let mut rest = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--account" => {
                let v = it.next().ok_or("--account needs a value")?;
                flags.account = v
                    .parse()
                    .map_err(|_| "--account: expected a non-negative integer")?;
            }
            "--ack" => flags
                .acks
                .push(it.next().ok_or("--ack needs a finding code")?.clone()),
            _ => rest.push(a.clone()),
        }
    }
    Ok((flags, rest))
}

fn sign_tx(args: &[String]) -> Result<Outcome, String> {
    let (flags, rest) = split_sign_flags(args)?;
    let [hex_in] = rest.as_slice() else {
        return Err(String::from(
            "sign-tx takes exactly one transaction hex argument",
        ));
    };
    let bytes = hex::decode(hex_in).map_err(|e| e.to_string())?;
    let review = clearsign::review_transaction_bytes(&bytes).map_err(|e| e.to_string())?;
    sign_reviewed(&review, &flags)
}

fn sign_safe_tx(args: &[String]) -> Result<Outcome, String> {
    let (flags, rest) = split_sign_flags(args)?;
    let (tx, version) = parse_safe_tx(&rest)?;
    let review = clearsign::review_safe_transaction(&tx, version);
    sign_reviewed(&review, &flags)
}

fn sign_reviewed(review: &Review, flags: &SignFlags) -> Result<Outcome, String> {
    print!("{}", review.render());
    let acks = parse_acks(&flags.acks)?;
    let approval = approve(review, &acks).map_err(|e| format!("signing refused: {e}"))?;

    let secrets = read_stdin()?;
    let mut lines = secrets.lines();
    let phrase = lines
        .next()
        .ok_or("expected a recovery phrase on the first line of stdin")?;
    let passphrase = lines.next().unwrap_or("");
    let wallet = Wallet::from_mnemonic(phrase, passphrase).map_err(|e| e.to_string())?;
    let account = wallet
        .ethereum_account(flags.account)
        .map_err(|e| e.to_string())?;
    let sig = account.sign(&approval).map_err(|e| e.to_string())?;

    println!("\n-- Signature --");
    println!(
        "Signer ........................... {}",
        clearsign::address::checksummed(&account.address())
    );
    println!(
        "Digest signed .................... {}",
        hex::encode_prefixed(&approval.digest())
    );
    println!(
        "r ................................ 0x{}",
        hex::encode(&sig.r)
    );
    println!(
        "s ................................ 0x{}",
        hex::encode(&sig.s)
    );
    println!(
        "v / y_parity ..................... {}",
        sig.v().map_err(|e| e.to_string())?
    );
    println!(
        "r || s || v (65 bytes) ........... {}",
        hex::encode_prefixed(&sig.to_rsv65())
    );
    Ok(Outcome::Done)
}

fn seed_from_dice() -> Result<Outcome, String> {
    let input = read_stdin()?;
    let rolls = DiceRolls::parse(&input).map_err(|e| e.to_string())?;
    let phrase = mnemonic_from_dice(&rolls).map_err(|e| e.to_string())?;
    println!("{}", phrase.as_str());
    eprintln!(
        "{} rolls. Verify independently: printf '%s' \"<rolls without spaces>\" | shasum -a 256, \
         then convert those 32 bytes to BIP-39 with a separate tool.",
        rolls.count()
    );
    Ok(Outcome::Done)
}

fn read_stdin() -> Result<Zeroizing<String>, String> {
    let mut s = Zeroizing::new(String::new());
    std::io::stdin()
        .read_to_string(&mut s)
        .map_err(|e| format!("reading stdin: {e}"))?;
    Ok(s)
}

fn parse_safe_tx(args: &[String]) -> Result<(SafeTransaction, DomainVersion), String> {
    let mut chain_id = None;
    let mut safe = None;
    let mut to = None;
    let mut nonce = None;
    let mut value = U256::ZERO;
    let mut data = Vec::new();
    let mut operation = 0u8;
    let mut safe_tx_gas = U256::ZERO;
    let mut base_gas = U256::ZERO;
    let mut gas_price = U256::ZERO;
    let mut gas_token = [0u8; 20];
    let mut refund_receiver = [0u8; 20];
    let mut version = DomainVersion::V1_3Plus;

    let mut it = args.iter();
    while let Some(flag) = it.next() {
        if flag == "--legacy-domain" {
            version = DomainVersion::Legacy;
            continue;
        }
        let val = it.next().ok_or_else(|| format!("{flag} needs a value"))?;
        match flag.as_str() {
            "--chain-id" => chain_id = Some(dec(flag, val)?),
            "--safe" => safe = Some(addr(flag, val)?),
            "--to" => to = Some(addr(flag, val)?),
            "--nonce" => nonce = Some(dec(flag, val)?),
            "--value" => value = dec(flag, val)?,
            "--data" => data = hex::decode(val).map_err(|e| format!("{flag}: {e}"))?,
            "--operation" => {
                operation = val
                    .parse::<u8>()
                    .map_err(|_| format!("{flag}: expected 0 or 1"))?;
            }
            "--safe-tx-gas" => safe_tx_gas = dec(flag, val)?,
            "--base-gas" => base_gas = dec(flag, val)?,
            "--gas-price" => gas_price = dec(flag, val)?,
            "--gas-token" => gas_token = addr(flag, val)?,
            "--refund-receiver" => refund_receiver = addr(flag, val)?,
            other => return Err(format!("unknown flag '{other}'")),
        }
    }

    let tx = SafeTransaction {
        chain_id: chain_id.ok_or("--chain-id is required")?,
        safe: safe.ok_or("--safe is required")?,
        to: to.ok_or("--to is required")?,
        value,
        data,
        operation,
        safe_tx_gas,
        base_gas,
        gas_price,
        gas_token,
        refund_receiver,
        nonce: nonce.ok_or("--nonce is required")?,
    };
    Ok((tx, version))
}

fn dec(flag: &str, v: &str) -> Result<U256, String> {
    U256::from_decimal(v)
        .map_err(|_| format!("{flag}: expected a non-negative decimal integer below 2^256"))
}

fn addr(flag: &str, v: &str) -> Result<Address, String> {
    let bytes = hex::decode(v).map_err(|e| format!("{flag}: {e}"))?;
    <[u8; 20]>::try_from(bytes.as_slice()).map_err(|_| format!("{flag}: address must be 20 bytes"))
}

// ---------------------------------------------------------------------------
// QR transport (EIP-4527 over Uniform Resources)
// ---------------------------------------------------------------------------

/// Read `ur:` strings, one per line, until a message completes.
///
/// The codes are not secret, so they come from a file or from stdin. The
/// recovery phrase is never read from either of those places at the same time:
/// it stays on stdin, and never in an argument.
fn read_sign_request(source: &Source) -> Result<(SignRequest, Vec<u8>), String> {
    let input = match source {
        Source::Stdin => read_stdin()?.as_str().to_owned(),
        Source::File(path) => {
            std::fs::read_to_string(path).map_err(|e| format!("cannot read {path}: {e}"))?
        }
    };
    let mut decoder = QrDecoder::new();
    let mut scanned = 0usize;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        scanned = scanned.saturating_add(1);
        match decoder.receive(line) {
            Ok(Some(message)) => {
                let message = message.to_vec();
                let request =
                    decode_sign_request(&message).map_err(|e| format!("request {scanned}: {e}"))?;
                eprintln!("read {scanned} QR code(s)");
                return Ok((request, message));
            }
            Ok(None) => {
                let (have, total) = decoder.progress();
                eprintln!("fragment {have}/{total}");
            }
            Err(e) => return Err(format!("QR code {scanned}: {e}")),
        }
    }
    let (have, total) = decoder.progress();
    Err(format!(
        "the request is incomplete: {have} of {total} fragments after {scanned} QR code(s)"
    ))
}

/// Everything about the request except the transaction itself.
fn print_request_context(request: &SignRequest) {
    println!("-- Signing request (from the QR codes) --");
    println!(
        "Requested by ..................... {}",
        request.origin.as_deref().unwrap_or("(not stated)")
    );
    println!(
        "Content .......................... {}",
        request.data_type.label()
    );
    println!(
        "Key path ......................... {}",
        request.path_string()
    );
    match request.address {
        Some(a) => println!(
            "Expected signer .................. {}",
            clearsign::address::display(&a)
        ),
        None => println!("Expected signer .................. (not stated)"),
    }
    match request.chain_id {
        Some(id) => println!("Chain ID stated by the wallet .... {id}"),
        None => println!("Chain ID stated by the wallet .... (not stated)"),
    }
    println!();
}

/// The transaction review for a request, refusing anything not in v1 scope.
fn review_request(request: &SignRequest) -> Result<Review, String> {
    match request.data_type {
        DataType::Transaction | DataType::TypedTransaction => {
            clearsign::review_transaction_bytes(&request.sign_data).map_err(|e| e.to_string())
        }
        other => Err(format!(
            "this request asks for a signature over {}, which this signer does not decode, \
             so it will not sign it",
            other.label()
        )),
    }
}

/// The account index in `m/44'/60'/0'/0/i`, or an error for any other path.
///
/// A signer that ignores the requested path can produce a valid signature from
/// the wrong key, which the wallet then cannot use and cannot explain.
fn account_index_for_path(request: &SignRequest) -> Result<u32, String> {
    let expected = [(44u32, true), (60, true), (0, true), (0, false)];
    let components = request.path.as_slice();
    let (last, head) = components
        .split_last()
        .ok_or("the request has no derivation path")?;
    let matches_head = head.len() == expected.len()
        && head
            .iter()
            .zip(expected.iter())
            .all(|(c, (i, h))| c.index == *i && c.hardened == *h);
    if !matches_head || last.hardened {
        return Err(format!(
            "the request asks for key path {}, which this signer does not derive \
             (it holds m/44'/60'/0'/0/i)",
            request.path_string()
        ));
    }
    Ok(last.index)
}

/// Where the scanned QR codes come from.
enum Source {
    Stdin,
    File(String),
}

fn qr_review(args: &[String]) -> Result<Outcome, String> {
    let source = match args {
        [] => Source::Stdin,
        [one] if one == "-" => Source::Stdin,
        [one] => Source::File(one.clone()),
        _ => return Err(String::from("qr-review takes at most one file argument")),
    };
    let (request, _) = read_sign_request(&source)?;
    print_request_context(&request);
    review_request(&request).map(Outcome::Review)
}

fn qr_sign(args: &[String]) -> Result<Outcome, String> {
    let (flags, rest) = split_sign_flags(args)?;
    let source = match rest.as_slice() {
        [one] => Source::File(one.clone()),
        _ => {
            return Err(String::from(
                "qr-sign takes the file of scanned ur: codes as its only argument; \
                 the recovery phrase is read from stdin",
            ));
        }
    };
    let (request, _) = read_sign_request(&source)?;
    print_request_context(&request);
    let review = review_request(&request)?;
    print!("{}", review.render());

    let acks = parse_acks(&flags.acks)?;
    let approval = approve(&review, &acks).map_err(|e| format!("signing refused: {e}"))?;

    let index = account_index_for_path(&request)?;
    if flags.account != 0 && flags.account != index {
        return Err(format!(
            "--account {} disagrees with the key path the request asks for ({})",
            flags.account,
            request.path_string()
        ));
    }

    let secrets = read_stdin()?;
    let mut lines = secrets.lines();
    let phrase = lines
        .next()
        .ok_or("expected a recovery phrase on the first line of the phrase input")?;
    let passphrase = lines.next().unwrap_or("");
    let wallet = Wallet::from_mnemonic(phrase, passphrase).map_err(|e| e.to_string())?;
    let account = wallet.ethereum_account(index).map_err(|e| e.to_string())?;

    // If the wallet said which address it expects, the key that signs must be
    // that address. Otherwise the signature is unusable and the mismatch is
    // discovered by the wallet, not by the person holding the device.
    if let Some(expected) = request.address {
        if expected != account.address() {
            return Err(format!(
                "the request expects signer {}, but this recovery phrase and path give {}",
                clearsign::address::checksummed(&expected),
                clearsign::address::checksummed(&account.address())
            ));
        }
    }

    let sig = account.sign(&approval).map_err(|e| e.to_string())?;
    let body = encode_signature(
        request.request_id.as_deref(),
        &sig.to_rsv65(),
        Some("clearsign"),
    );
    let response = encode_single("eth-signature", &body);

    println!("\n-- Signature --");
    println!(
        "Signed by ........................ {}",
        clearsign::address::checksummed(&account.address())
    );
    println!(
        "Digest signed .................... {}",
        hex::encode_prefixed(&approval.digest())
    );
    println!("\n-- Show this to the wallet --");
    println!("{response}\n");
    print!("{}", qr_text(&response)?);
    Ok(Outcome::Done)
}

/// Render a UR string as a QR code drawn with block characters.
fn qr_text(data: &str) -> Result<String, String> {
    // A UR is uppercase-safe, and uppercase uses the alphanumeric QR mode, which
    // is denser. Wallets accept either; the scanner lowercases it again.
    let upper = data.to_uppercase();
    let qr = QrCode::encode_text(&upper, QrCodeEcc::Low)
        .map_err(|e| format!("cannot render a QR code: {e}"))?;
    let size = qr.size();
    let mut out = String::new();
    // Two rows of modules per line of text, plus a quiet zone.
    let quiet = 2;
    let mut y = -quiet;
    while y < size + quiet {
        for x in -quiet..size + quiet {
            let top = qr.get_module(x, y);
            let bottom = qr.get_module(x, y + 1);
            out.push(match (top, bottom) {
                (true, true) => '\u{2588}',
                (true, false) => '\u{2580}',
                (false, true) => '\u{2584}',
                (false, false) => ' ',
            });
        }
        out.push('\n');
        y += 2;
    }
    Ok(out)
}
