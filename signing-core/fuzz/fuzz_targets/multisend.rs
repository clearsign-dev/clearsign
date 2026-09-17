#![no_main]
//! INV-4 and INV-7: the MultiSend batch decoder must never panic, and anything
//! it accepts must account for every byte of the packed argument exactly.
//!
//! Note the two different limits. The decoder *parses* up to
//! `MAX_BATCH_CALLS_PARSED` elements so it can judge what is in a long batch;
//! it only *displays* `MAX_BATCH_CALLS` of them. Asserting the display limit
//! here would be asserting something the decoder no longer promises — which is
//! exactly what this target caught when the two were separated.
use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: &[u8]| {
    if let Ok(calls) = clearsign::multisend::decode_batch(data) {
        assert!(!calls.is_empty(), "accepted an empty batch");
        assert!(
            calls.len() <= clearsign::multisend::MAX_BATCH_CALLS_PARSED,
            "parsed more calls than the parsing limit"
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
