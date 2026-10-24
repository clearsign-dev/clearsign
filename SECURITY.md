# Reporting a vulnerability

Use GitHub's private advisory form:
**[Report a vulnerability](https://github.com/clearsign-dev/clearsign/security/advisories/new)**
— on the repository, under Security → Advisories. It is private until we publish
it together, and it keeps the whole exchange in one place.

Please don't open a public issue for a security problem, and please don't put
one in a pull request description. If the advisory form is not working for you
for any reason, open a normal issue saying only that you have something to
report, with no detail, and we will find another channel.

## What to expect

I read these myself. I will acknowledge within **three days**, and come back with
either a fix, a disagreement, or a date, within **fourteen**. If I go quiet past
that, chase me — it will mean the mail went astray, not that I lost interest.

There is **no bug bounty**. This is an unfunded project and I would rather say
that plainly than have you find out after the work. What I can offer is credit
in the advisory and in the commit that fixes it, worded however you prefer, or
no credit at all if you would rather not be named.

Coordinated disclosure, on a timeline we agree once I understand the issue. If we
can't agree, tell me the date you intend to publish and I will work to it.

## What is worth reporting

Anything that breaks one of the twelve invariants in
[docs/01-threat-model.md](docs/01-threat-model.md). The ones most worth
attacking:

- **Bytes that decode to a display that does not describe them.** The whole
  product is the claim that what you see is derived from what you are signing.
  Anything that makes the two disagree is the most serious class of bug here.
- **A transaction that is dangerous and comes back clean.** Particularly a
  delegatecall, an owner or threshold change, an implementation or fallback
  handler change, or an unlimited approval, reaching exit code 0.
- **A Safe transaction hash that differs from what the chain would compute.** It
  has agreed with Safe's own service on 238 real transactions; find the 239th.
- **A panic, hang or unbounded allocation from any input.** `clippy` denies
  indexing, `unwrap`, `expect`, `panic` and unchecked arithmetic in the decoder
  and key crates, so a crash means the denial has a hole in it.
- **Signing something that was never reviewed**, or with acknowledgements that do
  not match the findings. There is deliberately no API that signs a raw hash.
- **Anything escaping the seL4 compartments**, or reaching the signer from the
  untrusted Linux guest by a path other than the shared region.
- **A build that does not reproduce** in the canonical environment described in
  [docs/03-verification-status.md](docs/03-verification-status.md).

[docs/05-review-package.md](docs/05-review-package.md) is written for exactly
this purpose: the claims worth attacking, the trust boundaries, and what is
already known to be missing. Start there rather than here.

## What is already known, so you don't waste an afternoon

These are real, and reporting them tells me nothing new:

- **No hardware root of trust, no verified boot, no secure element.** Everything
  runs under emulation. Physical attacks, side channels and fault injection are
  entirely untested.
- **EIP-712 typed data is reviewed, not signed.** Permits, Permit2 and Safe
  transactions are interpreted; any other structure is shown field by field and
  marked BLIND. The window does not read typed data yet, and the signer refuses
  it. A typed-data request reported as understood that should have been BLIND
  is a bug worth reporting.
- **Anything outside the verified selector set comes back BLIND by design**,
  which is most DeFi activity: swaps, staking, bridging. That is not a gap in
  coverage, it is the design — but if you can make something come back
  *decoded* that should have come back BLIND, that very much is a bug.
- **On a Safe v1.1.x, `SIGNATURE_NOT_CHAIN_BOUND` fires on every transaction.**
  Correct, and it must still be acknowledged; it is now numbered after the
  transaction's own findings and says it is about the Safe. See
  [docs/09-against-real-transactions.md](docs/09-against-real-transactions.md).
- **Builds do not reproduce across compiler hosts.** Measured, documented, and
  not claimed.
- **The signing and seed commands are gated behind an environment variable**
  because a recovery phrase typed into an everyday computer is exposed. Bypassing
  that gate on your own machine is not a finding; it is the door being where the
  sign says it is.
- **The released binaries are not code-signed or notarised.**
- **The Linux desktop's glib dependency required a security backport.** v0.1.0
  and v0.1.1 use the affected upstream 0.18.5 source. The v0.1.2 source includes
  the exact upstream correction for RUSTSEC-2024-0429, with source-provenance
  checks and the upstream iterator tests. See
  [desktop/vendor/README.md](desktop/vendor/README.md). The version remains
  0.18.5, so version-only scanners may still flag it. This does not exclude
  reports of an incomplete fix or a newly demonstrated path. The signing core
  does not depend on glib; macOS and Windows do not compile this GTK dependency.

## Scope

In scope: everything in this repository — the Rust workspace under
`signing-core/`, the seL4 and image work under `platform/`, the desktop
application under `desktop/`, and the build and release workflows.

Out of scope: the website, which lives in its own repository and is a static
export with no server and no analytics — report anything there against
[clearsign.dev](https://github.com/clearsign-dev/clearsign.dev) instead.

A compromised transaction-producing computer or untrusted Linux guest is an
in-scope adversary. Malicious input, guest-to-signer boundary violations and
review/signature mismatches remain reportable. Control of the trusted host
running the desktop reviewer or QEMU is not a protection those deployments
provide; merely replacing their code or display after gaining that control is
outside the claimed boundary. A vulnerability that obtains such control through
ClearSign remains reportable.

## Please don't

Run anything against a Safe or an address you do not control. Every example in
this repository uses transactions that are already public and already executed.
There is nothing here that requires touching a live system, and nothing worth
finding that can only be found that way.

## One honest note

This has had one external review, in September 2026. Ten findings, all
reproduced and all closed — and nine of those fixes changed signing-critical
code that nobody outside the project has read since. If you are deciding where
to spend your time, that code is the least examined part of the system and the
most worth examining.
