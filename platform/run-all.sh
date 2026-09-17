#!/usr/bin/env bash
# Build and test the whole platform from pinned, verified sources.
#   1. fetch and verify every borrowed component       (config/pins.env)
#   2. Rust signer, keys and authority engine tests    (signing-core)
#   3. GrapheneOS hardened_malloc build + upstream tests (Docker)
#  3b. Linux guest: kernel.org kernel + Debian userland, from verified sources (Docker)
#   4. seL4: untrusted wallet UI + bare-metal signer    (QEMU)
#   5. seL4: Linux compartment + signer + hardened_malloc (QEMU)
#   6. signer-only image: no network, no shell, signer as process 1 (QEMU)
# Prints a summary and exits non-zero if any stage fails.
set -uo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
LOGS="$ROOT/platform/logs"; mkdir -p "$LOGS"
declare -a NAMES RESULTS
stage() { # name, command...
  local name="$1"; shift
  local log="$LOGS/$(echo "$name" | tr ' /:' '___').log"
  printf '\n=== %s ===\n' "$name"
  local start=$SECONDS
  if "$@" >"$log" 2>&1; then
    RESULTS+=("PASS"); printf 'PASS  (%ss)  log: %s\n' "$((SECONDS-start))" "$log"
  else
    RESULTS+=("FAIL"); printf 'FAIL  (%ss)  log: %s\n' "$((SECONDS-start))" "$log"; tail -15 "$log"
  fi
  NAMES+=("$name")
}

stage "1 fetch and verify dependencies" bash "$ROOT/platform/fetch-deps.sh"
stage "2 Rust tests and lints" bash -c "cd '$ROOT/signing-core' && cargo test --workspace --release --locked && cargo clippy --workspace --all-targets --locked -- -D warnings"
stage "3 GrapheneOS hardened_malloc" bash "$ROOT/platform/grapheneos/build-hardened-malloc.sh"
stage "3b Linux guest from verified source" bash "$ROOT/platform/guest/build-guest.sh"
stage "4 seL4 wallet UI and signer" bash -c "'$ROOT/platform/sel4/signer-system/build.sh' && python3 '$ROOT/platform/sel4/signer-system/run.py' 180"
stage "5 seL4 Linux compartment and signer" bash -c "'$ROOT/platform/sel4/linux-signer/build.sh' && python3 '$ROOT/platform/sel4/linux-signer/run.py'"
stage "6 signer-only image" bash -c "'$ROOT/platform/signer-image/build.sh' && python3 '$ROOT/platform/signer-image/run.py'"

printf '\n==================== SUMMARY ====================\n'
fail=0
for i in "${!NAMES[@]}"; do
  printf '%-5s %s\n' "${RESULTS[$i]}" "${NAMES[$i]}"
  [[ "${RESULTS[$i]}" == "PASS" ]] || fail=1
done
exit $fail
