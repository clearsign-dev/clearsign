//! C interface to `clearsign` for compartments with no operating system.
//!
//! This is the only crate in the signer that uses `unsafe`, and only for three
//! things: the C function boundary, a fixed-size arena allocator, and the panic
//! handler. The decoding logic stays in the safe, lint-hardened `clearsign` crate.
//!
//! ## Request framing
//!
//! | byte 0 | rest |
//! |---|---|
//! | `1` | unsigned EVM transaction bytes |
//! | `2` | Safe transaction: chain_id(32) safe(20) to(20) value(32) operation(1) safe_tx_gas(32) base_gas(32) gas_price(32) gas_token(20) refund_receiver(20) nonce(32) data_len(4, big-endian) data |
//! | `3` | An action plan proposed by a planner, in the `authority` wire format |
//!
//! Kind `3` is the case that matters most for the rest of the system: the plan
//! is written by an untrusted planner running in another compartment, and this
//! side decides what it actually does before a person is asked to approve it.
//!
//! All integers are big-endian.

#![cfg_attr(target_os = "none", no_std)]

extern crate alloc;

use clearsign::{DomainVersion, SafeTransaction, Severity, U256};

/// Result codes returned in `*severity`.
pub const SEV_NONE: u8 = 0;
pub const SEV_INFO: u8 = 1;
pub const SEV_WARNING: u8 = 2;
pub const SEV_BLIND: u8 = 3;
pub const SEV_CRITICAL: u8 = 4;

/// Error returns (negative).
pub const ERR_NULL: isize = -1;
pub const ERR_DECODE: isize = -2;
pub const ERR_OUTPUT_TOO_SMALL: isize = -3;

/// Decode a request and render the review as text. Pure and safe to test on a host.
pub fn review_request(request: &[u8]) -> Option<(alloc::string::String, u8)> {
    let (&kind, body) = request.split_first()?;
    let review = match kind {
        1 => clearsign::review_transaction_bytes(body).ok()?,
        2 => {
            let tx = parse_safe(body)?;
            clearsign::review_safe_transaction(&tx, DomainVersion::V1_3Plus)
        }
        3 => return review_plan_request(body),
        _ => return None,
    };
    let sev = match review.highest_severity() {
        None => SEV_NONE,
        Some(Severity::Info) => SEV_INFO,
        Some(Severity::Warning) => SEV_WARNING,
        Some(Severity::Blind) => SEV_BLIND,
        Some(Severity::Critical) => SEV_CRITICAL,
    };
    Some((review.render(), sev))
}

/// Review a plan proposed by an untrusted planner.
///
/// Decoding happens here, in the compartment that decides, not in the one that
/// proposes. A plan that cannot be read, or that fails validation, produces a
/// refusal a person can read rather than a silent fallback.
fn review_plan_request(body: &[u8]) -> Option<(alloc::string::String, u8)> {
    use alloc::format;
    let plan = match authority::decode_plan(body) {
        Ok(plan) => plan,
        Err(e) => {
            return Some((
                format!("REFUSED: this is not a plan this device will act on ({e}).\n"),
                SEV_CRITICAL,
            ));
        }
    };
    let review = match authority::review_plan(&plan) {
        Ok(review) => review,
        Err(e) => {
            return Some((
                format!("REFUSED: the plan is not structurally valid ({e:?}).\n"),
                SEV_CRITICAL,
            ));
        }
    };
    let sev = match review.highest_severity() {
        None => SEV_NONE,
        Some(Severity::Info) => SEV_INFO,
        Some(Severity::Warning) => SEV_WARNING,
        Some(Severity::Blind) => SEV_BLIND,
        Some(Severity::Critical) => SEV_CRITICAL,
    };
    Some((review.render(), sev))
}

struct Reader<'a>(&'a [u8]);
impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Option<&'a [u8]> {
        if self.0.len() < n {
            return None;
        }
        let (a, b) = self.0.split_at(n);
        self.0 = b;
        Some(a)
    }
    fn u256(&mut self) -> Option<U256> {
        U256::from_be_slice(self.take(32)?).ok()
    }
    fn addr(&mut self) -> Option<[u8; 20]> {
        self.take(20)?.try_into().ok()
    }
}

fn parse_safe(body: &[u8]) -> Option<SafeTransaction> {
    let mut r = Reader(body);
    let chain_id = r.u256()?;
    let safe = r.addr()?;
    let to = r.addr()?;
    let value = r.u256()?;
    let operation = *r.take(1)?.first()?;
    let safe_tx_gas = r.u256()?;
    let base_gas = r.u256()?;
    let gas_price = r.u256()?;
    let gas_token = r.addr()?;
    let refund_receiver = r.addr()?;
    let nonce = r.u256()?;
    let len_bytes: [u8; 4] = r.take(4)?.try_into().ok()?;
    let len = usize::try_from(u32::from_be_bytes(len_bytes)).ok()?;
    let data = r.take(len)?.to_vec();
    if !r.0.is_empty() {
        return None; // trailing bytes: refuse, same rule as the decoder
    }
    Some(SafeTransaction {
        chain_id,
        safe,
        to,
        value,
        data,
        operation,
        safe_tx_gas,
        base_gas,
        gas_price,
        gas_token,
        refund_receiver,
        nonce,
    })
}

/// C entry point.
///
/// Writes the NUL-terminated review text into `out` and the highest severity
/// into `*severity`. Returns the text length excluding NUL, or a negative error.
///
/// # Safety
/// `request` must point to `request_len` readable bytes, `out` to `out_cap`
/// writable bytes, and `severity` to one writable byte. The buffers must not overlap.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn clearsign_review(
    request: *const u8,
    request_len: usize,
    out: *mut u8,
    out_cap: usize,
    severity: *mut u8,
) -> isize {
    if request.is_null() || out.is_null() || severity.is_null() {
        return ERR_NULL;
    }
    #[cfg(target_os = "none")]
    arena::reset();
    // SAFETY: the caller guarantees `request` is valid for `request_len` bytes.
    let request = unsafe { core::slice::from_raw_parts(request, request_len) };
    let Some((text, sev)) = review_request(request) else {
        return ERR_DECODE;
    };
    let bytes = text.as_bytes();
    let Some(needed) = bytes.len().checked_add(1) else {
        return ERR_OUTPUT_TOO_SMALL;
    };
    if needed > out_cap {
        return ERR_OUTPUT_TOO_SMALL;
    }
    // SAFETY: caller guarantees `out` is valid for `out_cap` bytes; needed <= out_cap.
    let out = unsafe { core::slice::from_raw_parts_mut(out, out_cap) };
    if let Some((nul, dst)) = out.get_mut(..needed).and_then(|w| w.split_last_mut()) {
        dst.copy_from_slice(bytes);
        *nul = 0;
    }
    // SAFETY: caller guarantees `severity` points to one writable byte.
    unsafe { severity.write(sev) };
    isize::try_from(bytes.len()).unwrap_or(ERR_OUTPUT_TOO_SMALL)
}

/// A fixed arena allocator. Every call to `clearsign_review` resets it, which is
/// sound because nothing allocated during one call outlives that call. No heap
/// state survives between requests, so one request cannot influence the next.
#[cfg(target_os = "none")]
mod arena {
    use core::alloc::{GlobalAlloc, Layout};
    use core::cell::UnsafeCell;

    const SIZE: usize = 4 * 1024 * 1024;

    struct Arena {
        mem: UnsafeCell<[u8; SIZE]>,
        next: UnsafeCell<usize>,
    }

    // SAFETY: a seL4 protection domain in this design is single-threaded.
    unsafe impl Sync for Arena {}

    #[global_allocator]
    static ARENA: Arena = Arena {
        mem: UnsafeCell::new([0; SIZE]),
        next: UnsafeCell::new(0),
    };

    pub fn reset() {
        // SAFETY: single-threaded; called only when no allocations are live.
        unsafe { *ARENA.next.get() = 0 };
    }

    unsafe impl GlobalAlloc for Arena {
        unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
            // SAFETY: single-threaded access to the bump pointer.
            let next = unsafe { &mut *self.next.get() };
            let align = layout.align();
            let Some(start) = next
                .checked_add(align.wrapping_sub(1))
                .map(|v| v & !(align.wrapping_sub(1)))
            else {
                return core::ptr::null_mut();
            };
            let Some(end) = start.checked_add(layout.size()) else {
                return core::ptr::null_mut();
            };
            if end > SIZE {
                return core::ptr::null_mut();
            }
            *next = end;
            // SAFETY: start < end <= SIZE, so the pointer is inside the arena.
            unsafe { self.mem.get().cast::<u8>().add(start) }
        }
        unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
    }

    #[panic_handler]
    fn panic(_: &core::panic::PanicInfo<'_>) -> ! {
        // The decoder is linted to never panic. If it somehow does, halt this
        // protection domain rather than continue in an unknown state.
        loop {
            core::hint::spin_loop();
        }
    }
}

#[cfg(test)]
#[allow(
    clippy::indexing_slicing,
    clippy::unwrap_used,
    clippy::arithmetic_side_effects
)]
mod tests {
    use super::*;

    pub fn safe_frame(operation: u8, data: &[u8]) -> Vec<u8> {
        let mut f = vec![2u8];
        f.extend_from_slice(&U256::from_u64(1).0);
        f.extend_from_slice(&[0x11; 20]);
        f.extend_from_slice(&[0x22; 20]);
        f.extend_from_slice(&[0; 32]);
        f.push(operation);
        f.extend_from_slice(&[0; 32 * 3]);
        f.extend_from_slice(&[0; 40]);
        f.extend_from_slice(&U256::from_u64(7).0);
        f.extend_from_slice(&(data.len() as u32).to_be_bytes());
        f.extend_from_slice(data);
        f
    }

    #[test]
    #[allow(clippy::unwrap_used, clippy::arithmetic_side_effects)]
    fn frames_decode_and_render() {
        let (text, sev) = review_request(&safe_frame(1, &[0xa9, 0x05, 0x9c, 0xbb])).unwrap();
        assert_eq!(sev, SEV_CRITICAL);
        assert!(text.contains("SAFE_DELEGATECALL"));
        let mut trailing = safe_frame(0, &[]);
        trailing.push(0);
        assert!(review_request(&trailing).is_none());
        assert!(review_request(&[9]).is_none());
        assert!(review_request(&[]).is_none());
    }

    #[test]
    #[allow(clippy::unwrap_used)]
    fn c_entry_point_writes_nul_terminated_text() {
        let req = safe_frame(0, &[]);
        let mut out = vec![0xffu8; 16 * 1024];
        let mut sev = 0xff;
        let n = unsafe {
            clearsign_review(
                req.as_ptr(),
                req.len(),
                out.as_mut_ptr(),
                out.len(),
                &mut sev,
            )
        };
        assert!(n > 0);
        assert_eq!(out[n as usize], 0);
        let mut tiny = [0u8; 8];
        let e = unsafe {
            clearsign_review(
                req.as_ptr(),
                req.len(),
                tiny.as_mut_ptr(),
                tiny.len(),
                &mut sev,
            )
        };
        assert_eq!(e, ERR_OUTPUT_TOO_SMALL);
    }
}
