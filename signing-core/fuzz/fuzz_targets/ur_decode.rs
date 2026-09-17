#![no_main]
//! INV-7: the QR transport parses input from a computer the signer does not
//! trust. It must never panic, and must never report a message it did not
//! actually reassemble and check.
use libfuzzer_sys::fuzz_target;

use clearsign_qr::{crc32::crc32, Decoder};

fuzz_target!(|data: &[u8]| {
    let Ok(text) = core::str::from_utf8(data) else {
        return;
    };
    let mut decoder = Decoder::new();
    for line in text.lines().take(64) {
        match decoder.receive(line) {
            Ok(Some(message)) => {
                // Anything returned as complete is a real message: non-empty and
                // within the stated bound.
                assert!(!message.is_empty(), "completed an empty message");
                assert!(message.len() <= clearsign_qr::ur::MAX_MESSAGE_LEN);
                let _ = crc32(message);
                // Whatever it is, decoding it as a request must not panic.
                let _ = clearsign_qr::decode_sign_request(message);
                return;
            }
            Ok(None) => {
                let (have, total) = decoder.progress();
                assert!(have <= total, "more fragments than the message has");
            }
            Err(_) => {}
        }
    }
});
