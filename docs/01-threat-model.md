# 01 — Threat model

**Status:** DRAFT v0.1, 16 Sep 2026. Covers the `clearsign` decoder and the signing device it runs on. Revisit whenever scope changes.

The history study's rule applies here more than anywhere: **be crude in features, never in the trust claim.** Every property below is either tested, or explicitly listed as not yet provided.

---

## 1. What we protect

| Asset | Why it matters |
|---|---|
| **A1. Signing intent** | The user's decision must match what the signature actually authorises. This is the asset Bybit lost. The keys were never stolen. |
| **A2. Private keys and seed** | Loss means irreversible theft. |
| **A3. Release integrity** | If our build or signing keys are compromised, every user is compromised at once. |
| **A4. The user's exit** | If the project dies, users must still be able to move their funds using standard formats. |

## 2. Who attacks, and what they control

| ID | Adversary | Controls | In scope for v1 |
|---|---|---|---|
| **T1** | Compromised online computer | The machine that builds the transaction: its browser, its display, its clipboard, its OS. Includes Triada-class malware in the system partition. | **Yes. The primary adversary.** |
| **T2** | Compromised web interface or dapp | Everything the user sees before the transaction reaches the signer, as in Bybit | **Yes. The primary adversary.** |
| **T3** | Malicious transaction author | Crafts calldata to look harmless: nested calls, delegatecall, proxy upgrades, approvals, look-alike addresses | **Yes** |
| **T4** | Malformed-input attacker | Sends malformed or non-canonical encodings to crash the decoder or make two decoders disagree | **Yes** |
| **T5** | Supply chain | A dependency, compiler or build machine inserts code | **Partly.** Minimal dependencies, pinned toolchain, lockfile. Reproducible builds are a v1 exit criterion. |
| **T6** | Physical thief or evil maid | Has the device for minutes or hours | **Partly.** Covered by the device, not the decoder. Detailed in the device threat model later. |
| **T7** | Lab-grade physical attacker | Side channels, fault injection, decapping | **No.** An explicit non-goal for v1. |
| **T8** | Coercion | Forces the user to sign | **No** for v1. Duress features are a later item. |
| **T9** | The project itself | Our release keys, our infrastructure, our continued existence | **Yes.** Release keys under 2-of-3 control, and a documented exit path. |

## 3. Trust boundaries

```
 ┌──────────── UNTRUSTED ─────────────┐        ┌──────────────── TRUSTED ────────────────┐
 │ online computer, browser, dapp UI, │ bytes  │ signing device                          │
 │ wallet software, clipboard, network│ ─────► │  transport (QR / USB, no network stack) │
 │                                    │        │     │                                   │
 │  may lie about EVERYTHING,         │        │     ▼                                   │
 │  including what the tx does        │        │  clearsign: decode ONLY the signed bytes│
 └────────────────────────────────────┘        │     │                                   │
                                               │     ▼                                   │
                                               │  trusted display → human decision       │
                                               │     │                                   │
                                               │     ▼                                   │
                                               │  key store → signature                  │
                                               └─────────────────────────────────────────┘
```

**The single rule behind the boundary:** nothing that arrives from the untrusted side may influence what the trusted display says, except the bytes that are actually signed.

## 4. Security invariants

These are the properties the code must hold. Each gets a test ID in the source.

| ID | Invariant | Defends against |
|---|---|---|
| **INV-1** | Every value shown is derived only from the bytes being signed. No labels, token names or "descriptions" supplied by the untrusted side are ever displayed. | T1, T2 |
| **INV-2** | Nothing is signed silently. Anything the decoder cannot fully interpret produces an explicit BLIND finding. | T2, T3 |
| **INV-3** | Decoding is deterministic. The same bytes produce the identical display on every build and platform. | T5, auditor verification |
| **INV-4** | Non-canonical or trailing-garbage encodings are rejected rather than tolerated. | T4 |
| **INV-5** | For a Safe transaction, the Safe transaction hash is computed locally and displayed, so signers can compare it out of band with each other. | T1, T2 |
| **INV-6** | High-risk actions are flagged CRITICAL: delegatecall, changes to the Safe implementation, owners, threshold, modules, guards or fallback handler, and unlimited token approvals. | T3 |
| **INV-7** | The decoder never panics on any input. Every failure is a typed error. | T4 |
| **INV-8** | The signer image contains no network stack. | T1 |
| **INV-9** | Release builds are reproducible from public source. | T5, T9 |
| **INV-10** | Nothing can be signed except a digest attached to a review. The digest is recomputed from the reviewed bytes, never taken from the caller, and signing requires acknowledging **exactly** the set of BLIND and CRITICAL findings. There is no API that signs a raw hash. | T1, T2, T3 |
| **INV-11** | No seed is ever generated from a hardware random number generator alone. Seeds come from at least 99 dice rolls, or from hardware entropy mixed with at least 50 dice rolls, so a silently broken generator cannot compromise them. | T5 and the Coldcard failure class |
| **INV-12** | Every signature is verified by public-key recovery, and checked to be low-S, before it is released. | T5, fault injection |

## 5. Explicit non-goals

State these publicly. Overclaiming is how Subgraph died.

- **Correctly displayed but malicious transactions.** If the screen honestly says "delegatecall to an unknown contract" and the user approves it, we have done our job.
- **VM builds.** A signer running in a virtual machine on a compromised host protects nothing. VM builds exist only for development and are labelled so.
- **Lab-grade physical attacks and coercion**, for v1.
- **Token symbols and decimals.** These cannot be derived from the signed bytes. v1 shows raw token addresses and raw integer amounts. A signed, pinned token registry is a later item.
- **Address book names**, for the same reason.

## 6. Known open risks

| Risk | Current state | Plan |
|---|---|---|
| Decoder bug shows a benign summary of a malicious call | Invariant tests, differential tests against Foundry `cast` and alloy, and about 92 million fuzzing executions with security-property assertions. See `03-verification-status.md` | Longer fuzzing campaigns in CI, external audit before real funds |
| A batch is decoded on the strength of its address, and the signer cannot read the code deployed there | Implemented 17 Sep 2026: a DELEGATECALL is decoded only when its target is one of the 11 published Safe MultiSend deployments, taken from `safe-global/safe-deployments`, and the review says in words that the address is confirmed and the deployed code is not. Every other DELEGATECALL stays CRITICAL and undecoded | Hardened the same day: decoding also requires the chain ID to be one Safe publishes that deployment on (1,404 address-chain pairs), so a published address on an unlisted chain is CRITICAL and undecoded. Residual risk: a chain Safe lists where the address was later self-destructed or re-created with other code. Mitigate by pinning per-chain code hashes, which Safe also publishes, once the signer can be given chain state by a channel it trusts. **This looks less permanent than it did (noted 27 Sep 2026, source in `04-platform-architecture.md` §5a).** Ethereum's stated direction is SNARK-verified blocks and much lighter node requirements, which makes a verifier small enough to sit inside a signing device plausible rather than fanciful — at which point "the signer cannot read chain state" stops being an assumption and becomes a thing we chose not to build yet |
| A malicious wallet sends a signing request the device cannot really read | The QR reader accepts only `eth-sign-request`, refuses everything that is not a transaction (typed data, `personal_sign`), refuses a derivation path the device does not hold, and refuses to sign when the address the request names is not the address the key produces. Every parser is bounded, total and fuzzed | Support EIP-712 typed data only when it can be displayed field by field |
| Anything else on the signing machine is a way in | The signer image is one statically linked program as process 1, with no shell, no second binary, no storage and a kernel built without a network stack. The build fails rather than produce an image that contains anything else | Hardware with verified boot, so the image that runs is the image that was built |
| Look-alike addresses | Full addresses shown, checksummed, in groups of four | Consider visual fingerprints |
| Users acknowledge warnings without reading | The acknowledgement must name each finding code exactly, which blocks blanket lists but not careless people | The device screen must require a distinct deliberate action per CRITICAL finding |
| Recovery-phrase word indices inside the `bip39` crate are not zeroised on drop | Our own buffers, seeds and keys are zeroised; the crate's internal copy is short-lived | Upstream a zeroize feature, or wrap parsing |
| Reproducible builds proven on only one machine | Reproducible in a named canonical environment — the pinned Debian image and pinned Rust — across build path, locale, timezone, umask, hostname and container instance. Hashes are recorded so anyone can compare. Builds made outside that environment, including on the maintainer's own macOS machine, differ, and the project says so rather than implying otherwise | Closed 28 Sep 2026: a GitHub runner reproduced the recorded binaries exactly |
| The signing scheme itself is assumed to be ECDSA over secp256k1 | True throughout: key derivation, the low-S rule, and the recover-and-compare self-check in INV-12 are all specific to it. The *review* layer — decoding, findings, acknowledgement, approval binding — knows nothing about the scheme, which is the part worth protecting | Ethereum's own roadmap puts "sometimes quantum-safe signature (or several), sometimes zero-knowledge proof" in its 2030 column (source and date in `04-platform-architecture.md` §5a). A device shipped before then should expect the scheme underneath it to change. Keep the review layer scheme-agnostic, and treat a second scheme as a v2 requirement rather than a rewrite |
| Single maintainer | True today | Recruit a co-maintainer before any release, and split release keys |
