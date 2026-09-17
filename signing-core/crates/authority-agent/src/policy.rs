//! Local policy: what counts as sensitive on this machine.
//!
//! This is the device's own judgement, not the planner's. The planner proposes
//! *what to do*; the device decides *what that data is*.

use authority::Sensitivity;

/// Where sensitive things live on this machine.
#[derive(Debug, Clone)]
pub struct Policy {
    /// Paths under these prefixes hold credentials, keys or recovery material.
    pub secret_prefixes: Vec<String>,
    /// Paths under these prefixes hold personal data.
    pub personal_prefixes: Vec<String>,
    /// Everything the agent may read or write at all.
    pub allowed_roots: Vec<String>,
}

impl Default for Policy {
    fn default() -> Self {
        Policy {
            secret_prefixes: vec![
                String::from("/etc/ssh"),
                String::from("/home/user/.ssh"),
                String::from("/home/user/.config/keys"),
                String::from("/home/user/wallet"),
            ],
            personal_prefixes: vec![
                String::from("/home/user"),
                String::from("/Users"),
            ],
            allowed_roots: vec![String::from("/home/user"), String::from("/tmp")],
        }
    }
}

impl Policy {
    pub fn classify(&self, path: &str) -> Sensitivity {
        sensitivity_for_path(self, path)
    }

    pub fn allows(&self, path: &str) -> bool {
        self.allowed_roots.iter().any(|root| under(path, root))
    }
}

/// The sensitivity of a path, most sensitive match wins.
pub fn sensitivity_for_path(policy: &Policy, path: &str) -> Sensitivity {
    if policy.secret_prefixes.iter().any(|p| under(path, p)) {
        Sensitivity::Secret
    } else if policy.personal_prefixes.iter().any(|p| under(path, p)) {
        Sensitivity::Personal
    } else {
        Sensitivity::Public
    }
}

/// Whether `path` is inside `root`, without resolving symlinks.
///
/// `..` anywhere is treated as outside: a path that walks upwards is refused
/// rather than normalised, because normalising it here and resolving it later
/// is how a check and its use come apart.
fn under(path: &str, root: &str) -> bool {
    if path.split('/').any(|c| c == "..") {
        return false;
    }
    let root = root.trim_end_matches('/');
    match path.strip_prefix(root) {
        Some("") => true,
        Some(rest) => rest.starts_with('/'),
        None => false,
    }
}
