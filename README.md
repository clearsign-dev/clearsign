# ClearSign

ClearSign reviews Safe and supported EVM transactions locally. It decodes signed fields, recomputes transaction hashes and flags operations such as delegatecalls, Safe administration changes, contract ownership and upgrade changes, and unlimited approvals and permits.

[Website](https://clearsign-dev.github.io/clearsign.dev/) · [Downloads](https://github.com/clearsign-dev/clearsign/releases) · [Supported formats](docs/10-what-is-supported.md) · [Verification status](docs/03-verification-status.md) · [Benchmarks](docs/11-benchmarks.md)

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

- EVM transaction formats: legacy, EIP-2930, EIP-1559, EIP-4844 and EIP-7702, with canonical RLP checks. Each EIP-7702 authorization is flagged, because it hands an account to code.
- Safe transaction hashes for supported domain versions, including the inner call and operation type.
- Token calls shaped like `transfer`, `transferFrom`, `approve`, `increaseAllowance`, `setApprovalForAll` and Permit2 `approve`. Matching a selector does not establish the target's identity or execution behaviour.
- Contract administration: ownership transfers, role grants, proxy upgrades and admin changes, on any contract a Safe or an account calls.
- Calls that carry other calls — Safe MultiSend batches at listed address-chain pairs, `multicall`, timelock operations and smart-account batches (ERC-7579 / ERC-7821) — with every carried call judged by the same rules. Display and parsing limits are explicit; not every call in a large batch is shown.
- EIP-712 typed data, reviewed from the command line: permits, Permit2 and Safe transactions are interpreted, and any other structure is shown field by field and marked BLIND. Typed data is not yet read by the window or signed by the signer.
- Uniform Resources and EIP-4527 transport in the development signing tools.
- An experimental authority engine for reviewing proposed agent actions.

Unsupported formats and unknown calls are refused or marked BLIND — which is most DeFi activity, by design. Bitcoin, Solana and chain-specific transaction types (Celo's fee-currency transactions, zkSync's EIP-712 transactions) are outside the current scope. The [support matrix](docs/10-what-is-supported.md) separates implemented support, test coverage and external comparisons; a listed chain is not a tested wallet integration.

## Measured

[Benchmarks](docs/11-benchmarks.md), generated from the measurements, record what ClearSign does on real data and where it stops:

- On 9,998 real transactions from 31 chains it committed to exactly the digest each sender signed, every time. On 10,851 real Safe transactions from 27 chains its hash agreed with Safe's service and with an independent implementation every time, and every owner signature recovered over it.
- Replayed from the chain, 13 attacks that deceived signers — Bybit, WazirX, Radiant, Badger and the 2025 EIP-7702 drains among them: the mechanism was named in 9 (up from 3), one more was refused as unreadable, one drew a warning, one plain token transfer was shown faithfully but not flagged, and one used a format ClearSign does not read. Address-poisoning and impersonation cases are not caught: those transactions say exactly what they do.
- Of $21 billion lost across DeFiLlama's 1,293 recorded hacks, about 10% went through a deceived signer. Most of the rest was key theft or contract bugs, which no signing reviewer can see. ClearSign addresses one layer of the problem, not the whole of it.

## Network Access

The Rust decoder does not access the network. Pasted and dropped records are reviewed locally. The optional fetch-by-hash feature contacts Safe's transaction service; the returned fields are reviewed locally and its `dataDecoded` description is ignored. The experimental signer image is built without a network stack.

## Development and Reports

[CONTRIBUTING.md](CONTRIBUTING.md) covers builds, tests and pull requests. [The review package](docs/05-review-package.md) introduces the trust boundaries. [CHANGELOG.md](CHANGELOG.md) records release changes and known limitations.

[Release and pilot readiness](docs/13-release-readiness.md) separates the current
reviewer from the independent review, hardware validation and customer evidence
still needed before high-value deployment.

Report ordinary bugs through [GitHub issues](https://github.com/clearsign-dev/clearsign/issues/new/choose). Report vulnerabilities privately through [GitHub security advisories](https://github.com/clearsign-dev/clearsign/security/advisories/new). Do not include recovery phrases, private keys or confidential transaction data.

## Licence

MIT OR Apache-2.0. See [LICENSE-MIT](LICENSE-MIT) and [LICENSE-APACHE](LICENSE-APACHE).
