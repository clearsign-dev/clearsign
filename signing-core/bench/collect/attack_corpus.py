#!/usr/bin/env python3
"""Collect the transactions real attacks got people to sign.

Every case is a real incident with a public source. For each one this fetches
the attack transaction from a public RPC endpoint, with its receipt, and keeps
them exactly as the chain returns them. Nothing is taken from Safe's service or
from any decoder: the benchmark rebuilds what the victims signed from these
bytes, proves it (signatures must recover to the victims, Safe hashes must equal
the hash the Safe contract logged on-chain), and only then shows it to the
reviewer.

    python3 attack_corpus.py [--out ../corpus/attacks]

Cases are grouped by how the attack reached its victim:

- SIGNING_DECEPTION_MULTISIG: multisig owners approved a transaction whose real
  effect differed from what their screens showed.
- SIGNING_DECEPTION_EOA: people signed a transaction from a phishing site or a
  compromised frontend that did something other than what they believed.
- DECEIVED_INTENT: the transaction did exactly what its bytes said, but the
  signer was wrong about who they were paying. These are included on purpose:
  a reviewer that reads bytes cannot catch them, and the benchmark says so.
"""

import argparse
import gzip
import json
import os
import sys
from datetime import datetime, timezone

from safe_corpus import http_json

RPC = {
    1: ["https://ethereum-rpc.publicnode.com", "https://eth.drpc.org", "https://1rpc.io/eth"],
    56: ["https://bsc-dataseed.bnbchain.org", "https://bsc-dataseed1.defibit.io", "https://bsc-rpc.publicnode.com"],
    42161: ["https://arbitrum-one-rpc.publicnode.com", "https://arb1.arbitrum.io/rpc"],
    34443: ["https://mainnet.mode.network"],
    137: ["https://polygon.drpc.org", "https://1rpc.io/matic"],
    8453: ["https://base-rpc.publicnode.com", "https://mainnet.base.org"],
    10: ["https://optimism-rpc.publicnode.com", "https://mainnet.optimism.io"],
    25: ["https://evm.cronos.org"],
    8217: ["https://public-en.node.kaia.io"],
}

# Each case: the incident, its DeFiLlama hack-list id where it has one, and
# the transaction(s) the deceived signers approved. Hashes come from the
# research notes in bench/attacks.md, each traced to a source or to the chain.
CASES = [
    dict(id="bybit", incident="Bybit", llama_id=None, date="2025-02-21", loss_usd=1_400_000_000,
         vector="SIGNING_DECEPTION_MULTISIG", chain=1, kind="safe_exec",
         txs=["0x46deef0f52e3a983b67abf4714448a41dd7ffd6d32d32da69d62081c68ad7882"],
         what="Safe v1.1.1 DELEGATECALL to attacker code, shown to signers as a token transfer",
         sources=["https://www.ic3.gov/PSA/2025/PSA250226",
                  "https://www.sygnia.co/blog/sygnia-investigation-bybit-hack/"]),
    dict(id="wazirx", incident="WazirX", llama_id=492, date="2024-07-18", loss_usd=234_900_000,
         vector="SIGNING_DECEPTION_MULTISIG", chain=1, kind="safe_exec",
         txs=["0x48164d3adbab78c2cb9876f6e17f88e321097fcd14cadd57556866e4ef3e185d"],
         what="Safe DELEGATECALL replacing the implementation, shown to signers as GALA/USDT transfers",
         sources=["https://wazirx.com/blog/wazirx-cyber-attack-key-insights-and-findings/"]),
    dict(id="radiant-arbitrum", incident="Radiant Capital (Arbitrum)", llama_id=1205, date="2024-10-16",
         loss_usd=50_000_000, vector="SIGNING_DECEPTION_MULTISIG", chain=42161, kind="safe_exec",
         txs=["0x7856552db409fe51e17339ab1e0e1ce9c85d68bf0f4de4c110fc4e372ea02fb1"],
         what="Safe call transferring ownership of LendingPoolAddressesProvider to the attacker",
         sources=["https://medium.com/@RadiantCapital/radiant-post-mortem-fecd6cd38081"]),
    dict(id="radiant-bsc", incident="Radiant Capital (BNB Chain)", llama_id=1205, date="2024-10-16",
         loss_usd=None, vector="SIGNING_DECEPTION_MULTISIG", chain=56, kind="safe_exec",
         txs=["0xd97b93f633aee356d992b49193e60a571b8c466bf46aaf072368f975dc11841c"],
         what="The same ownership transfer on the BNB Chain deployment",
         sources=["https://medium.com/@RadiantCapital/radiant-post-mortem-fecd6cd38081"]),
    dict(id="uniswap-phishing", incident="Uniswap V3 LP phishing", llama_id=290, date="2022-07-11",
         loss_usd=8_000_000, vector="SIGNING_DECEPTION_EOA", chain=1, kind="evm_tx",
         txs=["0xeb89ad8f76e604fdd495fb3d43c3405d7f5e8fbfa78a0fd93099c3e7a32937b5",
              "0xaf2524403a7d863a6d9c3fae904105c9cd17fdf6157fdf2d1153809d73f38093"],
         what="Victims' setApprovalForAll on their Uniswap V3 positions to the phisher",
         sources=["https://www.theblock.co/post/156826/uniswap-liquidity-providers-hit-with-phishing-attack"]),
    dict(id="superfortune", incident="SUPERFORTUNE AI", llama_id=746, date=None, loss_usd=None,
         vector="DECEIVED_INTENT", chain=56, kind="safe_exec",
         txs=["0xd2689d12b22720e2fabcbceba683dd6eec61ba644187d0a60faf397b6d2fda7b"],
         what="Safe batch sending tokens to a look-alike of the airdrop contract's address",
         sources=["chain data; no post-mortem publishes the hash"]),
    dict(id="florence", incident="Florence Finance", llama_id=504, date="2023-11-30",
         loss_usd=1_450_000, vector="DECEIVED_INTENT", chain=1, kind="safe_exec",
         txs=["0x67a7a5f258378f953e1561a25f0e264d88ac229d4c6ec871b67a2b399137fed8"],
         what="Safe USDC transfer to an address-poisoning look-alike",
         sources=["https://twitter.com/FlorenceFinance"]),
    dict(id="ionic", incident="Ionic Protocol", llama_id=688, date="2025-02-04", loss_usd=8_600_000,
         vector="DECEIVED_INTENT", chain=34443, kind="safe_exec",
         txs=["0x0b867a1dfd937981202cef46d533bbf66b4f737ba38a37d7fc633647e96d4e6d"],
         what="Admin Safe listing a counterfeit LBTC as collateral",
         sources=["https://x.com/ionicmoney"]),
    dict(id="fake-giwa", incident="Fake GIWA bridge", llama_id=17, date="2026-09-27", loss_usd=2_080_000,
         vector="DECEIVED_INTENT", chain=1, kind="evm_tx",
         txs=["0xd8537ba2304ec8a473a2053c46ca2186dd206e8d3406cc194842aa237b98c6fb",
              "0x9c76ef4aa373c8d0509194b2bc332c77852ed02e7f6b7571197a3335dd50a6b0"],
         what="ETH deposited into a fake bridge portal",
         sources=["research/result-se2.json"]),
    dict(id="phantom-galaxies", incident="Phantom Galaxies", llama_id=340, date="2021-11-19",
         loss_usd=1_100_000, vector="DECEIVED_INTENT", chain=1, kind="evm_tx",
         txs=["0x344617c8dd4c8b4d9d5132e3a159d48b03ffc38f8f79275a852cdbaa19ec265c",
              "0x95808630d98eb67b57f19cdaf0d086de25271a58843f0b680ba18b7cd788b74c"],
         what="0.1 ETH 'mint fee' paid straight to the scammer",
         sources=["research/result-se2.json"]),
    dict(id="badger", incident="Badger DAO", llama_id=343, date="2021-12-02", loss_usd=120_000_000,
         vector="SIGNING_DECEPTION_EOA", chain=1, kind="evm_tx",
         txs=["0x5e4c7966b0eaddaf63f1c89fc1c4c84812905ea79c6bee9d2ada2d2e5afe1f34",
              "0x3cad03b779572c11c8188d9660d39ba76d5ae20ec254df89df9c79b5874d17f7"],
         what="Victims' unlimited increaseAllowance to the attacker, requested by an injected script",
         sources=["https://rekt.news/badger-rekt/",
                  "https://www.microsoft.com/en-us/security/blog/2022/02/16/ice-phishing-on-the-blockchain/"]),
    dict(id="nexus-mutual", incident="Nexus Mutual founder", llama_id=885, date="2020-12-14", loss_usd=8_000_000,
         vector="SIGNING_DECEPTION_EOA", chain=1, kind="evm_tx",
         txs=["0x4ddcc21c6de13b3cf472c8d4cdafd80593e0fc286c67ea144a76dbeddb7f3629"],
         what="370,000 NXM transferred to the attacker, signed on a Ledger behind a tampered MetaMask",
         sources=["https://medium.com/nexus-mutual/responsible-vulnerability-disclosure-ece3fe3bcefa"]),
    dict(id="poap-rpl", incident="POAP founder (listed as Rocket Pool)", llama_id=824, date="2023-03-30",
         loss_usd=3_800_000, vector="SIGNING_DECEPTION_EOA", chain=1, kind="evm_tx",
         txs=["0xd36e7d75f9d5ff768d5ae53277f44ce5d130c6362cb06ca12d02023bd0373add"],
         what="RPL approve to a drainer address",
         sources=["research/result-se1.json"]),
    dict(id="jaypegs-miso", incident="JayPegs Automart (SushiSwap MISO)", llama_id=34, date="2021-09-17",
         loss_usd=3_100_000, vector="SIGNING_DECEPTION_EOA", chain=1, kind="evm_tx",
         txs=["0x93743d15f4e9357bbb49bac53a5564bcd0e13f065fac465f83125d1a7dfff214"],
         what="MISO createMarket with the attacker's address injected as the auction wallet",
         sources=["https://www.paradigm.xyz/2021/08/two-rights-might-make-a-wrong"]),
    dict(id="mm-finance", incident="MM Finance (Cronos)", llama_id=323, date="2022-05-04", loss_usd=2_000_000,
         vector="SIGNING_DECEPTION_EOA", chain=25, kind="evm_tx",
         txs=["0xf84da9aa9b4af5b18722aeffa68c05d3bd0acef75435f159f6a99cde9f29107a",
              "0xdfb5b6f7698121adc9510d69687f9536c1abe320287e6b053e384956cd7be0da"],
         what="approve to an injected fake router, then a swap through it",
         sources=["https://mmfinance.gitbook.io/docs/post-mortem"]),
    dict(id="klayswap", incident="KlaySwap (BGP hijack)", llama_id=1283, date="2022-02-03", loss_usd=1_900_000,
         vector="SIGNING_DECEPTION_EOA", chain=8217, kind="evm_tx",
         txs=["0xaec1575fd11670cb75f17d0980d04c7209a5b468833952ce6d50bb0c9ee9b07a",
              "0xccb0c8e192ddb3a737e10c3326ed4ac31f76f1eb414ad1fda2f06f439e5c01e8"],
         what="approve to the attacker's fake factory, then a swap through it",
         sources=["https://medium.com/s2wblog/post-mortem-of-klayswap-incident-through-bgp-hijacking-en-3ed7e33de600"]),
    dict(id="eip7702-eth", incident="EIP-7702 batch phishing (Ethereum)", llama_id=None, date="2025-05-24",
         loss_usd=146_551, vector="SIGNING_DECEPTION_EOA", chain=1, kind="evm_tx",
         txs=["0x1ddc8cecbcaad5396fdf59ff8cddd8edd15f82f1e26779e581b7a4785a5f5e06"],
         what="The victim's own delegated account executes a batch of approvals and transfers",
         sources=["https://x.com/realScamSniffer/status/1926296681198326254"]),
    dict(id="eip7702-base", incident="EIP-7702 batch phishing (Base)", llama_id=None, date="2025-08-24",
         loss_usd=1_540_000, vector="SIGNING_DECEPTION_EOA", chain=8453, kind="evm_tx",
         txs=["0xa984840a3a6322fc0a51650a71870bc493bf1d7e1ca15945a82c37c603139bba"],
         what="The same shape on Base",
         sources=["https://x.com/realScamSniffer/status/1959423000752820374"]),
    dict(id="eigenlayer", incident="EigenLayer (OTC address swap)", llama_id=263, date="2024-10-05",
         loss_usd=5_700_000, vector="DECEIVED_INTENT", chain=1, kind="safe_exec",
         txs=["0x3f7e3d1a55d8aa5bbb3d1dc750df72b13fd2f1d122d83a6c0807b30d5d7587ac"],
         what="A 12-transfer Safe batch in which one recipient had been swapped for the attacker's",
         sources=["research/result-se1.json"]),
    dict(id="coindash", incident="CoinDash ICO", llama_id=350, date="2017-07-17", loss_usd=7_700_000,
         vector="DECEIVED_INTENT", chain=1, kind="evm_tx",
         txs=["0xd9778a70b9a4d042e45917b97e39de1ff6522c31f10cc7a6496a983a7321b4f8",
              "0x0ca438b8bc68bcec6af15af95ab4ad3a5b21fb07d2da385250658a29fc72db59"],
         what="ETH sent to the address that replaced the ICO address on the hacked site",
         sources=["https://www.bleepingcomputer.com/news/security/hacker-steals-7-million-worth-of-ethereum-from-coindash-platform/"]),
]


def rpc(chain, method, params):
    last = None
    for url in RPC[chain]:
        try:
            out = http_json(url, {"jsonrpc": "2.0", "id": 1, "method": method, "params": params}, timeout=60)
            if out and out.get("result") is not None:
                return out["result"], url
            last = out.get("error") if out else "empty"
        except Exception as e:  # noqa: BLE001 - try the next endpoint
            last = str(e)
    raise RuntimeError(f"{method} {params[0] if params else ''} on chain {chain}: {last}")


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--out", default=os.path.join(os.path.dirname(__file__), "..", "corpus", "attacks"))
    args = ap.parse_args()
    os.makedirs(args.out, exist_ok=True)

    rows, failures = [], []
    for case in CASES:
        for h in case["txs"]:
            try:
                tx, url = rpc(case["chain"], "eth_getTransactionByHash", [h])
                receipt, _ = rpc(case["chain"], "eth_getTransactionReceipt", [h])
            except Exception as e:  # noqa: BLE001 - recorded, not fatal
                print(f"   FAILED {case['id']} {h}: {e}", file=sys.stderr)
                failures.append({"case": case["id"], "tx": h, "error": str(e)})
                continue
            meta = {k: v for k, v in case.items() if k != "txs"}
            rows.append({
                "case": meta,
                "chainId": case["chain"],
                "hash": h,
                "endpoint": url,
                "tx": tx,
                "receipt": {"status": receipt.get("status"), "logs": receipt.get("logs", [])},
            })
            print(f"   {case['id']}: {h[:18]}… status {receipt.get('status')}", file=sys.stderr)

    path = os.path.join(args.out, "attacks.jsonl.gz")
    with open(path, "wb") as raw:
        with gzip.GzipFile(fileobj=raw, mode="wb", mtime=0) as f:
            for r in rows:
                f.write(json.dumps(r, separators=(",", ":")).encode() + b"\n")
    with open(os.path.join(args.out, "manifest.json"), "w") as f:
        json.dump({
            "collected": datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
            "cases": len(CASES),
            "transactions": len(rows),
            "failures": failures,
        }, f, indent=1)
        f.write("\n")
    print(f"\n{len(rows)} attack transactions from {len(CASES)} cases; {len(failures)} failed", file=sys.stderr)


if __name__ == "__main__":
    main()
