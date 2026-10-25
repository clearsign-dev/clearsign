# ClearSign — the application

A real desktop application: a window, an installer, an entry in your
Applications folder. Not a file someone emails you.

```sh
cd desktop
npm install
npm run build
```

That produces, on macOS, `ClearSign.app` and a `.dmg` to install it from;
on Windows an `.msi` and an installer `.exe`; on Linux a `.deb` and an AppImage.
The release workflow builds all three on a tag.

## The window does not decide anything

Every judgement about what a transaction does is made in Rust, by the same
crates the command-line tool uses and the same crates the security review
examined. `src-tauri/src/main.rs` exposes one command; the window sends the file
across and renders the answer.

That matters more than it sounds. A desktop application that re-implemented the
decoding in JavaScript would be a *second opinion* about what a transaction
does, and two opinions is the situation this project exists to remove. The
front end is generated from the same template as the standalone page by
`app/build.sh`, and the two differ in exactly one place: where the review comes
from.

## What it does not do

No network connection. No keys. No files written. It takes text in and gives a
review back; your hardware wallet still signs.

## Signing and notarisation

The published v0.1.2 application is not code-signed, so macOS and Windows warn
that it comes from an unidentified developer. Until a signed release replaces
it, the honest check is the hash of what you downloaded, not the badge on it.

The release workflow requires an explicit mode for every tag. `signed` fails
unless macOS is Developer ID signed, notarized and stapled and Windows is
Authenticode signed and timestamped. `unsigned-evaluation` skips those credentials,
checks that the platform artifacts are unsigned, and can publish only as a GitHub
prerelease with a warning in its notes and metadata. Branch builds remain unsigned.
Linux packages use checksums and GitHub build provenance. The required accounts,
variables, secrets, verification commands and recurring costs are in the
[release-signing record](../docs/15-release-signing.md).

When it is signed, the direct download stays: an app store build is re-signed and
repackaged by the store, which breaks the one property that makes this
verifiable — that you can rebuild it yourself and compare hashes.
