#!/usr/bin/env bash
# INV-9: anyone must be able to rebuild the signer and get the same bytes.
#
# Reproducibility is always relative to a stated build environment. Rust does
# not promise that a compiler hosted on macOS and the same compiler hosted on
# Linux emit identical machine code for the same target — and measurably they do
# not: building the same source both ways produces identical `.rodata`, `.data`
# and `.got`, and a different `.text`. So, like GrapheneOS, Tor Browser and
# Debian, this project names one canonical environment and reproduces inside it.
#
#   Canonical environment: DEBIAN_BUILD_IMAGE from config/pins.env, pinned by
#   digest, with the Rust toolchain from rust-toolchain.toml, building the
#   aarch64-unknown-linux-musl target, offline.
#
# The two builds below differ in everything that must not matter: build path,
# locale, timezone, umask, hostname and container instance. They must agree.
# The developer's own macOS build is also printed, for information only: it is
# not the artifact anyone is asked to verify.
set -euo pipefail
ROOT="$(cd "$(dirname "$0")/.." && pwd)"
REPO="$(cd "$ROOT/.." && pwd)"
source "$REPO/config/pins.env"
TARGET="${TARGET:-aarch64-unknown-linux-musl}"
CARGO_HOME_DIR="${CARGO_HOME:-$HOME/.cargo}"
TOOLCHAIN="$(sed -n 's/^channel = "\(.*\)"/\1/p' "$ROOT/rust-toolchain.toml")"
ARTIFACTS="${ARTIFACTS:-/tmp/repro-artifacts}"
PACKAGES=(clearsign-cli clearsign-device)
BINARIES=(clearsign clearsign-device)
mkdir -p "$ARTIFACTS"

prepare() { # source tree plus this machine's crate cache, so the build is offline
  local work="$1"
  (cd "$ROOT" && tar --exclude ./target --exclude ./fuzz --exclude ./vendor -cf - .) | (cd "$work" && tar -xf -)
  mkdir -p "$work/cargo-cache"
  cp -R "$CARGO_HOME_DIR/registry" "$work/cargo-cache/registry"
  cp "$ROOT/scripts/in-canonical-container.sh" "$work/in-container.sh"
  chmod +x "$work/in-container.sh"
}

canonical_build() { # $1 work dir, $2 label, $3 build path, $4 locale, $5 timezone, $6 umask
  local work="$1" label="$2"
  docker run --rm --platform linux/arm64 \
    --hostname "builder-$label" \
    -v "$work:/src:ro" -v "$ARTIFACTS:/artifacts" \
    -e BUILD_PATH="$3" -e BUILD_LOCALE="$4" -e BUILD_TZ="$5" -e BUILD_UMASK="$6" -e LABEL="$label" \
    "$DEBIAN_BUILD_IMAGE" /src/in-container.sh "$TOOLCHAIN" "$TARGET" "${PACKAGES[@]}" \
    | tail -n "${#BINARIES[@]}"
}

host_build() { # informational: the developer's own machine and toolchain
  local work="$1"
  (
    cd "$work"
    export CARGO_ENCODED_RUSTFLAGS=$'--remap-path-prefix='"$work"$'=/build\x1f--remap-path-prefix='"$CARGO_HOME_DIR"$'=/cargo\x1f-C\x1fstrip=symbols'
    export CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=rust-lld
    export SOURCE_DATE_EPOCH=0
    for p in "${PACKAGES[@]}"; do cargo build -q -p "$p" --release --locked --offline --target "$TARGET"; done
    for b in "${BINARIES[@]}"; do shasum -a 256 "target/$TARGET/release/$b" | awk -v b="$b" '{print $1"  "b}'; done
  )
}

WORK_A="$(mktemp -d)"; WORK_B="$(mktemp -d)"
trap 'rm -rf "$WORK_A" "$WORK_B"' EXIT
echo "canonical environment: $DEBIAN_BUILD_IMAGE"
echo "toolchain $TOOLCHAIN, target $TARGET"
echo "preparing two identical source trees"
prepare "$WORK_A"
prepare "$WORK_B"

echo
echo "--- canonical build 1: /build-one, C.UTF-8, UTC, umask 022"
A="$(canonical_build "$WORK_A" one /build-one C.UTF-8 UTC 022)"; echo "$A"
echo "--- canonical build 2: /a/much/longer/build/path/two, en_US.UTF-8, Pacific/Kiritimati, umask 077"
B="$(canonical_build "$WORK_B" two /a/much/longer/build/path/two en_US.UTF-8 Pacific/Kiritimati 077)"; echo "$B"

if [[ "${SKIP_HOST_BUILD:-0}" != "1" ]]; then
  echo
  echo "--- for information: this machine's own toolchain, outside the canonical environment"
  host_build "$WORK_A" || echo "(host build unavailable)"
fi

echo
if [[ "$A" == "$B" ]]; then
  {
    echo "# The signer binaries, as built in the canonical environment."
    echo "# Reproduce with signing-core/scripts/reproducible-cross-check.sh."
    echo "# Environment: $DEBIAN_BUILD_IMAGE, Rust $TOOLCHAIN, target $TARGET, offline."
    echo "# Anything built elsewhere, including on the maintainer's own machine, may"
    echo "# differ: rustc does not promise identical output across compiler hosts."
    echo "$A"
  } > "$ROOT/EXPECTED-HASHES.txt"
  echo "REPRODUCIBLE in the canonical environment; recorded in signing-core/EXPECTED-HASHES.txt"
else
  echo "NOT REPRODUCIBLE" >&2
  diff <(echo "$A") <(echo "$B") >&2 || true
  exit 1
fi
