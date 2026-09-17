//! Safe display of untrusted text.
//!
//! Strings that came from somewhere else — a planner, a wallet, a QR code, a
//! file name — can carry control characters, terminal escape sequences or
//! bidirectional overrides. On a terminal that honours them, and a serial
//! console does, they can clear the screen and draw a review that was never
//! produced. Everything untrusted goes through [`escape_untrusted`] before it
//! reaches a screen.
//!
//! This lives in the base crate so that there is exactly one of it: a second
//! implementation is a second set of rules about what is safe to print.

use alloc::format;
use alloc::string::String;

/// Quote untrusted text and escape anything that is not plainly visible.
pub fn escape_untrusted(s: &str) -> String {
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
