#!/usr/bin/env bash
# Build the signer-only image: a Linux kernel with no network stack, and an
# initramfs whose entire userland is the clearsign signer running as process 1.
#
# The gates below are the product. The build fails rather than ship an image
# that contains a shell, a second program, a dynamically linked binary, or a
# kernel that can speak to a network.
#
# Output: ~/.cache/osproject/signer-image/{Image,initramfs.cpio.gz,SHA256SUMS}
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/config/pins.env"
source "$ROOT/platform/lib/verify-kernel.sh"

SRC="$HOME/.cache/osproject/guest-src"          # shares the verified tarball
OUT="$HOME/.cache/osproject/signer-image"
TARGET=aarch64-unknown-linux-musl
mkdir -p "$OUT"

verify_kernel_tarball "$SRC" "$ROOT"

# The signer inside the image is built in the canonical environment, not on this
# machine, so that the image someone else builds from this source is the same
# image. See signing-core/scripts/reproducible-cross-check.sh for why that
# distinction matters: rustc's output differs between compiler hosts.
echo "building the signer binary in the canonical environment ($TARGET)"
TOOLCHAIN="$(sed -n 's/^channel = "\(.*\)"/\1/p' "$ROOT/signing-core/rust-toolchain.toml")"
BUILDWORK="$(mktemp -d)"
trap 'rm -rf "$BUILDWORK"' EXIT
(cd "$ROOT/signing-core" && tar --exclude ./target --exclude ./fuzz -cf - .) | (cd "$BUILDWORK" && tar -xf -)
mkdir -p "$BUILDWORK/cargo-cache"
cp -R "${CARGO_HOME:-$HOME/.cargo}/registry" "$BUILDWORK/cargo-cache/registry"
cp "$ROOT/signing-core/scripts/in-canonical-container.sh" "$BUILDWORK/in-container.sh"
chmod +x "$BUILDWORK/in-container.sh"
mkdir -p "$SRC/canonical"
docker run --rm --platform linux/arm64 --hostname builder-image \
  -v "$BUILDWORK:/src:ro" -v "$SRC/canonical:/artifacts" \
  -e BUILD_PATH=/build-one -e BUILD_LOCALE=C.UTF-8 -e BUILD_TZ=UTC -e BUILD_UMASK=022 -e LABEL=image \
  "$DEBIAN_BUILD_IMAGE" /src/in-container.sh "$TOOLCHAIN" "$TARGET" clearsign-device >/dev/null

BIN="$SRC/canonical/image-clearsign-device"
[[ -f "$BIN" ]] || { echo "the canonical build produced no signer binary" >&2; exit 1; }
# Gate 0: the binary in the image must be the one recorded in EXPECTED-HASHES.txt,
# so the image cannot quietly contain a locally built signer.
want="$(awk '/clearsign-device$/ {print $1}' "$ROOT/signing-core/EXPECTED-HASHES.txt")"
got="$(shasum -a 256 "$BIN" | cut -d' ' -f1)"
[[ -n "$want" ]] || { echo "EXPECTED-HASHES.txt has no entry for clearsign-device" >&2; exit 1; }
[[ "$want" == "$got" ]] || { echo "GATE FAILED: the signer binary is $got, not the recorded $want" >&2; exit 1; }
echo "signer binary matches the recorded canonical hash: $got"
cp "$BIN" "$SRC/clearsign-device"

cat > "$SRC/build-signer-image.sh" <<'IN'
#!/usr/bin/env bash
set -euo pipefail
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq >/dev/null
apt-get install -y -qq --no-install-recommends build-essential bc bison flex libssl-dev \
  libelf-dev xz-utils cpio python3 binutils >/dev/null
KV="$1"
export KBUILD_BUILD_TIMESTAMP="1970-01-01" KBUILD_BUILD_USER="builder" KBUILD_BUILD_HOST="osproject"
export SOURCE_DATE_EPOCH=0
mkdir -p /build && cd /build
[[ -d "linux-$KV" ]] || tar xJf "/src/linux-$KV.tar.xz"
cd "linux-$KV"

make -s ARCH=arm64 defconfig
# What the signer needs, and nothing else. NET is off: this kernel has no way to
# reach a network even if something in it wanted to.
./scripts/config \
  -d NET -d INET -d WIRELESS -d BT -d USB_SUPPORT -d SOUND -d DRM -d FB \
  -d MODULES -d BLOCK -d SWAP -d MAGIC_SYSRQ -d DEVMEM -d DEVKMEM -d DEBUG_FS \
  -d KEXEC -d HIBERNATION -d PROFILING -d BPF_SYSCALL -d PTP_1588_CLOCK \
  -e BLK_DEV_INITRD -e SERIAL_AMBA_PL011 -e SERIAL_AMBA_PL011_CONSOLE -e TTY \
  -e RANDOMIZE_BASE -e HARDENED_USERCOPY -e INIT_ON_ALLOC_DEFAULT_ON \
  -e INIT_ON_FREE_DEFAULT_ON -e STACKPROTECTOR_STRONG -e SLAB_FREELIST_RANDOM \
  -e SLAB_FREELIST_HARDENED -e STRICT_KERNEL_RWX -e STRICT_MODULE_RWX
make -s ARCH=arm64 olddefconfig

# Gate 1: the kernel must have no network stack at all.
grep -q '^# CONFIG_NET is not set' .config || { echo "GATE FAILED: the kernel still has networking" >&2; exit 1; }
# Gate 2: no loadable modules, so nothing can be added to this kernel later.
grep -q '^# CONFIG_MODULES is not set' .config || { echo "GATE FAILED: modules are enabled" >&2; exit 1; }
cp .config /out/kernel.config

make -s ARCH=arm64 -j"$(nproc)" Image
cp arch/arm64/boot/Image /out/Image

# ---- the whole userland ----
R=/build/signer-root; rm -rf "$R"; mkdir -p "$R/dev"
cp /src/clearsign-device "$R/init"
chmod 0755 "$R/init"
mknod "$R/dev/console" c 5 1

# Gate 3: the signer must be statically linked. A dynamic binary would need a
# loader and libraries, which would mean more than one program in the image.
if readelf -l "$R/init" | grep -q "Requesting program interpreter"; then
  echo "GATE FAILED: the signer is dynamically linked" >&2; exit 1
fi
# Gate 4: exactly one regular file in the image, and it is the signer.
files="$(cd "$R" && find . -type f | LC_ALL=C sort)"
[[ "$files" == "./init" ]] || { echo "GATE FAILED: the image contains more than the signer: $files" >&2; exit 1; }
# Gate 5: no shell, by name or by content.
if (cd "$R" && find . -name 'sh' -o -name 'bash' -o -name 'busybox' | grep -q .); then
  echo "GATE FAILED: a shell is present" >&2; exit 1
fi
if grep -qa 'BusyBox v' "$R/init"; then echo "GATE FAILED: busybox is linked in" >&2; exit 1; fi

# cpio --reproducible still records each file's mtime in this version, which
# would make the image differ between builds of identical content. Zero them.
find "$R" -exec touch -h -d @0 {} +
( cd "$R" && find . -mindepth 1 | LC_ALL=C sort \
    | cpio -o -H newc --reproducible -R 0:0 2>/dev/null | gzip -9n > /out/initramfs.cpio.gz )
cd /out && sha256sum Image initramfs.cpio.gz > SHA256SUMS
echo "image contents: $(cd "$R" && find . -mindepth 1 | LC_ALL=C sort | tr '\n' ' ')"
IN
chmod +x "$SRC/build-signer-image.sh"

docker run --rm --platform linux/arm64 -v "$SRC:/src" -v "$OUT:/out" \
  -v osproject-signer-build:/build "$DEBIAN_BUILD_IMAGE" /src/build-signer-image.sh "$GUEST_KERNEL_VERSION"

echo
cat "$OUT/SHA256SUMS"
