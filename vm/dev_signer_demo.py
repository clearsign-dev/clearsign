#!/usr/bin/env python3
"""
Boot the verified Alpine ARM64 image under QEMU with NO network device, hand it
the statically linked clearsign binary on a read-only virtual FAT drive, run the
decoder against the Bybit-pattern Safe transaction, and power off.

DEVELOPMENT ONLY. A signer inside a VM on your everyday computer protects
nothing: the host controls the VM's screen, keyboard and memory. This proves
the toolchain and the no-network boot path, nothing more.

Usage: python3 vm/dev_signer_demo.py [--timeout SECONDS]
"""
import argparse
import os
import pathlib
import select
import shutil
import subprocess
import sys
import tempfile
import time

ROOT = pathlib.Path(__file__).resolve().parent.parent
PINS = dict(l.split("=", 1) for l in (ROOT / "config/pins.env").read_text().splitlines() if "=" in l and not l.lstrip().startswith("#"))
ISO = ROOT / f"vm/images/alpine-virt-{PINS['ALPINE_VERSION']}-aarch64.iso"
BINARY = ROOT / "signing-core/target/aarch64-unknown-linux-musl/release/clearsign"
FIRMWARE = pathlib.Path("/opt/homebrew/share/qemu/edk2-aarch64-code.fd")

# Public Foundry/Anvil test phrase. Never use a real recovery phrase in a VM.
TEST_PHRASE = "test test test test test test test test test test test junk"

# Bybit-pattern Safe transaction (synthetic target), same as test vector B.
DEMO = (
    "/mnt/cs/clearsign safe-tx --chain-id 1 "
    "--safe 0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4 "
    "--to 0x00000000000000000000000000000000DeaDBeef --nonce 71 --operation 1 --safe-tx-gas 45746 "
    "--data 0xa9059cbb000000000000000000000000000000000000000000000000000000000000dead"
    "0000000000000000000000000000000000000000000000000000000000000000"
)


class Console:
    def __init__(self, proc, log):
        self.proc, self.log, self.buf = proc, log, ""

    def expect(self, needle, deadline):
        while needle not in self.buf:
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise TimeoutError(f"timed out waiting for {needle!r}")
            ready, _, _ = select.select([self.proc.stdout], [], [], remaining)
            if not ready:
                continue
            chunk = os.read(self.proc.stdout.fileno(), 4096)
            if not chunk:
                raise EOFError("QEMU exited")
            text = chunk.decode("utf-8", "replace")
            self.buf += text
            self.log.write(text)
            self.log.flush()
        idx = self.buf.index(needle) + len(needle)
        consumed, self.buf = self.buf[:idx], self.buf[idx:]
        return consumed

    def send(self, line):
        self.proc.stdin.write((line + "\n").encode())
        self.proc.stdin.flush()


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--timeout", type=int, default=180)
    args = ap.parse_args()

    for p in (ISO, BINARY, FIRMWARE):
        if not p.exists():
            sys.exit(f"missing {p}. Run vm/fetch-alpine.sh and the musl build first.")

    share = pathlib.Path(tempfile.mkdtemp(prefix="clearsign-share-"))
    shutil.copy2(BINARY, share / "clearsign")
    run_dir = ROOT / "vm/run"
    run_dir.mkdir(exist_ok=True)
    log_path = run_dir / "serial.log"

    cmd = [
        "qemu-system-aarch64",
        "-machine", "virt", "-accel", "hvf", "-cpu", "host",
        "-m", "1024", "-smp", "2",
        "-bios", str(FIRMWARE),
        "-drive", f"if=virtio,format=raw,readonly=on,file={ISO}",
        "-drive", f"if=virtio,format=raw,readonly=on,file=fat:ro:{share}",
        "-nic", "none",                       # no network device at all
        "-display", "none", "-serial", "stdio", "-monitor", "none",
    ]
    print("QEMU:", " ".join(cmd))
    deadline = time.monotonic() + args.timeout
    with open(log_path, "w") as log:
        proc = subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
        con = Console(proc, log)
        try:
            con.expect("login:", deadline)
            con.send("root")
            con.expect("# ", deadline)
            con.send("echo NET: $(ls /sys/class/net | tr '\\n' ' ')")
            con.expect("# ", deadline)
            con.send("mkdir -p /mnt/cs && mount -t vfat -o ro /dev/vdb1 /mnt/cs 2>/dev/null || mount -t vfat -o ro /dev/vdb /mnt/cs; echo MOUNT_RC=$?")
            con.expect("MOUNT_RC=", deadline)
            con.expect("# ", deadline)
            con.send("uname -m; " + DEMO + "; echo CLEARSIGN_EXIT=$?")
            con.expect("CLEARSIGN_EXIT=", deadline)
            con.expect("# ", deadline)
            # Signing flow with the public Foundry/Anvil test phrase (never a real one).
            con.send("export CLEARSIGN_DEV_ONLY=I_UNDERSTAND_THIS_COMPUTER_IS_NOT_A_SIGNING_DEVICE")
            con.expect("# ", deadline)
            sign = DEMO.replace("clearsign safe-tx", "clearsign sign-safe-tx")
            con.send("echo '" + TEST_PHRASE + "' | " + sign + " >/dev/null; echo REFUSED_EXIT=$?")
            con.expect("REFUSED_EXIT=", deadline)
            con.expect("# ", deadline)
            con.send("echo '" + TEST_PHRASE + "' | " + sign + " --ack SAFE_DELEGATECALL 2>/dev/null | tail -7; echo SIGN_EXIT=$?")
            con.expect("SIGN_EXIT=", deadline)
            con.expect("# ", deadline)
            con.send("poweroff")
            try:
                proc.wait(timeout=max(5, deadline - time.monotonic()))
            except subprocess.TimeoutExpired:
                proc.kill()
        finally:
            if proc.poll() is None:
                proc.kill()
            shutil.rmtree(share, ignore_errors=True)
    print(f"serial log: {log_path}")


if __name__ == "__main__":
    main()
