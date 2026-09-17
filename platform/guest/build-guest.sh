#!/usr/bin/env bash
# Build the Linux compartment's guest from verified sources, replacing the unsigned prebuilt:
#   kernel    kernel.org tarball, signature checked against the pinned kernel.org key
#   userland  BusyBox and glibc from Debian packages (apt-verified) in the digest-pinned image
# Output: ~/.cache/osproject/guest-built/{Image,rootfs.cpio.gz,SHA256SUMS}
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/../.." && pwd)"
source "$ROOT/config/pins.env"
SRC="$HOME/.cache/osproject/guest-src"
OUT="$HOME/.cache/osproject/guest-built"
KV="$GUEST_KERNEL_VERSION"
mkdir -p "$SRC" "$OUT"
cd "$SRC"

[[ -f "linux-$KV.tar.xz" ]] || curl -fsSL -o "linux-$KV.tar.xz" "$GUEST_KERNEL_URL/linux-$KV.tar.xz"
curl -fsSL -o "linux-$KV.tar.sign" "$GUEST_KERNEL_URL/linux-$KV.tar.sign"
[[ "$(shasum -a 256 "linux-$KV.tar.xz" | cut -d' ' -f1)" == "$GUEST_KERNEL_TARBALL_SHA256" ]] || { echo "kernel tarball SHA-256 mismatch" >&2; exit 1; }
GNUPGHOME="$(mktemp -d)"; export GNUPGHOME
gpg -q --import "$ROOT/config/keys/kernel-gregkh-$GUEST_KERNEL_SIGNING_KEY_FPR.asc"
xz -dc "linux-$KV.tar.xz" | gpg --status-fd 1 --verify "linux-$KV.tar.sign" - 2>/dev/null \
  | grep -q "VALIDSIG $GUEST_KERNEL_SIGNING_KEY_FPR" || { echo "kernel signature not from pinned key" >&2; exit 1; }
rm -rf "$GNUPGHOME"; unset GNUPGHOME
echo "kernel $KV: tarball hash and kernel.org signature verified"

cat > "$SRC/in-container.sh" <<'IN'
#!/usr/bin/env bash
set -euo pipefail
export DEBIAN_FRONTEND=noninteractive
apt-get update -qq >/dev/null
apt-get install -y -qq --no-install-recommends build-essential bc bison flex libssl-dev libelf-dev \
  xz-utils cpio kmod python3 busybox libc6 >/dev/null
KV="$1"
# Reproducibility: fixed build identity and timestamp.
export KBUILD_BUILD_TIMESTAMP="1970-01-01" KBUILD_BUILD_USER="builder" KBUILD_BUILD_HOST="osproject"
export SOURCE_DATE_EPOCH=0
mkdir -p /build && cd /build
[[ -d "linux-$KV" ]] || tar xJf "/src/linux-$KV.tar.xz"
cd "linux-$KV"
make -s ARCH=arm64 defconfig
# Security-relevant settings for this compartment, applied on top of defconfig.
./scripts/config \
  -e DEVMEM -e STRICT_DEVMEM -d IO_STRICT_DEVMEM \
  -e BLK_DEV_INITRD -e DEVTMPFS -e DEVTMPFS_MOUNT \
  -e SERIAL_AMBA_PL011 -e SERIAL_AMBA_PL011_CONSOLE \
  -d MODULES \
  -e RANDOMIZE_BASE -e HARDENED_USERCOPY -e INIT_ON_ALLOC_DEFAULT_ON -e INIT_ON_FREE_DEFAULT_ON \
  -e STACKPROTECTOR_STRONG -e SLAB_FREELIST_RANDOM -e SLAB_FREELIST_HARDENED
make -s ARCH=arm64 olddefconfig
make -s ARCH=arm64 -j"$(nproc)" Image
cp arch/arm64/boot/Image /out/Image

# ---- root filesystem ----
R=/build/rootfs; rm -rf "$R"; mkdir -p "$R"/{bin,sbin,etc,proc,sys,dev,tmp,root,lib,usr/bin,usr/lib}
cp /bin/busybox "$R/bin/busybox"
# glibc runtime needed by the dynamically linked BusyBox (dynamic, so hardened_malloc can be preloaded).
for lib in $(ldd /bin/busybox | grep -oE '/[^ ]+'); do
  mkdir -p "$R$(dirname "$lib")"; cp -L "$lib" "$R$lib"
done
# The loop above copies the dynamic loader to the exact interpreter path BusyBox requests.
# Fail the build rather than ship a rootfs whose programs cannot start.
interp=$(readelf -l /bin/busybox | sed -n 's/.*Requesting program interpreter: \(.*\)\]/\1/p')
[[ -f "$R$interp" && ! -L "$R$interp" ]] || { echo "rootfs is missing a real file at interpreter path $interp" >&2; exit 1; }
ln -s busybox "$R/bin/sh"
cat > "$R/init" <<'INIT'
#!/bin/sh
/bin/busybox --install -s
mount -t proc proc /proc
mount -t sysfs sysfs /sys
mount -t devtmpfs devtmpfs /dev 2>/dev/null
mount -t tmpfs tmpfs /tmp
hostname compartment
echo "Linux compartment ready (kernel $(uname -r), userland: Debian BusyBox)"
exec /sbin/init
INIT
chmod 0755 "$R/init"
# The console is reachable only through seL4's debug serial port, so it runs a root shell
# directly rather than a password login (Debian's BusyBox login also expects /usr/sbin/nologin
# and NSS configuration this minimal rootfs does not carry).
cat > "$R/etc/inittab" <<'TAB'
ttyAMA0::respawn:-/bin/sh
::ctrlaltdel:/sbin/reboot
TAB
printf 'export PS1="compartment# "\n' > "$R/etc/profile"
printf 'root::0:0:root:/root:/bin/sh\n' > "$R/etc/passwd"
printf 'root:x:0:\n' > "$R/etc/group"
echo compartment > "$R/etc/hostname"
# cpio --reproducible still records mtimes in this version, which would make the
# rootfs differ between builds of identical content. Zero them first.
find "$R" -exec touch -h -d @0 {} +
( cd "$R" && find . -mindepth 1 | LC_ALL=C sort | cpio -o -H newc --reproducible -R 0:0 2>/dev/null | gzip -9n > /out/rootfs.cpio.gz )
cd /out && sha256sum Image rootfs.cpio.gz > SHA256SUMS
echo "busybox: $(dpkg -s busybox | grep ^Version)"
echo "glibc:   $(dpkg -s libc6 | grep ^Version)"
IN
chmod +x "$SRC/in-container.sh"

docker run --rm --platform linux/arm64 -v "$SRC:/src" -v "$OUT:/out" \
  -v osproject-guest-build:/build "$DEBIAN_BUILD_IMAGE" /src/in-container.sh "$KV"
cat "$OUT/SHA256SUMS"
