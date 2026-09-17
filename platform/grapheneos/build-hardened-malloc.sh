#!/usr/bin/env bash
# Build GrapheneOS hardened_malloc for the Linux compartment (aarch64 glibc), at the
# exact commit pinned by the signature-verified GrapheneOS release manifest, and run
# its upstream test suite. Requires Docker (Linux arm64).
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
source "$HERE/../../config/pins.env"
MANIFEST="$HERE/../android/manifest/default.xml"
COMMIT=$(grep 'name="hardened_malloc"' "$MANIFEST" | grep -oE 'revision="[0-9a-f]{40}"' | cut -d'"' -f2)
SRC="$HERE/vendor/hardened_malloc"
if [[ ! -d "$SRC/.git" ]] || [[ "$(git -C "$SRC" rev-parse HEAD)" != "$COMMIT" ]]; then
  rm -rf "$SRC"; git init -q "$SRC"
  git -C "$SRC" fetch -q --depth 1 https://github.com/GrapheneOS/hardened_malloc.git "$COMMIT"
  git -C "$SRC" checkout -q FETCH_HEAD
fi
[[ "$(git -C "$SRC" rev-parse HEAD)" == "$COMMIT" ]] || { echo "commit mismatch" >&2; exit 1; }
mkdir -p "$HERE/out"
docker run --rm --platform linux/arm64 -v "$HERE:/work" -w /work "$DEBIAN_BUILD_IMAGE" bash -ec '
  export DEBIAN_FRONTEND=noninteractive
  apt-get update -qq >/dev/null 2>&1
  apt-get install -y -qq --no-install-recommends build-essential clang python3 >/dev/null 2>&1
  HM=vendor/hardened_malloc
  # 1. Upstream configuration, unmodified: the full test suite must pass.
  make -s -C $HM clean >/dev/null 2>&1 || true
  make -s -C $HM -j8 VARIANT=default
  make -s -C $HM test
  echo "upstream default configuration: all tests passed"

  # 2. Deployment variant for the Linux compartment: CONFIG_CXX_ALLOCATOR=false, so the
  #    library does not depend on libstdc++, which the minimal guest does not ship
  #    (preloading a library with a missing dependency stops init: observed kernel panic).
  #    Only the three C++ delete tests may fail, because that code is compiled out.
  make -s -C $HM clean >/dev/null 2>&1 || true
  # CONFIG_NATIVE=false and baseline ARMv8.0: upstream defaults to -march=native, which on an
  # Apple Silicon build machine emits instructions a Cortex-A53 lacks (observed: SIGILL in init).
  CFLAGS="-march=armv8-a" make -s -C $HM -j8 VARIANT=default CONFIG_CXX_ALLOCATOR=false CONFIG_NATIVE=false
  set +e
  CFLAGS="-march=armv8-a" make -s -C $HM test CONFIG_CXX_ALLOCATOR=false CONFIG_NATIVE=false > /tmp/hm_nocxx_test.log 2>&1
  set -e
  grep -E "^(FAIL|ERROR):" /tmp/hm_nocxx_test.log | sed -E "s/^(FAIL|ERROR): ([a-z_]+).*/\2/" | sort > /tmp/failed.txt || true
  printf "%s\n" test_delete_type_size_mismatch test_invalid_aligned_sized_delete_large test_invalid_aligned_sized_delete_small | sort > /tmp/allowed.txt
  if ! diff -q /tmp/failed.txt /tmp/allowed.txt >/dev/null && [ -s /tmp/failed.txt ]; then
    if comm -23 /tmp/failed.txt /tmp/allowed.txt | grep -q .; then
      echo "unexpected hardened_malloc test failures:"; comm -23 /tmp/failed.txt /tmp/allowed.txt; exit 1
    fi
  fi
  echo "deployment variant: all C allocator tests passed (C++ delete tests excluded by configuration)"
  if readelf -d $HM/out/libhardened_malloc.so | grep NEEDED | grep -vE "libc.so.6|ld-linux-aarch64.so.1" | grep -q .; then
    echo "deployment variant has unexpected dependencies"; readelf -d $HM/out/libhardened_malloc.so | grep NEEDED; exit 1
  fi
  cp vendor/hardened_malloc/out/libhardened_malloc.so out/
  clang -O0 -march=armv8-a -o out/write_after_free demo/write_after_free.c
  clang -O0 -march=armv8-a -static -o out/write_after_free_static demo/write_after_free.c
'
echo "hardened_malloc $COMMIT"
shasum -a 256 "$HERE/out/"*
