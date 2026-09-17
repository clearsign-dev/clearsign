# 05 — For a security reviewer

**What this is.** Everything a reviewer needs to attack this project, in one
place: what it claims, where the trust boundaries are, what is deliberately out
of scope, and how to reproduce every claim. Written 17 Sep 2026.

Nothing here has been reviewed outside the project. This document exists to make
that cheaper to fix.

---

## 1. What is worth your time

The project is a signer and an approval engine. Its whole value is one property:

> **A person sees what will happen, and only what they saw can happen.**

Everything else is in service of that. The attacks that matter are the ones that
break it. Six claims, in the order we would be most embarrassed to have wrong:

| # | Claim | How to falsify it |
|---|---|---|
| C1 | The review and the signature always describe the same thing | Find input where the rendered text and the signed digest disagree |
| C2 | A BLIND or CRITICAL finding cannot be bypassed | Obtain a signature without the exact, step-scoped acknowledgement |
| C3 | A plan cannot change between approval and execution | Make the executor run something the approved fingerprint did not cover |
| C4 | An untrusted planner cannot classify its own data | Get secret data to a network step without `SECRET_EGRESS` firing |
| C5 | Hostile input cannot panic, hang or exhaust a parser | Crash, wedge or balloon any decoder |
| C6 | The signer image contains only the signer | Find a second executable path, a network stack, or a shell in it |

## 2. Trust boundaries

```
  UNTRUSTED                          |  TRUSTED
  ---------------------------------- | ------------------------------------
  a wallet interface                 |  clearsign: decodes the signed bytes
  an AI planner or agent framework    |  authority: types, traces, classifies
  the Linux compartment (whole OS)    |  the signer protection domain
  scanned QR codes                    |  clearsign-qr: parses them
  the computer that prepares a tx     |  the person reading the review
```

Everything on the left is assumed hostile and may be perfectly formed, malformed,
or actively adaptive. Nothing on the left is ever asked what something *means* —
only what bytes it wants reviewed.

The person is trusted to read. The project's honest limit: an approval screen
cannot protect someone who approves without reading. It can only make the risk
specific, which is why findings name the destination, the amount and the step.

## 3. Where to look first

| Surface | Why it is interesting | Code |
|---|---|---|
| EVM and Safe decoding | The Bybit attack is the motivating case: a delegatecall dressed as a transfer | `signing-core/crates/clearsign/src/{evm,safe,calls,abi,rlp}.rs` |
| MultiSend batches | The only delegatecall the decoder will look inside, gated on a pinned address *and* a published chain | `crates/clearsign/src/multisend.rs` |
| The signing-target binding | The digest is attached only by the two functions that recompute it from reviewed bytes; the field is private | `crates/clearsign/src/review.rs`, `evm.rs:244`, `safe.rs:134` |
| QR transport | Parses a stream from a camera: Bytewords, UR, fountain codes, CBOR, EIP-4527 | `crates/clearsign-qr/src/*.rs` |
| Plan wire format | An untrusted planner's proposal, decoded on the deciding side | `crates/authority/src/wire.rs` |
| Information-flow tracing | The prompt-injection defence | `crates/authority/src/flow.rs`, `policy.rs` |
| The C in the compartments | Shared memory with an untrusted domain, barriers, a private copy before decode | `platform/sel4/*/signer.c` |
| The FFI and its arena | The only `unsafe` in the signer; a bump allocator with no free | `crates/clearsign-ffi/src/lib.rs` |

## 4. Known limits, stated so you do not waste time confirming them

These are real, known and written down elsewhere in `docs/`. Finding them again
is not a finding; finding something *worse* about them is.

- **Everything runs under emulation.** No hardware root of trust, no verified
  boot, no secure element. A VM signer protects nothing against the host.
- **The seL4 SDK key and the GrapheneOS key are pinned on first use.** Neither
  project publishes a second channel to confirm them.
- **MultiSend decoding trusts an address on a chain.** The signer cannot read
  chain state, so it confirms that the address is a published deployment for the
  stated chain, not that the code there is what was published.
- **The Linux compartment has a root shell** on the debug serial port. That
  compartment is for running ordinary software, not for custody; the
  signer-only image is the one with nothing in it.
- **`authority-agent` does not resolve symlinks.** A path is judged as written.
- **No formal verification of our own code.** seL4 is proven; `clearsign` is not.

## 5. Reproducing every claim

```sh
# everything: tests, lints, fuzzing corpora, bare-metal builds
cd signing-core && cargo test --workspace --release --locked
cargo clippy --workspace --all-targets --locked -- -D warnings

# the parsers, against hostile input
cargo +nightly fuzz run rlp_decode     # and tx_bytes, safe_tx, multisend,
                                       # plan, plan_wire, ur_decode

# the same bytes, built in the canonical environment
./scripts/reproducible-cross-check.sh   # compares against EXPECTED-HASHES.txt

# the signer image: no shell, no network stack, signer as process 1
cd ../platform && ./signer-image/build.sh && python3 signer-image/run.py

# seL4: an untrusted Linux compartment proposing, a signer compartment deciding
./sel4/linux-signer/build.sh && python3 sel4/linux-signer/run.py

# everything at once
./run-all.sh
```

`docs/03-verification-status.md` states, for each claim, what evidence exists —
including the places where the answer is "this is defensive, and no test reaches
it".

## 6. What we found ourselves, before asking you

An adversarial pass on 17 Sep 2026, listed so you can judge the quality of our
own review rather than repeat it:

| Finding | Where | Fix |
|---|---|---|
| Review published to shared memory without a release barrier: on a weakly ordered machine the reader could see the ready flag before the text | `platform/sel4/*/signer.c` | Acquire fence on entry, release fence before publishing |
| Shared memory read through non-`volatile` pointers | same | Both regions are now `volatile` |
| The wallet-UI demo decoded shared memory in place, so the untrusted domain could change bytes mid-review (TOCTOU) | `platform/sel4/signer-system/signer.c` | Private copy before decoding, as the other compartment already did |
| The QR decoder held mixed fountain parts without limit: a stream of crafted codes could reach tens of megabytes on a device with far less | `crates/clearsign-qr/src/ur.rs` | Bounded by the fragment count; solving *n* unknowns never needs more than *n* equations. Regression test included |
| Process 1 could panic on arithmetic in QR rendering, which on the signer image is a kernel panic | `crates/clearsign-device/src/main.rs` | Saturating arithmetic, and the no-panic lints now apply to that binary too |
| Unknown: how close a maximal request comes to exhausting the 4 MiB arena | `crates/clearsign-ffi` | Measured: worst case 1.57 MB. A budget test now fails at half the arena |

Two earlier passes found: a public `signing_hash` field that let a caller display
one transaction and sign another; a `Lines` iterator holding stdin so the signer
hung forever at the phrase prompt; a build that was not reproducible because cpio
recorded mtimes; three test-suite gaps where a planted bug went uncaught.

## 7. How to report

Findings by whatever channel you and the maintainer agreed. Please include the
smallest input that demonstrates it, and say which of C1-C6 it breaks, or that
it breaks something the list should have contained.
