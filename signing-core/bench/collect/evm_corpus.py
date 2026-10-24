#!/usr/bin/env python3
"""Collect real transactions from every chain in chains.py.

Blocks are spread evenly over a recent span of each chain rather than taken
from the head, so one busy minute does not stand for a whole chain. Every
transaction in those blocks is kept up to a per-chain cap, sampled with a fixed
seed when a chain produces more. Blocks never change once final, so the
manifest's block list is enough for anyone to collect exactly this corpus again.

    python3 evm_corpus.py [--out ../corpus/evm] [--blocks 24] [--cap 1200] [--chains 1,10]
"""

import argparse
import gzip
import json
import os
import random
import sys
import time
from datetime import datetime, timezone

from chains import CHAINS
from safe_corpus import http_json

# Roughly how many blocks back the sample reaches, by block time. A day or so
# of history on each chain is enough to stop one busy minute standing for it.
SPAN_SECONDS = 2 * 24 * 3600
# Stay this far behind the head so the sampled blocks are final everywhere.
FINALITY_MARGIN = 2000


def rpc_batch(url, calls, tries=4):
    body = [{"jsonrpc": "2.0", "id": i, "method": m, "params": p} for i, (m, p) in enumerate(calls)]
    for attempt in range(tries):
        try:
            out = http_json(url, body, timeout=120)
            if isinstance(out, dict):  # some endpoints answer a batch with one error object
                raise RuntimeError(str(out.get("error")))
            by_id = {r.get("id"): r for r in out}
            results = []
            for i in range(len(calls)):
                r = by_id.get(i)
                if r is None or "error" in r:
                    raise RuntimeError(str(r.get("error") if r else "missing reply"))
                results.append(r["result"])
            return results
        except Exception as e:  # noqa: BLE001 - retried, then reported
            last = e
            time.sleep(2 * (attempt + 1))
    raise RuntimeError(f"batch failed: {last}")


def rpc_one(url, method, params):
    return rpc_batch(url, [(method, params)])[0]


def collect_chain(chain, n_blocks, cap, rng, out_dir):
    url = chain["rpc"]
    print(f"== {chain['name']} ({chain['id']})", file=sys.stderr)
    head = int(rpc_one(url, "eth_blockNumber", []), 16)
    newest = head - FINALITY_MARGIN
    a = rpc_one(url, "eth_getBlockByNumber", [hex(newest), False])
    b = rpc_one(url, "eth_getBlockByNumber", [hex(newest - 1000), False])
    block_time = max(0.05, (int(a["timestamp"], 16) - int(b["timestamp"], 16)) / 1000)
    span = int(SPAN_SECONDS / block_time)
    stride = max(1, span // n_blocks)
    numbers = [newest - i * stride for i in range(n_blocks) if newest - i * stride > 0]

    txs = []
    for i in range(0, len(numbers), 4):
        chunk = numbers[i:i + 4]
        blocks = rpc_batch(url, [("eth_getBlockByNumber", [hex(n), True]) for n in chunk])
        for blk in blocks:
            for tx in blk.get("transactions") or []:
                tx["_block"] = int(blk["number"], 16)
                tx["_timestamp"] = int(blk["timestamp"], 16)
                txs.append(tx)
    seen_total = len(txs)
    if len(txs) > cap:
        txs = rng.sample(txs, cap)
        txs.sort(key=lambda t: (t["_block"], int(t.get("transactionIndex") or "0x0", 16)))

    fname = f"{chain['id']}-{chain['path']}.jsonl.gz"
    with open(os.path.join(out_dir, fname), "wb") as raw:
        with gzip.GzipFile(fileobj=raw, mode="wb", mtime=0) as f:
            for tx in txs:
                tx["_chainId"] = chain["id"]
                f.write(json.dumps(tx, separators=(",", ":")).encode() + b"\n")
    types = {}
    for tx in txs:
        types[tx.get("type", "none")] = types.get(tx.get("type", "none"), 0) + 1
    print(f"   {len(numbers)} blocks, {seen_total} transactions, kept {len(txs)}; types {types}",
          file=sys.stderr)
    return {
        "chainId": chain["id"],
        "name": chain["name"],
        "rpc": url,
        "head_block": head,
        "block_time_estimate_s": round(block_time, 3),
        "blocks": numbers,
        "transactions_in_blocks": seen_total,
        "kept": len(txs),
        "types": types,
        "file": fname,
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=os.path.join(os.path.dirname(__file__), "..", "corpus", "evm"))
    ap.add_argument("--blocks", type=int, default=24)
    ap.add_argument("--cap", type=int, default=1200)
    ap.add_argument("--chains", default="")
    ap.add_argument("--seed", type=int, default=20261004)
    args = ap.parse_args()

    os.makedirs(args.out, exist_ok=True)
    only = {int(c) for c in args.chains.split(",") if c}
    rng = random.Random(args.seed)
    manifest_path = os.path.join(args.out, "manifest.json")
    manifest = {"chains": []}
    if os.path.exists(manifest_path) and only:
        manifest = json.load(open(manifest_path))
        manifest["chains"] = [c for c in manifest["chains"] if c["chainId"] not in only]

    failures = []
    for chain in CHAINS:
        if only and chain["id"] not in only:
            continue
        try:
            manifest["chains"].append(collect_chain(chain, args.blocks, args.cap, rng, args.out))
        except Exception as e:  # noqa: BLE001 - one dead endpoint must not lose the rest
            print(f"   FAILED: {e}", file=sys.stderr)
            failures.append({"chainId": chain["id"], "name": chain["name"], "error": str(e)})

    manifest["chains"].sort(key=lambda c: c["chainId"])
    manifest["collected"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    manifest["seed"] = args.seed
    manifest["blocks_per_chain"] = args.blocks
    manifest["cap_per_chain"] = args.cap
    manifest["failures"] = failures
    manifest["method"] = (
        f"{args.blocks} blocks per chain spread evenly over about two days, ending "
        f"{FINALITY_MARGIN} blocks behind the head; every transaction in them, sampled "
        f"down to {args.cap} per chain with a fixed seed where there were more."
    )
    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=1)
        f.write("\n")
    total = sum(c["kept"] for c in manifest["chains"])
    print(f"\n{total} transactions from {len(manifest['chains'])} chains; "
          f"{len(failures)} chains failed", file=sys.stderr)


if __name__ == "__main__":
    main()
