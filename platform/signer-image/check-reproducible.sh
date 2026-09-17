#!/usr/bin/env bash
# Build the signer image twice in a clean build volume and require identical
# output. Content that is identical must hash identically, or "verify the image
# you are running" is a claim nobody can check.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
OUT="$HOME/.cache/osproject/signer-image"

run() {
  docker volume rm -f osproject-signer-build >/dev/null 2>&1 || true
  "$HERE/build.sh" >/dev/null
  shasum -a 256 "$OUT/Image" "$OUT/initramfs.cpio.gz" | awk '{print $1"  "$2}'
}

echo "build 1 (clean volume)"; first="$(run)"; echo "$first"
echo "build 2 (clean volume)"; second="$(run)"; echo "$second"
if [[ "$first" == "$second" ]]; then
  echo "REPRODUCIBLE: both builds produced the same kernel and the same image"
else
  echo "NOT REPRODUCIBLE" >&2
  diff <(echo "$first") <(echo "$second") >&2 || true
  exit 1
fi
