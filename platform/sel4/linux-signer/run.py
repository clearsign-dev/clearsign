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
    # Prebuilt guest: password-less login prompt. Source-built guest: console shell directly.
    assert pump_any((b"login:", b"compartment# "), 600), "no login prompt or console shell"
    if buf.rfind(b"login:") > buf.rfind(b"compartment# "):
        send("root"); pump(b"# ", 60)
    send("grep -c libhardened_malloc /proc/self/maps; echo HM_MAPS_DONE")
    pump(b"HM_MAPS_DONE", 60); pump(b"# ", 30)
    send("write_after_free; echo WAF_EXIT=$?")
    pump(b"WAF_EXIT=", 120); pump(b"# ", 30)
    send("write_after_free_static; echo WAFS_EXIT=$?")
    pump(b"WAFS_EXIT=", 120); pump(b"# ", 30)
    send("signer-request; echo REQUEST_EXIT=$?")
    pump(b"REQUEST_EXIT=", 180); pump(b"# ", 30)
    send("signer-request plan; echo PLAN_EXIT=$?")
    pump(b"PLAN_EXIT=", 180); pump(b"# ", 30)
    send("signer-request tamper; echo TAMPER_EXIT=$?")
    # A blocked write stops the guest vCPU; wait for the VMM's report instead of a prompt.
    pump(b"TAMPER_EXIT=", 40)
    end = time.monotonic() + 5
    while time.monotonic() < end:
        r, _, _ = select.select([p.stdout], [], [], 0.5)
        if r:
            buf += os.read(p.stdout.fileno(), 65536)
finally:
    p.kill()
    LOG.write_bytes(buf)

text = buf.decode(errors="replace").replace("\r", "")
review_ok = "GUEST|verdict: CRITICAL" in text and "REQUEST_EXIT=3" in text and EXPECTED_HASH in text
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
hm_loaded = re.search(r"grep -c libhardened_malloc /proc/self/maps; echo HM_MAPS_DONE\n([1-9]\d*)\n", text) is not None
hm_detects = "fatal allocator error: detected write after free" in text and "WAF_EXIT=134" in text
glibc_misses = "WAFS_EXIT=0" in text
print(f"GrapheneOS hardened_malloc loaded system-wide: {hm_loaded}")
print(f"hardened_malloc stops write-after-free: {hm_detects}")
print(f"static glibc allocator misses the same bug: {glibc_misses}")
print(f"serial log: {LOG}")
print(f"review from signer inside Linux (CRITICAL, expected Safe hash): {review_ok}")
print(f"untrusted compartment proposed an assistant plan: {plan_reviewed}")
print(f"prompt-injected data exfiltration caught and refused: {plan_blocked}")
print(f"tampering attempted by Linux: {tamper_attempted}")
print(f"tampering blocked: {tamper_blocked}")
sys.exit(0 if review_ok and plan_blocked and tamper_blocked and hm_loaded and hm_detects else 1)
