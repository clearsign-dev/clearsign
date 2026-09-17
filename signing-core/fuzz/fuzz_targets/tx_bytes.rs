#![no_main]
//! INV-1, INV-3, INV-7 for raw unsigned transactions.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(review) = clearsign::review_transaction_bytes(data) {
        // INV-1: the signable digest is exactly keccak256 of the input bytes.
        let target = review.signing_target().expect("byte reviews must be signable");
        assert_eq!(target.digest, clearsign::keccak::keccak256(data));
        // INV-3: rendering is deterministic.
        let a = review.render();
        let b = review.render();
        assert_eq!(a, b);
        // A delegatecall anywhere must never be summarised as a known action.
        if review.has("SAFE_DELEGATECALL") {
            assert_eq!(review.highest_severity(), Some(clearsign::Severity::Critical));
        }
    }
});
