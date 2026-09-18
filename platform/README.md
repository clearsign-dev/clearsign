# Platform

The layers of `docs/04-platform-architecture.md`, built from borrowed, verified upstream components plus our own signer and authority code.

| Layer | Component | Source | How it is verified | Status |
|---|---|---|---|---|
| L1 Kernel | seL4 via Microkit SDK 2.3.0 | seL4 Foundation release | GPG signature and SHA-256, key stored in `config/keys/` and pinned on first use (no independent second channel exists) | Boots in QEMU |
| L2 Compartments | Microkit protection domains, shared memory with per-domain permissions | seL4 Foundation | Capability layout in `.system` files | Working |
| L3 Linux compatibility | libvmm 0.2.0 running a Linux 6.18 LTS guest built from source | libvmm tag `0.2.0`; kernel.org tarball; Debian BusyBox and glibc | Kernel signature verified against kernel.org's key; userland from apt-verified Debian packages in a digest-pinned image | Boots inside seL4 |
| L3 GrapheneOS hardening | `hardened_malloc` as the Linux compartment's system allocator | GrapheneOS, commit pinned by signed release manifest | Manifest tag signature verified; upstream test suite passes | Running inside seL4 |
| L3 Android | GrapheneOS release `2026091000`, 1,057 projects | GrapheneOS | Manifest tag signature verified with pinned key fingerprint | Pinned and verified; building needs an x86_64 Linux host (`android/build-host/`) |
| L0/L4 | Signer (`clearsign`) running bare-metal in its own compartment | This project | Tests, fuzzing, differential tests against Foundry and alloy | Serving the Linux compartment |

## What runs today

```sh
./sel4/signer-system/build.sh && python3 sel4/signer-system/run.py
```
seL4 with two compartments. An untrusted wallet UI submits a Safe transaction; the signer reviews it on bare metal; the UI's attempt to overwrite the verdict is stopped by seL4.

Everything at once, from pinned and verified sources:

```sh
./run-all.sh
```

Or individually:

```sh
./fetch-deps.sh                              # fetch and verify all borrowed components
./guest/build-guest.sh                       # Linux guest from verified source (Docker)
./grapheneos/build-hardened-malloc.sh        # needs Docker
./sel4/linux-signer/build.sh && python3 sel4/linux-signer/run.py
```
seL4 running a Linux guest and the signer side by side. Inside Linux, GrapheneOS's hardened_malloc is the system allocator and stops a write-after-free that glibc's allocator misses. A Linux program asks the signer compartment for a review over shared memory and a doorbell; the verdict comes back CRITICAL with the correct Safe hash. Linux then uses its own kernel privileges to try to overwrite the review, and seL4 blocks the write.

## The signer-only image

```sh
./signer-image/build.sh && python3 signer-image/run.py
```

A Linux image whose entire userland is the signer. There is no shell, no second
program, no storage and no network stack in the kernel — `CONFIG_NET` is off, so
the machine cannot speak to a network even if something in it wanted to. The
signer runs as process 1, reads scanned `ur:` codes from the console, shows what
the transaction does, and answers with an `eth-signature` QR code.

The build refuses to produce an image that betrays that description:

| Gate | What it refuses |
|---|---|
| 1 | a kernel with networking |
| 2 | a kernel that can load modules |
| 3 | a dynamically linked signer, which would drag in a loader and libraries |
| 4 | more than one regular file in the image |
| 5 | anything named or containing a shell |
| 0 | a signer binary that is not the one recorded in `signing-core/EXPECTED-HASHES.txt` |

The signer inside the image is built in the canonical environment rather than on
the maintainer's machine, so the image someone else builds from this source is
the same image. Expected hashes are in `signer-image/EXPECTED-HASHES.txt`.

## What the untrusted side may propose

The same shared-memory path carries two kinds of proposal, and neither is trusted:

| Kind | Proposed by | Decided by |
|---|---|---|
| A Safe transaction | a wallet interface | `clearsign`: DELEGATECALL to an unpinned target is CRITICAL |
| An assistant's action plan | an AI planner | the `authority` engine: data flow is traced, so secret data reaching the network is CRITICAL even when every step looks routine |

The second is the prompt-injection case. A person asks their assistant to
summarise their notes; a document among those notes contains instructions
addressed to the assistant. The plan that comes back reads the notes, summarises
them, files the summary — and also loads a stored credential and posts the result
to `notes-backup.example`. Every step is ordinary. The engine reports:

```
[CRITICAL] SECRET_EGRESS: Secret data from #1, #4 would leave this device
           through a web request to "notes-backup.example".
Must acknowledge ....... #5:SECRET_EGRESS
Plan fingerprint ....... 0x63cbaec3f092209c638f6a96f28e4891b95d247423d8e21d802200304f6e8284
```

## Data path between Linux and the signer

```
 Linux guest (untrusted)                    seL4                         signer compartment
 ───────────────────────                    ────                         ──────────────────
 write request  ──► request region (guest rw) ─────────────────────────► (signer read-only)
 write doorbell ──► unmapped page ──trap──► VMM ──notification──────────► copy request privately,
                                                                          decode, review
 read review    ◄── review region (guest READ-ONLY) ◄────────────────────  write review
 try to write review ──► stage-2 permission fault ──► VMM halts the guest
```

The signer copies the request into private memory before decoding, so a guest that keeps changing the shared region cannot alter bytes between checks.

## Known gaps

- The seL4 Linux compartment has no shell and no way in: it has no serial device, runs a fixed script, and its output is relayed by the VMM. That replaced the debug root shell, which shared a console with the signer.
- The seL4 SDK signing key is pinned on first use; seL4 publishes no second channel to confirm it. Building the SDK from source would remove this dependency.
- Android does not yet run as a compartment. GrapheneOS is pinned and verified at source level, and its allocator runs in the Linux compartment.
- Everything runs in QEMU with emulated virtualization. No hardware, no hardware root of trust, no verified boot of this image.
- seL4 build tooling cannot handle spaces in paths, so builds run from a working copy under `~/.cache/osproject`.
