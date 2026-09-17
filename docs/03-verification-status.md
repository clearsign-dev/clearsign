# 03 — Verification status

**What has been demonstrated, how, and what has not.** Updated 17 Sep 2026. Everything here is reproducible from the repository.

## Summary

| Area | Evidence | Status |
|---|---|---|
| Decoder correctness | Named invariant tests, and vectors produced by Foundry `cast` rather than by `clearsign` | 32 tests passing |
| Safe MultiSend batches | Batch calldata built by `cast`; addresses and the 1,404 address-chain pairs taken from `safe-global/safe-deployments` and re-checked against their published EIP-55 strings. An ordinary batch reads WARNING and shows every inner call; a delegatecall inside a batch, an unpinned batching contract, a published address on an unpublished chain, a truncated batch and an over-long batch all stay CRITICAL or BLIND and undecoded | 14 tests passing |
| The batch tests can actually fail | Three bugs planted deliberately: decode any delegatecall target, ignore each element's operation byte, ignore the chain ID. Each was caught by a different test, then reverted | Demonstrated |
| Agreement with a second implementation | Property tests against alloy: EIP-1559, legacy, Safe EIP-712 hashes, ERC-20 and `execTransaction` calldata | 6 properties × 3,000 random cases, passing |
| Never more permissive than a strict decoder | Mutated `execTransaction` calldata: whenever `clearsign` accepts it, alloy's validating decoder must accept and re-encode it identically | Passing |
| The differential tests can actually fail | Two bugs planted deliberately: fields swapped in the Safe hash, trailing bytes tolerated. Both caught with minimal failing inputs, then reverted | Demonstrated |
| Robustness | Coverage-guided fuzzing with `cargo-fuzz`, with security properties asserted on every input: digest equals keccak of input, deterministic rendering, delegatecall never decoded, BLIND or CRITICAL always yields DO NOT SIGN | About 92 million executions, zero failures; a further 47.5 million on the batch decoder, asserting that an accepted batch accounts for every byte of its argument, and 103.8 million on the QR reader, asserting that anything it reports as a complete message really was reassembled and checked |
| Key handling | All 24 official BIP-39 English vectors. Addresses, signatures, dice-roll phrases and a signed EIP-1559 transaction compared with `cast` | 14 tests passing |
| Approval binding | Tests that signing is refused without acknowledgements, with extra or duplicate acknowledgements, and for display-only reviews | Passing |
| QR transport against the specifications | Bytewords, CRC-32, Xoshiro256**, the Walker-Vose sampler, the degree chooser, the Fisher-Yates shuffle, fragment lengths, message partitioning and part CBOR, each checked against the test vectors published in BCR-2020-012 and BCR-2024-001. A message is reassembled from the spec's own 20 published parts with two plain fragments withheld, so the fountain mixing is exercised | 14 tests passing |
| QR transport against another implementation | Requests produced by Keystone's `@keystonehq/bc-ur-registry-eth` 0.22.1 (the library MetaMask's air-gapped flow uses) are read single-part, animated, and animated with frames missed and out of order. The `eth-signature` reply this signer emits is decoded back by that same library, and the signature equals `cast wallet sign --no-hash` | 6 tests passing |
| The QR tests can actually fail | Three bugs planted: skip the Bytewords CRC-32, accept parts from a different message, ignore the address the request names. The second and third were caught; the first was **not**, which exposed a test that was passing for the wrong reason. The test was rewritten to corrupt a payload byte into a different valid word, and then it caught the bug | Demonstrated, including one test fixed because of it |
| Command-line tool | Tests of the built binary: dev guard, refusals, signature equal to `cast` | 5 tests passing |
| Never panics, statically | `clippy` denies indexing, `unwrap`, `expect`, `panic` and unchecked arithmetic in the decoder and key crates; `unsafe` is forbidden | Clean |
| Portability | Decoder and key crates build for `aarch64-unknown-none` and `thumbv7em-none-eabihf`, with no operating system | Demonstrated |
| Isolation path | Verified Alpine image boots with no network device. The static binary reviews, refuses, then signs, matching `cast` | Demonstrated, development only |
| Authority engine | Tests for information-flow tracing, mislabelled steps, escape-sequence injection, transaction steps reviewed by `clearsign`, exact approval, tampering after approval and fail-closed execution; 20,000 random plans | 18 tests passing |
| Plan wire format | A plan proposed by an untrusted planner is decoded on the deciding side. 11 tests: round trip for every action, refusal of trailing bytes, every truncation, an unknown action tag, a four-gigabyte declared length, more steps than the engine accepts, and text that is not UTF-8. One test flips every bit of a real plan and requires each mutation to be refused or to change the fingerprint | 11 tests passing |
| Agent adapter | An agent's tool-call proposal becomes a typed plan or is refused. 10 tests: an unknown tool is refused rather than approximated, nonsense arguments are refused, a step using an unproposed step is refused, an oversized proposal is refused before review, a path containing `..` is outside every allowed root and cannot be classified by where it appears to end up, editing the destination changes the fingerprint an approval binds to, and the runner will not read a file local policy calls secret even after approval | 10 tests passing |
| The planner cannot classify its own data | The example proposal claims the notes are `"sensitivity": "public"`. The adapter ignores the claim and classifies by local policy: the notes come out PERSONAL and `/home/user/.config/keys/...` comes out SECRET, which is what makes the egress finding fire | Test: `the_planner_does_not_get_to_say_how_sensitive_data_is` |
| Prompt injection blocked across compartments | The Linux compartment writes an assistant's plan into shared memory and rings the doorbell. The signer compartment decodes it, traces the flow and answers CRITICAL `SECRET_EGRESS: Secret data from #1, #4 would leave this device through a web request to "notes-backup.example"`, with fingerprint `0x63cbaec3…`. The individual steps all look routine; the plan does not | Demonstrated (`platform/sel4/linux-signer/run.py`) |
| Plan decoding robustness | `plan_wire` fuzz target with the canonical-encoding property asserted, seeded with valid plans so the fuzzer starts inside the format: coverage rose from 34 to 1,386 edges once seeded | About 56 million executions, zero failures |
| The plan tests can actually fail | Two bugs planted: ignore the step-count limit, and accept a non-canonical encoding. The first was caught. The second was **not** — and 30 million further fuzzing runs with that check disabled found no input where it would fire, because the format's fixed-width integers and length prefixes already make each plan's encoding unique. The check stays as a guard for future fields, and is documented as defensive rather than as something the tests cover | Demonstrated, with one honest gap named |
| Authority engine robustness | `cargo-fuzz` target building arbitrary plans; asserts deterministic review, no raw escape characters, exact acknowledgements accepted, approved plans run fully, and a changed plan is refused | About 6.5 million executions, zero failures |
| Image build gates can actually fail | Two violations planted in the image build: a shell copied into the root filesystem, and networking re-enabled in the kernel config. Gate 4 caught the first ("the image contains more than the signer: ./bin_sh"), gate 1 the second ("the kernel still has networking"). Both reverted | Demonstrated |
| Signer image reproducibility | Two builds of identical content produce an identical kernel (`2aa6a4b8…`) and an identical initramfs (`ae1ce532…`). This exposed a real defect: GNU cpio's `--reproducible` still recorded each file's mtime, so the same content hashed differently every build. Fixed by zeroing mtimes before archiving, in the signer image and in the Linux guest, whose root filesystem now also reproduces (`fee4afcf…`) | Reproducible on one machine (`platform/signer-image/check-reproducible.sh`) |
| Signer-only image | A Linux image containing one program. Five build-time gates refuse a kernel with networking or modules, a dynamically linked signer, more than one regular file, or anything named or containing a shell. Booted in QEMU with no network device: it reads a request produced by another wallet's implementation, decodes the transaction, refuses a key the request did not ask for, and signs with the one it did | Demonstrated (`platform/signer-image/run.py`) |
| seL4 compartments | Microkit 2.3.0 in QEMU: untrusted wallet UI and bare-metal signer. Signer returns CRITICAL with the `cast`-matching Safe hash; the UI's write to the review region raises a seL4 permission fault | Demonstrated (`platform/sel4/signer-system/run.py`) |
| Linux inside seL4 | libvmm 0.2.0 runs a Linux 7.1 guest beside the signer. A Linux program gets a CRITICAL review over shared memory; Linux then uses kernel privileges to write the review region and seL4 blocks it | Demonstrated (`platform/sel4/linux-signer/run.py`) |
| Linux guest from verified source | Kernel 6.18.52 LTS from kernel.org, signature checked against Greg Kroah-Hartman's key from kernel.org's key directory; BusyBox 1.37.0 and glibc 2.41 from apt-verified Debian packages; every seL4 Linux-compartment check passes on it | Demonstrated |
| Guest kernel reproducibility | Clean rebuild in a brand-new build volume produced a bit-for-bit identical kernel Image (`d30fb888…f18f`), using fixed build identity and timestamp | Reproducible on one machine (`platform/guest/check-reproducible.sh`) |
| Central configuration | Every borrowed component is fetched and verified from `config/pins.env` by `platform/fetch-deps.sh`; `platform/run-all.sh` builds and tests everything | Demonstrated |
| GrapheneOS source | Release `2026091000` manifest tag signature verified against a pinned key fingerprint; 1,057 projects pinned to commit hashes | Verified |
| GrapheneOS hardened_malloc | Built at the manifest-pinned commit; full upstream suite passes; deployment variant runs as the system allocator in the seL4 Linux compartment and aborts a write-after-free that glibc misses | Demonstrated |
| Reproducible build, canonical environment | Two builds in the pinned Debian image, differing in build path, locale, timezone, umask, hostname and container instance, produce identical `aarch64-unknown-linux-musl` binaries (`66a86f32…` and `c3d968df…`, recorded in `signing-core/EXPECTED-HASHES.txt`). The signer image embeds that exact binary and the build refuses any other | Reproducible in the canonical environment (`scripts/reproducible-cross-check.sh`) |
| Reproducibility across compiler hosts | **Not achieved, and measured rather than assumed.** The same source and the same Rust 1.98.1, hosted on macOS instead of Linux, produces identical `.rodata`, `.data`, `.got`, `.gcc_except_table` and `.init_array`, and a different `.text` and `.comment`. rustc makes no cross-host determinism promise, so, as GrapheneOS, Tor Browser and Debian do, the project names one canonical environment instead of claiming more | Documented, with the evidence |
| Reproducible on a second machine | Still open. Everything above ran on one computer. `.github/workflows/verify.yml` performs exactly this check on GitHub's runners the first time the repository is pushed | v1 exit criterion, not yet met |

## Fuzzing run, 16 Sep 2026

Five minutes per target, run in parallel on Apple Silicon.

| Target | Executions | Coverage edges | Failures |
|---|---|---|---|
| `rlp_decode` | 58,879,305 | 159 | 0 |
| `tx_bytes` | 28,741,121 | 916 | 0 |
| `safe_tx` | 4,431,584 | 743 | 0 |
| `plan` (authority engine, 4 minutes) | 6,481,563 | 2,092 | 0 |

## What has NOT been verified

- **No external security review.** Nothing here substitutes for one. Do not use with real funds.
- **Android is not running as a compartment.** GrapheneOS is verified at source level and its allocator runs in the Linux compartment; building Android needs an x86_64 Linux host.
- **No hardware.** Every result is from software on a general-purpose computer. Side channels, fault injection and physical attacks are untested.
- **Reproducibility on a second machine.**
- **Long fuzzing campaigns.** Five minutes per target is a baseline, not assurance.
- **The Safe versions covered are v1.1.x and v1.3.0 and later.** Safe v1.0.0 uses a different struct and is unsupported.
- **The `bip39` crate's internal word indices are not zeroised.**

## How to reproduce

```sh
cd signing-core
cargo test --workspace --release                 # all tests, including alloy properties
cargo clippy --workspace --all-targets -- -D warnings
./scripts/reproducible-build.sh
cd fuzz && cargo fuzz run safe_tx -- -max_total_time=300
cd ../.. && python3 vm/dev_signer_demo.py        # after vm/fetch-alpine.sh and the musl build
```
