# The window

The same reviewer the command-line tool runs, with a face on it.

```sh
./build.sh
open dist/before-you-sign.html
```

That produces one HTML file. Send it to someone, or open it from a folder —
there is nothing to install, no server, and no connection to anything. Paste the
JSON from Safe{Wallet}, or drop the file onto the page.

## Why it is built this way

**The decoding is not in the page.** The reviewer is compiled to WebAssembly
from the same Rust the command line runs and the same crates the audit looked
at. The page does no decoding of its own: it hands over the file and renders
what comes back. A window that re-implemented any of this in JavaScript would be
a second opinion about what a transaction does, and two opinions is the exact
situation this project exists to remove.

**One file, no fetching.** Browsers refuse to let a page opened from a folder
fetch a file beside it, so the WebAssembly is embedded in the HTML. That is also
what makes it something you can hand over: one thing, openable anywhere,
verifiable by its hash.

**It opens on the Bybit transaction.** Not a mock-up — the real record from
Safe's own service, marked as an example. Someone seeing this for the first time
should see what it is for before they have anything of their own to paste in.

## What the page is careful about

- The hash is set in monospace, grouped four characters at a time, with
  alternating colour, because the actual job is comparing it against a hardware
  wallet screen by eye. There is a field to paste what your wallet shows.
- Severity is shown in shape and colour as well as words, so a CRITICAL cannot
  be skimmed past.
- The acknowledgement tokens are printed exactly as a signing device will want
  them.
- It says on its face that it runs locally and holds no keys, because a tool
  asking you to trust it should say what it does with your data where you can
  see it.

## Not the app store, yet

Direct download first. A Mac App Store build is re-signed and repackaged by
Apple, which breaks the one property that makes this checkable: that you can
rebuild it yourself and compare hashes. When there is a reason to be in a store,
it will be alongside the verifiable download rather than instead of it.
