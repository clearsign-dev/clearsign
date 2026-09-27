#!/usr/bin/env bash
# Build the reviewing tool for this machine and package it with its hash.
#
# For handing someone a copy today, without waiting for a tagged release. The
# release workflow builds the same thing for four platforms; this builds one.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
TARGET="${TARGET:-$(rustc -vV | sed -n 's|host: ||p')}"
OUT="$ROOT/dist"

cd "$ROOT"
rustup target add "$TARGET" >/dev/null 2>&1 || true
SOURCE_DATE_EPOCH=0 cargo build -p clearsign-cli --release --locked --target "$TARGET"

name="clearsign-$TARGET"
rm -rf "${OUT:?}/$name"
mkdir -p "$OUT/$name"
cp "target/$TARGET/release/clearsign" "$OUT/$name/"
cp "$ROOT/../docs/06-using-it-before-you-sign.md" "$OUT/$name/README.md"
cp "$ROOT/../LICENSE-MIT" "$ROOT/../LICENSE-APACHE" "$OUT/$name/"
tar -C "$OUT" -czf "$OUT/$name.tar.gz" "$name"
shasum -a 256 "$OUT/$name.tar.gz" | tee "$OUT/$name.tar.gz.sha256"

echo
echo "Built for $TARGET."
echo "Give them $OUT/$name.tar.gz and the hash above."
echo "They start with README.md inside it."
