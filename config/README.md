# config

`pins.env` is the single source of truth for every component this project borrows: exact versions, file hashes and signing-key fingerprints. Fetch and build scripts read it; nothing else should hard-code a version or hash.

Other configuration lives next to what it configures, because the tools require it there:

| File | Configures |
|---|---|
| `signing-core/rust-toolchain.toml` | Rust compiler version, pinned for reproducible builds |
| `signing-core/Cargo.toml`, `Cargo.lock` | Rust workspace and exact dependency versions |
| `platform/sel4/*/…system` | seL4 compartment layout: every memory region, permission and channel |
| `.gitignore` | Keeps downloaded upstream code and build output out of version control |
| `.claude/settings.local.json` | Claude Code's local permissions for this folder, not part of the product |

## Trust levels

Each pin in `pins.env` states how far it was verified:

- **pinned-independent**: the signing key was confirmed through a second, independent channel.
- **pinned-tofu**: the key was pinned the first time it was seen. A change later is detected, but the first download was not independently confirmed.
- **hash-only**: no signature exists. Only the file's SHA-256 is pinned.

Current weakest links: the seL4 SDK signing key and the GrapheneOS signing key (both pinned-tofu), and trust in Debian's archive signing for the build container's packages. The unsigned prebuilt Linux guest is no longer used by default.

## Changing a pin

1. Download the new version and verify it as strongly as possible.
2. Update the value and its dated note in `pins.env`.
3. Run `platform/run-all.sh` and confirm every check passes.

## keys/

Public signing keys stored in the repository, so verification never depends on a keyserver being reachable. Scripts still check each stored key's fingerprint against `pins.env`.

| File | Verifies |
|---|---|
| `microkit-release-FE91…FDCA.asc` | seL4 Microkit SDK releases |
| `alpine-ncopa-0482…495A.asc` | Alpine Linux images |
| `grapheneos_allowed_signers` | GrapheneOS release tags (SSH signatures) |
| `kernel-gregkh-647F…693E.asc` | Linux stable and long-term kernel tarballs from kernel.org |
