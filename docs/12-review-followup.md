# Review follow-up: 3 October 2026

This is a targeted, AI-assisted code review and regression-test pass, not an
independent audit or a completed Codex Security scan. The requested deep scan
was stopped. The starting revision was
`249006dc069e3ea3e50238c03917377d14ca485e`.

## Changes

| Boundary | Problem | Check |
| --- | --- | --- |
| Desktop frontend to Rust | Backend expected a global Tauri bridge that the configuration did not enable | Desktop bridge configuration and invocation test |
| JSON to signed fields | Duplicate keys were accepted; malformed chain IDs and missing numeric fields could become defaults | Shared adapter regression tests, including escaped duplicate keys and recursion limits |
| Network to review | A self-consistent service response could describe a different requested transaction | Browser checks both matching and substituted transaction hashes |
| Asynchronous input to display | A delayed file read could replace newer input; network changes could leave stale work | Browser input race and cancellation tests |
| Network response to memory | Size was checked after collecting the response | Streaming byte cap and cancellation test |
| Source to release | Release packaging compared against freshly regenerated expected hashes | Compare with a preserved copy of committed hashes before packaging |
| Release artifacts | Installer checksums and version consistency were incomplete | Required-artifact manifest, checksum tests, version and changelog gates |
| Allocator test harness | A build or harness failure without matching log lines could pass | Require successful test compilation and inspect structured unittest results |

The JSON adapter remains outside the minimal decoder. The additional `serde`
dependency is in that adapter, not in the decoder crate. The new rejection rules
are intentionally stricter: callers must supply the numeric fields that are
actually signed. Service metadata is never used as a decoding authority.

## Limits

The full locked Rust tests, Clippy and formatting passed locally. The desktop
Rust compilation check, bridge test, 15 browser checks and seven release/allocator
tooling tests passed. Two canonical signer builds matched. All seven stages of
`platform/run-all.sh` passed, including seL4 isolation and the signer-only image.
The recorded initramfs hash was refreshed from that build; the separate two-clean-
volume image reproducibility test was not repeated in this pass.

The desktop check exercises the bridge contract and configuration; it is not a
Windows or Linux installer smoke test. Browser network tests use controlled
responses, not an assertion about current service availability. Regression tests
for one input race do not prove the absence of every possible race.

The canonical rebuild establishes repeatability within the named environment,
not hermetic bootstrap or equivalence across compiler hosts. Checksums and build
metadata do not replace authenticated releases or platform code signing.

This pass does not establish a hardware root of trust, verified boot, rollback
protection, physical security, or independent human review of the changed code.
It does not expand supported transaction types or establish contract behavior
from selectors. A separate review tool also cannot enforce that an external
wallet signs the hash it displayed: the signing workflow must bind them.

### Open dependency advisory

GitHub reports [GHSA-wrw7-89jp-8q8g](https://github.com/advisories/GHSA-wrw7-89jp-8q8g)
for `glib 0.18.5` in the Linux desktop dependency graph. It affects
`VariantStrIter` and is fixed upstream in `glib 0.20.0`. Tauri's GTK3 dependencies
use the 0.18 series; changing one lockfile entry to 0.20 is not a compatible fix.
The CLI, WebAssembly reviewer and bare-metal decoder do not use this dependency.

No calls to `array_iter_str` or `VariantStrIter` were found in ClearSign or the
downloaded dependency sources outside glib itself and its tests. That source
search is not a complete reachability proof. The alert remains open. Before a
Linux desktop release, resolve this through a compatible reviewed backport or
supported dependency migration, or complete and document a platform-specific
reachability assessment. Do not dismiss the alert merely because the build passes.

## Release and deployment

v0.1.1 remains the published release while v0.1.2 is prepared. The release job
refuses a tag whose versions disagree or whose changelog is still unreleased.
Before publishing, finish all CI and packaging checks, test installed desktop
applications on supported platforms, date the changelog, and verify the exact
tagged source against the recorded hashes. Independent review is still needed
before presenting this as suitable for high-value custody.

For hardware work, choose one supported device and document its boot chain,
firmware update policy, rollback protection and trusted display/input path.
GrapheneOS's hardware requirements and release verification are useful models;
adding more integrations is not a substitute for those properties.
