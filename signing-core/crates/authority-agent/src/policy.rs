//! Local policy: what counts as sensitive on this machine.
//!
//! The rules themselves live in `authority::classify`, so the adapter and the
//! compartment that serves an untrusted guest judge a path the same way. This
//! module is the adapter's view of them.

use authority::Sensitivity;
pub use authority::classify::{PathPolicy, Resolved};

/// Where sensitive things live on this machine.
pub type Policy = PathPolicy;

/// The sensitivity of a path, most sensitive match wins. A path that cannot be
/// resolved without following symlinks is treated as secret.
pub fn sensitivity_for_path(policy: &Policy, path: &str) -> Sensitivity {
    policy.sensitivity(path)
}
