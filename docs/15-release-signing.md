# Release signing and verification

This document describes the repository's release controls. It does not claim
that already-published downloads were retroactively signed. ClearSign v0.1.2 and
v0.1.3 are unsigned evaluation releases. Every future release tag requires an
explicit mode; an unset or unknown mode fails before packaging begins.

## What the workflow enforces

Branch and manual builds remain unsigned. They exercise packaging, installation
and smoke tests without consuming notarization requests or exposing signing
credentials to ordinary development builds.

Set the public repository variable `CLEARSIGN_RELEASE_MODE` to exactly one of:

- `signed`: a normal stable release. The macOS and Windows credential and
  verification gates below are mandatory.
- `unsigned-evaluation`: a temporary evaluation release. Platform signing and
  notarization credentials are not loaded, macOS and Windows artifacts are
  checked to be unsigned, `BUILD-METADATA.json` records that status, the release
  notes begin with an explicit warning, and GitHub publishes it as a prerelease.

The validation job normalizes and snapshots this mode plus the public Apple Team
ID and Windows certificate thumbprint. Every later build, metadata and publish
decision uses those job outputs, so changing a repository variable during a run
cannot make its artifacts and recorded policy disagree.

For a `v*` tag, `.github/workflows/release.yml` then fails closed:

- macOS requires a Developer ID Application certificate and App Store Connect
  API key. The workflow imports them into a temporary keychain, asks Tauri to
  sign and notarize, then runs `codesign`, Gatekeeper assessment and `stapler
  validate` against the application and DMG.
- Windows requires a publicly trusted Authenticode certificate in a PFX file.
  Tauri signs the executable, MSI, NSIS installer and NSIS uninstaller. The
  workflow rejects any missing, invalid, unexpectedly signed or un-timestamped
  file, including the executable after each installer has placed it on disk.
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

The workflow deletes imported signing material before uploading artifacts. The
runners are ephemeral, but cleanup is still explicit. GitHub Actions secrets are
available only to the native platform step that needs them.

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

Configure these GitHub Actions repository secrets:

| Secret | Value |
| --- | --- |
| `APPLE_CERTIFICATE` | Base64 of the exported Developer ID `.p12` |
| `APPLE_CERTIFICATE_PASSWORD` | Password protecting that `.p12` |
| `APPLE_API_ISSUER` | App Store Connect team API issuer UUID |
| `APPLE_API_KEY` | App Store Connect API key ID |
| `APPLE_API_KEY_P8_BASE64` | Base64 of the downloaded `.p8` private key |

Configure the public repository variable `APPLE_TEAM_ID` with the ten-character
Developer Team ID. The workflow compares it with the signed application's
`TeamIdentifier`, and the release metadata records it.

The certificate and API key are separate credentials. Revoke and replace either
immediately if its secret is exposed. Track their expiry or revocation outside
the repository and test a manual workflow build before creating a release tag.

### Windows

The implemented backend accepts a password-protected PFX containing one
publicly trusted code-signing certificate and private key. Configure:

| Secret or variable | Value |
| --- | --- |
| `WINDOWS_CERTIFICATE` | Base64 of the PFX file |
| `WINDOWS_CERTIFICATE_PASSWORD` | PFX export password |
| `WINDOWS_CERTIFICATE_THUMBPRINT` | Repository variable containing the expected 40-hex SHA-1 certificate thumbprint |
| `WINDOWS_TIMESTAMP_URL` | Optional repository variable; defaults to DigiCert's RFC 3161 service |

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
2. Set `CLEARSIGN_RELEASE_MODE` through a reviewed repository-settings change.
   Use `unsigned-evaluation` for the next evaluation tag or `signed` for a stable
   release. Never use an approximate or third value.
3. For `signed`, confirm the Apple membership, certificate and API key are active.
   Confirm the Windows certificate is active and the timestamp service is reachable.
   For `unsigned-evaluation`, confirm the release is intended only for evaluation
   and that public copy does not imply platform publisher authentication.
4. Run the release workflow manually from the intended commit. This produces an
   unsigned candidate and tests installers; it does not prove tag signing.
5. Create the tag only from the reviewed commit. Never move or reuse a release tag.
6. Require complete-candidate validation and provenance attestation to pass. In
   `signed` mode, also require signing, notarization and Authenticode checks. In
   `unsigned-evaluation` mode, require the explicit unsigned-artifact checks and
   confirm the GitHub release is marked prerelease.
7. Download the published assets onto clean macOS, Windows and Linux machines.
   Verify the mode, checksums, provenance, installation, displayed version and
   historical transaction fixture. For `signed`, also verify platform signatures.
8. Preserve the workflow run URL, source commit, `SHA256SUMS`, metadata, SBOM and
   notarization/signing evidence in the release record. Do not replace assets in
   place; issue a new version if any published byte must change.

Repository administration is part of this gate and cannot be encoded completely
in a workflow file. Create a GitHub environment named `release`, require a second
reviewer and prevent self-review; protect `v*` tags with a ruleset; and enable
immutable releases. Until those three settings are visibly active, a maintainer
account with tag and release permission remains a single-person publication
path even though the selected release mode itself fails closed.

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
