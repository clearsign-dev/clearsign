#![no_main]
//! INV-4 and INV-7: the MultiSend batch decoder must never panic, and anything
//! it accepts must account for every byte of the packed argument exactly.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(calls) = clearsign::multisend::decode_batch(data) {
        assert!(!calls.is_empty(), "accepted an empty batch");
        assert!(
            calls.len() <= clearsign::multisend::MAX_BATCH_CALLS,
            "accepted more calls than the display limit"
        );
        // Every accepted batch must be exactly the bytes it was given: 85 bytes
        // of header per call plus each call's data, with nothing left over.
        let consumed: usize = calls
            .iter()
            .map(|c| 85usize.saturating_add(c.data.len()))
            .sum();
        assert_eq!(consumed, data.len(), "batch did not consume its argument");
    }
});
