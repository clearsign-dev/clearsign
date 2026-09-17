#!/usr/bin/env bash
# Build the signer system image for seL4 on QEMU virt (aarch64).
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
source "$ROOT/config/pins.env"
SDK="$HERE/../vendor/microkit-sdk-$MICROKIT_VERSION"
BOARD="${BOARD:-qemu_virt_aarch64}"
CONFIG="${CONFIG:-debug}"
BD="$SDK/board/$BOARD/$CONFIG"
OUT="$HERE/build"
mkdir -p "$OUT"

( cd "$ROOT/signing-core" && cargo build -q -p clearsign-ffi --release --locked --target aarch64-unknown-none )
LIB="$ROOT/signing-core/target/aarch64-unknown-none/release/libclearsign_ffi.a"

CFLAGS=(-nostdlib -ffreestanding -g -O2 -Wall -Werror -I"$BD/include" -I"$HERE" -mstrict-align)
for pd in signer wallet_ui; do
  aarch64-elf-gcc -c "${CFLAGS[@]}" "$HERE/$pd.c" -o "$OUT/$pd.o"
done
aarch64-elf-ld -L"$BD/lib" "$OUT/signer.o" "$LIB" -lmicrokit -Tmicrokit.ld --gc-sections -o "$OUT/signer.elf"
aarch64-elf-ld -L"$BD/lib" "$OUT/wallet_ui.o" -lmicrokit -Tmicrokit.ld -o "$OUT/wallet_ui.elf"
"$SDK/bin/microkit" "$HERE/signer.system" --search-path "$OUT" --board "$BOARD" --config "$CONFIG" \
  -o "$OUT/loader.img" -r "$OUT/report.txt"
echo "image: $OUT/loader.img"
