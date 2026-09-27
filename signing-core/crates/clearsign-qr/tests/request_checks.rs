//! The claims a wallet makes about a request, and what happens when they are
//! wrong or simply absent. These were "leads" in the external review of
//! 17 Sep 2026: not exploits, but checks nobody was performing.

#![allow(clippy::unwrap_used, clippy::panic, clippy::indexing_slicing)]

use clearsign_qr::{DataType, PathComponent, SignRequest, check_request};

fn request() -> SignRequest {
    SignRequest {
        request_id: None,
        sign_data: vec![0x02, 0xf8, 0x6d],
        data_type: DataType::TypedTransaction,
        chain_id: Some(1),
        path: vec![
            PathComponent {
                index: 44,
                hardened: true,
            },
            PathComponent {
                index: 60,
                hardened: true,
            },
            PathComponent {
                index: 0,
                hardened: true,
            },
            PathComponent {
                index: 0,
                hardened: false,
            },
            PathComponent {
                index: 0,
                hardened: false,
            },
        ],
        source_fingerprint: Some(0x16a9_3ed0),
        address: Some([0x11; 20]),
        origin: None,
    }
}

fn codes(c: &[clearsign_qr::RequestConcern]) -> Vec<&'static str> {
    c.iter().map(|c| c.code).collect()
}

#[test]
fn a_request_that_agrees_with_everything_raises_nothing() {
    let c = check_request(
        &request(),
        Some(1),
        Some(0x16a9_3ed0u32.to_be_bytes()),
        Some([0x11; 20]),
    );
    assert!(c.is_empty(), "{:?}", codes(&c));
}

#[test]
fn a_wallet_claiming_a_different_chain_than_the_bytes_is_refused() {
    // The wallet's chain ID is not signed. Where the transaction commits to one,
    // a disagreement means the wallet is showing its user something other than
    // what would happen.
    let c = check_request(
        &request(),
        Some(137),
        Some(0x16a9_3ed0u32.to_be_bytes()),
        Some([0x11; 20]),
    );
    assert_eq!(codes(&c), ["QR_CHAIN_ID_DISAGREES"]);
    assert!(c[0].refuse);
}

#[test]
fn a_request_naming_no_address_is_reported_rather_than_silently_signed() {
    // The lead: a request for m/44'/60'/0'/0/7 with no address signs account 7,
    // and nothing checks that it was the account anyone meant.
    let mut r = request();
    r.address = None;
    let c = check_request(
        &r,
        Some(1),
        Some(0x16a9_3ed0u32.to_be_bytes()),
        Some([0x22; 20]),
    );
    assert_eq!(codes(&c), ["QR_NO_EXPECTED_SIGNER"]);
    assert!(
        !c[0].refuse,
        "it is a warning, not a refusal: the wallet may simply not say"
    );
    assert!(
        c[0].message.contains("0x2222"),
        "it must say which key will sign: {}",
        c[0].message
    );
}

#[test]
fn a_request_built_for_a_different_wallet_is_refused() {
    let c = check_request(
        &request(),
        Some(1),
        Some(0xdead_beefu32.to_be_bytes()),
        Some([0x11; 20]),
    );
    assert_eq!(codes(&c), ["QR_DIFFERENT_WALLET"]);
    assert!(c[0].refuse);
}

#[test]
fn a_request_that_names_no_wallet_is_reported() {
    let mut r = request();
    r.source_fingerprint = None;
    let c = check_request(
        &r,
        Some(1),
        Some(0x16a9_3ed0u32.to_be_bytes()),
        Some([0x11; 20]),
    );
    assert_eq!(codes(&c), ["QR_NO_WALLET_FINGERPRINT"]);
    assert!(!c[0].refuse);
}

#[test]
fn a_wrong_address_is_refused_and_names_both() {
    let c = check_request(
        &request(),
        Some(1),
        Some(0x16a9_3ed0u32.to_be_bytes()),
        Some([0x33; 20]),
    );
    assert_eq!(codes(&c), ["QR_WRONG_SIGNER"]);
    assert!(c[0].refuse);
    assert!(c[0].message.contains("0x1111"), "{}", c[0].message);
    assert!(c[0].message.contains("0x3333"), "{}", c[0].message);
}
