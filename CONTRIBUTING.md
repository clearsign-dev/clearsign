# Contributing

Bug reports should include the ClearSign version, chain, input format and
expected result. A public historical transaction is a useful reproducer; do not
publish private keys, recovery phrases or confidential transaction data.
Incorrect hashes, misleading descriptions and unexpected refusals are all
worth reporting.

If it is a security problem, do not open an issue. See [SECURITY.md](SECURITY.md).

## Running it

```sh
cd signing-core
cargo test --workspace --release --locked
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all --check
```

The toolchain is pinned in `rust-toolchain.toml`; `rustup` will fetch it. The
platform work needs Docker and QEMU, and `platform/run-all.sh` runs all of it —
about ten minutes, and the signer image build is most of that.

## Rules that are not style preferences

These exist because of the threat model, and a patch that breaks one will be
sent back however good it is otherwise.

- **The decoder cannot panic.** `clippy` denies indexing, `unwrap`, `expect`,
  `panic` and unchecked arithmetic in the decoder and key crates. `unsafe` is
  forbidden outright. Return a typed error instead.
- **Keep signed fields separate from external claims.** Derive transaction
  fields from the input bytes, escape untrusted text and label transport
  metadata explicitly. Do not substitute API descriptions, token names or
  `dataDecoded` for decoding. A selector match is not proof of contract behaviour.
- **Anything not fully understood is BLIND.** A guess that looks like an answer
  is the failure this project exists to prevent. Refusing is a feature.
- **The v1 decoder scope is frozen.** The original selector boundary was
  reopened once after the attack benchmark showed that routine BLIND findings
  hid important mechanisms. That dated exception and its payment are recorded
  in [docs/02-v1-scope.md](docs/02-v1-scope.md). Further protocol-specific
  readers still require a measured gap and an explicit scope tradeoff.
- **A new finding needs a test that fails without it.** Several tests here were
  caught passing for the wrong reason, by planting the bug they claimed to catch
  and watching them stay green. If yours cannot fail, it is not a test.

## Tests

New decoding goes with vectors produced by something that is not this project —
Foundry's `cast`, `alloy`, or a published specification's own test vectors.
Agreeing with ourselves proves nothing.

If you change the decoder, run the fuzz targets briefly before opening the pull
request:

```sh
cd signing-core/fuzz && cargo +nightly fuzz run <target> -- -max_total_time=60
```

## Pull requests

Describe the behaviour changed, the reason for the change and how it was tested.
Include compatibility changes and remaining limitations. Distinguish checks
actually run from checks still needed; record review provenance accurately.

Commits carry a `Co-Authored-By` line when that is honest. Nothing else is
required.

## Releases

Every release gets a section in [CHANGELOG.md](CHANGELOG.md), and the release
notes lead with it. A fix that changes what the tool reports says what an
affected version does wrong, not just that something was fixed — somebody
deciding whether to update needs to know what staying put costs them, and
"various improvements" is how people end up running a version that misreads
their transactions.

## What is unlikely to be merged

Protocol decoders outside the frozen v1 set, token metadata, address books, anything
that reaches the network from the signer, and anything that makes the tool
quieter about a risk without removing the risk. The last one is the one people
are most often surprised by: a warning that fires less often is not the same as
a system that is safer.
