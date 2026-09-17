//! Deciding how sensitive data is — on the device, never on the wire.
//!
//! A plan arrives from a planner that may have been talked into anything. If the
//! planner could say how sensitive a file is, it could label a recovery phrase
//! "public" and the flow tracing would agree with it: the review would read like
//! a routine backup while a secret left the machine. So the tag that arrives is
//! discarded and replaced by what this device believes about that path.
//!
//! The same policy is used by the agent adapter and by the compartment that
//! serves an untrusted guest, because two implementations of "is this secret"
//! are two chances to disagree.

use alloc::string::{String, ToString};
use alloc::vec::Vec;

use crate::plan::{Action, Plan, Sensitivity};

/// What this device believes about where sensitive things live.
#[derive(Debug, Clone)]
pub struct PathPolicy {
    /// Paths under these hold credentials, keys or recovery material.
    pub secret_prefixes: Vec<String>,
    /// Paths under these hold personal data.
    pub personal_prefixes: Vec<String>,
    /// Everything an agent may touch at all.
    pub allowed_roots: Vec<String>,
}

impl Default for PathPolicy {
    fn default() -> Self {
        PathPolicy {
            secret_prefixes: [
                "/etc/ssh",
                "/home/user/.ssh",
                "/home/user/.config/keys",
                "/home/user/wallet",
            ]
            .iter()
            .map(|s| s.to_string())
            .collect(),
            personal_prefixes: ["/home/user", "/Users"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
            allowed_roots: ["/home/user", "/tmp"]
                .iter()
                .map(|s| s.to_string())
                .collect(),
        }
    }
}

/// What a path turned out to be.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Resolved {
    /// The path is absolute and plain: here it is, tidied, and what it holds.
    Path {
        canonical: String,
        sensitivity: Sensitivity,
        allowed: bool,
    },
    /// The path cannot be resolved without following symlinks or guessing —
    /// it climbs with `..`, or is relative. Treated as secret and not allowed.
    Unresolvable,
}

impl PathPolicy {
    /// Tidy a path and say what it holds.
    ///
    /// `.` segments and empty segments (`//`) are collapsed rather than
    /// rejected, because they are ordinary typing rather than an attack — but
    /// they are collapsed *before* classification, since `/home/user/./.ssh/id`
    /// and `/home/user/.ssh/id` are the same file and must be judged the same.
    /// `..` is not collapsed: resolving it correctly needs the filesystem, and a
    /// device that guesses is a device that can be walked out of its own roots.
    pub fn resolve(&self, path: &str) -> Resolved {
        let Some(canonical) = canonicalize(path) else {
            return Resolved::Unresolvable;
        };
        let sensitivity = if self.secret_prefixes.iter().any(|p| under(&canonical, p)) {
            Sensitivity::Secret
        } else if self.personal_prefixes.iter().any(|p| under(&canonical, p)) {
            Sensitivity::Personal
        } else {
            Sensitivity::Public
        };
        let allowed = self.allowed_roots.iter().any(|r| under(&canonical, r));
        Resolved::Path {
            canonical,
            sensitivity,
            allowed,
        }
    }

    /// Shorthand: how sensitive is this path? Unresolvable paths are secret,
    /// because the safe assumption about a path nobody can read is the worst one.
    pub fn sensitivity(&self, path: &str) -> Sensitivity {
        match self.resolve(path) {
            Resolved::Path { sensitivity, .. } => sensitivity,
            Resolved::Unresolvable => Sensitivity::Secret,
        }
    }

    /// May an agent touch this path at all?
    pub fn allows(&self, path: &str) -> bool {
        matches!(self.resolve(path), Resolved::Path { allowed: true, .. })
    }
}

/// Replace every file path's sensitivity with this device's own judgement, and
/// tidy the path so what is displayed, classified and executed is one string.
///
/// Returns the paths that could not be resolved, so the review can say so.
pub fn classify_plan(plan: &mut Plan, policy: &PathPolicy) -> Vec<String> {
    let mut unresolvable = Vec::new();
    for step in &mut plan.steps {
        let path_ref = match &mut step.action {
            Action::ReadFile { path, .. }
            | Action::WriteFile { path }
            | Action::DeleteFile { path } => path,
            _ => continue,
        };
        match policy.resolve(path_ref) {
            Resolved::Path {
                canonical,
                sensitivity,
                ..
            } => {
                *path_ref = canonical;
                if let Action::ReadFile { sensitivity: s, .. } = &mut step.action {
                    *s = sensitivity;
                }
            }
            Resolved::Unresolvable => {
                unresolvable.push(path_ref.clone());
                if let Action::ReadFile { sensitivity: s, .. } = &mut step.action {
                    *s = Sensitivity::Secret;
                }
            }
        }
    }
    unresolvable
}

/// Collapse `.` and empty segments. `None` for a relative path or one with `..`.
fn canonicalize(path: &str) -> Option<String> {
    if !path.starts_with('/') {
        return None;
    }
    let mut out = String::with_capacity(path.len());
    for segment in path.split('/') {
        match segment {
            "" | "." => continue,
            ".." => return None,
            s => {
                out.push('/');
                out.push_str(s);
            }
        }
    }
    if out.is_empty() {
        out.push('/');
    }
    Some(out)
}

/// Whether a canonical path is inside a root. Prefix matching alone would put
/// `/home/user.evil` inside `/home/user`.
fn under(path: &str, root: &str) -> bool {
    let root = root.trim_end_matches('/');
    match path.strip_prefix(root) {
        Some("") => true,
        Some(rest) => rest.starts_with('/'),
        None => false,
    }
}
