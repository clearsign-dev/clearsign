#!/usr/bin/env python3
"""Collect real typed-data signatures that were submitted on-chain.

A permit signature never touches the chain until someone submits it, and then
its fields are in the calldata of that submission. That makes the chain a
source of real EIP-712 signatures made by real wallets: rebuild the typed data
from the calldata, hash it, and the signature must recover to the owner. If
the hash were wrong in any field, it would recover to a stranger.

Three kinds are collected on Ethereum mainnet:

- ERC-2612 `permit` on USDC, submitted directly to the token;
- DAI-style `permit` on DAI, submitted directly to the token;
- Permit2 `permit` (single and batch), submitted directly to Permit2.

Transactions are found through the tokens' and Permit2's own events, so the
sample is whatever wallets and relayers actually sent in the scanned blocks.

    python3 permit_corpus.py [--out ../corpus/permits] [--blocks 40000]
"""

import argparse
import gzip
import json
import os
import sys
from datetime import datetime, timezone

from safe_corpus import http_json

RPC = "https://ethereum-rpc.publicnode.com"
USDC = "0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48"
DAI = "0x6b175474e89094c44da98b954eedeac495271d0f"
PERMIT2 = "0x000000000022d473030f116ddee9f6b43ac78ba3"
APPROVAL = "0x8c5be1e5ebec7d5bd14f71427d1e84f3dd0314c0f7b2291e5b200ac8c7c3b925"
# keccak256("Permit(address,address,address,uint160,uint48,uint48)"), Permit2's event.
PERMIT2_EVENT = None  # computed below

SELECTORS = {
    "0xd505accf": "erc2612",
    "0x8fcbaf0c": "dai",
    "0x2b67b570": "permit2_single",
    "0x2a2d80d1": "permit2_batch",
}


def rpc(method, params):
    out = http_json(RPC, {"jsonrpc": "2.0", "id": 1, "method": method, "params": params}, timeout=90)
    if out is None or "error" in out:
        raise RuntimeError(f"{method}: {out}")
    return out["result"]


FALLBACK = ["https://rpc.mevblocker.io", "https://1rpc.io/eth"]
BATCH_ENDPOINTS = [RPC] + FALLBACK
_turn = [0]


def rpc_batch(calls, tries=6):
    """One batch, retried on the next endpoint in turn when one refuses or
    rate-limits, so a free service's limits slow the run instead of stalling it."""
    import time
    body = [{"jsonrpc": "2.0", "id": i, "method": m, "params": p} for i, (m, p) in enumerate(calls)]
    for attempt in range(tries):
        url = BATCH_ENDPOINTS[_turn[0] % len(BATCH_ENDPOINTS)]
        _turn[0] += 1
        try:
            out = http_json(url, body, timeout=40, tries=1)
            if isinstance(out, list):
                by_id = {r.get("id"): r.get("result") for r in out}
                return [by_id.get(i) for i in range(len(calls))]
        except Exception:  # noqa: BLE001 - the next endpoint gets the batch
            time.sleep(1 + attempt)
    return [None] * len(calls)


def tx_hashes(address, topic, head, blocks, window=1000):
    """Transactions that emitted `topic` from `address`, rotating endpoints and
    shrinking the window when one refuses, and pausing so as not to be rate
    limited by a free service."""
    import time
    import urllib.error
    endpoints = [RPC, "https://rpc.mevblocker.io"]
    hashes = []
    to_block = head
    turn = 0
    while to_block > head - blocks:
        from_block = max(head - blocks, to_block - window + 1)
        url = endpoints[turn % len(endpoints)]
        try:
            out = http_json(url, {"jsonrpc": "2.0", "id": 1, "method": "eth_getLogs", "params": [{
                "address": address, "fromBlock": hex(from_block), "toBlock": hex(to_block), "topics": [topic]}]},
                timeout=60, tries=2)
            if not out or "result" not in out:
                raise RuntimeError(str(out.get("error") if out else "empty"))
        except (RuntimeError, urllib.error.HTTPError, urllib.error.URLError, TimeoutError) as e:
            turn += 1
            if window > 50:
                window //= 2
            if turn > 40:
                raise RuntimeError(f"log queries keep failing: {e}")
            time.sleep(2)
            continue
        for l in out["result"]:
            hashes.append(l["transactionHash"])
        to_block = from_block - 1
        time.sleep(0.5)
    return list(dict.fromkeys(hashes))


def call_string(token, selector):
    """name() / version() through eth_call, decoded as an ABI string."""
    raw = rpc("eth_call", [{"to": token, "data": selector}, "latest"])
    b = bytes.fromhex(raw[2:])
    length = int.from_bytes(b[32:64], "big")
    return b[64:64 + length].decode()


def main():
    global PERMIT2_EVENT
    from keccak import keccak256
    PERMIT2_EVENT = "0x" + keccak256(b"Permit(address,address,address,uint160,uint48,uint48)").hex()

    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=os.path.join(os.path.dirname(__file__), "..", "corpus", "permits"))
    ap.add_argument("--blocks", type=int, default=40000)
    args = ap.parse_args()
    os.makedirs(args.out, exist_ok=True)

    head = int(rpc("eth_blockNumber", []), 16) - 64
    # USDC approvals are numerous, so its window is short; Permit2 emits its
    # Permit event only for permit submissions, so its window can be long.
    sources = [(USDC, APPROVAL, args.blocks // 30), (DAI, APPROVAL, args.blocks), (PERMIT2, PERMIT2_EVENT, args.blocks // 2)]
    candidates = []
    for address, topic, blocks in sources:
        found = tx_hashes(address, topic, head, blocks)
        print(f"   {address}: {len(found)} transactions with the event in {blocks} blocks", file=sys.stderr, flush=True)
        candidates += found
    candidates = list(dict.fromkeys(candidates))

    kept = []
    print(f"   fetching {len(candidates)} transactions", file=sys.stderr, flush=True)
    for i in range(0, len(candidates), 20):
        chunk = candidates[i:i + 20]
        if i % 2000 == 0:
            print(f"   {i}/{len(candidates)}", file=sys.stderr, flush=True)
        for tx in rpc_batch([("eth_getTransactionByHash", [h]) for h in chunk]):
            if not tx or not tx.get("to"):
                continue
            kind = SELECTORS.get(tx["input"][:10])
            to = tx["to"].lower()
            if kind in ("erc2612",) and to == USDC or kind == "dai" and to == DAI \
                    or kind in ("permit2_single", "permit2_batch") and to == PERMIT2:
                kept.append({"kind": kind, "chainId": 1, "tx": tx})
    domains = {
        USDC: {"name": call_string(USDC, "0x06fdde03"), "version": call_string(USDC, "0x54fd4d50")},
        DAI: {"name": call_string(DAI, "0x06fdde03"), "version": call_string(DAI, "0x54fd4d50")},
    }
    path = os.path.join(args.out, "permits.jsonl.gz")
    with open(path, "wb") as raw:
        with gzip.GzipFile(fileobj=raw, mode="wb", mtime=0) as f:
            for row in kept:
                f.write(json.dumps(row, separators=(",", ":")).encode() + b"\n")
    counts = {}
    for row in kept:
        counts[row["kind"]] = counts.get(row["kind"], 0) + 1
    with open(os.path.join(args.out, "manifest.json"), "w") as f:
        json.dump({
            "collected": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "rpc": RPC,
            "head_block": head,
            "blocks_scanned": args.blocks,
            "token_domains": domains,
            "kept": counts,
            "candidates": len(candidates),
        }, f, indent=1)
        f.write("\n")
    print(f"\nkept {counts} from {len(candidates)} candidate transactions", file=sys.stderr)


if __name__ == "__main__":
    main()
