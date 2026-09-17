#![no_main]
//! INV-4 and INV-7: the RLP decoder must never panic, and anything it accepts
//! must be fully consumed with no nesting beyond the limit.
use libfuzzer_sys::fuzz_target;

fn depth(item: &clearsign::rlp::Item<'_>) -> usize {
    match item {
        clearsign::rlp::Item::Bytes(_) => 0,
        clearsign::rlp::Item::List(l) => 1 + l.iter().map(depth).max().unwrap_or(0),
    }
}

fuzz_target!(|data: &[u8]| {
    if let Ok(item) = clearsign::rlp::decode(data) {
        assert!(depth(&item) <= clearsign::rlp::MAX_DEPTH + 1, "nesting limit exceeded");
    }
});
