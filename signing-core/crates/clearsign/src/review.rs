//! The review model: what a signer sees, and its deterministic rendering (INV-3).

use alloc::format;
use alloc::string::String;
use alloc::vec::Vec;

/// Ordered from least to most serious.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    /// Worth knowing. Does not change the decision.
    Info,
    /// Read carefully before approving.
    Warning,
    /// Part of the transaction could not be interpreted. INV-2.
    Blind,
    /// High-risk action. Approve only if you intended exactly this.
    Critical,
}

impl Severity {
    pub fn label(self) -> &'static str {
        match self {
            Severity::Info => "INFO",
            Severity::Warning => "WARNING",
            Severity::Blind => "BLIND",
            Severity::Critical => "CRITICAL",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    pub severity: Severity,
    /// Stable machine-readable identifier, e.g. `SAFE_DELEGATECALL`.
    pub code: &'static str,
    pub message: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Section {
    pub title: String,
    pub fields: Vec<(String, String)>,
}

impl Section {
    pub fn new(title: &str) -> Self {
        Section {
            title: String::from(title),
            fields: Vec::new(),
        }
    }

    pub fn field(&mut self, label: &str, value: String) {
        self.fields.push((String::from(label), value));
    }
}

/// More findings than this and the review cannot be approved at all.
///
/// Two reasons. A person acknowledging risks one at a time stopped reading long
/// before this many, and an approval nobody read is the thing acknowledgement
/// exists to prevent. And it keeps the identifier below any width it is stored
/// in: a number that saturates is a number two findings can share, and an
/// authorization identifier that two things can share authorizes the wrong one.
pub const MAX_ACKNOWLEDGEABLE_FINDINGS: usize = 4096;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    pub title: String,
    pub sections: Vec<Section>,
    /// Private: a caller that could empty this could show one transaction and
    /// obtain permission to sign another. Read it with [`Review::findings`].
    findings: Vec<Finding>,
    /// Hashes the signer should compare out of band. INV-5.
    pub digests: Vec<(String, String)>,
    /// The exact digest this review covers, if it may be signed. Private so that
    /// code outside this crate cannot attach a digest the review did not derive.
    signing_target: Option<SigningTarget>,
}

/// What a signature over this review's digest authorises.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TargetKind {
    /// An EVM transaction. `chain_id` is `None` for pre-EIP-155 legacy transactions.
    EvmTransaction {
        tx_type: crate::TxType,
        chain_id: Option<crate::U256>,
    },
    /// A Safe transaction hash signed by an owner.
    SafeTransaction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SigningTarget {
    pub kind: TargetKind,
    pub digest: [u8; 32],
}

impl Review {
    pub fn new(title: &str) -> Self {
        Review {
            title: String::from(title),
            sections: Vec::new(),
            findings: Vec::new(),
            digests: Vec::new(),
            signing_target: None,
        }
    }

    /// The digest a signer may sign after a human approves this review.
    /// `None` means the review is for display only and must not be signed.
    pub fn signing_target(&self) -> Option<&SigningTarget> {
        self.signing_target.as_ref()
    }

    pub(crate) fn set_signing_target(&mut self, target: SigningTarget) {
        self.signing_target = Some(target);
    }

    /// Make the review display-only, for a review built by a path that signs
    /// but shown where nothing may be signed.
    pub(crate) fn clear_signing_target(&mut self) {
        self.signing_target = None;
    }

    /// Everything the review found, in the order it was found.
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }

    /// The acknowledgements a signer must be given, one per finding that needs
    /// one, numbered as they are displayed.
    ///
    /// Two findings with the same code are two separate requirements: a review
    /// showing unlimited approvals to two different spenders must be confirmed
    /// twice, not once.
    pub fn required_acknowledgements(&self) -> Vec<(u32, &'static str)> {
        self.numbered()
            .into_iter()
            .filter(|(_, f)| f.severity >= Severity::Blind)
            .map(|(n, f)| (n, f.code))
            .collect()
    }

    /// True when there are more findings than can be acknowledged one by one.
    /// Approval refuses outright rather than accepting a partial list.
    pub fn too_many_to_acknowledge(&self) -> bool {
        self.findings.len() > MAX_ACKNOWLEDGEABLE_FINDINGS
    }

    /// Findings in display order, each with the number shown beside it.
    ///
    /// The number is `u32` and is never saturated. It used to be a `u16` that
    /// clamped at 65,535, so the 65,536th finding and every one after it shared
    /// an identifier — and two findings sharing an identifier means confirming
    /// one confirms the others.
    fn numbered(&self) -> Vec<(u32, &Finding)> {
        let mut findings: Vec<&Finding> = self.findings.iter().collect();
        // Most serious first; stable within a severity.
        findings.sort_by_key(|f| core::cmp::Reverse(f.severity));
        findings
            .into_iter()
            .enumerate()
            // A list long enough to exhaust u32 cannot be allocated, and
            // approval has already refused far below it. Dropping rather than
            // clamping keeps the rule absolute: no two findings share a number.
            .filter_map(|(i, f)| u32::try_from(i.saturating_add(1)).ok().map(|n| (n, f)))
            .collect()
    }

    pub fn find(&mut self, severity: Severity, code: &'static str, message: String) {
        self.findings.push(Finding {
            severity,
            code,
            message,
        });
    }

    pub fn highest_severity(&self) -> Option<Severity> {
        self.findings.iter().map(|f| f.severity).max()
    }

    pub fn has(&self, code: &str) -> bool {
        self.findings.iter().any(|f| f.code == code)
    }

    /// Deterministic plain-text rendering. Same review, same bytes, every time.
    pub fn render(&self) -> String {
        const WIDTH: usize = 34;
        let mut out = String::new();
        out.push_str(&format!("== {} ==\n", self.title));
        for section in &self.sections {
            out.push_str(&format!("\n-- {} --\n", section.title));
            for (label, value) in &section.fields {
                out.push_str(&dotted(label, value, WIDTH));
            }
        }

        let numbered = self.numbered();
        out.push_str("\n-- Findings --\n");
        if numbered.is_empty() {
            out.push_str("(none)\n");
        }
        for (n, f) in &numbered {
            // The number is part of how a finding is acknowledged, so it has to
            // be on screen next to the thing it refers to.
            out.push_str(&format!(
                "[{}] {n}:{} - {}\n",
                f.severity.label(),
                f.code,
                f.message
            ));
        }
        let required = self.required_acknowledgements();
        if !required.is_empty() {
            out.push_str("\n-- Must be acknowledged, one by one --\n");
            for (n, code) in &required {
                out.push_str(&format!("  {n}:{code}\n"));
            }
        }

        if !self.digests.is_empty() {
            out.push_str("\n-- Verify out of band --\n");
            for (label, value) in &self.digests {
                out.push_str(&dotted(label, value, WIDTH));
            }
        }

        out.push_str("\n-- Verdict --\n");
        out.push_str(match self.highest_severity() {
            Some(Severity::Critical) => {
                "DO NOT SIGN unless you deliberately intended every CRITICAL item above.\n"
            }
            Some(Severity::Blind) => {
                "DO NOT SIGN: part of this transaction could not be decoded, so you cannot know what it does.\n"
            }
            Some(Severity::Warning) => "Review each WARNING before signing.\n",
            Some(Severity::Info) | None => "No risks detected in the decoded content. Confirm the details match your intent.\n",
        });
        out
    }
}

fn dotted(label: &str, value: &str, width: usize) -> String {
    let dots = width.saturating_sub(label.chars().count()).max(2);
    let mut line = String::with_capacity(width.saturating_add(value.len()).saturating_add(4));
    line.push_str(label);
    line.push(' ');
    for _ in 0..dots.saturating_sub(1) {
        line.push('.');
    }
    line.push(' ');
    line.push_str(value);
    line.push('\n');
    line
}
