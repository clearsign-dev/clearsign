#![no_main]
//! INV-4 and INV-7: a plan arrives from an untrusted planner across a trust
//! boundary. Decoding must never panic, and anything accepted must be exactly
//! the bytes that were sent, so the fingerprint a person approves pins down the
//! plan that was actually transmitted.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(plan) = authority::decode_plan(data) {
        // Canonical: accepted bytes must re-encode to themselves.
        let reencoded = authority::encode_plan(&plan);
        assert_eq!(reencoded, data, "accepted a non-canonical encoding");
        // The fingerprint must be a function of those same bytes.
        let again = authority::decode_plan(&reencoded).expect("re-decodes");
        assert_eq!(
            authority::fingerprint(&again),
            authority::fingerprint(&plan),
            "fingerprint changed across a round trip"
        );
        // Reviewing whatever was decoded must not panic either.
        let _ = authority::review_plan(&plan);
    }
});
