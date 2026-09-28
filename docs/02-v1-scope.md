# 02 — Version 1 scope, frozen

> **Progress, 16 Sep 2026 (second pass):**
> - **Phase A** items 1 to 6 are implemented and hardened: invariant tests, differential tests against Foundry `cast` and alloy, and coverage-guided fuzzing.
> - **Phase B** item 7 is done (17 Sep 2026): `platform/signer-image/` builds a Linux image whose entire userland is the signer, with no shell, no second program and a kernel built without a network stack. The earlier Alpine prototype remains as a development convenience.
> - **Phase C** items 8, 9 and 10 are implemented: BIP-39 and BIP-32 key handling, dice-roll and mixed-entropy seed generation, signing for EIP-1559, legacy and Safe transactions, and air-gapped QR transport (17 Sep 2026) using Uniform Resources and EIP-4527 — the format MetaMask and Keystone already speak.
> - **Exit criteria:** invariant tests, differential testing and fuzzing are done in baseline form. Reproducible builds are proven on one machine, not yet two. There has been no external review.
>
> **Scope change made 17 Sep 2026, still [needs sign-off]:** item 2 now decodes Safe MultiSend batches, but only at addresses Safe publishes *and* only on the chains Safe publishes them for. Without this, every batch transaction was flagged CRITICAL, and a warning that always fires is ignored — which is blind signing with extra steps.
>
> This file's rule is that anything added must be paid for by something removed. The payment: **the decoder's selector set is now closed for v1.** No Permit2, no Uniswap, no bridges, no token metadata, no further batching contracts. `multiSend(bytes)` is the last selector v1 learns.

**Status:** DRAFT freeze, 16 Sep 2026. **[needs sign-off]**

The history study found that scope creep and stacked research risks killed Copland, the Hurd and Coyotos. This list is the defence. Adding anything requires removing something, in writing, in this file.

## In scope

### Phase A — the `clearsign` library (started)
1. Parse unsigned EVM transactions, the payload a signer is asked to sign: EIP-1559 (type 2) and legacy, with strict canonical RLP.
2. Decode calldata for a fixed set of high-value actions:
   - ERC-20 `transfer`, `transferFrom`, `approve`, including unlimited-approval detection
   - Safe `execTransaction`, including the inner call and its operation type
   - Safe administration: `changeMasterCopy`, `addOwnerWithThreshold`, `removeOwner`, `swapOwner`, `changeThreshold`, `enableModule`, `disableModule`, `setGuard`, `setFallbackHandler`
3. Compute the Safe transaction hash locally for out-of-band comparison.
4. Risk findings with three severities: INFO, WARNING, CRITICAL, plus BLIND for anything not understood.
5. Deterministic plain-text rendering.
6. A command-line tool so auditors can use the library with no device at all.

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
- EIP-712 typed data other than the Safe transaction type. **Noted 27 Sep 2026: this is the exclusion most likely to age badly, and it should be the first thing considered for v2.** The reasoning for leaving it out has not changed — typed data is an open-ended schema, and displaying a structure this device does not understand is exactly what it refuses to do. What has changed is the direction of travel: Ethereum's roadmap has work aggregated off-chain before it reaches a block, so what a person signs drifts from a concrete call toward an intent that something else expands later. The further that goes, the more of what happens is *not* in the bytes being signed, and the wider the gap this project exists to close. Today the device refuses typed data, which is honest and increasingly limiting
- Permit2, Uniswap, bridges and other protocol decoders
- Token names, symbols, decimals and address books
- Any network connectivity, updates over the air, or companion cloud service
- A graphical interface beyond a single text screen
- Our own kernel, our own cryptography, our own programming language
- Phones, GrapheneOS integration, Qubes integration. These are vehicles for after v1.
- Hardware. Chosen only after the library and development vehicle work.
