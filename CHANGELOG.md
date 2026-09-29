# Changes

What changed in each release, and why you would want it. Written so that
somebody deciding whether to update can tell what staying put costs them.

Security fixes say what an affected version does wrong, not just that something
was fixed. A release note that says "various improvements" is a release note
that keeps people on a broken version.

## v0.1.1 — 29 September 2026

**Update from v0.1.0 if you review batched transactions.** A second independent
review found a decoding gap that could hide a dangerous call from you.

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
