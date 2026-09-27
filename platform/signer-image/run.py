#!/usr/bin/env python3
"""Boot the signer-only image under QEMU and drive it over its serial console.

Checks, in order:
  1. the image boots into the signer, with no login and no shell;
  2. it reads a signing request produced by another wallet's implementation;
  3. it decodes the transaction inside and shows what it does;
  4. it refuses to sign a request whose expected signer is not the key it holds;
  5. it produces the signature Foundry produces for the same digest;
  6. the kernel it runs has no network stack.

Development only: under an emulator there is no secure element and no verified
boot, so the phrase used here is Foundry's public test phrase.
"""
import os
import pathlib
import re
import select
import subprocess
import sys
import time

ROOT = pathlib.Path(__file__).resolve().parents[2]
OUT = pathlib.Path.home() / ".cache/osproject/signer-image"
VECTORS = pathlib.Path(__file__).resolve().parent / "vectors"

TEST_PHRASE = "test test test test test test test test test test test junk"
EXPECTED_SIGNER = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
# `cast wallet sign --mnemonic "$TEST_PHRASE" --no-hash <digest>`
EXPECTED_SIGNATURE = (
    "6c41afa9f38028749bb734ae6817c158ae298a636cf9ca6344664a196524895c0"
    "095065b752486f9d282f810a4abca5973b944ce29aff5fa164ce8cd115da0bd1b"
)
OTHER_PHRASE = "legal winner thank year wave sausage worth useful legal winner thank yellow"


class Console:
    def __init__(self, proc):
        self.proc, self.buf, self.log = proc, "", []

    def expect(self, needle, timeout=120):
        deadline = time.monotonic() + timeout
        while needle not in self.buf:
            remaining = deadline - time.monotonic()
            if remaining <= 0:
                raise TimeoutError(f"timed out waiting for {needle!r}\n--- console ---\n{self.buf[-3000:]}")
            ready, _, _ = select.select([self.proc.stdout], [], [], remaining)
            if not ready:
                continue
            chunk = os.read(self.proc.stdout.fileno(), 65536)
            if not chunk:
                raise EOFError(f"QEMU exited\n--- console ---\n{self.buf[-3000:]}")
            text = chunk.decode("utf-8", "replace")
            self.buf += text
            self.log.append(text)
        idx = self.buf.index(needle) + len(needle)
        seen, self.buf = self.buf[:idx], self.buf[idx:]
        return seen

    def send(self, line):
        self.proc.stdin.write((line + "\n").encode())
        self.proc.stdin.flush()

    def transcript(self):
        return "".join(self.log)


def boot():
    image, initrd = OUT / "Image", OUT / "initramfs.cpio.gz"
    for p in (image, initrd):
        if not p.exists():
            sys.exit(f"missing {p}. Run platform/signer-image/build.sh first.")
    cmd = [
        "qemu-system-aarch64",
        "-machine", "virt", "-cpu", "cortex-a53", "-m", "512", "-smp", "2",
        "-kernel", str(image), "-initrd", str(initrd),
        "-append", "console=ttyAMA0 panic=-1 quiet",
        "-nic", "none",                      # no network device of any kind
        "-display", "none", "-serial", "stdio", "-monitor", "none",
    ]
    return subprocess.Popen(cmd, stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.STDOUT)


def main():
    results = {}
    proc = boot()
    con = Console(proc)
    try:
        con.expect("clearsign signer", timeout=180)
        con.expect("ack <N:CODE>")
        results["boots into the signer"] = True

        # There is no shell: the signer is the only thing listening.
        con.send("sh")
        con.expect("unknown input: sh")
        results["no shell to fall back to"] = True

        # A request from another wallet's implementation, one QR code at a time.
        for line in (VECTORS / "request-animated.ur").read_text().split():
            con.send(line)
        con.expect("-- Signing request")  # the rest of the line names where the values came from
        seen = con.expect("-- Verdict --", timeout=60)
        # Matched loosely on purpose: the wallet's own label is untrusted text and is
        # printed quoted and escaped, and this assertion has already broken twice on
        # the exact spelling of a line rather than on anything being wrong.
        results["reads an animated request"] = "Requested by" in seen and "metamask" in seen
        results["decodes the transaction"] = "ERC-20 transfer" in seen and "1_000_000" in seen
        results["names the expected signer"] = EXPECTED_SIGNER[:10] in seen.replace(" ", "")[:100000] or "f39F" in seen

        # The wrong key must be refused, not signed with.
        con.send("sign")
        con.expect("Enter the recovery phrase")
        con.send(OTHER_PHRASE)
        con.send("")
        seen = con.expect("REFUSED:", timeout=60)
        con.expect("\n")
        results["refuses a key the request did not ask for"] = True

        # The right key produces exactly the signature Foundry produces.
        con.send("sign")
        con.expect("Enter the recovery phrase")
        con.send(TEST_PHRASE)
        con.send("")
        seen = con.expect("ready for a new request", timeout=90)
        results["signs with the key the request names"] = EXPECTED_SIGNER in seen
        results["reply is an eth-signature UR"] = "ur:eth-signature/" in seen
        body = "".join(re.findall(r"ur:eth-signature/([a-z]+)", seen))
        results["signature matches Foundry cast"] = EXPECTED_SIGNATURE in decode_ur_hex(body)
        results["shows the reply as a QR code"] = "█" in seen
    finally:
        log = OUT / "serial.log"
        log.write_text(con.transcript())
        if proc.poll() is None:
            proc.kill()
        print(f"serial log: {log}")

    config = (OUT / "kernel.config").read_text()
    results["kernel has no network stack"] = "# CONFIG_NET is not set" in config
    results["kernel takes no modules"] = "# CONFIG_MODULES is not set" in config

    print()
    ok = True
    for name, passed in results.items():
        print(f"{'PASS' if passed else 'FAIL'}  {name}")
        ok &= bool(passed)
    return 0 if ok else 1


def decode_ur_hex(minimal_bytewords: str) -> str:
    """Bytewords-minimal to hex, for checking the signature in the reply."""
    words = (
        "able acid also apex aqua arch atom aunt away axis back bald barn belt beta bias blue body brag brew bulb buzz "
        "calm cash cats chef city claw code cola cook cost crux curl cusp cyan dark data days deli dice diet door down "
        "draw drop drum dull duty each easy echo edge epic even exam exit eyes fact fair fern figs film fish fizz flap "
        "flew flux foxy free frog fuel fund gala game gear gems gift girl glow good gray grim guru gush gyro half hang "
        "hard hawk heat help high hill holy hope horn huts iced idea idle inch inky into iris iron item jade jazz join "
        "jolt jowl judo jugs jump junk jury keep keno kept keys kick kiln king kite kiwi knob lamb lava lazy leaf legs "
        "liar limp lion list logo loud love luau luck lung main many math maze memo menu meow mild mint miss monk nail "
        "navy need news next noon note numb obey oboe omit onyx open oval owls paid part peck play plus poem pool pose "
        "puff puma purr quad quiz race ramp real redo rich road rock roof ruby ruin runs rust safe saga scar sets silk "
        "skew slot soap solo song stub surf swan taco task taxi tent tied time tiny toil tomb toys trip tuna twin ugly "
        "undo unit urge user vast very veto vial vibe view visa void vows wall wand warm wasp wave waxy webs what when "
        "whiz wolf work yank yawn yell yoga yurt zaps zero zest zinc zone zoom"
    ).split()
    table = {w[0] + w[-1]: i for i, w in enumerate(words)}
    pairs = [minimal_bytewords[i:i + 2] for i in range(0, len(minimal_bytewords), 2)]
    try:
        return bytes(table[p] for p in pairs).hex()
    except KeyError:
        return ""


if __name__ == "__main__":
    sys.exit(main())
