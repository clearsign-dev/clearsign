#!/usr/bin/env bash
# Fetch and verify every borrowed component, using only the pins in config/pins.env.
# Idempotent: components already present and matching their pins are not downloaded again.
# Any mismatch stops the script.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
# shellcheck disable=SC1091
source "$ROOT/config/pins.env"
V="$ROOT/platform/sel4/vendor"
mkdir -p "$V"
ok() { printf '  \033[32mOK\033[0m %s\n' "$*"; }
die() { printf '  \033[31mFAIL\033[0m %s\n' "$*" >&2; exit 1; }
sha() { shasum -a 256 "$1" | cut -d' ' -f1; }

echo "[1/5] seL4 Microkit SDK $MICROKIT_VERSION"
cd "$V"
if [[ ! -f "$MICROKIT_SDK_ASSET" ]] || [[ "$(sha "$MICROKIT_SDK_ASSET")" != "$MICROKIT_SDK_SHA256" ]]; then
  gh release download "$MICROKIT_VERSION" -R seL4/microkit -p "$MICROKIT_SDK_ASSET" -p "$MICROKIT_SDK_ASSET.asc" --clobber
fi
[[ "$(sha "$MICROKIT_SDK_ASSET")" == "$MICROKIT_SDK_SHA256" ]] || die "SDK SHA-256 mismatch"
ok "SHA-256 matches pin"
GNUPGHOME="$(mktemp -d)"; export GNUPGHOME
gpg --quiet --import "$ROOT/config/keys/microkit-release-$MICROKIT_SIGNING_KEY_FPR.asc" >/dev/null 2>&1 \
  || die "stored signing key missing: config/keys/microkit-release-$MICROKIT_SIGNING_KEY_FPR.asc"
gpg --with-colons --fingerprint | grep -q "^fpr:::::::::$MICROKIT_SIGNING_KEY_FPR:" \
  || die "stored key does not have the pinned fingerprint"
gpg --status-fd 1 --verify "$MICROKIT_SDK_ASSET.asc" "$MICROKIT_SDK_ASSET" 2>/dev/null | grep -q "VALIDSIG $MICROKIT_SIGNING_KEY_FPR" \
  || die "SDK signature is not from the pinned key"
rm -rf "$GNUPGHOME"; unset GNUPGHOME
ok "GPG signature from pinned key $MICROKIT_SIGNING_KEY_FPR (trust: pinned-tofu)"
[[ -d "microkit-sdk-$MICROKIT_VERSION" ]] || tar xzf "$MICROKIT_SDK_ASSET"
ok "extracted"

echo "[2/5] libvmm $LIBVMM_TAG and sDDF"
tag_commit=$(git ls-remote https://github.com/au-ts/libvmm.git "refs/tags/$LIBVMM_TAG^{}" "refs/tags/$LIBVMM_TAG" | awk '{print $1}' | tail -1)
[[ "$tag_commit" == "$LIBVMM_COMMIT" ]] || die "upstream tag $LIBVMM_TAG now points to $tag_commit, pinned $LIBVMM_COMMIT"
ok "upstream tag still points to pinned commit"
if [[ ! -d "libvmm-$LIBVMM_TAG/.git" ]] || [[ "$(git -C "libvmm-$LIBVMM_TAG" rev-parse HEAD)" != "$LIBVMM_COMMIT" ]]; then
  rm -rf "libvmm-$LIBVMM_TAG"
  git clone -q --depth 1 --branch "$LIBVMM_TAG" https://github.com/au-ts/libvmm.git "libvmm-$LIBVMM_TAG"
fi
[[ "$(git -C "libvmm-$LIBVMM_TAG" rev-parse HEAD)" == "$LIBVMM_COMMIT" ]] || die "libvmm commit mismatch"
git -C "libvmm-$LIBVMM_TAG" submodule update --init --depth 1 dep/sddf >/dev/null 2>&1
[[ "$(git -C "libvmm-$LIBVMM_TAG/dep/sddf" rev-parse HEAD)" == "$SDDF_COMMIT" ]] || die "sDDF commit mismatch"
ok "libvmm $LIBVMM_COMMIT, sDDF $SDDF_COMMIT"

echo "[3/5] Linux guest images (trust: hash-only)"
G="$HOME/.cache/osproject/guest-images"; mkdir -p "$G"
fetch_image() { # name sha inner-file
  local name="$1" want="$2" inner="$3"
  if [[ ! -f "$G/$name" ]] || [[ "$(sha "$G/$name")" != "$want" ]]; then
    local tmp; tmp="$(mktemp -d)"
    curl -fsSL "$GUEST_IMAGE_BASE_URL/$name.tar.gz" -o "$tmp/a.tar.gz"
    tar xzf "$tmp/a.tar.gz" -C "$tmp"
    cp "$tmp/$name/$inner" "$G/$name"; rm -rf "$tmp"
  fi
  [[ "$(sha "$G/$name")" == "$want" ]] || die "$name SHA-256 mismatch"
  ok "$name"
}
fetch_image "$GUEST_LINUX_NAME" "$GUEST_LINUX_SHA256" Image
fetch_image "$GUEST_INITRD_NAME" "$GUEST_INITRD_SHA256" rootfs.cpio.gz

echo "[4/5] GrapheneOS release $GRAPHENEOS_RELEASE manifest"
A="$ROOT/platform/android"; mkdir -p "$A"
cp "$ROOT/config/keys/grapheneos_allowed_signers" "$A/grapheneos_allowed_signers"
fp=$(awk '{print $2" "$3}' "$A/grapheneos_allowed_signers" | ssh-keygen -lf - | awk '{print $2}')
[[ "$fp" == "$GRAPHENEOS_SSH_KEY_FP" ]] || die "stored GrapheneOS key does not match pin: $fp"
live=$(curl -fsSL https://grapheneos.org/allowed_signers | awk '{print $2" "$3}' | ssh-keygen -lf - | awk '{print $2}') || live="unreachable"
[[ "$live" == "$GRAPHENEOS_SSH_KEY_FP" ]] || echo "  WARNING: grapheneos.org now publishes key $live; investigate before changing the pin"
ok "signing key matches pin"
if [[ ! -d "$A/manifest/.git" ]] || ! git -C "$A/manifest" rev-parse -q --verify "refs/tags/$GRAPHENEOS_RELEASE" >/dev/null; then
  rm -rf "$A/manifest"; git init -q "$A/manifest"
  git -C "$A/manifest" fetch -q --depth 1 https://github.com/GrapheneOS/platform_manifest.git "refs/tags/${GRAPHENEOS_RELEASE}:refs/tags/${GRAPHENEOS_RELEASE}"
  git -C "$A/manifest" checkout -q "$GRAPHENEOS_RELEASE"
fi
git -C "$A/manifest" -c gpg.ssh.allowedSignersFile="$A/grapheneos_allowed_signers" verify-tag "$GRAPHENEOS_RELEASE" >/dev/null 2>&1 \
  || die "manifest tag signature invalid"
n=$(grep -c '<project ' "$A/manifest/default.xml"); pinned=$(grep '<project ' "$A/manifest/default.xml" | grep -cE 'revision="[0-9a-f]{40}"')
[[ "$n" == "$pinned" ]] || die "not every project is pinned to a commit hash"
ok "tag signature valid; $n projects, all hash-pinned"

echo "[5/5] Tooling"
for t in qemu-system-aarch64 aarch64-elf-gcc dtc docker cargo gh gpg; do
  command -v "$t" >/dev/null || die "missing tool: $t"
done
[[ -x /opt/homebrew/opt/llvm/bin/clang && -x /opt/homebrew/opt/lld/bin/ld.lld ]] || die "missing Homebrew llvm and lld"
docker info >/dev/null 2>&1 || die "Docker is not running"
ok "all tools present"
echo "all dependencies fetched and verified"
