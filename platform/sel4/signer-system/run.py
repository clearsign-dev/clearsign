#!/usr/bin/env python3
"""Boot the signer system on seL4 under QEMU and capture the serial console.

Stops after the trusted display reports the wallet's tampering attempt. Exit code
0 only if the signer-produced review is CRITICAL and the wallet never reaches the
post-write failure notification."""
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
done_markers = (
    b"DISPLAY|trusted signer started the forbidden-write probe",
    b"DISPLAY|ISOLATION FAILURE",
)
while time.monotonic() < end:
    r, _, _ = select.select([p.stdout], [], [], 1)
    if r:
        c = os.read(p.stdout.fileno(), 65536)
        if not c:
            break
        buf += c
        if any(marker in buf for marker in done_markers):
            t = time.monotonic() + 5
            while time.monotonic() < t:
                r, _, _ = select.select([p.stdout], [], [], 0.2)
                if r:
                    buf += os.read(p.stdout.fileno(), 65536)
            break
p.kill()
LOG.write_bytes(buf)
text = buf.decode(errors="replace")
critical = "DISPLAY|highest severity: CRITICAL" in text
tamper_attempted = "DISPLAY|trusted signer started the forbidden-write probe" in text
isolated = tamper_attempted and "DISPLAY|ISOLATION FAILURE" not in text
print(f"serial log: {LOG}")
print(f"trusted display reported CRITICAL: {critical}")
print(f"tampering blocked by seL4: {isolated}")
sys.exit(0 if (critical and isolated) else 1)
