# Release and pilot readiness

This is an engineering decision record, not an independent audit or a promise
of market demand. It separates the reviewer that can be tested today from the
hardware platform and commercial evidence that still need work.

## What to offer first

An independent, offline-first reviewer for Safe transaction bytes is a specific
product. A general-purpose operating system for every blockchain is not what
this repository implements. Supported input types, contract selectors and
deployment tables are listed in [the support matrix](10-what-is-supported.md).
A deployment-table entry is not a tested integration. Unknown input must stay
BLIND or refused, including unsupported typed data.

The desktop reviewer holds no keys. It cannot force a separate wallet to sign
the hash it reviewed. The next useful integration is a documented, tested Safe
and hardware-wallet approval workflow that compares the exact signing digest.
Choose one wallet model and firmware version with a pilot team; verify its
signing mode and domain handling before claiming support. Never import real
keys into a development signer to demonstrate the workflow.

## Release gates

- Record the exact source commit, workflow runs and installer checks for each
  candidate. A green compile is not an installed-app test.
- Run source tests, desktop dependency backport checks, browser regressions,
  canonical rebuilds, native UI checks and the complete artifact-manifest job.
- Test the downloaded macOS DMG and installed application. Native CI checks are
  defined for Debian, AppImage, Windows NSIS and MSI. Record the successful run
  for the candidate; the presence of a test is not evidence that it passed.
  Linux checks use Ubuntu 22.04 with build dependencies installed, not every
  distribution or a clean end-user machine.
- Check the displayed version, historical fixture hash, malformed-input refusal,
  clearing and input changes. Test on a clean non-developer machine as well.
- Verify the candidate's existing manifest before publishing, without regenerating
  checksums to make changed downloads pass. Require the expected versioned
  installer names and source commit in the metadata.
- Date the changelog only when the candidate is approved. Keep one version across
  source, desktop configuration and tag. Verify the published downloads and their
  checksums after publication, not just the intermediate workflow artifacts.
- Keep platform signing/notarization status explicit. Checksums detect changes
  relative to a trusted checksum; they do not establish publisher identity on
  their own. Unsigned builds are evaluation releases, not a finished custody
  distribution strategy.
- Retain a rollback procedure, supported-release policy and private vulnerability
  reporting route. Do not silently replace existing release assets.

The glib fix, native UI tests and candidate-manifest gate are implemented in this
pass. CI results, installer coverage and release publication are separate facts;
check the linked workflow run and Releases page rather than treating this file
as a completion certificate. The website and its existing download links are
unchanged.

## Before high-value custody

Commission independent review of the post-fix source, especially JSON/domain
selection, recursive decoding, severity aggregation, UI state invalidation,
signer/display binding and release provenance. Give reviewers a frozen commit,
the previous findings and their regression tests. AI-assisted follow-up is not
a replacement for an independent assessment.

For the isolated platform, select one physical target. Specify the trusted boot
chain, signed updates, anti-rollback, firmware support, display/input ownership,
key storage and recovery. Test on the actual hardware. QEMU isolation and an
allocator demonstration do not establish those properties. Repeat the separate
two-clean-volume signer-image build check when preparing an image release.

## Pilot and buying evidence

Start with three to five Safe treasury or custody teams as a proposed pilot,
not a claim that those customers exist. Run in observation mode alongside their
existing controls. Ask them to supply redacted workflows and consent before
retaining any transaction records; do not add telemetry by default.
Use the [pilot evaluation record](14-pilot-evaluation.md) to collect comparable
results and buying evidence without inventing adoption or customer numbers.

Measure installation success, time per review, unsupported-input rate, warnings
that operators repeatedly ignore, digest-comparison completion and disagreements
with an independent implementation. Test deliberately substituted transactions
using fixtures, never live funds. Any unexplained hash mismatch or stale verdict
blocks expansion of the pilot.

Ask who owns the approval risk and budget, and what they would pay for supported
integration, policy management and review evidence. Those are revenue hypotheses,
not validated demand. Set a four-week decision point: continue commercial
expansion only with repeated use and credible buying commitments. Otherwise
concentrate on the decoder library and integrations rather than adding chains.
An investor packet should include reproducible evidence, open risks, review
scope, pilot results and support costs. Star counts and download totals are not
substitutes for retention or willingness to pay.

## References and next integrations

- [GrapheneOS hardware requirements](https://grapheneos.org/faq#supported-devices)
  and [installation verification](https://grapheneos.org/install/web): useful
  models for narrow hardware support and a verifiable boot chain. Reusing an
  allocator does not confer the rest of that security architecture.
- [Safe transaction hash API](https://docs.safe.global/reference-sdk-protocol-kit/transactions/gettransactionhash)
  and [signature workflow](https://docs.safe.global/sdk/protocol-kit/guides/signatures/transactions):
  use as independent interoperability references. Keep network metadata outside
  the decoding trust boundary.
- [EIP-712](https://eips.ethereum.org/EIPS/eip-712): a future, separately scoped
  typed-data implementation needs canonical test vectors, resource limits and
  domain tests. Do not label all typed messages supported after adding one schema.
- [ERC-7730](https://eips.ethereum.org/EIPS/eip-7730) is a draft clear-signing
  presentation format worth tracking. Imported display descriptions must not
  override decoded bytes or become an authority for contract behavior.
- [Tauri native WebDriver](https://v2.tauri.app/develop/tests/webdriver/manual-setup/):
  used for the installed Linux application. Windows uses Microsoft's
  [WebView2 attach method](https://learn.microsoft.com/en-us/microsoft-edge/webview2/how-to/webdriver#step-4b-attaching-microsoft-edge-webdriver-to-a-running-webview2-app),
  with loopback debugging enabled only for testing. Elevated CI runners require
  an app-specific machine-policy override because WebView2 ignores environment
  flags in elevated hosts; the workflow restores the prior value in `finally`.
  See [Microsoft's elevated-host guidance](https://learn.microsoft.com/en-us/microsoft-edge/webview2/concepts/security#for-an-elevated-host-app-use-appropriate-override-flags).
  Neither adds an embedded test server or persistent automation configuration
  to the shipped application.
- [glib advisory](https://rustsec.org/advisories/RUSTSEC-2024-0429.html) and
  [upstream correction](https://github.com/gtk-rs/gtk-rs-core/pull/1343): provenance
  for the narrowly scoped desktop backport.

Prefer these bounded integrations and evidence gaps over broad simulation,
token metadata or additional chain families until a real pilot requires them.
