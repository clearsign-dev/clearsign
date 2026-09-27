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

The application is not code-signed yet, so macOS and Windows will warn that it
comes from an unidentified developer — because it does. Certificates are a paid
registration that has not been taken out, and pretending otherwise would be the
wrong kind of polish. Until then the honest check is the hash of what you
downloaded, not the badge on it.

When it is signed, the direct download stays: an app store build is re-signed and
repackaged by the store, which breaks the one property that makes this
verifiable — that you can rebuild it yourself and compare hashes.
