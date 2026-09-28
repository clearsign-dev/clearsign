# ClearSign

**Read what you are about to sign, from the bytes alone.**

Your hardware wallet shows you a hash. Your multisig interface shows you a
summary produced by a service. Neither of those is the transaction — they are
descriptions of it, and a description can be wrong.

On 21 February 2025 that gap cost Bybit about $1.5 billion. Every signer saw
what looked like an ordinary token transfer. What they approved was a
`DELEGATECALL` that ran someone else's code with the Safe's own storage and
balance.

ClearSign decodes the transaction in front of you from the bytes themselves,
with no network access, no knowledge of your wallet, and no input from any
service — including the one that showed it to you. It holds no keys and signs
nothing. Your hardware wallet still does that.

> **Early development. Unaudited beyond one review. Not for real funds.**
>
> Reviewing a transaction is safe on any computer. The signing and seed commands
> are locked behind an environment variable, because a recovery phrase typed into
> an everyday machine must be treated as exposed.

## Try it

```sh
cd signing-core
cargo build --release -p clearsign-cli
```

Then read the transaction that took $1.5 billion out of Bybit. The record is
Safe's own, fetched from their production service and kept here as a fixture:

```sh
./target/release/clearsign safe-json \
  crates/clearsign-cli/tests/fixtures/bybit-safe-tx.json --chain-id 1
```

```
Operation ........................ DELEGATECALL
Code that will run as the Safe ... 0x9622 1423 681A 6d52 E184 D440 a8eF CEbB 105C 7242
Calldata selector ................ 0xa9059cbb

[CRITICAL] 2:SAFE_DELEGATECALL — DELEGATECALL runs the code at 0x9622…7242 with
full control over this Safe's storage, owners, modules and funds.

DO NOT SIGN
```

Exit code `0` means nothing alarming, `2` means something could not be decoded,
`3` means something critical. [Using it before you sign](docs/06-using-it-before-you-sign.md)
is the guide for anyone who approves transactions on a Safe.

## What it reads

Safe multisig transactions, including the inner call and its operation type, with
the Safe transaction hash recomputed locally so you can hold it against your
hardware wallet's screen. MultiSend batches, unpacked, showing every inner call.
ERC-20 transfers and approvals, with unlimited approvals named rather than
rendered as a number nobody counts. Air-gapped signing requests over QR codes, in
the format MetaMask and Keystone already speak. And plans an AI agent proposes,
traced for where data would flow before a person approves them.

Anything it does not fully understand is reported as BLIND rather than guessed
at. A reviewer that answers every question is not a reviewer.

## How it ships

The same Rust core in three shapes: a **desktop application** for macOS, Windows
and Linux; a **command-line tool** for people who already live there and for
anything that needs an exit code; and a **signer-only operating system image**
containing exactly one program, on a kernel built without a network stack.

## Where it stands

Run against 238 real Safe transactions pulled from Safe's own service, the hash
it computed agreed with the hash Safe published **238 times out of 238**. There
is exactly one `DELEGATECALL` in that set, and it is the Bybit one.

170 tests, seven fuzz targets, differential testing against a second
implementation, and builds that reproduce byte-for-byte on a machine that is not
the maintainer's.

And the part most projects leave out:

- **No users yet.** That is the number that matters.
- One external review, September 2026: ten findings, all closed. Nine of those
  fixes changed signing-critical code that nobody outside has read since.
- No hardware root of trust, no verified boot, no secure element. Everything runs
  under emulation.
- EIP-712 typed data is not covered. The reviewer refuses it rather than guessing.
- The binaries are not code-signed.

[What has been checked](docs/03-verification-status.md) sets out the evidence for
each of those, including the gaps.

## Documentation

Start with [using it before you sign](docs/06-using-it-before-you-sign.md) if you
approve transactions on a Safe, or with [the review package](docs/05-review-package.md)
if you are here to attack it. [docs/](docs/) has the rest: the threat model, the
verification record, the platform architecture and the decisions behind them.

Security reports go through GitHub's private advisory form — see
[SECURITY.md](SECURITY.md).

## Licence

MIT OR Apache-2.0.
