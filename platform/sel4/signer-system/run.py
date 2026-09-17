#!/usr/bin/env python3
"""Boot the signer system on seL4 under QEMU and capture the serial console.

Stops when the wallet UI's tampering attempt is resolved (seL4 fault report, or the
UI claiming the write succeeded), or at the timeout. Exit code 0 only if the signer
produced a CRITICAL verdict AND the tampering attempt faulted."""
import os, pathlib, select, subprocess, sys, time

HERE = pathlib.Path(__file__).resolve().parent
IMG = HERE / "build/loader.img"
LOG = HERE / "build/serial.log"
cmd = ["qemu-system-aarch64", "-machine", "virt,virtualization=on", "-cpu", "cortex-a53",
       "-m", "size=2G", "-nographic", "-serial", "mon:stdio",
       "-device", f"loader,file={IMG},addr=0x70000000,cpu-num=0"]
p = subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
buf = b""
end = time.monotonic() + float(sys.argv[1] if len(sys.argv) > 1 else 120)
done_markers = (b"ISOLATION FAILURE", b"fault", b"FAULT")
while time.monotonic() < end:
    r, _, _ = select.select([p.stdout], [], [], 1)
    if r:
        c = os.read(p.stdout.fileno(), 65536)
        if not c:
            break
        buf += c
        tail = buf.split(b"attempting to overwrite", 1)
        if len(tail) == 2 and any(m in tail[1] for m in done_markers):
            t = time.monotonic() + 2
            while time.monotonic() < t:
                r, _, _ = select.select([p.stdout], [], [], 0.2)
                if r:
                    buf += os.read(p.stdout.fileno(), 65536)
            break
p.kill()
LOG.write_bytes(buf)
text = buf.decode(errors="replace")
critical = "WALLET_UI|signer verdict received: CRITICAL" in text
isolated = "ISOLATION FAILURE" not in text and "attempting to overwrite" in text and ("fault" in text.lower())
print(f"serial log: {LOG}")
print(f"signer verdict CRITICAL: {critical}")
print(f"tampering blocked by seL4: {isolated}")
sys.exit(0 if (critical and isolated) else 1)
