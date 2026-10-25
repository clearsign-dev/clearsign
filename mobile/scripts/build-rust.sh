#!/usr/bin/env bash
set -euo pipefail

PLATFORM=${1:-}
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
MOBILE_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
REPO_DIR=$(CDPATH= cd -- "$MOBILE_DIR/.." && pwd)
MANIFEST="$REPO_DIR/signing-core/Cargo.toml"
MODULE_NATIVE="$MOBILE_DIR/modules/clearsign-core/native"

case "$PLATFORM" in
  ios)
    command -v xcodebuild >/dev/null || {
      echo "xcodebuild is required to package the iOS Rust library" >&2
      exit 1
    }
    rustup target add aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios
    for target in aarch64-apple-ios aarch64-apple-ios-sim x86_64-apple-ios; do
      cargo build --manifest-path "$MANIFEST" --locked --release \
        -p clearsign-native --target "$target"
    done

    rm -rf "$MODULE_NATIVE/ios"
    mkdir -p "$MODULE_NATIVE/ios"
    TMP=$(mktemp -d)
    trap 'rm -rf "$TMP"' EXIT
    lipo -create \
      "$REPO_DIR/signing-core/target/aarch64-apple-ios-sim/release/libclearsign_native.a" \
      "$REPO_DIR/signing-core/target/x86_64-apple-ios/release/libclearsign_native.a" \
      -output "$TMP/libclearsign_native-simulator.a"
    xcodebuild -create-xcframework \
      -library "$REPO_DIR/signing-core/target/aarch64-apple-ios/release/libclearsign_native.a" \
      -headers "$MODULE_NATIVE/include" \
      -library "$TMP/libclearsign_native-simulator.a" \
      -headers "$MODULE_NATIVE/include" \
      -output "$MODULE_NATIVE/ios/ClearSignNative.xcframework"
    ;;
  android)
    : "${ANDROID_NDK_HOME:?ANDROID_NDK_HOME must point to an Android NDK installation}"
    command -v cargo-ndk >/dev/null || {
      echo "cargo-ndk is required: cargo install cargo-ndk --locked" >&2
      exit 1
    }
    rm -rf "$MODULE_NATIVE/android"
    cargo ndk \
      --manifest-path "$MANIFEST" \
      -t arm64-v8a -t armeabi-v7a -t x86_64 \
      -o "$MODULE_NATIVE/android" \
      build --locked --release -p clearsign-native
    ;;
  *)
    echo "usage: $0 ios|android" >&2
    exit 2
    ;;
esac
