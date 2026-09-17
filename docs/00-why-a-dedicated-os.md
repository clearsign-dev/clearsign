# 00 — Why a dedicated operating system at all?

**Status:** DRAFT, written 16 Sep 2026. The decisions marked **[needs sign-off]** are assumptions you need to confirm or change.

The failure-history study ([research/os-failures-analysis.md](../research/os-failures-analysis.md)) set one test to pass before any kernel code gets written:

> Which user gets a guarantee from a dedicated OS that they could not get from an app plus hardware isolation on a platform they already own?

This document answers it. If the answer fails, the project should ship as a component inside existing platforms and never become an OS of its own.

---

## The user

**People who approve high-value on-chain transactions and cannot afford to trust the screen of the computer that prepared them.**

Concretely, three groups:

1. **Multisig signers** at protocols, DAOs, funds and exchanges who approve Safe transactions. In February 2025, Bybit's signers approved a transaction worth about $1.5 billion after the Safe{Wallet} web interface they used had been tampered with. Their hardware wallets signed a transaction the signers never saw decoded.
2. **Security-conscious individuals** holding meaningful value, who already use hardware wallets but mostly approve opaque data they cannot read.
3. **Auditors and incident responders** who need to verify, independently of any web interface, what a pending transaction really does.

**Not the target:** people who want a private everyday phone. GrapheneOS already serves them, and the history study says a standalone OS will not reach them.

## The guarantee

> **What you see on this screen is derived only from the exact bytes being signed. It was computed by code that you can reproduce bit-for-bit from public source. It ran on a machine with no network, no browser, and no other software.**

Broken down:

| Property | App on a normal phone or laptop | Hardware wallet | Dedicated signing OS |
|---|---|---|---|
| Decodes what is being signed, including nested multisig calls | Sometimes | Rarely for complex calls | **Yes, the core job** |
| Display cannot be altered by other software on the same machine | No. Malware such as Triada lives in the system partition of every process | Yes | **Yes** |
| Whole stack reproducible and auditable, with no closed secure element firmware | Partly | Mostly no | **Yes, the design goal** |
| No network attack surface | No | Yes | **Yes** |
| Can run complex, updatable decoders for new contracts | Yes | Constrained by memory and vendor release cycles | **Yes** |

The last row is what a hardware wallet structurally struggles with, and the second row is what an app structurally cannot promise. The combination, **rich decoding plus isolation plus full reproducibility**, is the job nothing else holds.

**Verdict: the test passes, narrowly, and only for this job.** It does not justify a general-purpose OS.

## What this means for the build order

1. **The decoder is the product.** It lives in a portable library called `clearsign`. It must work identically inside a command-line tool, a GrapheneOS app, a Qubes offline domain, firmware on third-party signer hardware, and our own minimal OS.
2. **The OS is a vehicle.** It exists to provide the isolation row of the table, not as the thing people adopt.
3. **Ship the library first.** An auditor can use it on day one by piping a transaction in, with no OS at all. That puts the history lesson into practice: the idea survives even if the OS never ships.

## Decisions taken in this draft

| Decision | Choice | Why | Status |
|---|---|---|---|
| First chain family | EVM, starting with Safe multisig | Where the blind-signing losses are; matches your audit portfolio | **[needs sign-off]** |
| Language | Rust, `no_std` plus `alloc` for the core | Memory safety, embeddable in firmware and apps, KeyOS precedent | **[needs sign-off]** |
| Dependencies | Decoder: one direct dependency, RustCrypto `sha3`, with RLP and ABI parsing written in-house. Keys: rust-bitcoin `bip39`, `bip32`, RustCrypto `k256` and `sha2`, and `zeroize`; 48 crates in the full tree. No cryptography of our own | Every dependency enters the trusted computing base, so the parsers stay in-house and small, while cryptography is always borrowed from widely used implementations | **[needs sign-off]** |
| Licence | MIT OR Apache-2.0 | Permissive, standard for Rust, keeps the code embeddable by wallet vendors | **[needs sign-off]** |
| Working name | `clearsign` for the library | A placeholder describing the job. The product name is yours to choose | **[needs sign-off]** |
| Kernel | **Borrow, do not write.** Linux only for the development VM. For the signing device, target the formally verified seL4 microkernel, with our Rust code running on top | Writing a kernel is the most consistently fatal pattern in the failure study, and the product's value is not in the kernel. seL4 gives a far smaller and better-verified base than a solo kernel could. `clearsign` stays `no_std`, so a custom kernel remains possible later if evidence justifies it | **[needs sign-off]** |
| Low-level languages | C and assembly appear only inside borrowed components and in the few lines of start-up code bare-metal Rust needs. We do not write the product in them | Memory-safety bugs dominate serious vulnerabilities in large C and C++ codebases. Assembly ties code to one processor family | **[needs sign-off]** |
| Network in the signer | None, ever | Removes a whole attack class, and history shows one public break is fatal | Fixed |
| VM builds | Development only, never custody | A signer in a VM on a compromised host protects nothing | Fixed |

## Why not write our own kernel? Why not C or assembly?

Raised by the project owner on 16 Sep 2026. Recorded here so the reasoning stays attached to the decision.

**What borrowing costs.** A borrowed kernel brings code we did not write and cannot fully understand, a roadmap we do not control, and features we do not need. Linux in particular is a very large general-purpose kernel with a steady stream of vulnerabilities, which is why it is only acceptable here as a development vehicle.

**What writing our own costs.** Redox, a from-scratch Rust OS, still had no hardware-accelerated graphics after ten years and more than forty contributors. Coyotos combined a new kernel, language and proof method and never shipped. Foundation's KeyOS did ship, as a single-purpose signing OS, but took about three years at a company that has raised $16.5 million. The seL4 kernel's machine-checked proof was reported to take on the order of twenty person-years.

**Why seL4 is the better "from scratch".** The appeal of writing our own kernel is a small, fully understood, trustworthy base. seL4 already is that: a microkernel of roughly ten thousand lines of C with a formal proof that the implementation matches its specification. A solo kernel would be less small, less understood by anyone else, and unproven.

**Where the losses actually happen.** The Bybit loss happened in a web interface and in what the signers were shown. The Coldcard loss came from a build configuration and link-time error. Neither was a kernel bug. The guarantee users need lives in `clearsign`, the display, the build and the key handling.

**When to revisit.** If a specific hardware target cannot run seL4, or seL4's constraints block a requirement in the threat model, writing a minimal kernel becomes a legitimate option. The decoder is `no_std` and already builds for bare-metal ARM targets, so that door stays open. A small kernel written as a learning exercise, off the critical path, is also worthwhile.

**Why Rust rather than C or assembly for our code.** C is the traditional OS language, and most existing kernels are written in it. But in large C and C++ codebases, memory-safety errors have repeatedly been reported as the majority of serious vulnerabilities, and Google has reported the share falling in Android as new code moved to memory-safe languages. Assembly offers no safety and ties code to one processor family, which is the "tied to a dying platform" failure family. Rust compiles to the same kind of machine code as C, runs without an operating system, and removes whole classes of memory bug at compile time. Assembly still appears where it must, for example the handful of instructions that run before any higher-level code can.
