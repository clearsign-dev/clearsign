#!/usr/bin/env bash
# The canonical build, run inside the pinned image. Everything the environment
# can vary without changing the result is varied by the caller.
#
# Note on "offline": the cargo build below is offline, from a registry cache
# copied in by the caller. This bootstrap is not — apt and rustup both reach the
# network, and neither is pinned by digest. So the canonical environment is
# named and reproducible in practice, not sealed. Pinning the bootstrap is the
# obvious next step and is not done yet.
set -euo pipefail
export DEBIAN_FRONTEND=noninteractive
apt-get -o Acquire::Retries=5 update -qq >/dev/null 2>&1
apt-get -o Acquire::Retries=5 install -y -qq --no-install-recommends curl ca-certificates gcc libc6-dev locales >/dev/null 2>&1
TOOLCHAIN="$1"; TARGET="$2"; shift 2

export TZ="${BUILD_TZ:-UTC}" LANG="${BUILD_LOCALE:-C.UTF-8}" LC_ALL="${BUILD_LOCALE:-C.UTF-8}"
umask "${BUILD_UMASK:-022}"
export RUSTUP_HOME=/opt/rustup CARGO_HOME=/opt/cargo
RUSTUP_INSTALLER="$(mktemp)"
trap 'rm -f "$RUSTUP_INSTALLER"' EXIT
curl -sSfL --retry 5 --retry-all-errors --retry-delay 2 \
  --connect-timeout 20 --max-time 300 https://sh.rustup.rs -o "$RUSTUP_INSTALLER"
sh "$RUSTUP_INSTALLER" -y --no-modify-path --profile minimal \
  --default-toolchain "$TOOLCHAIN" --target "$TARGET" >/dev/null
export PATH="/opt/cargo/bin:$PATH"

BUILD_PATH="${BUILD_PATH:-/build-one}"
mkdir -p "$(dirname "$BUILD_PATH")"
cp -a /src "$BUILD_PATH"
if [[ -d /cargo-registry ]]; then
  REGISTRY_SNAPSHOT=/cargo-registry
elif [[ -d "$BUILD_PATH/cargo-cache/registry" ]]; then
  # The signer-image builder packages its snapshot with the source tree rather
  # than mounting it separately. Supporting both callers keeps this helper
  # reusable without allowing Cargo to reach the network.
  REGISTRY_SNAPSHOT="$BUILD_PATH/cargo-cache/registry"
else
  echo "no frozen Cargo registry snapshot was supplied" >&2
  exit 1
fi
rm -rf /opt/cargo/registry && cp -a "$REGISTRY_SNAPSHOT" /opt/cargo/registry
cd "$BUILD_PATH"
export CARGO_ENCODED_RUSTFLAGS=$'--remap-path-prefix='"$BUILD_PATH"$'=/build\x1f--remap-path-prefix=/opt/cargo=/cargo\x1f-C\x1fstrip=symbols'
export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=rust-lld
export SOURCE_DATE_EPOCH=0
for p in "$@"; do cargo build -q -p "$p" --release --locked --offline --target "$TARGET"; done
# Report only the binaries whose packages were asked for.
for p in "$@"; do
  case "$p" in
    clearsign-cli) b=clearsign ;;
    *)             b="$p" ;;
  esac
  cp "target/$TARGET/release/$b" "/artifacts/${LABEL:-x}-$b" 2>/dev/null || true
  sha256sum "target/$TARGET/release/$b" | awk -v b="$b" '{print $1"  "b}'
done
