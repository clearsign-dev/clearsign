# Timeline

**Written 27 September 2026.** Dates are targets, not promises. Everything before
the first paying user is under your control; everything after depends on other
people, and the honest version of this document says which is which.

The numbers in the last column come from the failure study in `research/` and
from what comparable projects actually took, not from optimism.

---

## Where things stand today

| | |
|---|---|
| The reviewer | Works. 170 tests, seven fuzz targets, reproducible builds, one external audit with all ten findings fixed |
| The command-line tool | Works. Reads Safe's own JSON, catches the Bybit transaction, agrees with Safe's service on its hash |
| The desktop application | Builds and runs on macOS; Windows and Linux build in CI. Unsigned |
| The compartment prototype | Runs on seL4 in an emulator: untrusted Linux, no shell, signer owns the console |
| Users | **None.** This is the number that matters |

---

## Phase 1 — A first user (October 2026)

The whole project is currently worth as much as its first real user, which is
nothing. Everything here is inside your control.

| Work | Who | Time |
|---|---|---|
| Code-sign and notarise the application for macOS and Windows | You (accounts), me (the pipeline) | 1 week, $99/yr Apple + ~$200–400/yr Windows cert |
| Website: what it is, the download, the runbook, the audit record | Me | 1 week |
| Tag a release, publish installers with hashes | Me | 1 day |
| Put it in front of one multisig team you already know | **You** | Days to weeks, and no amount of engineering substitutes |

**Done when:** one team that is not you has run it on one real transaction and
told you whether it said anything useful. Until that sentence is true, nothing
below is worth starting.

## Phase 2 — Cover what they actually sign (November–December 2026)

Driven by what the first users hit, not by a wishlist. The likely order:

| Work | Why | Time |
|---|---|---|
| EIP-712 typed data | The largest gap today; the reviewer currently refuses it, which is honest and limiting | 3–4 weeks |
| Whatever the first users' transactions contain | Their protocols, their patterns | Unknown until they run it |
| Second external review | Nine of the ten audit fixes changed signing-critical code that nobody outside has read | 1–2 weeks, $8k–25k for a focused engagement |

**Done when:** the tool covers their normal week, and a reviewer who has seen the
fixes signs off on them.

## Phase 3 — Earn from it (January–March 2027)

| Work | Why | Time |
|---|---|---|
| Paid integration with one desk | The auditor's read and the failure study agree: this is the first real money | Ongoing |
| Wallet SDK: the reviewer as a library others embed | The channel. Keystone-class wallets have users; you have the review | 4–6 weeks |
| Bitcoin PSBT, if a user needs it | Different format entirely; substantial | 4–6 weeks |

**Done when:** money has changed hands for something, once.

## Phase 4 — Hardware (mid 2027 onwards, funded)

Not before. A signing device is a hardware company, and the reason to wait is in
your own research: a device with no user is inventory.

| Work | Time |
|---|---|
| Reference device on a Raspberry Pi 4 with a real screen | 6–8 weeks |
| Secure element, verified boot, key storage that survives a stolen device | 3–4 months |
| Industrial design, manufacturing, certification, support | 9–18 months and a team |

Comparable: Foundation raised $16.5M and took roughly three years to ship KeyOS.
Budget like a hardware company, because that is what this becomes.

## What is deliberately not on this timeline

- **A general-purpose operating system.** The 143-system study is unambiguous, and
  Copland is the specific warning: total reinvention while preserving everything
  ships nothing.
- **A token.** The threat model already lists the project itself as an adversary.
- **App store listings**, until the direct download is signed and the store
  version can sit beside it rather than replace it.
- **"Support for every chain."** Each ecosystem is its own decoder and its own
  review. A tool that claims to read everything and guesses at the edges is the
  Bybit failure with better marketing.

---

## The one dependency that is not mine

Phases 2 through 4 are all shaped by what real users hit. I can build ahead of
that, and some of it would be wasted. The single highest-value thing available
right now is not a feature — it is one conversation with a team that signs on a
Safe.
