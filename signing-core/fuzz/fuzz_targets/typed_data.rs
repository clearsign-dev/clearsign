#![no_main]
//! EIP-712 requests from arbitrary text: the reader and the reviewer must
//! never panic, hashing must be deterministic, a request the reviewer accepts
//! must hash, and BLIND or CRITICAL must always end in DO NOT SIGN.
//!
//! Seeded with the EIP's own example, ERC-2612, DAI and every Permit2 type, so
//! mutations start inside the format rather than in random bytes.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    let Ok(text) = std::str::from_utf8(data) else { return };
    let Ok(td) = clearsign_safe_json::typed_data::parse(text) else { return };
    let first = clearsign::typed::hash_typed_data(&td);
    assert_eq!(first, clearsign::typed::hash_typed_data(&td), "hashing is not deterministic");
    if let Ok(review) = clearsign::typed::review_typed_data(&td) {
        assert!(first.is_ok(), "reviewed a request that does not hash");
        let rendered = review.render();
        assert_eq!(rendered, review.render(), "rendering is not deterministic");
        if review.highest_severity() >= Some(clearsign::Severity::Blind) {
            assert!(rendered.contains("DO NOT SIGN"));
        }
        // Every escaped string stays escaped: no raw control characters reach a screen.
        assert!(!rendered.chars().any(|c| c.is_control() && c != '\n'), "a control character reached the screen");
    }
});
