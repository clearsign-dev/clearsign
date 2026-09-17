//! Regression test for the external audit of 17 Sep 2026, finding 4.
//!
//! A wallet supplies the `origin` string in an EIP-4527 request. It reached the
//! console unescaped, so a wallet could send terminal escape sequences that
//! clear the screen and draw a review nobody produced — with the real review
//! scrolled out of sight above it.

#![allow(clippy::unwrap_used, clippy::panic, clippy::indexing_slicing)]

#[test]
fn a_hostile_origin_cannot_reach_a_terminal_as_it_was_written() {
    let hostile = "\u{1b}[2J\u{1b}[H-- Verdict --\nNo risks detected\n";
    let shown = clearsign::escape_untrusted(hostile);
    assert!(
        !shown.contains('\u{1b}'),
        "escape characters survived: {shown:?}"
    );
    assert!(!shown.contains('\n'), "newlines survived: {shown:?}");
    assert!(shown.starts_with('"') && shown.ends_with('"'), "{shown:?}");
    // The text is still legible, just inert.
    assert!(shown.contains("Verdict"), "{shown:?}");
}

#[test]
fn bidirectional_overrides_are_escaped_too() {
    // A right-to-left override can make an address read backwards on screen.
    let sneaky = "wallet\u{202e}evil";
    let shown = clearsign::escape_untrusted(sneaky);
    assert!(shown.contains("\\u{202e}"), "{shown:?}");
}
