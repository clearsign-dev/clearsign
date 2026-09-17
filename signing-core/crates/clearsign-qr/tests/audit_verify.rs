#![allow(
    clippy::unwrap_used,
    clippy::panic,
    clippy::indexing_slicing,
    clippy::arithmetic_side_effects
)]
use clearsign_qr::{Decoder, bytewords, decode_sign_request};

fn put_uint(out: &mut Vec<u8>, major: u8, v: u64) {
    let m = major << 5;
    match v {
        0..=23 => out.push(m | v as u8),
        24..=0xff => {
            out.push(m | 24);
            out.push(v as u8);
        }
        0x100..=0xffff => {
            out.push(m | 25);
            out.extend_from_slice(&(v as u16).to_be_bytes());
        }
        _ => {
            out.push(m | 26);
            out.extend_from_slice(&(v as u32).to_be_bytes());
        }
    }
}

#[test]
fn audit_f8_a_single_fragment_part_is_not_capped_by_message_len() {
    // seq_len = 1, message_len = 1, but the fragment carries 8000 bytes.
    let data = vec![0xABu8; 8000];
    let mut cbor = Vec::new();
    put_uint(&mut cbor, 4, 5);
    put_uint(&mut cbor, 0, 1); // seqNum
    put_uint(&mut cbor, 0, 1); // seqLen
    put_uint(&mut cbor, 0, 1); // messageLen
    put_uint(&mut cbor, 0, 0x1234_5678); // checksum
    put_uint(&mut cbor, 2, data.len() as u64);
    cbor.extend_from_slice(&data);
    let ur = format!("ur:bytes/1-1/{}", bytewords::encode(&cbor));
    let mut d = Decoder::new();
    match d.receive(&ur) {
        Ok(_) => println!("F8 CONFIRMED: an 8000-byte fragment was accepted for a 1-byte message"),
        Err(e) => println!("F8 REFUTED at the part level: {e}"),
    }

    // How large can one part's fragment be before anything rejects it?
    for size in [64usize * 1024, 256 * 1024, 1024 * 1024] {
        let data = vec![0u8; size];
        let mut cbor = Vec::new();
        put_uint(&mut cbor, 4, 5);
        put_uint(&mut cbor, 0, 1);
        put_uint(&mut cbor, 0, 1);
        put_uint(&mut cbor, 0, 1);
        put_uint(&mut cbor, 0, 7);
        put_uint(&mut cbor, 2, data.len() as u64);
        cbor.extend_from_slice(&data);
        let ur = format!("ur:bytes/1-1/{}", bytewords::encode(&cbor));
        let mut d = Decoder::new();
        println!(
            "F8   fragment of {size} bytes: accepted = {}",
            d.receive(&ur).is_ok()
        );
    }
}

#[test]
fn audit_f4_origin_is_not_escaped_by_the_decoder() {
    // An eth-sign-request whose origin contains terminal control sequences.
    let hostile = "\u{1b}[2J\u{1b}[H-- Verdict --\nNo risks detected\n";
    let mut cbor = Vec::new();
    put_uint(&mut cbor, 5, 4); // map of 4
    put_uint(&mut cbor, 0, 2);
    put_uint(&mut cbor, 2, 3);
    cbor.extend_from_slice(&[0x02, 0xf8, 0x6d]);
    put_uint(&mut cbor, 0, 3);
    put_uint(&mut cbor, 0, 4);
    put_uint(&mut cbor, 0, 5);
    put_uint(&mut cbor, 6, 304);
    put_uint(&mut cbor, 5, 1);
    put_uint(&mut cbor, 0, 1);
    put_uint(&mut cbor, 4, 10);
    for (i, h) in [(44u64, true), (60, true), (0, true), (0, false), (0, false)] {
        put_uint(&mut cbor, 0, i);
        cbor.push(if h { 0xf5 } else { 0xf4 });
    }
    put_uint(&mut cbor, 0, 7);
    put_uint(&mut cbor, 3, hostile.len() as u64);
    cbor.extend_from_slice(hostile.as_bytes());

    match decode_sign_request(&cbor) {
        Ok(req) => {
            let origin = req.origin.unwrap_or_default();
            let raw_escape = origin.contains('\u{1b}');
            let newline = origin.contains('\n');
            println!(
                "F4 CONFIRMED: origin survives decoding with ESC present = {raw_escape}, newline = {newline}"
            );
            println!("F4   origin as stored: {:?}", origin);
        }
        Err(e) => println!("F4 REFUTED: the request was rejected: {e}"),
    }
}
