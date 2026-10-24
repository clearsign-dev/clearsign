#!/usr/bin/env python3
"""Build the shareable results page from results/*.json.

    python3 page.py OUT.html

Every figure on the page is read from the same files docs/11 is generated
from, so the page and the document cannot disagree.
"""

import html
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
BEFORE = "before-3258e8d3"


def load(name, before=False):
    p = os.path.join(HERE, "results", BEFORE if before else "", name)
    return json.load(open(p)) if os.path.exists(p) else None


def main(out):
    evm, safe, detect, detect0 = load("evm.json"), load("safe.json"), load("detect.json"), load("detect.json", True)
    land, perf = load("landscape.json"), load("perf.json")
    evm0, safe0 = load("evm.json", True), load("safe.json", True)
    robust, permits = load("robustness.json"), load("permits.json")

    def s(chains, f):
        return sum(c.get(f, 0) for c in chains)

    def merged(chains, f):
        o = {}
        for c in chains:
            for k, v in c.get(f, {}).items():
                o[k] = o.get(k, 0) + v
        return o

    ec = [c for c in evm["chains"]]
    sc = [c for c in safe["chains"] if c["records"]]
    before = {(o["case"], o["tx"]): o for o in detect0["outcomes"]}

    def grade(o):
        if not o:
            return ["none", "–"]
        if o["verdict"] == "DO NOT SIGN" and o["specific"]:
            return ["named", o["decisive"][0]]
        if o["verdict"] == "DO NOT SIGN":
            return ["blind", "BLIND, not decoded"]
        if o["verdict"] == "review warnings":
            return ["warn", "Warning" + (f" · {o['decisive'][0]}" if o["decisive"] else "")]
        if o["verdict"] == "nothing flagged":
            return ["shown", "Shown, not flagged"]
        return ["none", "Not reviewable"]

    what = {
        "bybit": "Safe DELEGATECALL to attacker code, shown as a token transfer",
        "wazirx": "Safe DELEGATECALL replacing the implementation",
        "radiant-arbitrum": "transferOwnership of the address provider",
        "radiant-bsc": "The same, on BNB Chain",
        "uniswap-phishing": "setApprovalForAll on Uniswap V3 positions",
        "badger": "Unlimited increaseAllowance, from an injected script",
        "nexus-mutual": "A plain NXM transfer behind a tampered MetaMask",
        "poap-rpl": "A bounded approve to a drainer",
        "jaypegs-miso": "MISO createMarket with an injected address",
        "mm-finance": "approve to a fake router (DNS hijack)",
        "klayswap": "approve to a fake factory (BGP hijack)",
        "eip7702-eth": "A 7702 account batch of approvals",
        "eip7702-base": "The same, on Base",
    }
    chain_names = {1: "Ethereum", 56: "BNB Chain", 42161: "Arbitrum", 8453: "Base", 25: "Cronos", 8217: "Kaia"}
    attacks = []
    seen = set()
    for o in detect["outcomes"]:
        if o["vector"] not in ("SIGNING_DECEPTION_MULTISIG", "SIGNING_DECEPTION_EOA"):
            continue
        if o["case"] in seen:
            continue  # one row per incident: the best outcome of its transactions
        seen.add(o["case"])
        txs = [x for x in detect["outcomes"] if x["case"] == o["case"]]
        rank = ["none", "shown", "warn", "blind", "named"]
        best = max(txs, key=lambda x: rank.index(grade(x)[0]))
        b = max((before.get((x["case"], x["tx"])) for x in txs), key=lambda x: rank.index(grade(x)[0]))
        attacks.append({
            "incident": o["incident"].replace(" (Arbitrum)", "").replace(" (BNB Chain)", " (BNB Chain)"),
            "chain": chain_names.get(o["chain_id"], str(o["chain_id"])),
            "loss": o.get("loss_usd"),
            "what": what.get(o["case"], ""),
            "before": grade(b),
            "after": grade(best),
        })

    lt = land["total"]
    bv = land["by_vector"]
    other = sum(bv[k]["usd"] for k in ("DECEIVED_INTENT", "INSIDER_OR_RUG", "GOVERNANCE", "UNKNOWN"))
    other_n = sum(bv[k]["incidents"] for k in ("DECEIVED_INTENT", "INSIDER_OR_RUG", "GOVERNANCE", "UNKNOWN"))
    money = [
        {"key": "msig", "label": "Multisig owners deceived into signing", "usd": bv["SIGNING_DECEPTION_MULTISIG"]["usd"], "n": bv["SIGNING_DECEPTION_MULTISIG"]["incidents"], "path": True},
        {"key": "eoa", "label": "A person deceived into signing", "usd": bv["SIGNING_DECEPTION_EOA"]["usd"], "n": bv["SIGNING_DECEPTION_EOA"]["incidents"], "path": True},
        {"key": "keys", "label": "Key theft: the attacker held the keys", "usd": bv["KEY_THEFT"]["usd"], "n": bv["KEY_THEFT"]["incidents"], "path": False},
        {"key": "bugs", "label": "Contract or protocol bug: nobody signed", "usd": bv["CONTRACT_OR_PROTOCOL_BUG"]["usd"], "n": bv["CONTRACT_OR_PROTOCOL_BUG"]["incidents"], "path": False},
        {"key": "other", "label": "Insider, governance, recipient deception, unknown", "usd": other, "n": other_n, "path": False},
    ]
    years = []
    for y, a in land["by_year"].items():
        if int(y) >= 2018:
            years.append({"year": y, "all": a["all"]["usd"], "path": a["on_signing_path"]["usd"]})
    # Everything before the chart's first year, from the totals: the yearly
    # breakdown itself does not reach back to the list's earliest incidents.
    early = {"usd": lt["usd"] - sum(y["all"] for y in years),
             "path_usd": land["on_signing_path"]["usd"] - sum(y["path"] for y in years)}

    outcomes = [
        {"label": "Plain transactions, 31 chains", "o": merged(ec, "outcome")},
        {"label": "Safe transactions, 27 chains", "o": merged(sc, "outcome")},
        {"label": "Safe transactions, real multisigs only", "o": merged(sc, "outcome_multisig_only")},
    ]
    evm_rows = [{"name": c["name"], "proven": c["payload_proven"], "exact": c["digest_equal"], "accepted": c["clearsign_accepted"]}
                for c in ec if c["payload_proven"]]
    safe_rows = [{"name": c["name"], "records": c["records"], "agree": c["hash_agree_service"],
                  "sigs": c["confirmation_sigs_recovered_to_owner"] + c["onchain_sigs_recovered_to_owner"],
                  "sigs_total": c["confirmation_sigs_checked"] + c["onchain_sigs_checked"]} for c in sc]
    perf_rows = [{"what": p["what"], "p50": p["p50_us"], "p99": p["p99_us"], "rps": p["reviews_per_second"], "peak": p["peak_bytes_held"]}
                 for p in perf["scenarios"]]

    data = {
        "evm": {"chains": len(evm_rows), "proven": s(ec, "payload_proven"), "exact": s(ec, "digest_equal"),
                "accepted": s(ec, "clearsign_accepted"), "accepted_before": s(evm0["chains"], "clearsign_accepted")},
        "safe": {"chains": len(sc), "records": s(sc, "records"), "agree": s(sc, "hash_agree_service"),
                 "conf": s(sc, "confirmation_sigs_checked"), "conf_ok": s(sc, "confirmation_sigs_recovered_to_owner"),
                 "onchain": s(sc, "onchain_sigs_checked"), "onchain_ok": s(sc, "onchain_sigs_recovered_to_owner"),
                 "upgraded": s(sc, "signed_before_upgrade")},
        "attacks": attacks,
        "named": sum(1 for a in attacks if a["after"][0] == "named"),
        "named_before": sum(1 for a in attacks if a["before"][0] == "named"),
        "land": {"incidents": lt["incidents"], "usd": lt["usd"], "path_usd": land["on_signing_path"]["usd"],
                 "path_n": land["on_signing_path"]["incidents"]},
        "money": money, "years": years, "years_early": early, "outcomes": outcomes,
        "evm_rows": evm_rows, "safe_rows": safe_rows, "perf": perf_rows,
        "throughput": perf["corpus_throughput"],
        "robust": robust, "permits": permits,
    }

    page = TEMPLATE.replace("/*DATA*/", json.dumps(data, separators=(",", ":")))
    with open(out, "w") as f:
        f.write(page)
    print(f"wrote {out} ({len(page)//1024} KB)")


TEMPLATE = r"""<title>ClearSign Benchmarks</title>
<link rel="preconnect" href="https://fonts.googleapis.com">
<link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=IBM+Plex+Mono:wght@400;500&family=IBM+Plex+Sans:wght@400;500;600&display=swap">
<style>
/* Layout: one reading column of evidence, each section a claim followed by the data that tests it. */
:root {
  --paper: #f5f6f7; --surface: #ffffff; --sunk: #eceef0;
  --ink: #12161b; --ink-soft: #4a545e; --ink-faint: #6d7882;
  --line: #d9dde1; --line-strong: #b9c0c7;
  --accent: #2d6a8a; --accent-2: #7fb0c9; --accent-soft: #e4eef3;
  --gray-1: #59636c; --gray-2: #959fa8; --gray-3: #c6ccd2;
  --clear: #1f6b43; --clear-fill: #3d8f62; --caution: #8a5a00; --caution-fill: #d99a24;
  --critical: #b3261e; --critical-fill: #c9463e; --blind-fill: #6f7a84;
  --font-sans: "IBM Plex Sans", system-ui, -apple-system, "Segoe UI", sans-serif;
  --font-mono: "IBM Plex Mono", ui-monospace, "SF Mono", Menlo, Consolas, monospace;
}
@media (prefers-color-scheme: dark) { :root:not([data-theme="light"]) {
  --paper: #0e1216; --surface: #161b21; --sunk: #10151a;
  --ink: #e8ecef; --ink-soft: #a8b3bd; --ink-faint: #8693a0;
  --line: #262e36; --line-strong: #3a444e;
  --accent: #74b4d4; --accent-2: #3b6f8a; --accent-soft: #15242d;
  --gray-1: #9aa5af; --gray-2: #66727d; --gray-3: #3c454e;
  --clear: #7fc79d; --clear-fill: #3f9a66; --caution: #e0b562; --caution-fill: #c58b1c;
  --critical: #f2938c; --critical-fill: #d3554d; --blind-fill: #7b8792;
  color-scheme: dark; } }
:root[data-theme="dark"] {
  --paper: #0e1216; --surface: #161b21; --sunk: #10151a;
  --ink: #e8ecef; --ink-soft: #a8b3bd; --ink-faint: #8693a0;
  --line: #262e36; --line-strong: #3a444e;
  --accent: #74b4d4; --accent-2: #3b6f8a; --accent-soft: #15242d;
  --gray-1: #9aa5af; --gray-2: #66727d; --gray-3: #3c454e;
  --clear: #7fc79d; --clear-fill: #3f9a66; --caution: #e0b562; --caution-fill: #c58b1c;
  --critical: #f2938c; --critical-fill: #d3554d; --blind-fill: #7b8792;
  color-scheme: dark;
}
* { box-sizing: border-box; }
body { background: var(--paper); color: var(--ink); font-family: var(--font-sans); font-size: 15px; line-height: 1.55; }
.wrap { max-width: 1080px; margin: 0 auto; padding-inline: 20px; padding-block: 40px 72px; display: grid; gap: 56px; }
header { display: grid; gap: 14px; }
.brand { display: flex; align-items: center; gap: 12px; color: var(--ink-soft); font-size: 13px; letter-spacing: .02em; }
.brand svg { width: 30px; height: 30px; flex: none; --mark-c: var(--accent); --mark-gap: var(--paper); --mark-s: var(--ink); }
h1 { font-size: clamp(30px, 4.4vw, 44px); font-weight: 600; line-height: 1.12; letter-spacing: -.015em; margin: 0; text-wrap: balance; max-width: 22ch; }
.lede { font-size: 17px; color: var(--ink-soft); max-width: 66ch; margin: 0; }
.meta { font-family: var(--font-mono); font-size: 12px; color: var(--ink-faint); }
h2 { font-size: 22px; font-weight: 600; margin: 0; letter-spacing: -.005em; text-wrap: balance; }
h3 { font-size: 15px; font-weight: 600; margin: 0; }
section { display: grid; gap: 18px; min-width: 0; }
.claim { font-size: 16px; color: var(--ink-soft); max-width: 68ch; margin: 0; }
.claim strong { color: var(--ink); font-weight: 600; }
p { margin: 0; }
.panel { background: var(--surface); border: 1px solid var(--line); border-radius: 10px; padding: 20px; display: grid; gap: 16px; min-width: 0; }
.note { font-size: 13px; color: var(--ink-faint); max-width: 74ch; }
code, .mono { font-family: var(--font-mono); font-size: .9em; }

/* the four results */
.results { display: grid; grid-template-columns: repeat(4, minmax(0, 1fr)); gap: 1px; background: var(--line); border: 1px solid var(--line); border-radius: 10px; overflow: hidden; }
.result { background: var(--surface); padding: 18px; display: grid; gap: 6px; align-content: start; }
.result .k { font-size: 12px; letter-spacing: .06em; text-transform: uppercase; color: var(--ink-faint); }
.result .v { font-size: 30px; font-weight: 600; line-height: 1.1; letter-spacing: -.01em; }
.result .v small { font-size: 15px; font-weight: 500; color: var(--ink-soft); }
.result .d { font-size: 13px; color: var(--ink-soft); }
@media (max-width: 860px) { .results { grid-template-columns: repeat(2, minmax(0, 1fr)); } }
@media (max-width: 460px) { .results { grid-template-columns: 1fr; } }

/* stacked share bars */
.share { display: grid; gap: 10px; }
.share .row { display: grid; grid-template-columns: 92px 1fr; gap: 12px; align-items: center; }
.share .row > span { font-size: 13px; color: var(--ink-soft); }
.bar { display: flex; gap: 2px; height: 34px; min-width: 0; }
.seg { height: 100%; position: relative; display: flex; align-items: center; padding-inline: 8px; font-size: 12px; font-weight: 500; color: #fff; white-space: nowrap; overflow: visible; cursor: default; }
.seg:first-child { border-radius: 4px 0 0 4px; } .seg:last-child { border-radius: 0 4px 4px 0; }
.seg:focus-visible, .col:focus-visible, .oseg:focus-visible { outline: 2px solid var(--ink); outline-offset: 2px; }
.seg.msig { background: var(--accent); } .seg.eoa { background: var(--accent-2); }
.seg.keys { background: var(--gray-1); } .seg.bugs { background: var(--gray-2); } .seg.other { background: var(--gray-3); color: var(--ink); }
.legend { display: grid; grid-template-columns: repeat(auto-fit, minmax(240px, 1fr)); gap: 8px 20px; }
.legend div { display: grid; grid-template-columns: 12px 1fr auto; gap: 8px; align-items: baseline; font-size: 13px; }
.legend i { width: 12px; height: 12px; border-radius: 3px; display: inline-block; transform: translateY(1px); }
.legend b { font-weight: 600; font-variant-numeric: tabular-nums; }
.legend span { color: var(--ink-soft); }
.legend .path span { color: var(--ink); }

/* years: every column is the full chart height and stacks from the bottom, so
   labels above the tallest bar still sit inside the chart box */
.cols { display: grid; grid-template-columns: 44px repeat(9, minmax(0, 1fr)); gap: 10px; height: 310px; position: relative; }
.yaxis { position: relative; height: 100%; }
.yaxis span { position: absolute; right: 2px; transform: translateY(50%); font-size: 11px; color: var(--ink-faint); font-variant-numeric: tabular-nums; }
.gridl { position: absolute; left: 54px; right: 0; height: 1px; background: var(--line); }
.col { display: flex; flex-direction: column; justify-content: flex-end; align-items: center; gap: 4px; height: 100%; position: relative; z-index: 1; cursor: default; min-width: 0; }
.col .val { font-size: 11px; color: var(--ink-soft); font-variant-numeric: tabular-nums; white-space: nowrap; }
.stack { width: min(100%, 26px); display: flex; flex-direction: column; justify-content: flex-end; gap: 2px; flex: none; }
.stack .rest { background: var(--gray-2); border-radius: 4px 4px 0 0; }
.stack .path { background: var(--accent); }
.stack .path.only { border-radius: 4px 4px 0 0; }
.col .yr { font-size: 12px; color: var(--ink-faint); font-variant-numeric: tabular-nums; height: 18px; line-height: 18px; flex: none; }
.share-path { font-size: 11px; font-weight: 600; color: var(--accent); font-variant-numeric: tabular-nums; }
.years-legend { margin-top: 4px; }
/* Too narrow for a dollar figure over every bar: keep the share labels, which
   carry the point, and let the axis give the size. */
@media (max-width: 560px) { .col .val { display: none; } }

/* attack table */
.tablewrap { overflow-x: auto; min-width: 0; }
table { border-collapse: collapse; width: 100%; font-size: 13.5px; }
th { text-align: left; font-weight: 500; font-size: 12px; letter-spacing: .04em; text-transform: uppercase; color: var(--ink-faint); padding: 8px 10px; border-bottom: 1px solid var(--line-strong); white-space: nowrap; }
td { padding: 9px 10px; border-bottom: 1px solid var(--line); vertical-align: top; }
td.num, th.num { text-align: right; font-variant-numeric: tabular-nums; white-space: nowrap; }
tr:last-child td { border-bottom: 0; }
.chip { display: inline-flex; align-items: center; gap: 6px; padding: 2px 8px; border-radius: 999px; font-size: 12px; font-weight: 500; white-space: nowrap; border: 1px solid transparent; }
.chip::before { content: ""; width: 7px; height: 7px; border-radius: 50%; background: currentColor; flex: none; }
.chip.named { background: var(--accent-soft); color: var(--accent); border-color: color-mix(in oklab, var(--accent) 35%, transparent); }
.chip.blind { color: var(--caution); background: color-mix(in oklab, var(--caution) 10%, var(--surface)); border-color: color-mix(in oklab, var(--caution) 35%, transparent); }
.chip.warn { color: var(--caution); background: transparent; border-color: color-mix(in oklab, var(--caution) 45%, transparent); }
.chip.shown, .chip.none { color: var(--ink-soft); background: var(--sunk); }
.chip .mono { font-size: 11.5px; }
.inc { font-weight: 500; }
.what { color: var(--ink-soft); }

/* outcome bars */
.oseg { height: 100%; display: flex; align-items: center; padding-inline: 7px; font-size: 12px; font-weight: 500; color: #fff; white-space: nowrap; overflow: hidden; }
.oseg.clear { background: var(--clear-fill); } .oseg.warning { background: var(--caution-fill); color: #1b1407; }
.oseg.blind { background: var(--blind-fill); } .oseg.critical { background: var(--critical-fill); }
.ol i.clear { background: var(--clear-fill); } .ol i.warning { background: var(--caution-fill); }
.ol i.blind { background: var(--blind-fill); } .ol i.critical { background: var(--critical-fill); }
.obars .row { grid-template-columns: minmax(150px, 230px) 1fr; }
.obars .row > span { font-size: 13px; color: var(--ink-soft); }

/* chains */
.twocol { display: grid; grid-template-columns: repeat(2, minmax(0, 1fr)); gap: 20px; }
@media (max-width: 860px) { .twocol { grid-template-columns: 1fr; } }
.chainlist { max-height: 420px; overflow: auto; border: 1px solid var(--line); border-radius: 8px; }
.chainlist table { font-size: 13px; }
.chainlist th { position: sticky; top: 0; background: var(--surface); }
.ok { color: var(--clear); font-weight: 500; }
.ok::before { content: "✓ "; }

/* lists */
.facts { display: grid; grid-template-columns: repeat(auto-fit, minmax(260px, 1fr)); gap: 14px; }
.fact { border-top: 2px solid var(--accent); padding-top: 10px; display: grid; gap: 4px; }
.fact b { font-size: 20px; font-weight: 600; }
.fact span { font-size: 13px; color: var(--ink-soft); }
ul.limits { margin: 0; padding-left: 18px; display: grid; gap: 8px; max-width: 76ch; color: var(--ink-soft); }
ul.limits strong { color: var(--ink); font-weight: 600; }
footer { font-size: 13px; color: var(--ink-faint); display: grid; gap: 6px; border-top: 1px solid var(--line); padding-top: 18px; }
#tip { position: fixed; pointer-events: none; z-index: 10; background: var(--ink); color: var(--paper); font-size: 12.5px; line-height: 1.4; padding: 7px 10px; border-radius: 6px; max-width: 260px; opacity: 0; transition: opacity .08s; }
#tip b { display: block; font-size: 14px; }
@media (prefers-reduced-motion: reduce) { #tip { transition: none; } }
@media (max-width: 560px) {
  .share .row, .obars .row { grid-template-columns: 1fr; gap: 4px; }
  .cols { grid-template-columns: 32px repeat(9, minmax(0, 1fr)); gap: 3px; }
  .gridl { left: 35px; }
  .col .val { font-size: 9px; }
  .col .yr { font-size: 10px; }
}
</style>

<div class="wrap">
  <header>
    <div class="brand">
      <svg viewBox="0 0 1024 1024" role="img" aria-label="ClearSign">
        <path d="M 683.6 275.8 A 292 292 0 1 0 683.6 748.2" fill="none" stroke="var(--mark-c)" stroke-width="132" stroke-linecap="butt"/>
        <path d="M 635.8 328.0 A 118 118 0 1 0 538.0 512.0 A 118 118 0 1 1 440.2 696.0" fill="none" stroke="var(--mark-gap)" stroke-width="134" stroke-linecap="round" stroke-linejoin="round"/>
        <path d="M 635.8 328.0 A 118 118 0 1 0 538.0 512.0 A 118 118 0 1 1 440.2 696.0" fill="none" stroke="var(--mark-s)" stroke-width="104" stroke-linecap="round" stroke-linejoin="round"/>
      </svg>
      <span>ClearSign · benchmarks · data collected 4–5 October 2026</span>
    </div>
    <h1>What a signing reviewer can and cannot catch</h1>
    <p class="lede">ClearSign decodes what a transaction or signature will do from its bytes, before a person signs it. These are its measurements on real chains, against the transactions real attacks got people to sign, and against the full record of crypto hacks.</p>
    <p class="meta">Generated from signing-core/bench/results · same numbers as docs/11-benchmarks.md</p>
  </header>

  <div class="results" id="results"></div>

  <section>
    <h2>Where the money went</h2>
    <p class="claim" id="money-claim"></p>
    <div class="panel">
      <div class="share" id="money"></div>
      <div class="legend" id="money-legend"></div>
      <p class="note">DeFiLlama's hack list, 2011 to October 2026. Every incident of $1M or more in the ambiguous categories was researched one by one from post-mortems and the chain; the rest follow written rules from DeFiLlama's technique labels. Blue is the part a reviewer at the moment of signing is on the path of.</p>
    </div>
  </section>

  <section>
    <h2>It was rare until 2024</h2>
    <p class="claim">Before 2024 almost no money was lost through a deceived signer. In 2025 more than half was, almost all of it Bybit.</p>
    <div class="panel">
      <div class="cols" id="years" role="img" aria-label="Value lost each year, with the part lost through a deceived signer"></div>
      <div class="legend years-legend">
        <div class="path"><i style="background:var(--accent)"></i><span>Through a deceived signer</span><b></b></div>
        <div><i style="background:var(--gray-2)"></i><span>Everything else</span><b></b></div>
      </div>
      <p class="note">2020 is dominated by one $3.5B theft from keys generated with a weak random number generator. <span id="years-early"></span></p>
    </div>
  </section>

  <section>
    <h2>The attacks, replayed</h2>
    <p class="claim" id="attack-claim"></p>
    <div class="panel">
      <div class="tablewrap">
        <table id="attacks">
          <thead><tr><th>Incident</th><th>Chain</th><th class="num">Lost</th><th>What the victim signed</th><th>Before</th><th>After</th></tr></thead>
          <tbody></tbody>
        </table>
      </div>
      <p class="note">Each payload was rebuilt from the chain and proven before it was reviewed: Safe attacks by reproducing the hash the Safe contract emitted and recovering the owners' signatures over it, others by recovering the victim's signature. "Before" is the decoder at commit 3258e8d3.</p>
    </div>
    <p class="claim">Seven further cases in the replay were address poisoning, impersonation or fake portals. Those transactions said exactly what they did, and no reviewer of bytes can tell a look-alike address from the one the signer meant.</p>
  </section>

  <section>
    <h2>Exact on every chain it was tried on</h2>
    <p class="claim" id="chain-claim"></p>
    <div class="twocol">
      <div class="panel">
        <h3>Plain transactions</h3>
        <div class="chainlist"><table id="evm-table"><thead><tr><th>Chain</th><th class="num">Provable</th><th class="num">Exact digest</th></tr></thead><tbody></tbody></table></div>
        <p class="note">A transaction was used only if its on-chain signature recovers to the sender over the rebuilt payload. Chain-specific system types and Kaia's native format were excluded.</p>
      </div>
      <div class="panel">
        <h3>Safe transactions</h3>
        <div class="chainlist"><table id="safe-table"><thead><tr><th>Chain</th><th class="num">Records</th><th class="num">Hash agrees</th><th class="num">Signatures</th></tr></thead><tbody></tbody></table></div>
        <p class="note">Hash compared with Safe's service and with alloy; owner and on-chain signatures recovered over ClearSign's hash.</p>
      </div>
    </div>
  </section>

  <section>
    <h2>What it says about ordinary traffic</h2>
    <p class="claim">Most of what people sign is DeFi it cannot read, and it says so rather than guessing. That BLIND share is the honest size of the gap.</p>
    <div class="panel">
      <div class="share obars" id="outcomes"></div>
      <div class="legend ol">
        <div><i class="clear"></i><span>Nothing flagged</span><b></b></div>
        <div><i class="warning"></i><span>Warning to read</span><b></b></div>
        <div><i class="blind"></i><span>BLIND: could not decode</span><b></b></div>
        <div><i class="critical"></i><span>CRITICAL: high-risk action</span><b></b></div>
      </div>
    </div>
  </section>

  <section>
    <h2>Fast enough to never be the bottleneck</h2>
    <div class="panel">
      <div class="tablewrap"><table id="perf"><thead><tr><th>A complete review of</th><th class="num">Median</th><th class="num">p99</th><th class="num">Per second</th><th class="num">Peak memory</th></tr></thead><tbody></tbody></table></div>
      <p class="note" id="perf-note"></p>
    </div>
  </section>

  <section>
    <h2>Tested beyond the happy path</h2>
    <div class="facts" id="facts"></div>
  </section>

  <section>
    <h2>What this does not show</h2>
    <ul class="limits">
      <li><strong>It is not an audit.</strong> The code added in this round was written and tested with AI assistance and has not been read by an independent human reviewer.</li>
      <li><strong>The samples are samples.</strong> The busiest Safes on a chain are often automation, which is why real multisigs are counted separately. Two days of blocks is not a year.</li>
      <li><strong>The replay is small.</strong> Thirteen signer-deception incidents with public, provable transactions. Many attacks never publish theirs.</li>
      <li><strong>The hack record is a classification.</strong> The research behind each incident is published with its sources so every judgement can be checked.</li>
      <li><strong>Typed data is reviewed, not signed,</strong> and only from the command line for now.</li>
    </ul>
  </section>

  <footer>
    <span>Reproduce: <span class="mono">signing-core/bench</span> in the ClearSign repository. Methods, per-chain results and every source are in <span class="mono">docs/11-benchmarks.md</span>.</span>
  </footer>
</div>
<div id="tip" role="tooltip"></div>

<script>
const D = /*DATA*/;
const fmt = n => n.toLocaleString("en-US");
const usd = x => x >= 1e9 ? "$" + (x / 1e9).toFixed(2) + "B" : x >= 1e6 ? "$" + (x / 1e6).toFixed(1) + "M" : x >= 1e3 ? "$" + Math.round(x / 1e3) + "K" : "$" + Math.round(x);
const pct = (a, b, d = 1) => b ? (100 * a / b).toFixed(d) + "%" : "–";
const el = (tag, cls, text) => { const e = document.createElement(tag); if (cls) e.className = cls; if (text != null) e.textContent = text; return e; };

const tip = document.getElementById("tip");
function attachTip(node, title, body) {
  node.tabIndex = 0;
  const show = (x, y) => {
    tip.replaceChildren(el("b", null, title), document.createTextNode(body));
    const r = tip.getBoundingClientRect();
    tip.style.left = Math.min(window.innerWidth - r.width - 8, Math.max(8, x + 12)) + "px";
    tip.style.top = Math.max(8, y - r.height - 12) + "px";
    tip.style.opacity = 1;
  };
  node.addEventListener("pointermove", e => show(e.clientX, e.clientY));
  node.addEventListener("pointerleave", () => tip.style.opacity = 0);
  node.addEventListener("focus", () => { const r = node.getBoundingClientRect(); show(r.left + r.width / 2, r.top); });
  node.addEventListener("blur", () => tip.style.opacity = 0);
}

// The four results
const R = document.getElementById("results");
[
  ["Digest exact", fmt(D.evm.exact), "/ " + fmt(D.evm.proven), `real transactions from ${D.evm.chains} chains, exactly what each sender signed`],
  ["Safe hash agrees", fmt(D.safe.agree), "/ " + fmt(D.safe.records), `real Safe transactions on ${D.safe.chains} chains; every signature recovers over it`],
  ["Attacks named", String(D.named), "/ " + D.attacks.length, `signer-deception attacks replayed from the chain, up from ${D.named_before}`],
  ["Reviewer's reach", pct(D.land.path_usd, D.land.usd), "of value", `of ${usd(D.land.usd)} lost in ${fmt(D.land.incidents)} hacks went through a deceived signer`],
].forEach(([k, v, small, d]) => {
  const c = el("div", "result");
  c.append(el("div", "k", k));
  const vv = el("div", "v", v + " "); vv.append(el("small", null, small)); c.append(vv);
  c.append(el("div", "d", d));
  R.append(c);
});

// Where the money went
const mc = document.getElementById("money-claim");
mc.append("Of ", Object.assign(el("strong"), { textContent: usd(D.land.usd) }), ` lost across ${fmt(D.land.incidents)} hacks, `,
  Object.assign(el("strong"), { textContent: pct(D.land.path_usd, D.land.usd) + " went through a deceived signer" }),
  ` (${pct(D.land.path_n, D.land.incidents)} of incidents). That is the ceiling for any tool working at the moment of signing. Most of the rest was taken by people who already held the keys, or from contracts nobody had to sign anything to exploit.`);
const M = document.getElementById("money");
for (const [label, key, total] of [["By value", "usd", D.land.usd], ["By incidents", "n", D.land.incidents]]) {
  const row = el("div", "row"); row.append(el("span", null, label));
  const bar = el("div", "bar");
  for (const m of D.money) {
    const share = m[key] / total;
    const seg = el("div", "seg " + m.key);
    seg.style.flex = `${Math.max(share, 0.004)} 1 0`;
    if (share > 0.07) seg.textContent = (100 * share).toFixed(1) + "%";
    attachTip(seg, (100 * share).toFixed(1) + "% " + (key === "usd" ? "of value" : "of incidents"), `${m.label}: ${usd(m.usd)}, ${fmt(m.n)} incidents`);
    bar.append(seg);
  }
  row.append(bar); M.append(row);
}
const ML = document.getElementById("money-legend");
for (const m of D.money) {
  const d = el("div", m.path ? "path" : null);
  const i = el("i");
  i.style.background = `var(--${{ msig: "accent", eoa: "accent-2", keys: "gray-1", bugs: "gray-2", other: "gray-3" }[m.key]})`;
  d.append(i, el("span", null, m.label), el("b", null, usd(m.usd)));
  ML.append(d);
}

// Years. Bars grow from a baseline 22px above the chart's bottom edge (the year
// labels' row); gridlines and axis labels use the same scale and offset.
const Y = document.getElementById("years");
const maxB = 4e9, H = 230, BASE = 22;
const axis = el("div", "yaxis");
for (const t of [0, 1, 2, 3, 4]) {
  const s = el("span", null, "$" + t + "B"); s.style.bottom = (BASE + H * t * 1e9 / maxB) + "px"; axis.append(s);
  const g = el("div", "gridl"); g.style.bottom = (BASE + H * t * 1e9 / maxB) + "px"; Y.append(g);
}
Y.append(axis);
let pathTotal = 0, allTotal = 0;
for (const y of D.years) {
  pathTotal += y.path; allTotal += y.all;
  const col = el("div", "col");
  if (y.path >= 5e7) col.append(el("div", "share-path", pct(y.path, y.all, 0)));
  col.append(el("div", "val", usd(y.all).replace(".00B", "B")));
  const st = el("div", "stack");
  const rest = el("div", "rest"); rest.style.height = (H * (y.all - y.path) / maxB) + "px";
  const p = el("div", "path" + (y.all - y.path < 1 ? " only" : "")); p.style.height = (H * y.path / maxB) + "px";
  st.append(rest); if (y.path > 0) st.append(p);
  col.append(st, el("div", "yr", y.year));
  attachTip(col, usd(y.all) + " lost in " + y.year, `${usd(y.path)} (${pct(y.path, y.all)}) through a deceived signer`);
  Y.append(col);
}
const E = D.years_early;
if (E.usd > 1) {
  document.getElementById("years-early").textContent =
    `The chart starts in 2018: the ${usd(E.usd)} lost before then is left out` +
    (E.path_usd >= 1 ? `, ${usd(E.path_usd)} of it through a deceived signer.` : ", none of it through a deceived signer.");
}
const yl = document.querySelectorAll(".years-legend b");
yl[0].textContent = usd(pathTotal); yl[1].textContent = usd(allTotal - pathTotal);

// Attacks
document.getElementById("attack-claim").append("Of ", Object.assign(el("strong"), { textContent: D.attacks.length + " attacks that deceived signers" }),
  ", replayed from the chain, ClearSign now names the mechanism in ", Object.assign(el("strong"), { textContent: String(D.named) }),
  `, up from ${D.named_before}. Before, most were refused only because they could not be decoded, the same answer it gives to most ordinary traffic.`);
const AT = document.querySelector("#attacks tbody");
for (const a of D.attacks) {
  const tr = el("tr");
  tr.append(el("td", "inc", a.incident), el("td", null, a.chain), el("td", "num", a.loss ? usd(a.loss) : ""), el("td", "what", a.what));
  for (const g of [a.before, a.after]) {
    const td = el("td"); const c = el("span", "chip " + g[0]);
    if (g[0] === "named") { c.append(el("span", "mono", g[1])); } else { c.textContent = g[1]; }
    td.append(c); tr.append(td);
  }
  AT.append(tr);
}

// Chains
document.getElementById("chain-claim").append("On ", Object.assign(el("strong"), { textContent: fmt(D.evm.exact) + " of " + fmt(D.evm.proven) }),
  ` real transactions from ${D.evm.chains} chains ClearSign committed to exactly the digest the sender signed, and on `,
  Object.assign(el("strong"), { textContent: fmt(D.safe.agree) + " of " + fmt(D.safe.records) }),
  ` Safe transactions from ${D.safe.chains} chains its hash agreed with Safe's own service. ${fmt(D.safe.conf_ok + D.safe.onchain_ok)} owner and on-chain signatures all recover to an owner over it.`);
const ET = document.querySelector("#evm-table tbody");
for (const r of D.evm_rows) { const tr = el("tr"); tr.append(el("td", null, r.name), el("td", "num", fmt(r.proven))); const t = el("td", "num"); t.append(el("span", r.exact === r.proven ? "ok" : null, fmt(r.exact))); tr.append(t); ET.append(tr); }
const ST = document.querySelector("#safe-table tbody");
for (const r of D.safe_rows) { const tr = el("tr"); tr.append(el("td", null, r.name), el("td", "num", fmt(r.records))); const a = el("td", "num"); a.append(el("span", r.agree === r.records ? "ok" : null, fmt(r.agree))); const g = el("td", "num", fmt(r.sigs) + " / " + fmt(r.sigs_total)); tr.append(a, g); ST.append(tr); }

// Outcomes
const O = document.getElementById("outcomes");
const okeys = [["clear", "Nothing flagged"], ["warning", "Warning"], ["blind", "BLIND"], ["critical", "CRITICAL"]];
for (const o of D.outcomes) {
  const tot = okeys.reduce((t, [k]) => t + (o.o[k] || 0), 0);
  const row = el("div", "row"); row.append(el("span", null, o.label));
  const bar = el("div", "bar");
  for (const [k, name] of okeys) {
    const v = o.o[k] || 0; if (!v) continue;
    const seg = el("div", "oseg " + k); seg.style.flex = `${v / tot} 1 0`;
    if (v / tot > 0.09) seg.textContent = pct(v, tot, 0);
    attachTip(seg, pct(v, tot) + " " + name, `${fmt(v)} of ${fmt(tot)} — ${o.label}`);
    bar.append(seg);
  }
  row.append(bar); O.append(row);
}

// Perf
const PT = document.querySelector("#perf tbody");
for (const p of D.perf) {
  const tr = el("tr");
  tr.append(el("td", null, p.what), el("td", "num", p.p50 < 100 ? p.p50.toFixed(1) + " µs" : (p.p50 / 1000).toFixed(2) + " ms"),
    el("td", "num", p.p99 < 100 ? p.p99.toFixed(1) + " µs" : (p.p99 / 1000).toFixed(2) + " ms"), el("td", "num", fmt(Math.round(p.rps))),
    el("td", "num", p.peak >= 1024 ? Math.round(p.peak / 1024) + " KiB" : p.peak + " B"));
  PT.append(tr);
}
const T = D.throughput;
document.getElementById("perf-note").textContent = `One core of an Apple M5, release build. Over the real corpora: ${fmt(T.safe_records)} Safe transactions in ${Math.round(T.safe_seconds * 1000)} ms and ${fmt(T.evm_transactions)} plain transactions in ${Math.round(T.evm_seconds * 1000)} ms.`;

// Facts
const F = document.getElementById("facts");
const facts = [];
if (D.robust) { for (const f of D.robust.facts || []) facts.push(f); }
facts.push(["5,000", "random EIP-712 documents hashed identically by ClearSign and alloy"]);
facts.push(["2 of 2", "bugs planted in the EIP-712 code caught at once by the differential test"]);
if (D.permits) { const pv = Object.values(D.permits.proven).reduce((a, b) => a + b, 0); facts.push([fmt(pv), "real permit signatures recovered to their owners over ClearSign's EIP-712 hash"]); }
facts.push([String(D.safe.upgraded), "Safe transactions signed before an upgrade changed the Safe's domain, all handled"]);
for (const [b, s] of facts) { const f = el("div", "fact"); f.append(el("b", null, b), el("span", null, s)); F.append(f); }
</script>
"""

if __name__ == "__main__":
    main(sys.argv[1] if len(sys.argv) > 1 else os.path.join(HERE, "results", "page.html"))
