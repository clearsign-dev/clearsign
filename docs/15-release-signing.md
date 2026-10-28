# Release signing and verification

This document describes the repository's release controls. It does not claim
that already-published downloads were retroactively signed. ClearSign v0.1.2 is
unsigned. The protected v0.1.3 tag produced no release because its macOS
evaluation gate rejected an identity-free ad-hoc linker signature. v0.1.4 was
published as an unsigned evaluation release, and v0.1.7 is configured the same
way.

The former signed path imported publisher credentials before repository build
code ran. That is not an acceptable signing boundary. The repository workflow
therefore no longer consumes Apple or Windows private credentials, and
`CLEARSIGN_RELEASE_MODE=signed` fails during validation. A stable release remains
blocked until a separately administered signer can accept only immutable,
digest-verified artifacts. This is a security gate, not a missing CI toggle.

## What the workflow enforces

Branch and manual builds remain without publisher identities. They exercise
packaging, installation and smoke tests without consuming notarization requests
or exposing signing credentials to ordinary development builds.

The current repository workflow accepts:

- `unsigned-evaluation`: a temporary evaluation release. Platform signing and
  notarization credentials are not loaded. Windows artifacts and the macOS DMG
  must be unsigned. The macOS application may carry only an identity-free ad-hoc
  linker signature: no authority, no TeamIdentifier, no Gatekeeper acceptance
  and no stapled ticket. `BUILD-METADATA.json` records the mode, the release
  notes begin with an explicit warning, and GitHub publishes it as a prerelease.

`signed` remains a recognized policy value so external release tooling and
metadata can describe a properly signed candidate, but this workflow rejects it.
The future signer must be protected independently from the tagged source and
must not run package managers, build scripts, newly built applications or
downloaded test tools while publisher authority is available.

The validation job normalizes and snapshots this mode plus the public Apple Team
ID and Windows certificate thumbprint. Every later build, metadata and publish
decision uses those job outputs, so changing a repository variable during a run
cannot make its artifacts and recorded policy disagree.

For a `v*` tag, `.github/workflows/release.yml` then fails closed:

- the workflow rejects signed mode before any build starts;
- macOS and Windows outputs must prove they contain no publisher identity;
- Linux `.deb` and AppImage downloads have no universal publisher-signature
  mechanism in this distribution channel. They are covered by `SHA256SUMS`, the
  release metadata and GitHub's OIDC-backed build-provenance attestation.
- Every candidate includes `ClearSign.spdx.json`, an SPDX JSON software bill of
  materials. `BUILD-METADATA.json` records the source commit and the different
  signing guarantees by platform, including the selected release mode. Both
  files are covered by `SHA256SUMS`.
- Pull requests and releases run `cargo audit` against the desktop lockfile.
  Published vulnerabilities and RustSec `unsound` warnings block the build;
  unmaintained and yanked transitive packages remain visible as warnings.

The workflow contains no Apple or Windows publisher-secret references. Release
notes are prepared without repository write authority. The final publication
step receives `GH_TOKEN`, accepts only a fixed filename allowlist, confirms the
tag resolves to the candidate commit, verifies the exact `SHA256SUMS` contents
and every listed asset with system tools, and only then uploads those explicit
files. It uses the publisher workflow from the protected default branch and
never checks out or executes code from the release tag.

## Accounts and credentials

### macOS

Create an Apple Developer Program organization account if ClearSign should show
the organization, rather than a person's legal name, as the publisher. Apple
currently charges **USD 99 per membership year**, with local pricing where
available. The Account Holder creates a Developer ID Application certificate.
Export the certificate and private key from Keychain Access as a password-
protected `.p12` file.

Create a **team** App Store Connect API key with Developer access. Individual API
keys cannot use `notarytool`. Download the `.p8` file once and retain an offline
backup; Apple does not offer the private key for download again.

Do not configure these as repository secrets. The separate signing service will
need equivalent inputs in its own administrative boundary:

| Secret | Value |
| --- | --- |
| `APPLE_CERTIFICATE` | Base64 of the exported Developer ID `.p12` |
| `APPLE_CERTIFICATE_PASSWORD` | Password protecting that `.p12` |
| `APPLE_API_ISSUER` | App Store Connect team API issuer UUID |
| `APPLE_API_KEY` | App Store Connect API key ID |
| `APPLE_API_KEY_P8_BASE64` | Base64 of the downloaded `.p8` private key |

The public `APPLE_TEAM_ID` value may remain in repository settings for metadata
and verification, but it grants no signing authority.

The certificate and API key are separate credentials. Revoke and replace either
immediately if its secret is exposed. Track their expiry or revocation outside
the repository and test a manual workflow build before creating a release tag.

### Windows

The retired in-workflow backend accepted a password-protected PFX. Do not put a
PFX or its password in repository secrets. Prefer a non-exportable HSM or managed
signing service, and keep these values in that service's boundary:

| Secret or variable | Value |
| --- | --- |
| `WINDOWS_CERTIFICATE` | Base64 of the PFX file |
| `WINDOWS_CERTIFICATE_PASSWORD` | PFX export password |
| `WINDOWS_CERTIFICATE_THUMBPRINT` | Public expected 40-hex SHA-1 certificate thumbprint |
| `WINDOWS_TIMESTAMP_URL` | Approved RFC 3161 timestamp service |

Do not use a self-signed certificate for public downloads. Microsoft recommends
Artifact Signing for eligible publishers; its entry plan is currently about
**USD 9.99 per month** and requires Microsoft identity validation. Traditional
OV certificates are typically a recurring **USD 150-300 per year**, depending
on the certificate authority. Artifact Signing Public Trust is currently
region-limited and is not offered to a Nigeria-based individual or organization,
so verify eligibility before designing around it. Current certificate rules may
require a hardware token or managed signing service and may prohibit exporting a
PFX. If the chosen provider does not issue an exportable PFX, replace this
backend with its HSM or
cloud signing command before buying the certificate; never copy a protected key
out of its approved storage to satisfy this workflow.

The expected thumbprint prevents a valid but unintended certificate from being
used. Update it through a reviewed repository-settings change when the
certificate is renewed, before creating a tag; the thumbprint is public and must
not be stored as a secret.

Signing does not guarantee that a new Windows application immediately avoids a
SmartScreen prompt. Reputation accumulates over time. Use the same publisher
identity for each release and timestamp every signature so that it remains
valid after certificate expiry.

### GitHub and Linux

The public repository can use GitHub artifact attestations without a separate
signing account. The workflow receives short-lived OIDC credentials through
`id-token: write` and writes attestations through `attestations: write`; there is
no long-lived attestation key to store. Normal GitHub Actions plan and runner
usage limits still apply.

Linux users authenticate the source repository and workflow through the GitHub
attestation, then compare the downloaded file with the release checksum. This
is build provenance, not a Linux package-maintainer signature or a guarantee
that the program is safe.

## Release operator checklist

1. Confirm all product versions and the dated changelog agree. Refresh both the
   canonical binary hashes and signer-image hashes before tagging. The tag
   workflow rebuilds each twice and refuses stale records.
2. Keep `CLEARSIGN_RELEASE_MODE=unsigned-evaluation` for repository-produced
   tags. Setting it to `signed` must fail validation.
3. Do not create a stable tag until the separate signing system exists and has
   been exercised with non-production test identities. Confirm the Apple
   membership, certificate and notarization key and the Windows signing service
   only inside that isolated system.
4. Run the release workflow manually from the intended commit. This produces a
   candidate without publisher identities and tests installers; it does not
   prove tag signing.
5. Create the tag only from the reviewed commit. Never move or reuse a release tag.
6. Require complete-candidate validation, provenance attestation and explicit
   no-publisher-identity checks to pass, and confirm the GitHub release is marked
   prerelease.
7. Download the published assets onto clean macOS, Windows and Linux machines.
   Verify the mode, checksums, provenance, installation, displayed version and
   historical transaction fixture.
8. Preserve the workflow run URL, source commit, `SHA256SUMS`, metadata, SBOM and
   notarization/signing evidence in the release record. Do not replace assets in
   place; issue a new version if any published byte must change.

Repository administration is part of this gate and cannot be encoded completely
in a workflow file. The `release` environment exists and accepts only `v*` tags,
an active tag ruleset prevents those release tags from being updated or deleted,
and immutable releases are enabled for future publications. The environment
cannot yet require a second reviewer or prevent self-review because the
organization has only one member. Until independent approval is configured, a
maintainer account with tag and release permission remains a single-person
publication path even though the selected release mode itself fails closed.

## Desktop dependency remediation

The October 2026 audit found `quick-xml` 0.38.4 through
`plist -> tauri` affected by RUSTSEC-2026-0194 (quadratic duplicate-attribute
checking) and RUSTSEC-2026-0195 (unbounded namespace allocation). The desktop
lockfile now uses `plist` 1.10.3 and `quick-xml` 0.42.0. These parsers are in the
Tauri desktop dependency graph, not ClearSign's transaction decoder; no path
from transaction bytes to plist XML was found, but malformed XML in packaging
or platform metadata could still consume release-machine resources.

The same lockfile moved `tokio` 1.53.1 to 1.54.1 to clear
RUSTSEC-2026-0386. ClearSign does not directly construct Tokio `AsyncFd` values,
so no reachable exploit was demonstrated, but retaining a known I/O-safety
warning in the installer runtime was not justified. The remaining audit output
contains transitive maintenance/yank warnings; they are reported but are not
known vulnerabilities or unsoundness findings.

## User verification

First compare a download with `SHA256SUMS` from the same release:

```sh
sha256sum -c SHA256SUMS --ignore-missing
```

On a `signed` release, inspect the macOS Developer ID identity and notarization
ticket. These commands are expected to reject an `unsigned-evaluation` build:

```sh
codesign --verify --deep --strict --verbose=2 /Applications/ClearSign.app
spctl --assess --type execute --verbose=4 /Applications/ClearSign.app
xcrun stapler validate /Applications/ClearSign.app
```

On a `signed` release, inspect the Windows installer in PowerShell before running
it. An `unsigned-evaluation` build must report `NotSigned` instead:

```powershell
Get-AuthenticodeSignature .\ClearSign_VERSION_x64-setup.exe | Format-List
```

The status must be `Valid`; inspect the displayed signer rather than accepting
an arbitrary valid certificate.

For any release asset, verify GitHub build provenance while online:

```sh
gh attestation verify PATH_TO_ASSET --repo clearsign-dev/clearsign
```

Official references: [Tauri macOS signing](https://v2.tauri.app/distribute/sign/macos/),
[Tauri Windows signing](https://v2.tauri.app/distribute/sign/windows/),
[Apple Developer membership](https://developer.apple.com/programs/whats-included/),
[Apple Developer ID certificates](https://developer.apple.com/help/account/certificates/create-developer-id-certificates/),
[Microsoft code-signing options](https://learn.microsoft.com/en-us/windows/apps/package-and-deploy/code-signing-options),
[Artifact Signing prerequisites](https://learn.microsoft.com/en-us/azure/artifact-signing/quickstart),
[GitHub artifact attestations](https://docs.github.com/en/actions/how-tos/secure-your-work/use-artifact-attestations/use-artifact-attestations),
[protected environments](https://docs.github.com/en/actions/reference/workflows-and-actions/deployments-and-environments),
and [immutable releases](https://docs.github.com/en/code-security/concepts/supply-chain-security/immutable-releases).
