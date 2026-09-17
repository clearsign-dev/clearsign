# GrapheneOS build host

These scripts build the Android layer of the platform from GrapheneOS source. They **cannot run on the project's Apple Silicon Mac**: GrapheneOS supports only x86_64 Linux build hosts, and needs at least 32 GiB of memory. Use a dedicated x86_64 Linux machine or a rented cloud instance.

| Step | Script | Notes |
|---|---|---|
| 0 | `00-check-host.sh [dir]` | Refuses to continue on an unsuitable machine |
| 1 | `01-provision-debian-bookworm.sh` | Run as root on Debian 12 |
| 2 | `02-sync-verified.sh [dir]` | Pins GrapheneOS's signing-key fingerprint, verifies the release tag, then syncs about 136 GiB |
| 3 | `03-build-emulator.sh <dir>` | Builds the recommended development target |

## What was verified on 16 Sep 2026 from the Mac

- Release `2026091000` manifest tag: valid ED25519 signature from `contact@grapheneos.org`, key fingerprint `SHA256:AhgHif0mei+9aNyKLfMZBh2yptHdw/aN7Tlh/j2eFwM`.
- 1,057 projects, every one pinned to a commit hash, based on `android-17.0.0_r1`.
- `hardened_malloc` at the manifest-pinned commit `5433e97f…` built and passed its full upstream test suite, and runs as the system allocator inside the seL4 Linux compartment. See `../grapheneos/`.

## Where Android fits

On a phone, GrapheneOS runs directly on supported Pixel hardware with its own verified boot. In this platform's architecture it is the application-compatibility compartment. Running a full Android system inside a seL4 compartment is a later milestone that needs a real ARM64 device or an x86_64 host with hardware virtualization; it is not demonstrated yet.
