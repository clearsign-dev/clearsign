//! The signer as process 1.
//!
//! This binary is the entire userland of the signer image. There is no shell,
//! no package manager, no network stack and no second program: an attacker who
//! reaches this machine finds one process that reads QR codes and nothing to
//! run. That is the point of the image, and it is checked at build time by
//! `platform/signer-image/build.sh`, which refuses to package anything else.
//!
//! It never exits. Process 1 exiting is a kernel panic, so every failure here
//! is printed and the loop continues.

use std::io::{BufRead, Write};

use clearsign::{Review, Severity};
use clearsign_keys::{Wallet, approve};
use clearsign_qr::ur::encode_single;
use clearsign_qr::{DataType, Decoder, SignRequest, decode_sign_request, encode_signature};
use qrcodegen::{QrCode, QrCodeEcc};
use zeroize::Zeroizing;

const BANNER: &str = "\
clearsign signer
================
This image contains one program: this one. No shell, no network, no storage.

  ur:…            paste one scanned QR code per line; repeat until complete
  ack <CODE>      acknowledge one BLIND or CRITICAL finding, by its exact code
  sign            enter the recovery phrase and sign what was reviewed
  reset           forget the current request and start again

DEVELOPMENT IMAGE. Under an emulator there is no secure element, no verified
boot and no protection from the machine running it. Never enter a recovery
phrase that holds real funds.
";

fn main() {
    print!("{BANNER}");
    flush();
    // One handle on the console, held for the life of the process and passed
    // down to whatever needs to read. Taking a second lock while this one is
    // held blocks forever, which as process 1 means a device that stops.
    let mut input = std::io::stdin().lock();
    let mut session = Session::new();
    loop {
        let mut line = String::new();
        match input.read_line(&mut line) {
            // A closed console must not end process 1.
            Ok(0) => park(),
            Ok(_) => session.handle(line.trim(), &mut input),
            Err(e) => {
                println!("input error: {e}");
                flush();
            }
        }
    }
}

/// What the device is holding right now.
struct Session {
    decoder: Decoder,
    request: Option<SignRequest>,
    review: Option<Review>,
    acks: Vec<String>,
}

impl Session {
    fn new() -> Session {
        Session {
            decoder: Decoder::new(),
            request: None,
            review: None,
            acks: Vec::new(),
        }
    }

    fn handle(&mut self, line: &str, input: &mut impl BufRead) {
        let result = match line {
            "" => Ok(()),
            "reset" => {
                *self = Session::new();
                println!("ready for a new request");
                Ok(())
            }
            "sign" => self.sign(input),
            _ if line.starts_with("ack ") => {
                let code = line.get(4..).unwrap_or("").trim();
                if code.is_empty() {
                    Err(String::from("ack needs a finding code"))
                } else {
                    self.acks.push(String::from(code));
                    println!("acknowledged {code}");
                    Ok(())
                }
            }
            _ if line.starts_with("ur:") || line.starts_with("UR:") => self.scan(line),
            other => Err(format!("unknown input: {other}")),
        };
        if let Err(message) = result {
            println!("REFUSED: {message}");
        }
        flush();
    }

    fn scan(&mut self, line: &str) -> Result<(), String> {
        if self.review.is_some() {
            return Err(String::from(
                "a request is already under review; type reset to discard it",
            ));
        }
        match self.decoder.receive(line) {
            Ok(Some(message)) => {
                let request = decode_sign_request(message).map_err(|e| e.to_string())?;
                let review = review_request(&request)?;
                print_request(&request);
                print!("{}", review.render());
                match review.highest_severity() {
                    Some(Severity::Critical | Severity::Blind) => println!(
                        "\nAcknowledge every BLIND and CRITICAL finding with `ack <CODE>`, then `sign`."
                    ),
                    _ => println!("\nType `sign` to sign this, or `reset` to discard it."),
                }
                self.request = Some(request);
                self.review = Some(review);
                Ok(())
            }
            Ok(None) => {
                let (have, total) = self.decoder.progress();
                println!("fragment {have}/{total}");
                Ok(())
            }
            Err(e) => {
                // A bad code is not fatal: the camera may have misread one.
                Err(format!("that QR code was not accepted: {e}"))
            }
        }
    }

    fn sign(&mut self, input: &mut impl BufRead) -> Result<(), String> {
        let (request, review) = match (&self.request, &self.review) {
            (Some(r), Some(v)) => (r, v),
            _ => return Err(String::from("nothing has been reviewed yet")),
        };
        let acks: Vec<&str> = self.acks.iter().map(String::as_str).collect();
        let approval = approve(review, &acks).map_err(|e| e.to_string())?;
        let index = account_index_for_path(request)?;

        println!("Enter the recovery phrase, then an optional passphrase on the next line,");
        println!("then a blank line. NEVER a phrase holding real funds.");
        flush();
        let phrase = Zeroizing::new(read_line(input)?);
        let passphrase = Zeroizing::new(read_line(input)?);
        let wallet =
            Wallet::from_mnemonic(phrase.trim(), passphrase.trim()).map_err(|e| e.to_string())?;
        let account = wallet.ethereum_account(index).map_err(|e| e.to_string())?;
        if let Some(expected) = request.address {
            if expected != account.address() {
                return Err(format!(
                    "the request expects signer {}, this phrase and path give {}",
                    clearsign::address::checksummed(&expected),
                    clearsign::address::checksummed(&account.address())
                ));
            }
        }
        let signature = account.sign(&approval).map_err(|e| e.to_string())?;
        let body = encode_signature(
            request.request_id.as_deref(),
            &signature.to_rsv65(),
            Some("clearsign"),
        );
        let response = encode_single("eth-signature", &body);

        println!(
            "\nSigned by {}",
            clearsign::address::checksummed(&account.address())
        );
        println!(
            "Digest    {}",
            clearsign::hex::encode_prefixed(&approval.digest())
        );
        println!("\nShow this to the wallet:\n{response}\n");
        match qr_text(&response) {
            Ok(code) => print!("{code}"),
            Err(e) => println!("(cannot draw the QR code: {e})"),
        }
        *self = Session::new();
        println!("\nready for a new request");
        Ok(())
    }
}

fn review_request(request: &SignRequest) -> Result<Review, String> {
    match request.data_type {
        DataType::Transaction | DataType::TypedTransaction => {
            clearsign::review_transaction_bytes(&request.sign_data).map_err(|e| e.to_string())
        }
        other => Err(format!(
            "this request asks for a signature over {}, which this device does not decode",
            other.label()
        )),
    }
}

/// The account index in `m/44'/60'/0'/0/i`, or an error for any other path.
fn account_index_for_path(request: &SignRequest) -> Result<u32, String> {
    let expected = [(44u32, true), (60, true), (0, true), (0, false)];
    let (last, head) = request
        .path
        .split_last()
        .ok_or("the request has no derivation path")?;
    let head_matches = head.len() == expected.len()
        && head
            .iter()
            .zip(expected.iter())
            .all(|(c, (i, h))| c.index == *i && c.hardened == *h);
    if !head_matches || last.hardened {
        return Err(format!(
            "the request asks for key path {}, which this device does not derive",
            request.path_string()
        ));
    }
    Ok(last.index)
}

fn print_request(request: &SignRequest) {
    println!("\n-- Signing request --");
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

fn qr_text(data: &str) -> Result<String, String> {
    let qr =
        QrCode::encode_text(&data.to_uppercase(), QrCodeEcc::Low).map_err(|e| format!("{e}"))?;
    let size = qr.size();
    let quiet = 2;
    let mut out = String::new();
    let mut y = -quiet;
    while y < size + quiet {
        for x in -quiet..size + quiet {
            out.push(match (qr.get_module(x, y), qr.get_module(x, y + 1)) {
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

fn read_line(input: &mut impl BufRead) -> Result<String, String> {
    let mut line = String::new();
    match input.read_line(&mut line) {
        Ok(0) => Err(String::from(
            "the console closed before the phrase was entered",
        )),
        Ok(_) => Ok(line),
        Err(e) => Err(e.to_string()),
    }
}

fn flush() {
    let _ = std::io::stdout().flush();
}

/// Process 1 must not exit. If the console closes, stop doing anything at all.
fn park() -> ! {
    loop {
        std::thread::sleep(std::time::Duration::from_secs(3600));
    }
}
