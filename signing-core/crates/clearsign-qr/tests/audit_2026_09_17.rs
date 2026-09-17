//! Regression test for the external audit of 17 Sep 2026, finding 8.
//!
//! A part declared a one-byte message and carried thousands of bytes of
//! fragment. The decoder buffered all of it and only noticed at the checksum,
//! so the memory was spent either way. On the device this is how you stop the
//! only process on the machine by waving QR codes at it.

#![allow(clippy::unwrap_used, clippy::panic, clippy::indexing_slicing)]

use clearsign_qr::{Decoder, bytewords};

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

fn part(seq_len: usize, message_len: usize, data: &[u8]) -> String {
    let mut cbor = Vec::new();
    put_uint(&mut cbor, 4, 5);
    put_uint(&mut cbor, 0, 1);
    put_uint(&mut cbor, 0, seq_len as u64);
    put_uint(&mut cbor, 0, message_len as u64);
    put_uint(&mut cbor, 0, 0x1234_5678);
    put_uint(&mut cbor, 2, data.len() as u64);
    cbor.extend_from_slice(data);
    format!("ur:bytes/1-{seq_len}/{}", bytewords::encode(&cbor))
}

#[test]
fn a_fragment_larger_than_its_message_is_refused() {
    let huge = vec![0xABu8; 8000];
    let ur = part(1, 1, &huge);
    let mut d = Decoder::new();
    let err = d.receive(&ur).unwrap_err();
    assert!(
        matches!(err, clearsign_qr::Error::Ur(_)),
        "a fragment bigger than its own message must be refused: {err}"
    );
    assert_eq!(d.progress(), (0, 0), "nothing should have been buffered");
}

#[test]
fn a_fragment_that_fits_its_message_is_still_accepted() {
    let data = vec![0u8; 100];
    let ur = part(1, 100, &data);
    let mut d = Decoder::new();
    // It fails on the checksum, not on the size: the shape is legitimate.
    let err = d.receive(&ur).unwrap_err();
    assert!(format!("{err}").contains("checksum"), "{err}");
}
