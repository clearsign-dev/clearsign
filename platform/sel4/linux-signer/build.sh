#!/usr/bin/env bash
# Build seL4 + Linux compartment + signer compartment for QEMU virt (aarch64).
# The seL4 build systems cannot handle spaces in paths, so the build runs in a
# space-free working copy under ~/.cache/osproject. Sources of truth stay here.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/../../.." && pwd)"
VENDOR="$HERE/../vendor"
source "$ROOT/config/pins.env"
W="$HOME/.cache/osproject/sel4"
LV="libvmm-$LIBVMM_TAG"
EX="$W/$LV/examples/linux_signer"
BUILD="$W/build-linux-signer"
LLVM=/opt/homebrew/opt/llvm/bin
LLD=/opt/homebrew/opt/lld/bin
export PATH="$LLVM:$LLD:$PATH"

mkdir -p "$W" "$BUILD"
rsync -a --delete --exclude examples/linux_signer "$VENDOR/$LV/" "$W/$LV/"
ln -sfn "$VENDOR/microkit-sdk-$MICROKIT_VERSION" "$W/microkit-sdk-$MICROKIT_VERSION"
SDK="$W/microkit-sdk-$MICROKIT_VERSION"

# 1. Rust decoder for the signer compartment
( cd "$ROOT/signing-core" && cargo build -q -p clearsign-ffi --release --locked --target aarch64-unknown-none )
FFI="$ROOT/signing-core/target/aarch64-unknown-none/release/libclearsign_ffi.a"

# 1b. The plan a prompt-injected assistant would propose, generated from the
# authority crate so the bytes on the wire are the ones that crate encodes.
( cd "$ROOT/signing-core" && cargo run -q -p authority --example emit-injected-plan ) > "$HERE/guest/request_injected_plan.h"

# 2. Guest tool, and an extra initramfs archive that adds it (Linux accepts concatenated archives)
"$LLVM/clang" --target=aarch64-linux-gnu -O1 -fno-vectorize -fno-slp-vectorize -ffreestanding -fno-stack-protector -fno-builtin -nostdlib -static \
  -fuse-ld=lld -B"$LLD" -Wl,-e,_start -Wl,--build-id=none -o "$HERE/guest/signer-request" "$HERE/guest/signer_request.c"
STAGE="$(mktemp -d)"; trap 'rm -rf "$STAGE"' EXIT
mkdir -p "$STAGE/usr/bin" && cp "$HERE/guest/signer-request" "$STAGE/usr/bin/" && chmod 0755 "$STAGE/usr/bin/signer-request"
# GrapheneOS hardened_malloc as the system-wide allocator (built by platform/grapheneos/build-hardened-malloc.sh)
GOS="$HERE/../../grapheneos/out"
if [[ -f "$GOS/libhardened_malloc.so" ]]; then
  mkdir -p "$STAGE/usr/lib" "$STAGE/etc"
  # Safety gate: a globally preloaded library with an unsatisfied dependency kills init.
  NEEDED=$(/opt/homebrew/opt/llvm/bin/llvm-readelf -d "$GOS/libhardened_malloc.so" | grep NEEDED | grep -oE '\[[^]]+\]' | tr -d '[]')
  for lib in $NEEDED; do
    case "$lib" in libc.so.6|ld-linux-aarch64.so.1) ;; *) echo "refusing to preload: guest lacks $lib" >&2; exit 1 ;; esac
  done
  cp "$GOS/libhardened_malloc.so" "$STAGE/usr/lib/" && chmod 0755 "$STAGE/usr/lib/libhardened_malloc.so"
  echo /usr/lib/libhardened_malloc.so > "$STAGE/etc/ld.so.preload"
  cp "$GOS/write_after_free" "$GOS/write_after_free_static" "$STAGE/usr/bin/"
  chmod 0755 "$STAGE/usr/bin/write_after_free" "$STAGE/usr/bin/write_after_free_static"
  echo "including GrapheneOS hardened_malloc"
fi
( cd "$STAGE" && find usr etc 2>/dev/null | LC_ALL=C sort | cpio -o -H newc -R 0:0 2>/dev/null | gzip -9n > "$BUILD/overlay.cpio.gz" )

# 3. Base guest images.
#    GUEST=built (default): kernel.org kernel + Debian userland, built by platform/guest/build-guest.sh
#    GUEST=prebuilt: third-party images pinned by hash only (fallback, weaker trust)
GUEST="${GUEST:-built}"
case "$GUEST" in
  built)
    GB="$HOME/.cache/osproject/guest-built"
    [[ -f "$GB/SHA256SUMS" ]] || { echo "no source-built guest: run platform/guest/build-guest.sh" >&2; exit 1; }
    ( cd "$GB" && shasum -a 256 -c SHA256SUMS >/dev/null ) || { echo "source-built guest does not match its SHA256SUMS" >&2; exit 1; }
    BASE_LINUX="guest-Image"; BASE_INITRD="guest-rootfs.cpio.gz"
    cp "$GB/Image" "$BUILD/$BASE_LINUX"; cp "$GB/rootfs.cpio.gz" "$BUILD/$BASE_INITRD"
    echo "guest: source-built (kernel $GUEST_KERNEL_VERSION, Debian userland)"
    ;;
  prebuilt)
    BASE_LINUX="$GUEST_LINUX_NAME"; BASE_INITRD="$GUEST_INITRD_NAME"
    IMAGES_CACHE="$HOME/.cache/osproject/guest-images"
    for f in "$BASE_LINUX" "$BASE_INITRD"; do
      [[ -f "$IMAGES_CACHE/$f" ]] || { echo "missing $f: run platform/fetch-deps.sh" >&2; exit 1; }
      cp "$IMAGES_CACHE/$f" "$BUILD/$f"
    done
    check() { [[ "$(shasum -a 256 "$1" | cut -d' ' -f1)" == "$2" ]] || { echo "SHA-256 mismatch: $1" >&2; exit 1; }; }
    check "$BUILD/$BASE_LINUX"  "$GUEST_LINUX_SHA256"
    check "$BUILD/$BASE_INITRD" "$GUEST_INITRD_SHA256"
    echo "guest: prebuilt (hash-only trust)"
    ;;
  *) echo "GUEST must be built or prebuilt" >&2; exit 1 ;;
esac
cat "$BUILD/$BASE_INITRD" "$BUILD/overlay.cpio.gz" > "$BUILD/rootfs-with-signer-request.cpio.gz"
# The guest device tree reserves a fixed 16 MiB window (0x4d000000-0x4e000000) for the initramfs.
size=$(stat -f %z "$BUILD/rootfs-with-signer-request.cpio.gz")
(( size < 16 * 1024 * 1024 )) || { echo "initramfs is $size bytes; exceeds the 16 MiB window in the guest device tree" >&2; exit 1; }

# 4. Example directory in the working copy
mkdir -p "$EX/board/qemu_virt_aarch64"
cp "$HERE/vmm.c" "$HERE/signer.c" "$EX/"
cp "$HERE/integrated.system" "$EX/board/qemu_virt_aarch64/linux_signer.system"
cp "$W/$LV/examples/simple/board/qemu_virt_aarch64/"*.dts "$EX/board/qemu_virt_aarch64/"
sed -e 's|^SYSTEM_FILE := .*|SYSTEM_FILE := $(SYSTEM_DIR)/linux_signer.system|' \
    -e 's|^IMAGES := vmm.elf|IMAGES := vmm.elf signer.elf|' \
    "$W/$LV/examples/simple/simple.mk" > "$EX/linux_signer.mk"
cat >> "$EX/linux_signer.mk" <<MK

signer.o: \$(EXAMPLE_DIR)/signer.c
	\$(CC) -ffreestanding -g -O2 -Wall -I\$(BOARD_DIR)/include \$(ARCH_FLAGS) -c -o \$@ \$<

signer.elf: signer.o
	\$(LD) \$(LDFLAGS) signer.o "$FFI" -lmicrokit -Tmicrokit.ld --gc-sections -o \$@
MK
sed -e 's|simple.mk|linux_signer.mk|g' "$W/$LV/examples/simple/Makefile" > "$EX/Makefile"

( cd "$EX" && make -s MICROKIT_BOARD=qemu_virt_aarch64 MICROKIT_SDK="$SDK" BUILD_DIR="$BUILD" \
    LINUX="$BASE_LINUX" INITRD="rootfs-with-signer-request.cpio.gz" )
echo "image: $BUILD/loader.img"
