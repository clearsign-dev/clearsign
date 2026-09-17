# Development VM

**Development only. Never use a VM build with real keys or real funds.** A signer inside a virtual machine on your everyday computer protects nothing: the host controls the VM's display, input and memory. See `docs/01-threat-model.md`, non-goals.

## What is here

| File | Does |
|---|---|
| `fetch-alpine.sh` | Downloads the Alpine Linux 3.24.1 ARM64 virt image and verifies it by SHA-256 **and** by GPG signature against a pinned Alpine release key fingerprint. Refuses to keep an unverified image. |
| `dev_signer_demo.py` | Boots that image under QEMU with hardware virtualization and **no network device**, passes in the static `clearsign` binary on a read-only drive, runs it against the Bybit-pattern Safe transaction, and powers off. Serial log goes to `run/serial.log`. |

## Run it

```sh
# once
brew install qemu
./vm/fetch-alpine.sh

# build the static ARM64 Linux binary
cd signing-core
CARGO_TARGET_AARCH64_UNKNOWN_LINUX_MUSL_LINKER=rust-lld \
  cargo build -p clearsign-cli --release --locked --target aarch64-unknown-linux-musl
cd ..

# boot, run, power off
python3 vm/dev_signer_demo.py
```

Verified on 16 September 2026: Apple Silicon, QEMU 11.1.1, Alpine 3.24.1 with kernel 6.18.35. The guest showed only the loopback interface, and the Safe transaction hash computed inside the VM matched the independent `cast` reference.

## Next for this vehicle
- Replace the general-purpose Alpine live image with a minimal image built for the signer only: no network drivers compiled in, no shell on the console, read-only root.
- Make the build of that image reproducible.
