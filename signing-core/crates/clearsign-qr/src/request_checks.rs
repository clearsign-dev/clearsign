//! What a signing request must agree with before anything is signed.
//!
//! An `eth-sign-request` carries claims a wallet makes about itself: which chain
//! it thinks this is, which key it expects, which wallet the key belongs to.
//! None of that is signed. Each claim is either checked against something that
//! *is* signed, or checked against this device — and a claim that is simply
//! absent is its own answer, because a request that does not say which key it
//! wants will be signed by whatever key the path happens to name.

use alloc::string::String;
use alloc::vec::Vec;

use crate::eth::SignRequest;

/// Something about the request that the person has to be told before signing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RequestConcern {
    /// Stable identifier, in the same style as the decoder's finding codes.
    pub code: &'static str,
    pub message: String,
    /// True when this should stop the signing rather than merely be read.
    pub refuse: bool,
}

/// Compare a request's claims with the transaction it asks to sign, and with
/// this device.
///
/// `signed_chain_id` is the chain ID inside the bytes to be signed, when there is
/// one. `wallet_fingerprint` is this wallet's BIP-32 master fingerprint.
/// `derived_address` is the address the requested path actually produces here.
pub fn check_request(
    request: &SignRequest,
    signed_chain_id: Option<u64>,
    wallet_fingerprint: Option<[u8; 4]>,
    derived_address: Option<[u8; 20]>,
) -> Vec<RequestConcern> {
    let mut out = Vec::new();

    // The wallet's chain ID is not signed. Where the transaction states one, the
    // two must agree: a wallet showing its user one chain while the bytes commit
    // to another is the whole shape of the attack this device exists to catch.
    match (request.chain_id, signed_chain_id) {
        (Some(claimed), Some(signed)) if claimed != signed => out.push(RequestConcern {
            code: "QR_CHAIN_ID_DISAGREES",
            message: alloc::format!(
                "The wallet says this is for chain {claimed}, but the bytes to be signed commit to \
                 chain {signed}. One of them is wrong, and only the signed one counts."
            ),
            refuse: true,
        }),
        _ => {}
    }

    // A request that names no address will be signed by whatever key the path
    // happens to name. That is not a check anyone can pass or fail.
    match (request.address, derived_address) {
        (Some(expected), Some(derived)) if expected != derived => out.push(RequestConcern {
            code: "QR_WRONG_SIGNER",
            message: alloc::format!(
                "The request expects {}, but the key this device derives for that path is {}.",
                clearsign::address::checksummed(&expected),
                clearsign::address::checksummed(&derived)
            ),
            refuse: true,
        }),
        (None, Some(derived)) => out.push(RequestConcern {
            code: "QR_NO_EXPECTED_SIGNER",
            message: alloc::format!(
                "The wallet did not say which address it expects, so nothing confirms that the key \
                 named by the path is the key it meant. This will sign as {}.",
                clearsign::address::checksummed(&derived)
            ),
            refuse: false,
        }),
        _ => {}
    }

    // The source fingerprint says which wallet the request was built for. If it
    // names a different one, the signature would be valid and useless.
    match (request.source_fingerprint, wallet_fingerprint) {
        (Some(claimed), Some(ours)) if claimed.to_be_bytes() != ours => out.push(RequestConcern {
            code: "QR_DIFFERENT_WALLET",
            message: alloc::format!(
                "The request was built for the wallet whose fingerprint is {:08x}; this one is {}.",
                claimed,
                hex4(&ours)
            ),
            refuse: true,
        }),
        (None, Some(_)) => out.push(RequestConcern {
            code: "QR_NO_WALLET_FINGERPRINT",
            message: String::from(
                "The request does not say which wallet it was built for, so nothing confirms this \
                 is the right device for it.",
            ),
            refuse: false,
        }),
        _ => {}
    }

    out
}

fn hex4(b: &[u8; 4]) -> String {
    let mut s = String::with_capacity(8);
    for byte in b {
        s.push_str(&alloc::format!("{byte:02x}"));
    }
    s
}
