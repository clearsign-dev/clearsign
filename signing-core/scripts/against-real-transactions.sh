#!/usr/bin/env bash
# Run the reviewer over real Safe transactions pulled from Safe's own service.
#
# One fixture proves the decoder handles one transaction. This asks a harder
# question: given every transaction a Safe has ever queued, what does the
# reviewer say, and does its recomputed hash agree with Safe's, every time?
#
#   ./against-real-transactions.sh [outdir]
#
# Needs network. Writes a summary to stdout and per-transaction output to
# <outdir>/. Exit 1 if any hash disagrees — that would be a correctness bug.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
out="${1:-/tmp/clearsign-real}"
cs="$here/../target/release/clearsign"

[ -x "$cs" ] || { echo "build it first: cargo build --release -p clearsign-cli" >&2; exit 2; }
mkdir -p "$out"

# Safes with real history, chosen to span the two domain versions that matter.
# The first is the Bybit Safe: 73 ordinary transactions and one that took $1.5bn.
SAFES="
0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4 mainnet 1.1.x
0xA7A93fd0a276fc1C0197a5B5623eD117786eeD06 mainnet 1.1.x
0xe1ab8c08294F8ee707D4eFa458eaB8BbEeB09215 mainnet 1.1.x
0x2ebF891f4718EB8367013d8D975a1E5Afcae277F mainnet 1.3.0+
0x6F4565c9D673DBDD379ABa0b13f8088d1AF3Bb0C mainnet 1.3.0+
0xc3350595eD42EbE94556277bc77D257c76065291 mainnet 1.3.0+
"

total=0; ok=0; blind=0; crit=0; hash_ok=0; hash_bad=0
: > "$out/rows.tsv"

while read -r safe network version; do
  [ -z "${safe:-}" ] && continue
  echo "== $safe  ($network, Safe $version)" >&2
  curl -sSL --max-time 120 \
    "https://safe-transaction-$network.safe.global/api/v1/safes/$safe/multisig-transactions/?limit=100" \
    -o "$out/$safe.json"

  # One file per transaction, named by its hash so replaced nonces do not collide.
  python3 - "$out" "$safe" <<'PY'
import json, sys, os
out, safe = sys.argv[1], sys.argv[2]
d = json.load(open(f"{out}/{safe}.json"))
os.makedirs(f"{out}/tx", exist_ok=True)
for t in d.get("results", []):
    open(f"{out}/tx/{safe}-{t['safeTxHash']}.json", "w").write(json.dumps(t))
PY

  for f in "$out"/tx/"$safe"-*.json; do
    [ -e "$f" ] || continue
    total=$((total + 1))
    set +e
    "$cs" safe-json "$f" --chain-id 1 --safe-version "$version" > "$f.out" 2>&1
    code=$?
    set -e
    case $code in
      0) ok=$((ok + 1)) ;;
      2) blind=$((blind + 1)) ;;
      3) crit=$((crit + 1)) ;;
    esac

    # The claim worth checking: our hash equals the one Safe's service published.
    want=$(python3 -c "import json,sys;print(json.load(open(sys.argv[1]))['safeTxHash'].lower())" "$f")
    if grep -qi "matches the one computed here, $want" "$f.out"; then
      hash_ok=$((hash_ok + 1)); agree=agree
    else
      hash_bad=$((hash_bad + 1)); agree=DISAGREE
    fi

    # No CRITICAL lines is the good case, and grep exits 1 for it, so this must
    # not be allowed to end the run under `set -e -o pipefail`.
    findings=$( { grep -oE '\[CRITICAL\] [0-9]+:[A-Z_]+' "$f.out" || true; } | sed 's/.*://' | sort -u | paste -sd, - )
    printf '%s\t%s\t%s\t%s\t%s\n' "$safe" "$version" "$code" "$agree" "${findings:--}" >> "$out/rows.tsv"
  done
done <<< "$SAFES"

echo
echo "==================== $total real transactions ===================="
printf '  exit 0  nothing alarming ....... %s\n' "$ok"
printf '  exit 2  BLIND ................... %s\n' "$blind"
printf '  exit 3  CRITICAL ................ %s\n' "$crit"
echo
printf '  hash agrees with Safe ........... %s\n' "$hash_ok"
printf '  hash DISAGREES .................. %s\n' "$hash_bad"
echo
echo "CRITICAL findings by code:"
{ cut -f5 "$out/rows.tsv" | tr ',' '\n' | grep -v '^-\?$' || true; } | sort | uniq -c | sort -rn | sed 's/^/  /'
echo
echo "By Safe version:"
for v in 1.1.x 1.3.0+; do
  n=$(awk -F'\t' -v v="$v" '$2==v' "$out/rows.tsv" | wc -l | tr -d ' ')
  c=$(awk -F'\t' -v v="$v" '$2==v && $3==3' "$out/rows.tsv" | wc -l | tr -d ' ')
  z=$(awk -F'\t' -v v="$v" '$2==v && $3==0' "$out/rows.tsv" | wc -l | tr -d ' ')
  printf '  Safe %-8s %3s transactions: %3s CRITICAL, %3s clean\n' "$v" "$n" "$c" "$z"
done

[ "$hash_bad" -eq 0 ] || { echo; echo "FAIL: $hash_bad hashes disagree with Safe's service."; exit 1; }
