//! Safe display of untrusted text.
//!
//! The implementation lives in `clearsign::text` so the whole project has one
//! set of rules about what may reach a screen.

use alloc::string::String;

pub(crate) fn display(s: &str) -> String {
    clearsign::escape_untrusted(s)
}
