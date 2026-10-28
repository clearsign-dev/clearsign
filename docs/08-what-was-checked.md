# What was checked, 27 September 2026

**Current test and fuzz-target counts updated 26 October 2026.** The narrative
below records the 27 September pass; later evidence is listed in
[03-verification-status.md](03-verification-status.md).

## 26 October security pass

A later repository-wide standard review found seven additional issues: the
release workflow exposed publisher credentials to tagged build code; publication
authority ran tagged scripts; hidden batch actions could share an
acknowledgement; the untrusted wallet shared the signer's debug console; the
Linux harness accepted guest-authored evidence; proposal JSON was unbounded
before parsing; and the public transaction/RLP parsers lacked total resource
limits.

The code in this change closes the parser and authorization issues, gives the
physical UART to a separate trusted display domain, makes the Linux harness
accept review and fault evidence only from trusted components, and moves release
publication to a no-checkout default-branch workflow with a fixed asset
allowlist. Repository signing is deliberately disabled until an independently
administered Apple/Windows signing service exists. This was an AI-assisted
standard review, not a second independent human assessment.

A separate post-fix pass then found three residual paths: an ambiguous Git ref
lookup in the publisher, count and nesting limits that still left a signing
target, and the inherited x86 VMM's direct COM1 passthrough. The publisher now
resolves only `refs/tags/<version>` and peels annotated tags; every undecoded
executable-call limit permanently removes signing authority; and the VMM refuses
to compile for any board except the tested QEMU AArch64 target.

A full pass over everything, run rather than assumed. This is the record of what
was exercised and what it found, so the next person does not have to take the
word "working" on trust.

## The result

| | |
|---|---|
| Rust workspace | 319 tests are defined (one diagnostic sample is ignored); the release suite passes, clippy is clean at `-D warnings`, and `rustfmt` is clean |
| Bare metal | All four device crates build for `aarch64-unknown-none` and `thumbv7em-none-eabihf` |
| Fuzzing | Nine targets are now defined; the seven-target 27 September pass ran ~35 million executions with no failures |
| Reproducible build | Two builds in the canonical environment agree; CI reproduces them on other hardware |
| Command-line tool | Refuses the real Bybit transaction with exit 3; passes a clean transfer |
| Application | Builds, installs, launches; the page parses; the WebAssembly reviewer agrees with the command line on the same transaction |
| Platform pipeline | All seven stages pass |

Platform stages, in order: dependencies fetched and signature-verified; Rust
tests and lints; GrapheneOS `hardened_malloc` built from the manifest-pinned
commit with its own suite passing; the Linux guest built from a kernel.org
tarball with a verified signature; seL4 with an untrusted wallet UI and a
bare-metal signer; seL4 running a Linux compartment that proposes and a signer
that decides; and the signer-only image, twelve checks including that it signs
with the key the request names and that the signature equals Foundry's.

## What the pass found

The September pass found no decoder defect. It found four problems in the
scaffolding around it: three were tests asserting on text that had been
deliberately changed, and they would have gone on passing if the text had never
been checked against reality. The later October review above did find security
defects, so this section should be read as the historical result of that earlier
pass, not a claim about all subsequent work.

**The signer image refused to build, correctly.** The gate that requires the
binary inside the image to match `EXPECTED-HASHES.txt` stopped the build, because
the recorded hashes were from before the audit fixes. That gate exists so the
image cannot quietly contain a locally built signer, and this is the first time
it has fired in anger. The hashes were regenerated canonically and it built.

**The device banner was mojibake.** It offered `ur:…` with a Unicode ellipsis.
The console is a serial line; what arrived was `ur:??`. Now ASCII.

**Three stale assertions in the image harness.** It waited for `ack <CODE>` after
acknowledgements became per-finding, for `-- Signing request --` after that line
started naming where its values come from, and for an unquoted `metamask` after
untrusted text started being escaped. Each of those product changes was correct,
and each left a test asserting on wording that no longer existed. The last one
now matches on substance rather than on exact spacing, with a note saying why.

## What this pass does not tell you

- **Nobody outside the project has reviewed the code since the audit fixes.** Nine
  of those ten fixes changed the code that decides whether a signature happens.
- **Everything runs under emulation.** No hardware, no secure element, no verified
  boot. A signer in a virtual machine protects nothing from the machine hosting it.
- **The application is not publisher-signed**, so both macOS and Windows will warn that
  it comes from an unidentified developer.
- **Zero people depend on this.** Every check above is the project marking its own
  work.

## Running it yourself

```sh
cd signing-core && cargo test --workspace --release --locked
cargo clippy --workspace --all-targets --locked -- -D warnings
cd ../platform && ./run-all.sh              # needs Docker running
cd ../app && ./build.sh                     # the window, both forms
cd ../desktop && npm run build              # the installer
```
