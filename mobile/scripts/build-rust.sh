#!/usr/bin/env bash
set -euo pipefail

PLATFORM=${1:-}
SCRIPT_DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd)
MOBILE_DIR=$(CDPATH= cd -- "$SCRIPT_DIR/.." && pwd)
REPO_DIR=$(CDPATH= cd -- "$MOBILE_DIR/.." && pwd)
MANIFEST="$REPO_DIR/signing-core/Cargo.toml"
MODULE_NATIVE="$MOBILE_DIR/modules/clearsign-core/native"
MODULE_IOS_NATIVE="$MOBILE_DIR/modules/clearsign-core/ios/native"

make_ios_framework() {
  library=$1
  destination=$2
  mkdir -p "$destination/Headers" "$destination/Modules"
  cp "$library" "$destination/ClearSignNative"
  cp "$MODULE_NATIVE/include/clearsign_native.h" "$destination/Headers/"
  cp "$MODULE_NATIVE/include/framework.modulemap" "$destination/Modules/module.modulemap"
  cp "$MODULE_NATIVE/include/framework-info.plist" "$destination/Info.plist"
}

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

    rm -rf "$MODULE_IOS_NATIVE"
    mkdir -p "$MODULE_IOS_NATIVE"
    TMP=$(mktemp -d)
    trap 'rm -rf "$TMP"' EXIT
    lipo -create \
      "$REPO_DIR/signing-core/target/aarch64-apple-ios-sim/release/libclearsign_native.a" \
      "$REPO_DIR/signing-core/target/x86_64-apple-ios/release/libclearsign_native.a" \
      -output "$TMP/libclearsign_native-simulator.a"
    make_ios_framework \
      "$REPO_DIR/signing-core/target/aarch64-apple-ios/release/libclearsign_native.a" \
      "$TMP/device/ClearSignNative.framework"
    make_ios_framework \
      "$TMP/libclearsign_native-simulator.a" \
      "$TMP/simulator/ClearSignNative.framework"
    xcodebuild -create-xcframework \
      -framework "$TMP/device/ClearSignNative.framework" \
      -framework "$TMP/simulator/ClearSignNative.framework" \
      -output "$MODULE_IOS_NATIVE/ClearSignNative.xcframework"
    ;;
  android)
    : "${ANDROID_NDK_HOME:?ANDROID_NDK_HOME must point to an Android NDK installation}"
    command -v cargo-ndk >/dev/null || {
      echo "cargo-ndk is required: cargo install cargo-ndk --locked" >&2
      exit 1
    }
    rustup target add \
      aarch64-linux-android \
      armv7-linux-androideabi \
      x86_64-linux-android
    rm -rf "$MODULE_NATIVE/android"
    (
      cd "$REPO_DIR/signing-core"
      cargo ndk \
        -t arm64-v8a -t armeabi-v7a -t x86_64 \
        -o "$MODULE_NATIVE/android" \
        build --locked --release -p clearsign-native
    )
    ;;
  *)
    echo "usage: $0 ios|android" >&2
    exit 2
    ;;
esac
