#!/usr/bin/env bash
# Download the Alpine Linux ARM64 "virt" image and verify it two independent ways:
#   1. SHA-256 against Alpine's published checksum
#   2. GPG signature against Alpine's release signing key, whose fingerprint is
#      pinned below. The key is fetched separately from alpinelinux.org.
# Refuses to leave an unverified image in place.
set -euo pipefail

source "$(cd "$(dirname "$0")/.." && pwd)/config/pins.env"
VERSION="$ALPINE_VERSION"
BRANCH="v${VERSION%.*}"
ISO="alpine-virt-${VERSION}-aarch64.iso"
BASE="https://dl-cdn.alpinelinux.org/alpine/${BRANCH}/releases/aarch64"
# Natanael Copa's Alpine release signing key, as published at
# https://alpinelinux.org/keys/ncopa.asc. Verify this fingerprint independently
# (for example on alpinelinux.org/downloads) before trusting it.
PINNED_FPR="$ALPINE_SIGNING_KEY_FPR"

HERE="$(cd "$(dirname "$0")" && pwd)"
IMAGES="$HERE/images"
mkdir -p "$IMAGES"
cd "$IMAGES"

echo "Fetching $ISO"
curl -fL --retry 3 -o "$ISO.part" "$BASE/$ISO"
curl -fsSL -o "$ISO.sha256" "$BASE/$ISO.sha256"
curl -fsSL -o "$ISO.asc" "$BASE/$ISO.asc"

echo "Checking SHA-256"
expected="$(awk '{print $1}' "$ISO.sha256")"
actual="$(shasum -a 256 "$ISO.part" | awk '{print $1}')"
if [[ "$expected" != "$actual" ]]; then
  echo "SHA-256 MISMATCH: expected $expected got $actual" >&2
  rm -f "$ISO.part"
  exit 1
fi

echo "Checking GPG signature"
GNUPGHOME="$(mktemp -d)"
export GNUPGHOME
trap 'rm -rf "$GNUPGHOME"' EXIT
curl -fsSL https://alpinelinux.org/keys/ncopa.asc | gpg --quiet --import
fpr="$(gpg --with-colons --fingerprint | awk -F: '/^fpr:/ {print $10; exit}')"
if [[ "$fpr" != "$PINNED_FPR" ]]; then
  echo "SIGNING KEY FINGERPRINT MISMATCH: got $fpr, pinned $PINNED_FPR" >&2
  rm -f "$ISO.part"
  exit 1
fi
if ! gpg --quiet --status-fd 1 --verify "$ISO.asc" "$ISO.part" 2>/dev/null | grep -q "VALIDSIG $PINNED_FPR"; then
  echo "GPG SIGNATURE INVALID" >&2
  rm -f "$ISO.part"
  exit 1
fi

mv "$ISO.part" "$ISO"
echo "Verified: $IMAGES/$ISO"
