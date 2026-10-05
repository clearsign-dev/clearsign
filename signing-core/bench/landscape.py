#!/usr/bin/env python3
"""Where a signing reviewer sits in the record of real hacks.

Inputs, all in corpus/hacks/:
  defillama-hacks-2026-10-05.json.gz   DeFiLlama's hack list (api.llama.fi/hacks)
  research-2026-10-05.json             per-incident research: how each attack
                                       actually reached its victim, with sources
and results/detect.json, the replay of the real attack transactions.

Every one of the 1,293 incidents gets one attack path:

  SIGNING_DECEPTION_MULTISIG  multisig owners approved a disguised transaction
  SIGNING_DECEPTION_EOA       a person signed something from a phishing site,
                              compromised frontend or tampered wallet
  DECEIVED_INTENT             the transaction said exactly what it did; the
                              person was wrong about who they were paying
  KEY_THEFT                   the attacker had the keys and signed for itself
  CONTRACT_OR_PROTOCOL_BUG    no victim signature at all
  INSIDER_OR_RUG, GOVERNANCE, UNKNOWN

Incidents worth $1M or more in the ambiguous DeFiLlama categories were
researched one by one. The rest are classified by the rules in RULES below,
from DeFiLlama's own technique label, and the output keeps the two apart.

    python3 landscape.py            # writes results/landscape.json and prints a summary
"""

import gzip
import json
import os
from collections import defaultdict
from datetime import datetime, timezone

HERE = os.path.dirname(os.path.abspath(__file__))
ON_PATH = {"SIGNING_DECEPTION_MULTISIG", "SIGNING_DECEPTION_EOA"}

# DeFiLlama technique -> attack path, for incidents nobody researched. Each
# rule says what the label almost always means; where a label is genuinely
# mixed, the safer reading for this benchmark (the one that does NOT credit
# the reviewer) is chosen, except for community-channel phishing and hijacked
# frontends, which nearly always mean users signing something.
RULES = {
    # Code, oracle, accounting and bridge flaws: nobody signs anything.
    "Protocol Logic": "CONTRACT_OR_PROTOCOL_BUG",
    "Oracle Manipulation": "CONTRACT_OR_PROTOCOL_BUG",
    "Token & Share Accounting": "CONTRACT_OR_PROTOCOL_BUG",
    "Input Validation": "CONTRACT_OR_PROTOCOL_BUG",
    "Reentrancy": "CONTRACT_OR_PROTOCOL_BUG",
    "Bridge & Cross-Chain": "CONTRACT_OR_PROTOCOL_BUG",
    "Market Manipulation": "CONTRACT_OR_PROTOCOL_BUG",
    "Rugpull": "INSIDER_OR_RUG",
    "Governance": "GOVERNANCE",
    "Key Compromise": "KEY_THEFT",
}
TECHNIQUE_RULES = {
    "Admin Drain": "INSIDER_OR_RUG",
    "Malware": "KEY_THEFT",
    "Key Leaked via Infrastructure": "KEY_THEFT",
    "API Key Compromised": "KEY_THEFT",
    "Database Breach": "KEY_THEFT",
    "Session Key Compromised": "KEY_THEFT",
    "Impersonation Scam": "DECEIVED_INTENT",
    "Address Poisoning": "DECEIVED_INTENT",
    # Users sent to a hijacked site and asked to sign.
    "DNS Hijack": "SIGNING_DECEPTION_EOA",
    "Frontend Compromise": "SIGNING_DECEPTION_EOA",
    "CDN Compromise": "SIGNING_DECEPTION_EOA",
    "Supply Chain Attack": "SIGNING_DECEPTION_EOA",
    "Phishing": "SIGNING_DECEPTION_EOA",
    "Social Account Takeover": "SIGNING_DECEPTION_EOA",
    "Signer Phishing": "SIGNING_DECEPTION_MULTISIG",
    "Blind Signing": "SIGNING_DECEPTION_MULTISIG",
}
NON_EVM = {
    "Bitcoin", "Solana", "Tron", "XRP", "NEM", "Nano", "Terra", "Elrond", "Sui", "Aptos", "Cardano",
    "Algorand", "Iota", "Stacks", "Liquid", "Monero", "Neutron", "Zcash", "Polkadot", "Cosmos", "Near",
    "Hyperliquid L1", "Mixin", "Loopring", "Starknet", "TON", "Litecoin", "Dogecoin", "EOS", "Waves",
    "Stellar", "Bitcoin Cash", "Osmosis", "Injective", "Sei", "Kusama", "Flow", "Hedera", "Tezos", "ICP",
    "Ripple", "Ethereum Classic", "Lisk", "Steem", "Hive", "Ontology", "Neo", "Klaytn", "Zilliqa",
}


def year(ts):
    return datetime.fromtimestamp(ts, timezone.utc).year


def rule_vector(h):
    t = h.get("technique") or ""
    if t in TECHNIQUE_RULES:
        return TECHNIQUE_RULES[t]
    c = h.get("classification") or ""
    if c in RULES:
        return RULES[c]
    if c == "Access Control":
        return "CONTRACT_OR_PROTOCOL_BUG"
    if c == "Social Engineering":
        return "SIGNING_DECEPTION_EOA"
    if c == "Frontend & Infrastructure":
        return "KEY_THEFT"
    return "UNKNOWN"


def main():
    hacks = json.loads(gzip.open(os.path.join(HERE, "corpus/hacks/defillama-hacks-2026-10-05.json.gz")).read())
    research = {r["id"]: r for r in json.load(open(os.path.join(HERE, "corpus/hacks/research-2026-10-05.json")))}
    detect = json.load(open(os.path.join(HERE, "results/detect.json")))["outcomes"]
    replayed = defaultdict(list)
    for o in detect:
        replayed[o["case"]].append(o)

    rows = []
    for i, h in enumerate(hacks):
        r = research.get(i)
        vector = r["vector"] if r else rule_vector(h)
        chains = h.get("chain") or []
        fmt = (r or {}).get("signed_payload_format") or ""
        if vector in ON_PATH | {"DECEIVED_INTENT"} and not fmt:
            fmt = "OTHER_NONEVM" if chains and all(c in NON_EVM for c in chains) else "EVM"
        rows.append({
            "id": i,
            "name": h["name"],
            "year": year(h["date"]),
            "usd": h.get("amount") or 0,
            "llama_technique": h.get("technique"),
            "vector": vector,
            "basis": "researched" if r else "rule",
            "format": fmt,
        })

    def agg(sel):
        n = len(sel)
        v = sum(x["usd"] for x in sel)
        return {"incidents": n, "usd": v}

    total = agg(rows)
    by_vector = {}
    for vec in sorted({x["vector"] for x in rows}):
        sel = [x for x in rows if x["vector"] == vec]
        by_vector[vec] = agg(sel) | {
            "researched": sum(1 for x in sel if x["basis"] == "researched"),
        }
    on_path = [x for x in rows if x["vector"] in ON_PATH]
    evm_formats = {"SAFE_TX", "EVM_TX", "EIP712", "EVM", "EIP7702_AUTH", "OTHER_MULTISIG"}
    on_path_evm = [x for x in on_path if x["format"] in evm_formats]

    by_year = {}
    for y in range(2016, 2027):
        sel = [x for x in rows if x["year"] == y]
        if not sel:
            continue
        op = [x for x in sel if x["vector"] in ON_PATH]
        by_year[y] = {
            "all": agg(sel),
            "on_signing_path": agg(op),
            "key_theft": agg([x for x in sel if x["vector"] == "KEY_THEFT"]),
            "contract_bug": agg([x for x in sel if x["vector"] == "CONTRACT_OR_PROTOCOL_BUG"]),
        }

    # The replayed attacks, by outcome: once per transaction, and once per
    # incident taking the best outcome among its transactions (an attack that
    # needed an approval and then a swap is caught if the approval is).
    rank = ["not reviewable", "shown faithfully, not flagged", "warning",
            "flagged, not decoded (BLIND)", "flagged, mechanism named"]

    def outcome_of(o):
        if o["verdict"] == "DO NOT SIGN" and o["specific"]:
            return "flagged, mechanism named"
        if o["verdict"] == "DO NOT SIGN":
            return "flagged, not decoded (BLIND)"
        if o["verdict"] == "review warnings":
            return "warning"
        if o["verdict"] == "nothing flagged":
            return "shown faithfully, not flagged"
        return "not reviewable"

    per_payload = defaultdict(int)
    per_incident = {}
    for o in detect:
        if o["vector"] not in ON_PATH:
            continue
        k = outcome_of(o)
        per_payload[k] += 1
        best = per_incident.get(o["incident"])
        if best is None or rank.index(k) > rank.index(best):
            per_incident[o["incident"]] = k
    outcomes = defaultdict(list)
    for incident, k in per_incident.items():
        outcomes[k].append(incident)

    out = {
        "generated": datetime.now(timezone.utc).strftime("%Y-%m-%d"),
        "rules": {"by_classification": RULES, "by_technique": TECHNIQUE_RULES},
        "total": total,
        "by_vector": by_vector,
        "on_signing_path": agg(on_path),
        "on_signing_path_evm": agg(on_path_evm),
        "on_signing_path_list": sorted(
            [{k: x[k] for k in ("name", "year", "usd", "vector", "format", "basis")} for x in on_path],
            key=lambda x: -x["usd"],
        ),
        "by_year": by_year,
        "replay_outcomes_by_incident": {k: sorted(v) for k, v in outcomes.items()},
        "replay_outcomes_by_payload": dict(per_payload),
    }
    os.makedirs(os.path.join(HERE, "results"), exist_ok=True)
    with open(os.path.join(HERE, "results/landscape.json"), "w") as f:
        json.dump(out, f, indent=1)
        f.write("\n")

    t = total
    print(f"{t['incidents']} incidents, ${t['usd']/1e9:.2f}B")
    for vec, a in sorted(by_vector.items(), key=lambda kv: -kv[1]["usd"]):
        print(f"  {vec:28s} {a['incidents']:5d}  ${a['usd']/1e9:6.2f}B  {100*a['usd']/t['usd']:5.1f}% of value  "
              f"{100*a['incidents']/t['incidents']:5.1f}% of incidents  ({a['researched']} researched)")
    op, ope = out["on_signing_path"], out["on_signing_path_evm"]
    print(f"\non the signing path: {op['incidents']} incidents, ${op['usd']/1e9:.2f}B "
          f"({100*op['usd']/t['usd']:.1f}% of value, {100*op['incidents']/t['incidents']:.1f}% of incidents)")
    print(f"  of which in formats ClearSign reads (EVM, Safe, EIP-712): {ope['incidents']} incidents, ${ope['usd']/1e9:.2f}B "
          f"({100*ope['usd']/t['usd']:.1f}% of all value)")
    print("\nby year: share of value on the signing path")
    for y, a in by_year.items():
        share = 100 * a["on_signing_path"]["usd"] / a["all"]["usd"] if a["all"]["usd"] else 0
        print(f"  {y}: ${a['all']['usd']/1e9:5.2f}B lost; on path ${a['on_signing_path']['usd']/1e6:8.1f}M ({share:4.1f}%); "
              f"key theft {100*a['key_theft']['usd']/a['all']['usd']:4.1f}%; code {100*a['contract_bug']['usd']/a['all']['usd']:4.1f}%")
    print("\nlargest on-path incidents:")
    for x in out["on_signing_path_list"][:20]:
        print(f"  {x['year']} {x['name'][:34]:34s} ${x['usd']/1e6:8.1f}M  {x['vector']:27s} {x['format']:12s} {x['basis']}")
    print("\nreplayed signer-deception attacks, by incident:")
    for k in reversed(rank):
        v = out["replay_outcomes_by_incident"].get(k, [])
        print(f"  {k:32s} {len(v):2d}  {', '.join(v)}")
    print("by payload:", out["replay_outcomes_by_payload"])


if __name__ == "__main__":
    main()
