# Changes

What changed in each release, and why you would want it. Written so that
somebody deciding whether to update can tell what staying put costs them.

Security fixes say what an affected version does wrong, not just that something
was fixed. A release note that says "various improvements" is a release note
that keeps people on a broken version.

## Unreleased

Measured against real chains and real attacks before and after; the numbers
below are from [docs/11-benchmarks.md](docs/11-benchmarks.md), which is
generated from the measurements.

### Added — the attacks it was reading as noise

**Calls that take control of a contract are read, not refused.** A Safe that
administers a protocol signs ownership transfers, role grants and proxy
upgrades, and until now each came back BLIND: the same verdict a routine
staking call gets. The transaction that cost Radiant Capital about $50M in
October 2024 — a Safe calling `transferOwnership` on the protocol's address
provider — read exactly like ordinary traffic. Now `transferOwnership`,
DSAuth's `setOwner`, `renounceOwnership`, `acceptOwnership`, `grantRole`,
`revokeRole`, `renounceRole`, `upgradeTo`, `upgradeToAndCall`, `changeAdmin`
and ProxyAdmin's `upgrade`, `upgradeAndCall` and `changeProxyAdmin` are decoded.
Ownership transfers, role grants, upgrades and admin changes are CRITICAL by
name. The call an upgrade runs as the new code is BLIND, for the same reason a
DELEGATECALL is.

**Permissions beyond `approve`.** `increaseAllowance` and `increaseApproval`
(Badger DAO's injected script used the first), `setApprovalForAll` (the 2022
Uniswap V3 position phishing) and Permit2's `approve`, with its expiry shown as
a date and a warning when the contract called is not Permit2's address.

**Calls that carry other calls are opened.** `multicall(bytes[])`,
TimelockController `schedule` and `execute`, and smart-account
`execute(bytes32,bytes)` (ERC-7579 and ERC-7821) — the shape of the 2025
EIP-7702 drains, in which victims' own accounts ran batches of approvals.
Every carried call is judged by the same rules, including the ones past the
display limit, as MultiSend batches already were.

**EIP-2930, EIP-4844 and EIP-7702 transactions are decoded.** They were refused:
2.0% of 9,998 real transactions sampled across 31 chains. Each EIP-7702
authorization is CRITICAL — it hands an account to code — and one valid on
every chain says so.

**EIP-712 typed data is reviewed**, from the command line: `clearsign
typed-data`. Hashing follows the standard exactly: it matches the EIP's own
example, Permit2's deployed domain separator, and alloy on 5,000 random
documents. ERC-2612 permits, DAI-style permits, Permit2's four signature types
and Safe transactions are read field by field, and an unlimited permission is
CRITICAL. Any other structure is shown field by field and is BLIND. The window
does not read typed data yet, and the signer does not sign it.

Two limits in the new reader were found by reading it, not by fuzzing, whose
inputs stop at 4 KiB: a type with 200,000 array dimensions overflowed the stack
and aborted the process, and recomputing each struct's type hash for every
value let a crafted request cost minutes of CPU. Dimensions are now capped at
eight and type hashes are computed once per request, both with tests.

A review of the new code found five more, each fixed with a test. A Permit2
allowance one below its maximum was a WARNING, not CRITICAL; any Permit2 amount
from 2^144 up is now unlimited, as ERC-20 amounts near the maximum already were.
An expiration of 0, which Permit2 replaces with the block's own time, was shown
as a date in 1970, as though the permission were long dead. Hex in typed data
was read leniently, so `0x0x12` hashed as a number other tools refuse. A Safe
transaction requested as typed data came back with a signing target, the one
typed-data review that did. And calls padded past the decoding limit were only
BLIND, where an over-long MultiSend batch was already CRITICAL; what was not
read could be anything, so it is CRITICAL too.

### Changed

**On a v1.1.x Safe, the transaction's own CRITICAL is numbered first.**
`SIGNATURE_NOT_CHAIN_BOUND` is true of every transaction such a Safe signs.
Listed first, it was the item readers learned to skip, and Bybit's DELEGATECALL
came second, behind it — the problem [09](docs/09-against-real-transactions.md)
recorded. It still must be acknowledged, and it now says that it is about the
Safe rather than the transaction. **Acknowledgement numbers on v1.1.x reviews
change:** Bybit's is now `1:SAFE_DELEGATECALL`.

**`transferFrom` no longer claims to be ERC-20.** ERC-721 uses the same
selector, so the last value is labelled as an amount or a token ID.

### Fixed

**A record from Safe's own service was refused.** For some executed
transactions the service gives `gasToken` and `refundReceiver` as `null`; one
in 10,851 records sampled across 28 chains did. It is now read as the zero
address only when the file's own hash proves that reading, and the review says
so, in the desktop app and the browser as well as the CLI.

**The arena budget test could pass or fail depending on what else ran.** It
counted allocations from every thread, so another test building its input at
the same moment was charged to the request being measured; under load a
maximal transaction read 2.2 MB against the 2 MB alarm. It now counts only the
thread doing the review, and gives the same figures on every run.

**The support matrix said 11 selectors were decoded when the code held 14.**
The generator counted only entries written on one line. It now reads the table.

**Tests that mutation testing showed were missing.** Of 332 mutants of the
previous decoder, 223 were caught and 42 survived every test: nothing checked
that an unsigned EIP-155 payload with only `r` filled in is refused, that refund
fields are shown when any one is set, or where the nesting limits fall. A second
run, against the extended decoder, found more of the same kind: limits on
arrays, nesting, blob hashes and authorizations never tested at their exact
boundary, and labels no test read. A first run against the JSON readers found
that nothing checked a listing of several Safe transactions is refused rather
than read as its first. Every survivor a test can kill now has one; the rest
are equivalent, and `crates/clearsign/tests/mutation_gaps.rs` lists each with
the reason.

### Measured

- **Real transactions, 31 chains:** clearsign committed to exactly the digest the
  sender signed in 9,998 of 9,998, and now accepts all of them.
- **Real Safe transactions, 27 chains:** the hash agreed with Safe's service and
  with alloy in 10,851 of 10,851; 13,467 owner signatures and 13,141 signatures
  accepted on-chain all recover to an owner over it.
- **Real attacks, replayed from the chain:** of 13 signer-deception attacks, the
  mechanism is now named in 9, up from 3.
- **The hack record:** of $21.07B lost across 1,293 incidents, 10.4% went through
  a deceived signer, the only kind of attack a signing reviewer can stop. Key
  theft was 51.3% and contract bugs 31.9%.
- **Real typed-data signatures:** 258 of 258 permits submitted on Ethereum
  recover to their owners over clearsign's EIP-712 hash.
- **Fuzzing:** 215.6 million executions across four targets, 45 minutes each, on
  the code this round added or changed, and 94.8 million more on the final code
  after the review fixes. No failures.
- **Mutation testing:** 98.1% of the extended decoder's 718 mutants caught (84.2%
  of the old decoder's), and 89.6% of the JSON readers'; every survivor is
  equivalent.

## v0.1.2 — 2026-10-04

**Unsigned evaluation release. Not approved for high-value custody.** This
release fixes review, desktop and packaging problems in v0.1.1. It holds no keys
and does not enforce what a separate wallet signs. Independent post-fix review,
platform code signing and production-hardware validation remain outstanding.
Use public fixtures or disposable test workflows for evaluation.

### Release checks - 4 October 2026

- Add native launch checks for the MSI and AppImage, alongside Debian and NSIS.
  Remove the installed copy before testing the next format.
- Verify the existing candidate manifest before publishing instead of generating
  new checksums. Reject stale installer versions, unexpected files, incomplete
  checksum lists and mismatched source metadata.
- Check version consistency on branch builds too, including npm lock metadata.
- Give oversized desktop IPC requests the same refusal response schema as other
  invalid input, and test this at the native Rust boundary.

### Fixed - direct review on 3 October 2026

- Backport the upstream glib `VariantStrIter` fix for the Linux desktop's GTK3
  dependency. The original optimized iterator tests crashed; all 11 pass with
  the two-line correction. Verify the vendored source against the original crate.
- Test installed Debian and Windows NSIS applications through their native
  WebViews, and validate complete release candidates on branch builds.
- Enable the Tauri JavaScript bridge required by the desktop backend.
- Declare and generate UTF-8 explicitly: the packaged macOS window previously
  misdecoded text and failed to initialize its controls despite browser tests.
- Discover Windows Chrome installations and use portable file URLs in the
  installer smoke tests.
- Bind fetched records to the requested transaction hash, cancel stale network
  requests, and prevent delayed file reads from replacing newer input.
- Limit network response bytes while streaming, before collecting the body.
- Reject duplicate JSON keys, malformed chain IDs, and absent signed numeric
  fields rather than replacing them with defaults.
- Gate publishing on tests and version checks; compare canonical binaries with
  the committed hashes rather than the build script's freshly written file.
- Include installers and the standalone HTML page in SHA256SUMS, publish build
  metadata, require every platform's artifacts, and use locked dependencies.
- Reject empty allocator test runs and harness errors instead of inferring
  success from the absence of failure lines in a log.

This was an AI-assisted code review with regression tests, not an independent
human audit. Hardware validation and independent re-review remain outstanding.

### Fixed — two items from a re-check

**The cap was tested beside the gate, not at it.** The test asserted
`too_many_to_acknowledge()`, the predicate `approve` consults — so deleting the
check from `approve` itself would have left it green. There is now a test in
`clearsign-keys/tests/approval_boundary.rs` that goes through `approve`, at the
cap and one over, with both an empty and a complete acknowledgement list.
Checked against the mutation it exists for: removing the gate's check fails it.

**The selector qualification named only the first destination.** It was
deduplicated on its code, so a batch touching four contracts carried a single
notice naming one of them — which reads as though the limitation applies to that
address and not the other three. There is now one notice per distinct
destination.

### Changed — how the review passes are described

The v0.1.1 and v0.1.2 entries said "a second independent review" and "the same
reviewer". Those rounds were **AI-assisted review passes**, not a human security
audit, and the reviewer said so plainly: they cannot accept payment, enter a
consulting contract, or provide a professional auditor's attestation.

The wording is corrected here, and `03-verification-status.md` now separates the
one human review in September from the four AI-assisted passes since. **The
second human review is still unmet.** It matters that the distinction is made by
us rather than discovered by someone else.


### Fixed — the window was broken

**`MAX_RECORD_BYTES` was referenced in three places and declared in none.**
Fetching a transaction or dropping a file threw `ReferenceError` and the page
did nothing. Introduced when input bounding was added: the edit targeted
`<script>` and the tag is `<script type="module">`, so it silently never
applied. Every Rust test passed, the build's parse check passed, and the page
shipped broken.

There is now a smoke test — `app/smoke-test.mjs` — that loads the built page in
a real browser and uses it: reviews the Bybit fixture, drops an oversized file,
edits the input, clears, and fails on anything the page throws. It runs as part
of `app/build.sh` and in CI. Checked against the bug it exists for: with the
constant removed, three of its checks fail.

Writing it found a second one. The oversized-file branch called `show(...)`,
which is not a function this page has.

**A finished review could outlive the input that produced it.** The review flow
had no request generation, so a slow fetch could land after a fast one, or after
Clear, and leave a result on screen belonging to a transaction nobody was
looking at. Every review now takes a generation; typing, changing network or
version, clearing and starting another review all move it on, and a reply from
an older generation is dropped.

### Changed — the interface says what decoding establishes

The results panel was headed **"What it actually does"**. It is now **"What the
bytes say"**, which is the checkable claim. The WARNING verdict said "Nothing
here is irreversible on its face"; it now says that of the part that could be
decoded, and adds that what the code at the other end does is not established by
these bytes. The clear verdict says plainly that nothing flagged is not the same
as safe.


### Changed — how a decoded call is described

**A four-byte selector says what shape a call has, not what the code at the
other end will do with it.** The reviewer sent a correctly encoded
`transfer(address,uint256)` to an address that is not a token; ClearSign called
it "ERC-20 transfer", labelled the target "Token contract", and returned exit 0.
Everything it printed was derived from the bytes, but two of the words were not.

The wording now separates the two:

```
Action ........................ Matches ERC-20 transfer(address,uint256)
Contract called ............... 0x…
Recipient, if it is a token ... 0x…
```

with an `INFO` finding, `SELECTOR_IS_NOT_BEHAVIOUR`, saying that whether the
address is a token and what its code does was not established — a contract can
answer to a familiar selector however it likes, and a proxy can point somewhere
new between one transaction and the next. `transfer`, `approve` and
`transferFrom` all carry it. The exit code is unchanged: refusing every token
transfer would make the tool useless, and the fix here is precision, not alarm.

### Added — a support matrix generated from the code

`docs/10-what-is-supported.md`, written by
`signing-core/scripts/support-matrix.sh`, separating **listed** (the code will
act on it), **tested** (a fixture exercises it) and **proven** (checked against
something that is not this project).

It exists because the deployment table was documented as **1,404** address-chain
pairs while the code held **2,089**. Nobody lied; the table grew and the prose
did not. That is what a hand-written support claim does. The corrected numbers
are 11 deployments, 2,089 pairs across 561 chain IDs, 12 networks in the
application, and Ethereum mainnet as the only chain anything has been proven on
live.

It also records, for the first time in one place, that **EVM support does not
mean every Ethereum transaction**: types `0x01`, `0x03` and `0x04` are refused,
the last being EIP-7702 authorizations. They are refused by name rather than
guessed at, which is the designed behaviour, but the gap was not written down.

**Changes since v0.1.1 for users of the authority engine.** A further review pass
went through the v0.1.1 fixes and found that one of them was incomplete and one
of the claims made about it was false.

### Fixed

**An acknowledgement identifier could name two findings.** Finding numbers were
`u16`, produced by clamping at 65,535. Past that every finding carried the same
number, and in the plan engine — where requirements were held in a set — the
duplicates merged. A 128-step plan producing 82,048 required findings collapsed
to 65,536 distinct ones: **16,512 requirements silently disappeared, and the
short list was accepted while the complete one was refused.**

Identifiers are now `u32` and are never clamped. A review with more findings
than `MAX_ACKNOWLEDGEABLE_FINDINGS` is refused outright rather than approved
from whatever fits — a list nobody could read through is not a list anybody
approved. The plan engine compares requirements as lists rather than through a
set, so two that look alike stay two.

*The transaction signer was affected differently: it compares lists already, so
clamped numbers made a review impossible to approve rather than approvable on a
partial list. Wrong, but wrong in the safe direction.*

**The old acknowledgement syntax was silently reinterpreted.** v0.1.1 changed
`authority --ack` from naming a step to naming a finding, and accepted the old
`#5:CODE` form by stripping the `#`. That turned "step 5" into "finding 5" — a
different finding, approved without comment. **The v0.1.1 changelog said the old
form was "refused loudly". That was not true.** It is now: the tool refuses it
and explains why, and the command's help no longer teaches the old form.

**Input is bounded before it is read.** The JSON parser built the whole value
before applying any limit, and the window read a dropped file entirely before
looking at its size. A limit applied after the allocation it exists to prevent
is not a limit. All three entry points — the parser, the window and the desktop
command — now check the size first.

### Corrected claims

The desktop package description still promised "no network connection", the
website's page description still said "No network access", and the release notes
still called the canonical build "offline". Each now says what the README
already said: reviewing is offline, fetching a queued transaction contacts
Safe's service, and the canonical build compiles offline while its own bootstrap
does not.

**The command-line tool was carrying its own copy of the JSON parser.**
`clearsign-cli` had `safe_json.rs` as a local module rather than using the
shared crate, so the size limit added above reached the window and the desktop
application but not the command line. The duplicate is deleted and the tool uses
the crate everyone else does. It also checks a file's size before opening it and
caps what it takes from standard input, because reading a file whole and then
declining to parse it has already done the allocating.

### Also

The plan fuzz target asked the engine for its requirements and handed them
straight back, which checks nothing an implementation can get wrong on its own.
It now asserts independently that there is one requirement per qualifying
finding, that no two share an identifier, and that omitting any one of them
refuses approval. Coverage rose from 1,995 to 2,036 edges.

The review renderer built the numbered list once per step. On a long plan, with
an arena that never frees, that cost as much as the plan had steps — enough to
breach the signer's allocation budget, which is what caught it.

## v0.1.1 — 29 September 2026

**Update from v0.1.0 if you review batched transactions.** A review pass found a
decoding gap that could hide a dangerous call from you.

### Fixed

**A dangerous call past the display limit was never named.** A Safe MultiSend
batch shows 32 calls and parses up to 1024. The calls past the display limit
were checked for `DELEGATECALL` and for an invalid operation byte, and then
reported only as "some calls are not shown". A call at position 33 granting an
**unlimited token approval**, or changing the Safe's owners, threshold or
implementation, got no finding of its own.

v0.1.0 reports such a batch as BLIND, so it still ends in `DO NOT SIGN` — it was
never silent. But the specific reason was missing, and `INV-6` says those
actions are CRITICAL. Every call is now judged by the same rules whether or not
there is room to display it, with the findings grouped by code so a long batch
stays readable.

**Two findings sharing a code in one plan step needed only one
acknowledgement.** In the authority engine, requirements were keyed on
`(step, code)`, so duplicates merged. Reachable through `SignTransaction`: a
batch granting unlimited approvals to two different spenders produced two
`UNLIMITED_APPROVAL` findings, and confirming once approved both — to two
different addresses. Findings are now numbered as the review displays them,
which is what the transaction signer has always done.

*This affects `authority` and the `authority` command-line tool. The transaction
reviewer was never affected.*

**The window contacted Google every time it opened.** It fetched a webfont from
`fonts.googleapis.com` on load. For a tool whose value is that it talks to
nothing, that told a third party who was reviewing a transaction, and when. The
request is gone and the font allowances are out of the application's content
security policy.

### Corrected claims

**"Never touches a network" was too broad.** Fetching a queued transaction by
its hash contacts Safe's transaction service, by design. The decoder still never
opens a socket, and the signer image has no network stack compiled into its
kernel — those are the precise statements, and they are stronger than the loose
one.

**"Offline" overstated the canonical build.** The compile is offline from a
copied registry cache. The container's own bootstrap is not: it installs three
packages with `apt` and downloads `rustup`, neither pinned by digest. The
recorded hashes say so now. Pinning the bootstrap is the obvious next step and
has not been done.

### Interface change

`authority --ack` now takes the number beside the finding rather than the step
number: `--ack 2:SECRET_EGRESS`, not `--ack #5:SECRET_EGRESS`. A leading `#` is
still accepted so an acknowledgement copied from an older transcript is refused
loudly rather than misread. Scripts that acknowledged by step number need
updating.

### Also

A fuzz target was building its acknowledgements by deduplicating on
`(step, code)` — the same assumption the engine was making — so it agreed with
the bug and could never have found it. About 6.5 million executions proved
nothing on that path. It now asks the review what it requires.

### Known issue in the v0.1.1 downloads

The installers attached to v0.1.1 are named `ClearSign_0.1.0_*` and the
application's own build information says `0.1.0`. The version lives in three
files, none of which is the tag, and none of them were bumped. **The contents
are v0.1.1** — the command-line archives and the reproducible binary are
correct, and the binary's hash matches what `EXPECTED-HASHES.txt` records for
this release.

Fixed for the next release, which also refuses to publish if the application's
version and the tag disagree.

## v0.1.0 — 28 September 2026

First release. A tool for reading a Safe transaction before you approve it: it
holds no keys, signs nothing, and decodes from the signed bytes alone.

170 tests, seven fuzz targets, differential testing against a second
implementation, and one external security review with ten findings, all closed.
Run against 238 real Safe transactions from Safe's own service, its recomputed
hash agreed with Safe's 238 times out of 238.

**Superseded by v0.1.1.** See above for what it gets wrong.
