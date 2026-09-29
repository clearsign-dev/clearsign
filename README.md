# ClearSign

ClearSign reviews Safe and supported EVM transactions locally. It decodes signed fields, recomputes transaction hashes and flags operations such as delegatecalls, Safe administration changes and unlimited approvals.

[Website](https://clearsign-dev.github.io/clearsign.dev/) · [Downloads](https://github.com/clearsign-dev/clearsign/releases) · [Supported formats](docs/10-what-is-supported.md) · [Verification status](docs/03-verification-status.md)

## Status

**Developer preview. Do not use the signing tools with real keys or funds.**

The latest published download is v0.1.1. Later fixes on `main` are not included
in those assets; consult the [changelog](CHANGELOG.md) when choosing a version.

The desktop reviewer does not need keys and does not sign transactions. It is a second opinion, not a trusted display: a compromised computer can alter what you see. Compare the recomputed hash with the signing device and follow your existing approval procedure. A successful decode is not a safety assessment of the destination contract.

The dedicated signer and seL4 platform are experimental and have been tested in emulation, not on production hardware. Installers are not yet code-signed. The [verification record](docs/03-verification-status.md) distinguishes the reported human review from later AI-assisted checks and records the outstanding review and hardware work.

## Try the Reviewer

Download the desktop application for macOS, Windows or Linux from [Releases](https://github.com/clearsign-dev/clearsign/releases), or build the CLI:

```sh
cd signing-core
cargo build --release --locked -p clearsign-cli
./target/release/clearsign safe-json \
  crates/clearsign-cli/tests/fixtures/bybit-safe-tx.json --chain-id 1
```

This historical fixture contains the Bybit attack transaction. The reviewer reports `SAFE_DELEGATECALL` and exits with code `3`. It is a retrospective test, not a claim that ClearSign prevented the incident.

For transaction review, exit `0` means no BLIND or CRITICAL finding was raised; warnings may still be present. Exit `2` reports BLIND content and exit `3` reports a CRITICAL finding. Malformed inputs are refused. See [Using it before you sign](docs/06-using-it-before-you-sign.md) for the workflow.

## Scope

- EIP-1559 and legacy EVM transaction formats, with canonical RLP checks.
- Safe transaction hashes for supported domain versions, including the inner call and operation type.
- ERC-20-shaped `transfer`, `transferFrom` and `approve` calls. Matching a selector does not establish the target's identity or execution behaviour.
- Safe administration calls and MultiSend batches at listed address-chain pairs. Display and parsing limits are explicit; not every call in a large batch is shown.
- Uniform Resources and EIP-4527 transport in the development signing tools.
- An experimental authority engine for reviewing proposed agent actions.

Unsupported formats and unknown calls are refused or marked BLIND. Bitcoin, Solana and general EIP-712 typed data are outside the current scope. The [support matrix](docs/10-what-is-supported.md) separates implemented support, test coverage and external comparisons; a listed chain is not a tested wallet integration.

## Network Access

The Rust decoder does not access the network. Pasted and dropped records are reviewed locally. The optional fetch-by-hash feature contacts Safe's transaction service; the returned fields are reviewed locally and its `dataDecoded` description is ignored. The experimental signer image is built without a network stack.

## Development and Reports

[CONTRIBUTING.md](CONTRIBUTING.md) covers builds, tests and pull requests. [The review package](docs/05-review-package.md) introduces the trust boundaries. [CHANGELOG.md](CHANGELOG.md) records release changes and known limitations.

Report ordinary bugs through [GitHub issues](https://github.com/clearsign-dev/clearsign/issues/new/choose). Report vulnerabilities privately through [GitHub security advisories](https://github.com/clearsign-dev/clearsign/security/advisories/new). Do not include recovery phrases, private keys or confidential transaction data.

## Licence

MIT OR Apache-2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
