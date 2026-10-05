#!/usr/bin/env python3
"""Collect real Safe transactions from every chain in chains.py.

The Safes are sampled, not chosen. On each chain the collector reads recent
blocks from a public RPC and keeps every address that emitted Safe's
ExecutionSuccess event. From those it takes the three that executed most often
in the window and seven more at random (fixed seed), asks Safe's Transaction
Service for each one's version and owners, and saves its most recent queued
transactions.

Only the standard library is used, so the collector can be read in one sitting
and run anywhere Python 3.9+ runs.

    python3 safe_corpus.py [--out ../corpus/safe] [--per-chain 10] [--chains 1,10]

Output, per chain: <chainId>-<path>.jsonl.gz, one service record per line with
the Safe's version, owners and threshold attached under keys starting "_".
A manifest records the block windows scanned, the endpoints used and when.
"""

import argparse
import gzip
import json
import os
import random
import ssl
import sys
import time
import urllib.error
import urllib.request
from datetime import datetime, timezone

from chains import CHAINS, EXECUTION_SUCCESS, SAFE_API
from keccak import checksum

USER_AGENT = "clearsign-benchmark/1 (+https://github.com/clearsign-dev/clearsign)"


def tls_context():
    """Python.org builds on macOS ship without a CA bundle until a post-install
    script is run. Prefer certifi, then the system bundle, then the default."""
    try:
        import certifi
        return ssl.create_default_context(cafile=certifi.where())
    except ImportError:
        pass
    for path in ("/etc/ssl/cert.pem", "/etc/ssl/certs/ca-certificates.crt"):
        if os.path.exists(path):
            return ssl.create_default_context(cafile=path)
    return ssl.create_default_context()


TLS = tls_context()

# The fields a Safe record needs for the benchmark. Everything the service adds
# about its own interpretation (dataDecoded and friends) is dropped: the
# reviewer ignores it, and keeping it would only make the corpus larger.
KEEP = [
    "safe", "to", "value", "data", "operation", "safeTxGas", "baseGas", "gasPrice",
    "gasToken", "refundReceiver", "nonce", "safeTxHash", "isExecuted", "isSuccessful",
    "transactionHash", "executionDate", "submissionDate", "blockNumber",
    "confirmationsRequired", "confirmations", "signatures", "trusted", "proposer",
]

safe_api_calls = 0


def http_json(url, body=None, timeout=60, tries=4):
    data = None if body is None else json.dumps(body).encode()
    headers = {"user-agent": USER_AGENT, "accept": "application/json"}
    if data is not None:
        headers["content-type"] = "application/json"
    last = None
    for attempt in range(tries):
        try:
            req = urllib.request.Request(url, data=data, headers=headers)
            with urllib.request.urlopen(req, timeout=timeout, context=TLS) as resp:
                return json.loads(resp.read().decode())
        except urllib.error.HTTPError as e:
            if e.code == 404:
                return None
            last = f"HTTP {e.code}"
            if e.code == 429:
                time.sleep(5 * (attempt + 1))
                continue
            if e.code >= 500:
                time.sleep(2 * (attempt + 1))
                continue
            raise
        except (urllib.error.URLError, TimeoutError, ConnectionError) as e:
            last = str(e)
            time.sleep(2 * (attempt + 1))
    raise RuntimeError(f"{url}: {last}")


def rpc(url, method, params, timeout=60, tries=4):
    out = http_json(url, {"jsonrpc": "2.0", "id": 1, "method": method, "params": params},
                    timeout=timeout, tries=tries)
    if out is None:
        raise RuntimeError(f"{method}: 404")
    if "error" in out:
        raise RuntimeError(f"{method}: {out['error']}")
    return out["result"]


def safe_api(path, endpoint):
    global safe_api_calls
    safe_api_calls += 1
    time.sleep(0.25)  # stay well inside the service's published rate limit
    return http_json(SAFE_API.format(path=path) + endpoint)


def discover_safes(chain, want=40, max_windows=400):
    """Addresses that emitted ExecutionSuccess, scanning back from the head.

    The window starts at the largest range the endpoint was seen to accept,
    shrinks when it refuses one, and grows again while logs are sparse, so a
    quiet chain is searched further back than a busy one.
    """
    url = chain["logs"]
    head = int(rpc(url, "eth_blockNumber", []), 16)
    limit = chain["window"]
    window = limit
    to_block = head
    seen = {}
    windows = []
    scanned = 0
    failures = 0
    while scanned < max_windows and to_block > 0 and len(seen) < want:
        from_block = max(0, to_block - window + 1)
        try:
            logs = rpc(url, "eth_getLogs", [{
                "fromBlock": hex(from_block),
                "toBlock": hex(to_block),
                "topics": [EXECUTION_SUCCESS],
            }], timeout=30, tries=2)
        except (RuntimeError, urllib.error.HTTPError) as e:
            failures += 1
            if failures > 8:
                raise RuntimeError(f"eth_getLogs keeps failing: {e}")
            if window > 10:
                window = max(10, window // 2)
                limit = window
            time.sleep(1)
            continue
        windows.append([from_block, to_block, len(logs)])
        for log in logs:
            a = log["address"].lower()
            seen[a] = seen.get(a, 0) + 1
        scanned += 1
        to_block = from_block - 1
        if len(logs) < 10 and window < limit:
            window = min(limit, window * 2)
    return head, windows, seen


def compact(record):
    return {k: record.get(k) for k in KEEP}


def collect_chain(chain, per_chain, rng, out_dir):
    chain_id, path, name = chain["id"], chain["path"], chain["name"]
    print(f"== {name} ({chain_id})", file=sys.stderr)
    head, windows, seen = discover_safes(chain)
    ranked = sorted(seen.items(), key=lambda kv: (-kv[1], kv[0]))
    busiest = [a for a, _ in ranked[:3]]
    rest = [a for a, _ in ranked[3:]]
    rng.shuffle(rest)
    candidates = busiest + rest

    chosen = []
    records = 0
    lines = []
    for addr in candidates:
        if len(chosen) >= per_chain:
            break
        # The service refuses addresses that are not EIP-55 checksummed.
        addr = checksum(addr)
        info = safe_api(path, f"/v1/safes/{addr}/")
        if not info or "version" not in info:
            continue
        listing = safe_api(path, f"/v1/safes/{addr}/multisig-transactions/?limit=100")
        if not listing or not listing.get("results"):
            continue
        how = "busiest" if addr.lower() in busiest else "random"
        chosen.append({
            "safe": addr,
            "version": info.get("version"),
            "threshold": info.get("threshold"),
            "owners": len(info.get("owners") or []),
            "executions_in_window": seen.get(addr.lower(), 0),
            "selected_as": how,
            "records": len(listing["results"]),
            "total_queued": listing.get("count"),
        })
        for r in listing["results"]:
            row = compact(r)
            row["_chainId"] = chain_id
            row["_safeVersion"] = info.get("version")
            row["_owners"] = info.get("owners")
            row["_threshold"] = info.get("threshold")
            row["_masterCopy"] = info.get("masterCopy")
            lines.append(json.dumps(row, separators=(",", ":")))
            records += 1

    fname = f"{chain_id}-{path}.jsonl.gz"
    # mtime=0 so the same records always compress to the same bytes.
    with open(os.path.join(out_dir, fname), "wb") as raw:
        with gzip.GzipFile(fileobj=raw, mode="wb", mtime=0) as f:
            for line in lines:
                f.write(line.encode() + b"\n")
    print(f"   {len(seen)} Safes seen, {len(chosen)} chosen, {records} records", file=sys.stderr)
    return {
        "chainId": chain_id,
        "name": name,
        "service_path": path,
        "logs_endpoint": chain["logs"],
        "head_block": head,
        "windows_scanned": windows,
        "safes_seen": len(seen),
        "safes": chosen,
        "records": records,
        "file": fname,
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=os.path.join(os.path.dirname(__file__), "..", "corpus", "safe"))
    ap.add_argument("--per-chain", type=int, default=10)
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
            manifest["chains"].append(collect_chain(chain, args.per_chain, rng, args.out))
        except Exception as e:  # one dead endpoint must not lose the other thirty chains
            print(f"   FAILED: {e}", file=sys.stderr)
            failures.append({"chainId": chain["id"], "name": chain["name"], "error": str(e)})
        write_manifest(manifest_path, manifest, args, failures)

    write_manifest(manifest_path, manifest, args, failures)
    total = sum(c["records"] for c in manifest["chains"])
    print(f"\n{total} records from {len(manifest['chains'])} chains; "
          f"{len(failures)} chains failed; {safe_api_calls} Safe API calls", file=sys.stderr)


def write_manifest(manifest_path, manifest, args, failures):
    manifest["chains"].sort(key=lambda c: c["chainId"])
    manifest["collected"] = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    manifest["seed"] = args.seed
    manifest["per_chain"] = args.per_chain
    manifest["failures"] = failures
    manifest["safe_api_calls"] = safe_api_calls
    manifest["method"] = (
        "Safes sampled from ExecutionSuccess emitters in recent blocks on each chain's "
        "public RPC: the three busiest in the window plus random others (fixed seed), "
        "keeping those Safe's Transaction Service indexes. Up to 100 most recent "
        "queued transactions per Safe."
    )
    with open(manifest_path, "w") as f:
        json.dump(manifest, f, indent=1)
        f.write("\n")


if __name__ == "__main__":
    main()
