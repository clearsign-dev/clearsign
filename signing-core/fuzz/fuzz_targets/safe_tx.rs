#![no_main]
//! INV-2, INV-5, INV-6, INV-7 for Safe transactions built from fuzzer bytes.
use clearsign::{DomainVersion, SafeTransaction, Severity, U256};
use libfuzzer_sys::fuzz_target;

fn take<const N: usize>(d: &mut &[u8]) -> [u8; N] {
    let mut out = [0u8; N];
    let n = d.len().min(N);
    out[..n].copy_from_slice(&d[..n]);
    *d = &d[n..];
    out
}

fuzz_target!(|input: &[u8]| {
    let mut d = input;
    let flags: [u8; 1] = take(&mut d);
    let operation = flags[0] % 3; // exercise CALL, DELEGATECALL and invalid
    let version = if flags[0] & 0x80 != 0 { DomainVersion::Legacy } else { DomainVersion::V1_3Plus };
    let chain: [u8; 8] = take(&mut d);
    let safe: [u8; 20] = take(&mut d);
    let to_is_safe = flags[0] & 0x40 != 0;
    let to_bytes: [u8; 20] = take(&mut d);
    let nonce: [u8; 8] = take(&mut d);
    let gas_price: [u8; 4] = take(&mut d);
    let value: [u8; 4] = take(&mut d);
    let tx = SafeTransaction {
        chain_id: U256::from_u64(u64::from_be_bytes(chain)),
        safe,
        to: if to_is_safe { safe } else { to_bytes },
        value: U256::from_u64(u64::from(u32::from_be_bytes(value))),
        data: d.to_vec(),
        operation,
        safe_tx_gas: U256::ZERO,
        base_gas: U256::ZERO,
        gas_price: U256::from_u64(u64::from(u32::from_be_bytes(gas_price))),
        gas_token: [0; 20],
        refund_receiver: [0; 20],
        nonce: U256::from_u64(u64::from_be_bytes(nonce)),
    };
    let review = clearsign::review_safe_transaction(&tx, version);
    let text = review.render();
    assert_eq!(text, review.render(), "INV-3 determinism");

    // INV-5: always signable, digest equals the independently computed hash.
    let target = review.signing_target().expect("safe reviews are signable");
    assert_eq!(target.digest, clearsign::safe_transaction_hash(&tx, version));

    match operation {
        1 => {
            assert!(review.has("SAFE_DELEGATECALL"));
            assert_eq!(review.highest_severity(), Some(Severity::Critical));
            // Nothing under a top-level delegatecall may be presented as a decoded action.
            assert!(!text.contains("Action ...."), "delegatecall content was decoded:\n{text}");
        }
        2 => assert!(review.has("SAFE_INVALID_OPERATION")),
        _ => {}
    }
    // INV-2: a BLIND or CRITICAL finding always produces a DO NOT SIGN verdict.
    if review.highest_severity() >= Some(Severity::Blind) {
        assert!(text.contains("DO NOT SIGN"));
    }
});
