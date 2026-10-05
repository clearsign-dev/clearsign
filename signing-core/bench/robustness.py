#!/usr/bin/env python3
"""Record fuzzing and mutation-testing results in results/robustness.json.

Those two runs are not part of this harness — they use cargo-fuzz and
cargo-mutants directly — but their numbers belong beside the others, and like
the others they should reach the documents from a file rather than by hand.

    python3 robustness.py \
        --fuzz tx_bytes=LOG --fuzz safe_tx=LOG ... \
        --mutants-before DIR/mutants.out --mutants-after DIR/mutants.out \
        --minutes 45 --cli-ms 3.8 --cli-runs 300 \
        [--fuzz-final tx_bytes=LOG ... --final-minutes 8]

`--fuzz-final` records a second, shorter round on the code as finally
committed, for when the long round ran before the last fixes.

The fuzz logs are libFuzzer's output from `cargo fuzz run TARGET -- -max_total_time=...`;
the mutants directories are what `cargo mutants -o DIR` writes.
"""

import argparse
import json
import os
import re

HERE = os.path.dirname(os.path.abspath(__file__))


def fuzz_stats(path):
    text = open(path, errors="replace").read()
    execs = re.findall(r"^#(\d+)\s", text, re.M)
    covs = re.findall(r"cov: (\d+)", text)
    failures = len(re.findall(r"panicked at|SUMMARY: libFuzzer|deadly signal|ERROR: AddressSanitizer", text))
    done = re.findall(r"Done (\d+) runs in (\d+) second", text)
    runs = int(done[-1][0]) if done else int(execs[-1]) if execs else 0
    secs = int(done[-1][1]) if done else None
    return {"executions": runs, "seconds": secs, "coverage_edges": int(covs[-1]) if covs else None, "failures": failures}


def mutants(paths):
    """Outcomes from one run, or from a run resumed with --iterate in several
    passes: each mutant keeps the outcome of the last pass that tested it."""
    status = {}
    for path in paths:
        for outcome in ("caught", "missed", "timeout", "unviable"):
            p = os.path.join(path, outcome + ".txt")
            if os.path.exists(p):
                for line in open(p):
                    if line.strip():
                        status[line.strip()] = outcome

    def count(outcome):
        return sum(1 for o in status.values() if o == outcome)
    caught, missed = count("caught"), count("missed")
    timeout, unviable = count("timeout"), count("unviable")
    missed_list = sorted(m for m, o in status.items() if o == "missed")
    return {
        "total": len(status),
        "caught": caught, "missed": missed, "timeout": timeout, "unviable": unviable,
        "score": round(100 * caught / (caught + missed), 1) if caught + missed else None,
        "missed_list": missed_list,
    }


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--fuzz", action="append", default=[])
    ap.add_argument("--fuzz-final", action="append", default=[])
    ap.add_argument("--final-minutes", type=int)
    ap.add_argument("--survivors-equivalent", action="store_true",
                    help="every surviving mutant was checked and is equivalent; say so")
    ap.add_argument("--mutants-before", action="append", default=[])
    ap.add_argument("--mutants-after", action="append", default=[],
                    help="repeat for each pass of a run resumed with --iterate, oldest first")
    ap.add_argument("--mutants-json", action="append", default=[],
                    help="the same for clearsign-safe-json, the JSON readers")
    ap.add_argument("--minutes", type=int, default=45)
    ap.add_argument("--cli-ms", type=float)
    ap.add_argument("--cli-runs", type=int)
    ap.add_argument("--binary-bytes", type=int)
    ap.add_argument("--wasm-bytes", type=int)
    ap.add_argument("--cli-note", default="",
                    help="how the CLI was timed, if not on a quiet machine")
    ap.add_argument("--arena", action="append", default=[],
                    help="WHAT=BYTES from crates/clearsign-ffi/tests/arena_budget.rs, one per case")
    args = ap.parse_args()

    fuzz = {}
    for spec in args.fuzz:
        name, path = spec.split("=", 1)
        fuzz[name] = fuzz_stats(path)
    final = {}
    for spec in args.fuzz_final:
        name, path = spec.split("=", 1)
        final[name] = fuzz_stats(path)
    total = sum(f["executions"] for f in fuzz.values()) + sum(f["executions"] for f in final.values())
    failures = sum(f["failures"] for f in fuzz.values()) + sum(f["failures"] for f in final.values())
    targets = len(set(fuzz) | {n.split(" ")[0] for n in final})
    before = mutants(args.mutants_before) if args.mutants_before else None
    after = mutants(args.mutants_after) if args.mutants_after else None
    json_readers = mutants(args.mutants_json) if args.mutants_json else None

    lines = []
    if fuzz:
        lines.append(f"**Fuzzing, {args.minutes} minutes per target on 5 Oct 2026**, on the targets the new code runs through:")
        lines.append("")
        lines.append("| Target | Executions | Coverage edges | Failures |")
        lines.append("|---|---:|---:|---:|")
        for name, f in fuzz.items():
            lines.append(f"| `{name}` | {f['executions']:,} | {f['coverage_edges'] or '–'} | {f['failures']} |")
        lines.append("")
    if final:
        lines.append(f"**Again on the final code**, after a review of the new decoders led to five more fixes, "
                     f"{args.final_minutes} minutes per target:")
        lines.append("")
        lines.append("| Target | Executions | Coverage edges | Failures |")
        lines.append("|---|---:|---:|---:|")
        for name, f in final.items():
            target, _, note = name.partition(" ")
            label = f"`{target}`" + (f" {note}" if note else "")
            lines.append(f"| {label} | {f['executions']:,} | {f['coverage_edges'] or '–'} | {f['failures']} |")
        lines.append("")
    if before and after:
        lines.append(f"**Mutation testing** (`cargo mutants -p clearsign`, with every test in the workspace): the decoder as it was "
                     f"before this work had {before['caught']} mutants caught and {before['missed']} missed, a score of "
                     f"{before['score']}% ({before['timeout']} more timed out and {before['unviable']} did not compile). "
                     f"After the new decoders and the tests written for the survivors: {after['caught']} caught "
                     f"and {after['missed']} missed of {after['total']}, {after['score']}% ({after['timeout']} timed out, "
                     f"{after['unviable']} did not compile)." +
                     ("" if not after["missed_list"] else
                      f" Each of the {after['missed']} survivors was examined and is equivalent: no input tells it from "
                      "the original, so no test can kill it. They are listed with the reason for each in "
                      "`crates/clearsign/tests/mutation_gaps.rs`." if args.survivors_equivalent else
                      " The survivors are listed in `results/robustness.json`."))
        if json_readers:
            lines.append("")
            lines.append(f"The JSON readers (`cargo mutants -p clearsign-safe-json`), which take Safe records and "
                         f"typed-data requests from files nobody vouches for: {json_readers['caught']} caught and "
                         f"{json_readers['missed']} missed of {json_readers['total']}, {json_readers['score']}% "
                         f"({json_readers['unviable']} did not compile)." +
                         (" The survivors are equivalent too; `crates/clearsign-safe-json/tests/strict_input.rs` "
                          "says why for each." if args.survivors_equivalent and json_readers["missed"] else ""))
    facts = []
    if fuzz:
        facts.append([f"{total / 1e6:.0f} million", f"fuzzing executions across {targets} targets, {failures} failures"])
    if before and after:
        facts.append([f"{after['score']}%", f"mutation score of the new decoder (was {before['score']}% before the gap tests)"])

    out = {
        "fuzz": fuzz,
        "fuzz_final": final,
        "fuzz_total_executions": total,
        "fuzz_failures": failures,
        "mutants_before": before,
        "mutants_after": after,
        "mutants_json_readers": json_readers,
        "fuzz_summary": f"{total:,} fuzzing executions across {targets} targets, {failures} failures." if fuzz else "",
        "mutation_summary": (f"{after['score']}% of mutants caught ({before['score']}% before)"
                             + ("; every survivor is equivalent." if args.survivors_equivalent and after["missed"] else ".")
                             if before and after else ""),
        "lines": lines,
        "facts": facts,
    }
    if args.cli_ms:
        out["cli"] = {"median_ms": args.cli_ms, "runs": args.cli_runs, "binary_bytes": args.binary_bytes,
                      "wasm_bytes": args.wasm_bytes, "note": args.cli_note}
    if args.arena:
        out["arena"] = [{"what": w, "bytes": int(b)} for w, b in (a.rsplit("=", 1) for a in args.arena)]
    os.makedirs(os.path.join(HERE, "results"), exist_ok=True)
    with open(os.path.join(HERE, "results", "robustness.json"), "w") as f:
        json.dump(out, f, indent=1)
        f.write("\n")
    print(json.dumps({k: v for k, v in out.items() if k not in ("lines", "facts")}, indent=1)[:2000])


if __name__ == "__main__":
    main()
