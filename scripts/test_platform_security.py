from pathlib import Path
import unittest
import xml.etree.ElementTree as ET


ROOT = Path(__file__).resolve().parents[1]


class PlatformSecurityTests(unittest.TestCase):
    def test_only_trusted_display_maps_the_signer_system_uart(self):
        system = ET.parse(
            ROOT / "platform/sel4/signer-system/signer.system"
        ).getroot()
        owners = []
        for domain in system.findall("protection_domain"):
            for mapping in domain.findall("map"):
                if mapping.get("mr") == "serial":
                    owners.append(domain.get("name"))
        self.assertEqual(owners, ["display"])

        display_peers = []
        for channel in system.findall("channel"):
            ends = [(end.get("pd"), end.get("id")) for end in channel.findall("end")]
            if any(pd == "display" for pd, _ in ends):
                display_peers.extend(pd for pd, _ in ends if pd != "display")
        self.assertEqual(display_peers, ["signer"])

        wallet = (
            ROOT / "platform/sel4/signer-system/wallet_ui.c"
        ).read_text(encoding="utf-8")
        self.assertNotIn("microkit_dbg_puts", wallet)
        self.assertNotIn("CH_DISPLAY", wallet)
        build = (
            ROOT / "platform/sel4/signer-system/build.sh"
        ).read_text(encoding="utf-8")
        self.assertIn('CONFIG="${CONFIG:-release}"', build)

    def test_linux_security_oracle_uses_trusted_producers(self):
        signer = (
            ROOT / "platform/sel4/linux-signer/signer.c"
        ).read_text(encoding="utf-8")
        vmm = (
            ROOT / "platform/sel4/linux-signer/vmm.c"
        ).read_text(encoding="utf-8")
        harness = (
            ROOT / "platform/sel4/linux-signer/run.py"
        ).read_text(encoding="utf-8")
        self.assertIn("SIGNER|REVIEW|", signer)
        self.assertIn("VMM|blocked guest write to signer review region", vmm)
        self.assertIn("#if !defined(BOARD_qemu_virt_aarch64)", vmm)
        self.assertIn(
            '#error "ClearSign Linux signer supports only BOARD_qemu_virt_aarch64"',
            vmm,
        )
        self.assertNotIn("microkit_vcpu_x86_enable_ioport", vmm)
        self.assertNotIn("COM1_IO_PORT", vmm)
        self.assertIn("EXPECTED_HASH in trusted_review", harness)
        self.assertNotIn("EXPECTED_HASH in text", harness)
        self.assertIn(
            '"VMM|blocked guest write to signer review region" in text',
            harness,
        )

    def test_linux_build_requires_hardened_allocator_outputs(self):
        build = (
            ROOT / "platform/sel4/linux-signer/build.sh"
        ).read_text(encoding="utf-8")
        self.assertIn(
            "for artifact in libhardened_malloc.so write_after_free write_after_free_static",
            build,
        )
        self.assertNotIn('if [[ -f "$GOS/libhardened_malloc.so" ]]', build)


if __name__ == "__main__":
    unittest.main()
