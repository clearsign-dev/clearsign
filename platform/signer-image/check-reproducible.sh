#!/usr/bin/env bash
# Build the signer image twice in a clean build volume and require identical
# output. Content that is identical must hash identically, or "verify the image
# you are running" is a claim nobody can check.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
OUT="$HOME/.cache/osproject/signer-image"
EXPECTED="$HERE/EXPECTED-HASHES.txt"

run() {
  docker volume rm -f osproject-signer-build >/dev/null 2>&1 || true
  # Command substitutions do not reliably inherit `errexit`. Remove previous
  # output and check the build explicitly so a failed build cannot make stale
  # artifacts look reproducible.
  rm -f "$OUT/Image" "$OUT/initramfs.cpio.gz" "$OUT/SHA256SUMS"
  "$HERE/build.sh" >/dev/null || return
  shasum -a 256 "$OUT/Image" "$OUT/initramfs.cpio.gz" \
    | awk '{name=$2; sub(/^.*\//, "", name); print $1"  "name}'
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

{
  cat <<'HEADER'
# The signer-only image, as built by platform/signer-image/build.sh in the
# canonical environment (config/pins.env: DEBIAN_BUILD_IMAGE, Rust pinned by
# signing-core/rust-toolchain.toml, kernel pinned in config/pins.env).
#
# Reproduce with platform/signer-image/check-reproducible.sh, which builds twice
# in clean volumes and records the agreeing hashes. The signer inside is the
# binary recorded in signing-core/EXPECTED-HASHES.txt, and the build refuses to
# package any other.
HEADER
  printf '%s\n' "$second"
} > "$EXPECTED"
echo "recorded in platform/signer-image/EXPECTED-HASHES.txt"
