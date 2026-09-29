# Contributing

The most useful thing anyone can send is **a transaction ClearSign read badly**
— one it called safe that was not, one it refused that it should have decoded,
or one whose hash it got wrong. Open an issue with the transaction, the chain,
and what you expected. That is worth more than a patch.

If it is a security problem, do not open an issue. See [SECURITY.md](SECURITY.md).

## Running it

```sh
cd signing-core
cargo test --workspace --release      # 170 tests
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
- **Nothing untrusted is ever displayed.** Every value shown must be derived
  from the bytes being signed. Not a label from an API, not a name from a token
  list, not a `dataDecoded` field. If the bytes do not say it, it does not
  appear.
- **Anything not fully understood is BLIND.** A guess that looks like an answer
  is the failure this project exists to prevent. Refusing is a feature.
- **The v1 selector set is closed.** `multiSend(bytes)` is the last selector v1
  learns. Adding one means removing something, in writing, in
  [docs/02-v1-scope.md](docs/02-v1-scope.md). That rule is why this project has
  shipped anything at all.
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

Write the description as prose, in whatever voice is yours. Say what changed and
why it needed changing. If you found something surprising on the way, that is
usually the most valuable paragraph — several of the fixes in this repository
exist because someone wrote down what confused them.

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

Protocol decoders outside the v1 set, token metadata, address books, anything
that reaches the network from the signer, and anything that makes the tool
quieter about a risk without removing the risk. The last one is the one people
are most often surprised by: a warning that fires less often is not the same as
a system that is safer.
