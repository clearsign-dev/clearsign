#!/usr/bin/env bash
# Build the guest kernel from scratch in a brand-new, empty build volume and compare its
# SHA-256 with the existing build. Matching hashes mean the kernel build is reproducible
# on this machine from the pinned inputs.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/config/pins.env"
SRC="$HOME/.cache/osproject/guest-src"
REF="$HOME/.cache/osproject/guest-built/SHA256SUMS"
OUT="$(mktemp -d)"
VOL="osproject-repro-$$"
trap 'docker volume rm -f "$VOL" >/dev/null 2>&1; rm -rf "$OUT"' EXIT
docker run --rm --platform linux/arm64 -v "$SRC:/src" -v "$OUT:/out" -v "$VOL:/build" \
  "$DEBIAN_BUILD_IMAGE" /src/in-container.sh "$GUEST_KERNEL_VERSION" >/dev/null
want=$(awk '$2=="Image"{print $1}' "$REF")
got=$(awk '$2=="Image"{print $1}' "$OUT/SHA256SUMS")
echo "reference build: $want"
echo "clean rebuild:   $got"
[[ "$want" == "$got" ]] && echo "REPRODUCIBLE: kernel Image identical from a clean build" || { echo "NOT REPRODUCIBLE" >&2; exit 1; }
