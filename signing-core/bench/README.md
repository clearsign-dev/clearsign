# Benchmarks

The measurements behind [docs/11-benchmarks.md](../../docs/11-benchmarks.md):
clearsign against real transactions from many chains, against the transactions
real attacks got people to sign, and against the record of real hacks.

This is its own Cargo workspace, like `fuzz/`. It needs alloy, its RPC types and
a gzip reader, and none of that belongs in the product's lockfile or near the
reproducible build. Nothing here is part of any signer build.

## Layout

| Path | What it is |
|---|---|
| `collect/` | Python collectors, standard library only (plus `certifi` if present). They read public RPC endpoints and Safe's Transaction Service and write `corpus/` |
| `corpus/safe/` | Safe transactions from 28 chains, one gzipped JSON-lines file per chain, with a manifest of how each Safe was chosen |
| `corpus/evm/` | Transactions from sampled blocks on 31 chains, with the block numbers, so the same corpus can be fetched again |
| `corpus/attacks/` | The attack transactions and receipts, exactly as the chains returned them |
| `corpus/permits/` | Permit submissions on Ethereum, for checking typed-data hashing against real signatures |
| `corpus/hacks/` | DeFiLlama's hack list as fetched on 5 Oct 2026, the per-incident research with sources, and the loss-landscape notes |
| `src/bin/` | The checks: `verify-evm`, `verify-safe`, `detect`, `verify-permits`, `perf` |
| `landscape.py`, `report.py` | The hack-record classification, and the generator for docs/11 |
| `results/` | What the checks wrote. `results/before-3258e8d3/` is the same harness run against the decoder as it was before this work |

## Running it

```sh
# Collect (needs network; collecting afresh samples newer blocks)
python3 collect/safe_corpus.py
python3 collect/evm_corpus.py
python3 collect/attack_corpus.py
python3 collect/permit_corpus.py

# Check (offline, from corpus/)
cargo run --release --bin verify-evm
cargo run --release --bin verify-safe
cargo run --release --bin detect
cargo run --release --bin verify-permits
cargo run --release --bin perf

# Classify and write docs/11
python3 landscape.py
python3 report.py
```

`verify-evm` and `verify-safe` exit non-zero on any disagreement: a digest that
is not what the sender signed, a hash that differs from Safe's service or from
alloy, or a signature that recovers to anyone but its owner.

## What each check proves, and what it rests on

- **`verify-evm`** rebuilds each transaction's unsigned payload with alloy and uses
  it only when the on-chain signature recovers to the sender over it. clearsign's
  digest is then compared with the digest that signature was made over.
- **`verify-safe`** settles each Safe transaction's domain from the owners' own
  ECDSA signatures — not from the Safe's current version, which an upgrade
  changes — then compares clearsign's hash with Safe's service and with alloy, and
  recovers every owner signature and every on-chain signature over it.
- **`detect`** rebuilds each attack from the chain: Safe attacks from the
  `execTransaction` calldata, decoded by alloy, with the nonce and domain found by
  reproducing the hash the Safe contract emitted; other attacks by signature
  recovery to the victim. Only then is the payload reviewed.
- **`verify-permits`** rebuilds the typed data each permit submission carried and
  requires the signature to recover to the owner over clearsign's EIP-712 hash.

The research in `corpus/hacks/research-2026-10-05.json` was done with AI
assistance from public sources, which are cited per incident. It is a
classification with judgement in it, published so the judgements can be checked.
