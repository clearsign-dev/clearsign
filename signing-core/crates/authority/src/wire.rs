//! The wire format for plans.
//!
//! A plan crosses a trust boundary: an untrusted planner, possibly running in
//! another compartment or on another machine, proposes it; the authority engine
//! reviews it. The bytes that cross are exactly the bytes the fingerprint is
//! taken over, so there is no second encoding that could disagree with the one
//! the person approved.
//!
//! The decoder is strict in the way the transaction decoder is strict: every
//! length is bounded, every tag must be known, text must be UTF-8, nothing may
//! follow the end, and a decoded plan must re-encode to the input byte for byte.
//! That last rule is what makes the encoding canonical: if two different byte
//! strings decoded to the same plan, a fingerprint would not pin down what was
//! transmitted.

use alloc::string::String;
use alloc::vec::Vec;

use crate::fingerprint;
use crate::plan::{
    Action, HttpMethod, MAX_INPUTS_PER_STEP, MAX_STEPS, MAX_TEXT_BYTES, MAX_TRANSACTION_BYTES,
    Plan, Sensitivity, Step, StepId,
};

/// Why a plan could not be read from the wire.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WireError {
    /// Input ended in the middle of a field.
    Truncated,
    /// Bytes remain after a complete plan.
    TrailingBytes,
    /// A length field exceeds the limit this engine will accept.
    TooLarge(&'static str),
    /// An action, method or sensitivity tag this engine does not know.
    UnknownTag(&'static str),
    /// Text that is not valid UTF-8.
    NotUtf8,
    /// The domain separator at the start is missing or wrong.
    WrongDomain,
    /// The bytes decode, but re-encoding them gives something different, so this
    /// encoding is not the canonical one.
    NotCanonical,
}

impl core::fmt::Display for WireError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            WireError::Truncated => f.write_str("plan ended in the middle of a field"),
            WireError::TrailingBytes => f.write_str("unexpected bytes after the end of the plan"),
            WireError::TooLarge(what) => write!(f, "{what} is larger than this engine accepts"),
            WireError::UnknownTag(what) => write!(f, "unknown {what} tag"),
            WireError::NotUtf8 => f.write_str("text is not valid UTF-8"),
            WireError::WrongDomain => f.write_str("not an authority plan"),
            WireError::NotCanonical => f.write_str("plan is not canonically encoded"),
        }
    }
}

/// Encode a plan exactly as its fingerprint is taken.
pub fn encode_plan(plan: &Plan) -> Vec<u8> {
    fingerprint::encode(plan)
}

/// Decode a plan proposed by an untrusted planner.
///
/// Structural only: the returned plan still has to pass `review_plan`, which is
/// what rejects cycles, unknown references and oversized graphs.
pub fn decode_plan(bytes: &[u8]) -> Result<Plan, WireError> {
    let mut r = Reader { data: bytes, pos: 0 };
    if r.bytes()? != fingerprint::DOMAIN {
        return Err(WireError::WrongDomain);
    }
    let goal = r.text(MAX_TEXT_BYTES, "goal")?;
    let step_count = r.count(MAX_STEPS, "step count")?;
    let mut steps = Vec::with_capacity(step_count);
    for _ in 0..step_count {
        let id = StepId(r.u16()?);
        let title = r.text(MAX_TEXT_BYTES, "step title")?;
        let action = r.action()?;
        let input_count = r.count(MAX_INPUTS_PER_STEP, "input count")?;
        let mut inputs = Vec::with_capacity(input_count);
        for _ in 0..input_count {
            inputs.push(StepId(r.u16()?));
        }
        steps.push(Step {
            id,
            title,
            action,
            inputs,
        });
    }
    if r.pos != r.data.len() {
        return Err(WireError::TrailingBytes);
    }
    let plan = Plan { goal, steps };
    // Canonical or nothing: the fingerprint must pin down these exact bytes.
    if encode_plan(&plan) != bytes {
        return Err(WireError::NotCanonical);
    }
    Ok(plan)
}

struct Reader<'a> {
    data: &'a [u8],
    pos: usize,
}

impl<'a> Reader<'a> {
    fn take(&mut self, n: usize) -> Result<&'a [u8], WireError> {
        let end = self.pos.checked_add(n).ok_or(WireError::Truncated)?;
        let out = self.data.get(self.pos..end).ok_or(WireError::Truncated)?;
        self.pos = end;
        Ok(out)
    }

    fn u16(&mut self) -> Result<u16, WireError> {
        let b: [u8; 2] = self.take(2)?.try_into().map_err(|_| WireError::Truncated)?;
        Ok(u16::from_be_bytes(b))
    }

    fn u32(&mut self) -> Result<u32, WireError> {
        let b: [u8; 4] = self.take(4)?.try_into().map_err(|_| WireError::Truncated)?;
        Ok(u32::from_be_bytes(b))
    }

    fn u64(&mut self) -> Result<u64, WireError> {
        let b: [u8; 8] = self.take(8)?.try_into().map_err(|_| WireError::Truncated)?;
        Ok(u64::from_be_bytes(b))
    }

    fn tag(&mut self) -> Result<u8, WireError> {
        self.take(1)?.first().copied().ok_or(WireError::Truncated)
    }

    /// A length-prefixed field, refused before allocating if it is too large.
    fn bytes_limited(&mut self, max: usize, what: &'static str) -> Result<&'a [u8], WireError> {
        let len = usize::try_from(self.u32()?).map_err(|_| WireError::TooLarge(what))?;
        if len > max {
            return Err(WireError::TooLarge(what));
        }
        self.take(len)
    }

    fn bytes(&mut self) -> Result<&'a [u8], WireError> {
        self.bytes_limited(MAX_TEXT_BYTES, "field")
    }

    fn text(&mut self, max: usize, what: &'static str) -> Result<String, WireError> {
        let raw = self.bytes_limited(max, what)?;
        core::str::from_utf8(raw)
            .map(String::from)
            .map_err(|_| WireError::NotUtf8)
    }

    fn count(&mut self, max: usize, what: &'static str) -> Result<usize, WireError> {
        let n = usize::try_from(self.u32()?).map_err(|_| WireError::TooLarge(what))?;
        if n > max {
            return Err(WireError::TooLarge(what));
        }
        Ok(n)
    }

    fn action(&mut self) -> Result<Action, WireError> {
        let tag = self.tag()?;
        let action = match tag {
            1 => Action::Transform {
                description: self.text(MAX_TEXT_BYTES, "description")?,
            },
            2 => {
                let path = self.text(MAX_TEXT_BYTES, "path")?;
                let sensitivity = match self.tag()? {
                    0 => Sensitivity::Public,
                    1 => Sensitivity::Personal,
                    2 => Sensitivity::Secret,
                    _ => return Err(WireError::UnknownTag("sensitivity")),
                };
                Action::ReadFile { path, sensitivity }
            }
            3 => Action::WriteFile {
                path: self.text(MAX_TEXT_BYTES, "path")?,
            },
            4 => Action::DeleteFile {
                path: self.text(MAX_TEXT_BYTES, "path")?,
            },
            5 => Action::ReadCredential {
                name: self.text(MAX_TEXT_BYTES, "credential name")?,
            },
            6 => {
                let method = match self.tag()? {
                    0 => HttpMethod::Get,
                    1 => HttpMethod::Post,
                    2 => HttpMethod::Put,
                    3 => HttpMethod::Delete,
                    _ => return Err(WireError::UnknownTag("HTTP method")),
                };
                Action::HttpRequest {
                    method,
                    host: self.text(MAX_TEXT_BYTES, "host")?,
                }
            }
            7 => Action::SendMessage {
                channel: self.text(MAX_TEXT_BYTES, "channel")?,
                recipient: self.text(MAX_TEXT_BYTES, "recipient")?,
            },
            8 => Action::Payment {
                amount_minor: self.u64()?,
                currency: self.text(MAX_TEXT_BYTES, "currency")?,
                payee: self.text(MAX_TEXT_BYTES, "payee")?,
            },
            9 => Action::SignTransaction {
                unsigned_tx: self
                    .bytes_limited(MAX_TRANSACTION_BYTES, "transaction")?
                    .to_vec(),
            },
            10 => Action::RunProgram {
                program: self.text(MAX_TEXT_BYTES, "program")?,
            },
            11 => Action::InstallApp {
                package: self.text(MAX_TEXT_BYTES, "package")?,
            },
            12 => Action::ChangeSetting {
                key: self.text(MAX_TEXT_BYTES, "setting key")?,
                value: self.text(MAX_TEXT_BYTES, "setting value")?,
            },
            _ => return Err(WireError::UnknownTag("action")),
        };
        Ok(action)
    }
}
