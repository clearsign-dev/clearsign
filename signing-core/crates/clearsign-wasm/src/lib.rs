//! The reviewer, compiled for a browser.
//!
//! The point of this crate is that the desktop application and the command-line
//! tool are the same reviewer. A window that re-implemented any of this in
//! JavaScript would be a second opinion about what a transaction does, and two
//! opinions is exactly the situation this project exists to remove.
//!
//! So the window does no decoding. It hands over the bytes of a JSON file and
//! gets back a finished review, a hash, and a verdict — all computed by the same
//! code the CLI runs and the same code the audited crates contain.
//!
//! ## The interface
//!
//! No bindings generator, no JavaScript dependency tree. Three exported
//! functions and a length-prefixed buffer, which is small enough to read in full:
//!
//! - `cs_alloc(len)` gives the caller somewhere to write input
//! - `cs_review(ptr, len)` returns a pointer to `[u32 length][UTF-8 JSON]`
//! - `cs_free(ptr, len)` gives the memory back
//!
//! Everything runs inside the WebAssembly sandbox on the machine doing the
//! looking. There is no network call in this crate, and nothing to configure.

use std::alloc::{Layout, alloc, dealloc};

use clearsign::{DomainVersion, Severity};

/// Reserve `len` bytes for the caller to write into.
///
/// # Safety
/// The caller must eventually pass the same pointer and length to [`cs_free`],
/// and must not write outside the range.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cs_alloc(len: usize) -> *mut u8 {
    let Ok(layout) = Layout::from_size_align(len.max(1), 1) else {
        return core::ptr::null_mut();
    };
    // SAFETY: the layout has a non-zero size.
    unsafe { alloc(layout) }
}

/// Release memory obtained from [`cs_alloc`] or returned by [`cs_review`].
///
/// # Safety
/// `ptr` must have come from this module with the same `len`.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cs_free(ptr: *mut u8, len: usize) {
    if ptr.is_null() {
        return;
    }
    let Ok(layout) = Layout::from_size_align(len.max(1), 1) else {
        return;
    };
    // SAFETY: the caller guarantees this pointer came from cs_alloc with this length.
    unsafe { dealloc(ptr, layout) }
}

/// Review a Safe transaction given as JSON.
///
/// `chain_id` of 0 means "not given". `version` is 0 for "work it out", 1 for
/// v1.1.x, 3 for v1.3.0 and later.
///
/// Returns a pointer to `[u32 little-endian length][UTF-8 JSON]`, or null if the
/// result could not be built at all.
///
/// # Safety
/// `ptr` must point to `len` readable bytes.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn cs_review(
    ptr: *const u8,
    len: usize,
    chain_id: u64,
    version: u32,
) -> *mut u8 {
    if ptr.is_null() {
        return core::ptr::null_mut();
    }
    // SAFETY: the caller guarantees `ptr` is valid for `len` bytes.
    let bytes = unsafe { core::slice::from_raw_parts(ptr, len) };
    let json = match core::str::from_utf8(bytes) {
        Ok(s) => s,
        Err(_) => return respond(&refusal("that file is not valid text")),
    };
    let version = match version {
        1 => Some(DomainVersion::Legacy),
        3 => Some(DomainVersion::V1_3Plus),
        _ => None,
    };
    let chain_id = if chain_id == 0 { None } else { Some(chain_id) };

    let body = match clearsign_safe_json::review_json(json, chain_id, version) {
        Ok(r) => {
            let severity = match r.review.highest_severity() {
                Some(Severity::Critical) => "critical",
                Some(Severity::Blind) => "blind",
                Some(Severity::Warning) => "warning",
                Some(Severity::Info) | None => "clear",
            };
            let findings: Vec<serde_json::Value> = r
                .review
                .findings()
                .iter()
                .map(|f| {
                    serde_json::json!({
                        "severity": f.severity.label(),
                        "code": f.code,
                        "message": f.message,
                    })
                })
                .collect();
            let must_ack: Vec<serde_json::Value> = r
                .review
                .required_acknowledgements()
                .into_iter()
                .map(|(n, code)| serde_json::json!({ "number": n, "code": code }))
                .collect();
            let sections: Vec<serde_json::Value> = r
                .review
                .sections
                .iter()
                .map(|s| {
                    serde_json::json!({
                        "title": s.title,
                        "fields": s.fields.iter().map(|(k, v)| serde_json::json!([k, v])).collect::<Vec<_>>(),
                    })
                })
                .collect();
            serde_json::json!({
                "ok": true,
                "severity": severity,
                "hash": clearsign::hex::encode_prefixed(&r.hash),
                "hashConfirmed": r.hash_confirmed,
                "domain": match r.version {
                    DomainVersion::Legacy => "v1.1.x",
                    DomainVersion::V1_3Plus => "v1.3.0 or later",
                },
                "domainSource": r.version_source,
                "chainIdSource": r.chain_id_source,
                "chainBound": !matches!(r.version, DomainVersion::Legacy),
                "sections": sections,
                "findings": findings,
                "mustAcknowledge": must_ack,
                "text": r.review.render(),
            })
        }
        Err(message) => refusal(&message),
    };
    respond(&body)
}

fn refusal(message: &str) -> serde_json::Value {
    serde_json::json!({ "ok": false, "severity": "refused", "message": message })
}

/// Write a length-prefixed UTF-8 JSON reply into fresh memory for the caller.
fn respond(value: &serde_json::Value) -> *mut u8 {
    let text = value.to_string();
    let body = text.as_bytes();
    let Ok(total) = u32::try_from(body.len()) else {
        return core::ptr::null_mut();
    };
    let Some(size) = body.len().checked_add(4) else {
        return core::ptr::null_mut();
    };
    let Ok(layout) = Layout::from_size_align(size, 1) else {
        return core::ptr::null_mut();
    };
    // SAFETY: size is non-zero because of the four-byte prefix.
    let out = unsafe { alloc(layout) };
    if out.is_null() {
        return out;
    }
    // SAFETY: `out` is valid for `size` bytes, and the two writes cover exactly
    // the prefix and the body without overlapping.
    unsafe {
        core::ptr::copy_nonoverlapping(total.to_le_bytes().as_ptr(), out, 4);
        core::ptr::copy_nonoverlapping(body.as_ptr(), out.add(4), body.len());
    }
    out
}
