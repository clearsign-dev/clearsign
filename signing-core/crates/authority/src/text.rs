//! Safe display of untrusted text.
//!
//! Planner-supplied strings can contain control characters, terminal escape
//! sequences or bidirectional overrides that make text on screen differ from
//! what it really is. Everything untrusted passes through [`display`].

use alloc::format;
use alloc::string::String;

/// Quote untrusted text and escape anything that is not plainly visible.
pub(crate) fn display(s: &str) -> String {
    let mut out = String::with_capacity(s.len().saturating_add(2));
    out.push('"');
    for c in s.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            c if c.is_control() || is_invisible_or_bidi(c) => {
                out.push_str(&format!("\\u{{{:04x}}}", u32::from(c)));
            }
            c => out.push(c),
        }
    }
    out.push('"');
    out
}

fn is_invisible_or_bidi(c: char) -> bool {
    matches!(
        u32::from(c),
        0x200B..=0x200F      // zero-width and directional marks
        | 0x202A..=0x202E    // bidirectional embeddings and overrides
        | 0x2060..=0x2064    // word joiner and invisible operators
        | 0x2066..=0x2069    // bidirectional isolates
        | 0xFEFF             // zero-width no-break space
    )
}
