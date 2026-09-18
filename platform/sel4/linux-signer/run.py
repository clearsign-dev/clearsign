#!/usr/bin/env python3
"""Boot seL4 + Linux + signer in QEMU. From inside Linux: request a review, then try
to tamper with it. Exit 0 only if the review is CRITICAL with the expected Safe hash
AND the tampering write never succeeds."""
import os, pathlib, select, subprocess, sys, time

BUILD = pathlib.Path.home() / ".cache/osproject/sel4/build-linux-signer"
LOG = pathlib.Path(__file__).resolve().parent / "serial.log"
EXPECTED_HASH = "0xa62b640da6d5c542052b0aa17fd0a8a549d0a02b48e2e5a0ec06aba49b47df2d"
cmd = ["qemu-system-aarch64", "-machine", "virt,virtualization=on", "-cpu", "cortex-a53", "-m", "size=2G",
       "-nographic", "-serial", "mon:stdio", "-device", f"loader,file={BUILD}/loader.img,addr=0x70000000,cpu-num=0"]
p = subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)
buf = b""

def pump(needle, timeout):
    global buf
    end = time.monotonic() + timeout
    while needle not in buf:
        if time.monotonic() > end or b"Kernel panic" in buf:
            return False
        r, _, _ = select.select([p.stdout], [], [], 1)
        if r:
            c = os.read(p.stdout.fileno(), 65536)
            if not c:
                return False
            buf += c
    return True

def pump_any(needles, timeout):
    global buf
    end = time.monotonic() + timeout
    while not any(n in buf for n in needles):
        if time.monotonic() > end or b"Kernel panic" in buf:
            return False
        r, _, _ = select.select([p.stdout], [], [], 1)
        if r:
            c = os.read(p.stdout.fileno(), 65536)
            if not c:
                return False
            buf += c
    return True

def send(line):
    p.stdin.write(line.encode() + b"\n"); p.stdin.flush()

try:
    # Nothing is sent to the guest: it has no input path. The compartment runs
    # its own script and this only reads what comes back.
    assert pump(b"DEMO|starting", 600), "the compartment never started its demo"
    assert pump(b"PLAN_EXIT=", 600), "the compartment never finished the plan step"
    # A blocked write stops the guest vCPU; wait for the VMM's report.
    pump(b"TAMPER_EXIT=", 120)
    end_at = time.monotonic() + 5
    while time.monotonic() < end_at:
        r, _, _ = select.select([p.stdout], [], [], 0.5)
        if r:
            buf += os.read(p.stdout.fileno(), 65536)
finally:
    p.kill()
    LOG.write_bytes(buf)

text = buf.decode(errors="replace").replace("\r", "")
# What the signer said, on lines only the signer can produce. The guest has no
# serial device: every line it prints is relayed by the VMM behind "GUEST|", so a
# line starting with "SIGNER|" cannot come from the untrusted side. Asserting on
# guest output — which this harness used to do — proved nothing at all.
signer_lines = [l for l in text.splitlines() if l.startswith("SIGNER|")]
guest_lines = [l for l in text.splitlines() if l.startswith("GUEST|")]
signer_reviewed = any("review written for the Linux compartment (severity code 4)" in l for l in signer_lines)
guest_cannot_forge = not any(l.startswith("SIGNER|") for l in text.splitlines() if "GUEST|" in l)
review_ok = signer_reviewed and "REQUEST_EXIT=3" in text and EXPECTED_HASH in text
# The prompt-injection case: the untrusted side proposed it, the compartment
# that decides caught it, and the finding names the step and the destination.
plan_reviewed = "submitting an assistant's action plan" in text
plan_blocked = (
    "SECRET_EGRESS" in text
    and "notes-backup.example" in text
    and "DO NOT APPROVE" in text
    and "PLAN_EXIT=3" in text
)
tamper_attempted = "Writing 'no risk' now" in text
tamper_blocked = tamper_attempted and "ISOLATION FAILURE" not in text and "TAMPER_EXIT=2" not in text
import re
hm_loaded = re.search(r"HM_MAPS=([1-9]\d*)", text) is not None
hm_detects = "fatal allocator error: detected write after free" in text and "WAF_EXIT=134" in text
glibc_misses = "WAFS_EXIT=0" in text
print(f"GrapheneOS hardened_malloc loaded system-wide: {hm_loaded}")
print(f"hardened_malloc stops write-after-free: {hm_detects}")
print(f"static glibc allocator misses the same bug: {glibc_misses}")
print(f"serial log: {LOG}")
print(f"signer reported CRITICAL on a line the guest cannot write: {signer_reviewed}")
print(f"no guest line can start with SIGNER|: {guest_cannot_forge}")
print(f"review from signer inside Linux (CRITICAL, expected Safe hash): {review_ok}")
print(f"untrusted compartment proposed an assistant plan: {plan_reviewed}")
print(f"prompt-injected data exfiltration caught and refused: {plan_blocked}")
print(f"tampering attempted by Linux: {tamper_attempted}")
print(f"tampering blocked: {tamper_blocked}")
sys.exit(0 if review_ok and guest_cannot_forge and plan_blocked and tamper_blocked and hm_loaded and hm_detects else 1)
