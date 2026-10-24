#!/usr/bin/env bash
# INV-9: build the signer binary twice, from two separate copies of the source in
# different directories with separate target directories, and require identical
# output. Paths that differ between machines are remapped out of the binary.
#
# This proves reproducibility on one machine. The v1 exit criterion also needs a
# second machine producing the same hash.
set -euo pipefail

TARGET="${TARGET:-aarch64-unknown-linux-musl}"
SRC="$(cd "$(dirname "$0")/.." && pwd)"
CARGO_HOME_DIR="${CARGO_HOME:-$HOME/.cargo}"
WORK="$(mktemp -d)"
trap 'rm -rf "$WORK"' EXIT

build() {
  local dir="$WORK/$1/signing-core"
  mkdir -p "$dir"
  # Copy tracked sources only, not build output.
  (cd "$SRC" && tar --exclude ./target --exclude ./fuzz --exclude ./bench -cf - .) | (cd "$dir" && tar -xf -)
  (
    cd "$dir"
    export RUSTFLAGS="--remap-path-prefix=$dir=/build --remap-path-prefix=$CARGO_HOME_DIR=/cargo -C strip=symbols"
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=rust-lld
    export SOURCE_DATE_EPOCH=0
    cargo build -q -p clearsign-cli --release --locked --target "$TARGET"
    shasum -a 256 "target/$TARGET/release/clearsign" | awk '{print $1}'
  )
}

A="$(build first-copy)"
B="$(build second-copy-with-a-longer-path)"
echo "build A: $A"
echo "build B: $B"
if [[ "$A" == "$B" ]]; then
  echo "REPRODUCIBLE: identical binaries for $TARGET"
else
  echo "NOT REPRODUCIBLE" >&2
  exit 1
fi
