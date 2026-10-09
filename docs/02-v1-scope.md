# 02 — Version 1 scope, frozen

> **Progress, updated 9 Oct 2026:**
> - **Phase A** items 1 to 6 are implemented and hardened: invariant tests, differential tests against Foundry `cast` and alloy, and coverage-guided fuzzing.
> - **Phase B** item 7 is done (17 Sep 2026): `platform/signer-image/` builds a Linux image whose entire userland is the signer, with no shell, no second program and a kernel built without a network stack. The earlier Alpine prototype remains as a development convenience.
> - **Phase C** items 8, 9 and 10 are implemented: BIP-39 and BIP-32 key handling, dice-roll and mixed-entropy seed generation, signing for EIP-1559, legacy and Safe transactions, and air-gapped QR transport (17 Sep 2026) using Uniform Resources and EIP-4527 — the format MetaMask and Keystone already speak.
> - **Exit criteria:** invariant tests, differential testing and fuzzing are done in baseline form. Reproducible builds hold in the named canonical environment and were repeated on a GitHub runner. The September review predates later signing-critical changes, which still require independent review.
>
> **Scope change accepted 17 Sep 2026:** item 2 now decodes Safe MultiSend batches, but only at addresses Safe publishes *and* only on the chains Safe publishes them for. Without this, every batch transaction was flagged CRITICAL, and a warning that always fires is ignored — which is blind signing with extra steps.
>
> This file's rule is that anything added must be paid for by something removed. The payment: **the decoder's selector set is now closed for v1.** No Permit2, no Uniswap, no bridges, no token metadata, no further batching contracts. `multiSend(bytes)` is the last selector v1 learns.
>
> **Scope change accepted 9 Oct 2026: the selector set is reopened, and EIP-712 review is added.** The closed set was measured against real attacks and found to report the transactions behind Radiant Capital (Oct 2024), the 2022 Uniswap V3 phishing, Badger DAO and the 2025 EIP-7702 drains as BLIND — the same answer it gives to more than half of ordinary multisig traffic. A refusal that cannot be told apart from routine is not a warning. Added: contract administration (ownership, roles, proxy upgrades and admins, DSAuth `setOwner`), token permissions beyond `approve` (`increaseAllowance`, `increaseApproval`, `setApprovalForAll`, Permit2 `approve`), calls that carry calls (`multicall`, timelock `schedule` and `execute`, smart-account `execute(bytes32,bytes)`), the EIP-2930, EIP-4844 and EIP-7702 transaction types, and EIP-712 typed-data *review* (not signing) for permits, Permit2 and Safe transactions, with every other structure shown and BLIND.
>
> **Payment for the change:** the experimental authority engine is frozen at its current state and removed from the v1 release criteria. No further protocol-specific readers are admitted to v1: still no Uniswap, bridges or token metadata. Unknown calls remain BLIND. This keeps the transaction reviewer, not the agent-plan experiment, as the v1 product boundary.

**Status:** frozen for v1, 9 Oct 2026.

The history study found that scope creep and stacked research risks killed Copland, the Hurd and Coyotos. This list is the defence. Adding anything requires removing something, in writing, in this file.

## In scope

### Phase A — the `clearsign` library (started)
1. Parse unsigned legacy, EIP-2930, EIP-1559, EIP-4844 and EIP-7702 transactions with strict canonical RLP.
2. Decode calldata for a fixed set of high-value actions:
   - ERC-20 `transfer`, `transferFrom`, `approve`, including unlimited-approval detection
   - Safe `execTransaction`, including the inner call and its operation type
   - Safe administration: `changeMasterCopy`, `addOwnerWithThreshold`, `removeOwner`, `swapOwner`, `changeThreshold`, `enableModule`, `disableModule`, `setGuard`, `setFallbackHandler`
   - contract ownership, role, proxy implementation and proxy-admin changes
   - token permissions including allowance changes, operator approvals and Permit2 `approve`
   - pinned Safe MultiSend, plus best-effort inner review for unverified `multicall`, timelock and smart-account carrier selectors; the unverified carrier itself remains BLIND
3. Compute the Safe transaction hash locally for out-of-band comparison.
4. Risk findings with three severities: INFO, WARNING, CRITICAL, plus BLIND for anything not understood.
5. Review EIP-712 permits, Permit2 and Safe transaction structures from the CLI. A matching schema is not proof of contract behaviour, so unverified semantic interpretations remain BLIND; the canonical Permit2 address is the pinned exception.
6. Deterministic plain-text rendering and a command-line tool usable without a device.

### Phase B — the development vehicle
7. A minimal Linux image under QEMU on Apple Silicon running the signer, with networking removed at build time — not merely unconfigured, but absent from the kernel. The signer is process 1 and the only program in the image. **Development only:** an emulator has no secure element and no verified boot.

### Phase C — keys and transport
8. BIP-39 and BIP-32 key handling using audited, borrowed crates.
9. Signing EIP-1559 transactions and Safe transaction hashes.
10. Air-gapped transport over QR codes using existing open formats: Uniform Resources (BCR-2020-005/012, multipart fountain codes BCR-2024-001) carrying EIP-4527 `eth-sign-request` in and `eth-signature` out. Requests for anything other than a transaction are refused, and a signature is only produced by the key the request names.

### Exit criteria for v1
- Every invariant in the threat model has a named test.
- Differential test against an independent reference, currently Foundry's `cast`.
- Fuzzing of every parser with no crashes over an agreed run length.
- Reproducible build demonstrated on two machines. *(**Met, 28 Sep 2026.** Reproducible in a named canonical environment, twice, varying everything that must not matter — and then reproduced byte-for-byte on a GitHub runner, which is the second machine. Cross-compiler-host reproducibility was tested separately and does not hold, which is a different claim the project does not make — see `03-verification-status.md`.)*
- External security review completed before any use with real funds.

## Explicitly out of scope for v1

- Bitcoin and PSBT. The design keeps room for it, but it is not in v1.
- EIP-712 signing and desktop ingestion. CLI review covers the structures listed above; all other structures are displayed field by field and marked BLIND.
- Uniswap, bridges and protocol-specific decoders beyond the listed Permit2 operations
- Token names, symbols, decimals and address books
- Any network connectivity, updates over the air, or companion cloud service
- Wallet-integrated approval flows and a general transaction-building interface
- Our own kernel, our own cryptography, our own programming language
- Phones, GrapheneOS integration, Qubes integration. These are vehicles for after v1.
- Hardware. Chosen only after the library and development vehicle work.
- Further authority-engine development. The existing experiment is not a v1 release criterion.
