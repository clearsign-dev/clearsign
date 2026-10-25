//! Native embedding boundary for iOS, Android and host applications.
//!
//! The caller owns both buffers. Passing a null output pointer with a zero
//! capacity returns the exact number of bytes required, so no Rust allocation
//! crosses a language runtime boundary.

pub const ERR_NULL_INPUT: isize = -1;
pub const ERR_INVALID_UTF8: isize = -2;
pub const ERR_OUTPUT_TOO_SMALL: isize = -3;
pub const ERR_NULL_OUTPUT: isize = -4;
pub const ERR_SERIALIZE: isize = -5;
pub const ERR_LENGTH: isize = -6;
pub const ERR_INPUT_TOO_LARGE: isize = -7;

#[unsafe(no_mangle)]
pub extern "C" fn clearsign_max_record_bytes() -> usize {
    clearsign_api::MAX_RECORD_BYTES
}

#[unsafe(no_mangle)]
pub extern "C" fn clearsign_api_version() -> u32 {
    clearsign_api::API_VERSION
}

/// Review one Safe transaction record and write UTF-8 JSON into `output`.
///
/// Call first with `output = NULL` and `output_capacity = 0` to obtain the
/// required byte count. The response is not NUL terminated.
///
/// # Safety
/// `input` must point to `input_length` readable bytes. When non-null, `output`
/// must point to `output_capacity` writable bytes and must not overlap input.
#[unsafe(no_mangle)]
pub unsafe extern "C" fn clearsign_review_safe_json(
    input: *const u8,
    input_length: usize,
    chain_id: u64,
    version: u32,
    output: *mut u8,
    output_capacity: usize,
) -> isize {
    if input_length > clearsign_api::MAX_RECORD_BYTES {
        return ERR_INPUT_TOO_LARGE;
    }
    if input.is_null() && input_length != 0 {
        return ERR_NULL_INPUT;
    }
    let input = if input_length == 0 {
        &[]
    } else {
        // SAFETY: the caller promises that input is readable for input_length bytes.
        unsafe { core::slice::from_raw_parts(input, input_length) }
    };
    let Ok(json_text) = core::str::from_utf8(input) else {
        return ERR_INVALID_UTF8;
    };
    let Ok(response) = clearsign_api::review_safe_json_string(json_text, chain_id, version) else {
        return ERR_SERIALIZE;
    };
    let bytes = response.as_bytes();
    let Ok(length) = isize::try_from(bytes.len()) else {
        return ERR_LENGTH;
    };

    if output.is_null() {
        return if output_capacity == 0 {
            length
        } else {
            ERR_NULL_OUTPUT
        };
    }
    if output_capacity < bytes.len() {
        return ERR_OUTPUT_TOO_SMALL;
    }
    // SAFETY: the caller promises a writable, non-overlapping output buffer;
    // the capacity check above proves that every byte fits.
    unsafe { core::ptr::copy_nonoverlapping(bytes.as_ptr(), output, bytes.len()) };
    length
}

#[cfg(target_os = "android")]
mod android {
    use jni::{
        JNIEnv,
        objects::{JObject, JString},
        sys::{jint, jstring},
    };

    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_clearsign_core_ClearSignNativeBridge_apiVersion<'local>(
        _env: JNIEnv<'local>,
        _object: JObject<'local>,
    ) -> jint {
        clearsign_api::API_VERSION as jint
    }

    /// JNI adapter used by the Expo module. The response remains the same
    /// versioned JSON returned by the C ABI and WebAssembly boundary.
    #[unsafe(no_mangle)]
    pub extern "system" fn Java_dev_clearsign_core_ClearSignNativeBridge_reviewSafeJson<'local>(
        mut env: JNIEnv<'local>,
        _object: JObject<'local>,
        input: JString<'local>,
        chain_id: JString<'local>,
        version: jint,
    ) -> jstring {
        let Ok(input) = env.get_string(&input).map(|value| String::from(value)) else {
            return core::ptr::null_mut();
        };
        let Ok(chain_id) = env.get_string(&chain_id).map(|value| String::from(value)) else {
            return core::ptr::null_mut();
        };
        let response = if input.len() > clearsign_api::MAX_RECORD_BYTES {
            clearsign_api::ReviewResponse::refusal(format!(
                "this record is {} bytes; the limit is {}",
                input.len(),
                clearsign_api::MAX_RECORD_BYTES
            ))
        } else {
            match chain_id.parse::<u64>() {
                Ok(chain_id) => clearsign_api::review_safe_json(&input, chain_id, version as u32),
                Err(_) => clearsign_api::ReviewResponse::refusal(
                    "chain ID must be a whole, non-negative number",
                ),
            }
        };
        let Ok(response) = serde_json::to_string(&response) else {
            return core::ptr::null_mut();
        };
        match env.new_string(response) {
            Ok(value) => value.into_raw(),
            Err(_) => core::ptr::null_mut(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const BYBIT: &[u8] = include_bytes!("../../clearsign-cli/tests/fixtures/bybit-safe-tx.json");

    fn review(input: &[u8]) -> Result<String, isize> {
        // SAFETY: input is a live slice, and the first call does not write.
        let needed = unsafe {
            clearsign_review_safe_json(input.as_ptr(), input.len(), 1, 0, core::ptr::null_mut(), 0)
        };
        let size = usize::try_from(needed).map_err(|_| needed)?;
        let mut output = vec![0; size];
        // SAFETY: both slices remain live and do not overlap.
        let written = unsafe {
            clearsign_review_safe_json(
                input.as_ptr(),
                input.len(),
                1,
                0,
                output.as_mut_ptr(),
                output.len(),
            )
        };
        if written != needed {
            return Err(written);
        }
        String::from_utf8(output).map_err(|_| ERR_INVALID_UTF8)
    }

    #[test]
    fn native_boundary_returns_the_versioned_bybit_verdict() -> Result<(), String> {
        let text = review(BYBIT).map_err(|code| format!("native review failed: {code}"))?;
        let expected = clearsign_api::review_safe_json_string(
            core::str::from_utf8(BYBIT).map_err(|error| error.to_string())?,
            1,
            0,
        )
        .map_err(|error| error.to_string())?;
        assert_eq!(text, expected);
        assert!(text.contains("SAFE_DELEGATECALL"));
        Ok(())
    }

    #[test]
    fn query_mode_and_small_buffers_fail_closed() {
        let mut byte = 0u8;
        // SAFETY: input and one-byte output are valid and do not overlap.
        let result =
            unsafe { clearsign_review_safe_json(BYBIT.as_ptr(), BYBIT.len(), 1, 0, &mut byte, 1) };
        assert_eq!(result, ERR_OUTPUT_TOO_SMALL);
    }

    #[test]
    fn invalid_utf8_is_refused_before_parsing() {
        let input = [0xff];
        // SAFETY: input is a live one-byte slice; query mode performs no write.
        let result = unsafe {
            clearsign_review_safe_json(input.as_ptr(), input.len(), 1, 0, core::ptr::null_mut(), 0)
        };
        assert_eq!(result, ERR_INVALID_UTF8);
    }

    #[test]
    fn oversized_input_is_refused_before_the_pointer_is_read() {
        // SAFETY: the length guard returns before it creates a slice from this
        // deliberately invalid non-null pointer.
        let result = unsafe {
            clearsign_review_safe_json(
                core::ptr::dangling(),
                clearsign_api::MAX_RECORD_BYTES.saturating_add(1),
                1,
                0,
                core::ptr::null_mut(),
                0,
            )
        };
        assert_eq!(result, ERR_INPUT_TOO_LARGE);
    }

    #[test]
    fn empty_input_reaches_the_parser_and_returns_a_refusal() -> Result<(), String> {
        let text = review(&[]).map_err(|code| format!("native review failed: {code}"))?;
        assert!(text.contains("\"ok\":false"));
        assert!(text.contains("not valid JSON"));
        Ok(())
    }
}
