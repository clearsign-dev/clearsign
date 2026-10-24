# 09 — The reviewer against real transactions

> **Superseded for breadth by [11-benchmarks.md](11-benchmarks.md)** (5 Oct 2026):
> 10,851 Safe transactions across 27 chains, every hash agreeing, and the
> severity-ordering problem described below addressed — on a v1.1.x Safe the
> transaction's own CRITICAL is now numbered first, ahead of the chain-binding
> notice. This file records the first run as it happened.

**Run 28 September 2026.** Reproduce with:

```sh
cd signing-core && cargo build --release -p clearsign-cli
./scripts/against-real-transactions.sh
```

Until now the decoder had been exercised against invariant tests, differential
tests, fuzzing, and one real transaction: the Bybit one. That last one is the
headline, but a single fixture proves very little. A fixture is chosen. Real
traffic is not.

So this run asks a harder question. Given **every transaction six real Safes
have ever queued**, what does the reviewer say about each, and does the hash it
computes agree with the hash Safe's own service published?

## What was run

238 transactions, fetched live from Safe's Transaction Service, across six
mainnet Safes chosen to span both domain versions that matter:

| Safe | Version | Transactions |
|---|---|---|
| `0x1Db92e2E…D2dFCF4` | 1.1.x | 74 — **the Bybit Safe** |
| `0xA7A93fd0…6eeD06` | 1.1.x | 87 |
| `0xe1ab8c08…eB09215` | 1.1.x | 15 |
| `0x2ebF891f…cae277F` | 1.3.0+ | 17 |
| `0x6F4565c9…F3Bb0C` | 1.3.0+ | 20 |
| `0xc3350595…065291` | 1.3.0+ | 25 |

## The result that matters

| | |
|---|---|
| **Hash agreed with Safe's service** | **238 of 238** |
| Hash disagreed | **0** |

Every one of those is an independent agreement with a second implementation, on
real data, computed from the signed bytes rather than read out of the file. This
is the property the whole tool rests on: if the hash it shows you is not the
hash you are signing, nothing else it says matters.

## What it said

| Outcome | Count |
|---|---|
| exit 0 — nothing alarming | 43 |
| exit 2 — BLIND, something could not be decoded | 9 |
| exit 3 — CRITICAL | 186 |

CRITICAL findings by code:

| Finding | Count |
|---|---|
| `SIGNATURE_NOT_CHAIN_BOUND` | 176 |
| `SAFE_OWNER_CHANGE` | 24 |
| `SAFE_THRESHOLD_CHANGE` | 1 |
| `SAFE_IMPLEMENTATION_CHANGE` | 1 |
| `SAFE_FALLBACK_HANDLER_CHANGE` | 1 |
| `SAFE_DELEGATECALL` | 1 |

**One DELEGATECALL in 238 transactions, and it is the Bybit one** —
`0xb3476d06…5ba004f8`, nonce 71. The other 73 transactions on that same Safe
are ordinary and the reviewer treats them as ordinary. It did not find the
attack because it was pointed at it; it found it because it was the only one of
its kind in the set.

Two further transactions are worth naming, because they are exactly the class of
change that a signer should never approve without understanding it, and both
came out of ordinary traffic rather than an incident:

- `0xeda7646b…030ced2d` on `0xe1ab8c08…` — replaces the Safe's **implementation**
  *and* its **fallback handler** in one transaction. The fallback handler is what
  answers EIP-1271 signature validation.
- `0x58e39832…5917df8a` on `0xc3350595…` — changes the **threshold**, which is how
  many owners must sign everything afterwards.

## The finding this run produced

Split by version, the picture is not the same on both sides:

| | Transactions | CRITICAL | Clean |
|---|---|---|---|
| Safe 1.3.0+ | 62 | 10 | **43** |
| Safe 1.1.x | 176 | **176** | **0** |

On modern Safes the reviewer behaves as intended: quiet on the 43 ordinary
transactions, loud on the 10 that change owners, thresholds or implementations.

On v1.1.x Safes it fires `SIGNATURE_NOT_CHAIN_BOUND` on **every single
transaction, without exception.** The finding is correct — the v1.1.x EIP-712
domain genuinely omits the chain ID, so every signature on such a Safe really is
replayable on any chain where that address exists. But it is a property of *the
Safe*, not of *the transaction*, and reporting a constant as though it were news
about the transaction in front of you is the failure this project has already
written down, in `02-v1-scope.md`:

> a warning that always fires is ignored — which is blind signing with extra steps

Nothing here is wrong. What is wrong is the shape of it: a Bybit signer running
this tool would have seen CRITICAL on all 71 transactions before the attack, and
CRITICAL again on the attack. The thing that distinguishes nonce 71 —
`2:SAFE_DELEGATECALL` — is present and correct and clearly worded, but it
arrives as the second item in a list whose first item the reader has already
learned to skip.

**This is a severity-modelling problem, not a decoding one, and it should be
fixed before the tool meets a v1.1.x Safe in anger.** The shape of the fix is to
report a Safe-wide property once, about the Safe, rather than once per
transaction — and to keep the acknowledgement requirement, since the risk is
real. The nine hundred words above are the argument for doing that; the
counter-argument is that suppressing it at all is how tools start lying.
Recorded here rather than decided.

## What it could not read

Nine transactions came back BLIND, all on one Safe and all calling one contract,
`0xe3cBd06D…D1489E8f`, with three selectors: `0xa694fc3a`, `0x891ef43e` and
`0x2bf67650`. The first is the familiar `stake(uint256)`.

The reviewer's answer to each was:

> Function `0xa694fc3a` on `0xe3cBd06D…` is not in the decoder's verified set.
> What it does cannot be determined from these bytes.

That is the correct behaviour and the designed behaviour — the v1 selector set
is deliberately closed, and `INV-2` says anything not fully understood produces
an explicit BLIND finding rather than a guess. It is also a piece of information
that no amount of unit testing could have produced: **staking is what these
users actually do.** `07-timeline.md` says phase 2 should be driven by what the
first users' transactions contain rather than by a wishlist. This is the first
real data for that list, and it arrived before the first user did.

## What this run does not show

- Six Safes on one chain. Nothing here says anything about Arbitrum, Base,
  Optimism or any other network, though the same script takes them.
- Only queued multisig transactions. Module transactions and direct executions
  are not covered.
- No transaction here was adversarial except the Bybit one. A set of 238 that
  contains exactly one attack says more about coverage than about detection.
- The reviewer was told each Safe's version from the table above. Where it is
  not told, it derives the domain from the hash or refuses — that path is
  covered by tests, not by this run.
