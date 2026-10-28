# ClearSign for iOS and Android

This is the phone and tablet client for the same Rust transaction reviewer used
by the ClearSign desktop app, command-line tool and browser build. The interface
can lay out a result differently; it cannot decode or assign severity itself.

The current mobile input is Safe transaction JSON. The app holds no keys, signs
nothing and does not send the transaction to a ClearSign service.

## Development build

Expo Go cannot load the custom Rust module. Build a native development client:

```sh
npm ci

# macOS with full Xcode
./scripts/build-rust.sh ios
npx expo run:ios

# Android SDK/NDK with cargo-ndk installed
./scripts/build-rust.sh android
npx expo run:android
```

Run `npm run typecheck` and `npx expo-doctor` before submitting a change. CI
builds the Rust libraries and both native applications from clean runners.

`npm run audit` fails if the dependency advisory set changes. The current Expo
SDK build CLI brings a high-severity resource-exhaustion advisory through Zod
and a low-severity prototype-poisoning advisory through `deepmerge`; neither has
a compatible upstream release yet. They are build-tool dependencies, not code
loaded by the transaction reviewer, and are named rather than suppressed so an
Expo update can remove the exception. Expo 58.0.20 also moves `node-forge` to
1.4.2, removing the earlier signature-verification advisory.

## Native boundary

`clearsign-native` exposes a caller-owned C buffer API. iOS links its generated
XCFramework directly. Android loads the same crate as a shared library through
a small JNI adapter. Both return Review API v1 JSON from `clearsign-api`.
The native boundary publishes and enforces the same one-megabyte record limit as
the Rust parser, and each structured finding carries its canonical acknowledgement
number.

Generated `ios/`, `android/`, XCFramework and `.so` files are ignored. They are
build products, not source or release artifacts.
