import json
from pathlib import Path
import tempfile
import unittest

from release_signing import (
    DEFAULT_WINDOWS_TIMESTAMP_URL,
    require_macos_adhoc_signature,
    require_credentials,
    windows_config,
    write_windows_config,
)


class ReleaseSigningTests(unittest.TestCase):
    def test_unsigned_platform_checks_run_on_branch_candidates(self):
        workflow = (
            Path(__file__).resolve().parents[1] / ".github/workflows/release.yml"
        ).read_text(encoding="utf-8")
        self.assertIn(
            "runner.os == 'macOS' && needs.validate.outputs.release_mode != 'signed'",
            workflow,
        )
        self.assertIn(
            "runner.os == 'Windows' && needs.validate.outputs.release_mode != 'signed'",
            workflow,
        )
        self.assertIn("check-macos-adhoc", workflow)

    def test_macos_evaluation_accepts_only_identity_free_adhoc_signatures(self):
        require_macos_adhoc_signature(
            "flags=0x20002(adhoc,linker-signed)\n"
            "Signature=adhoc\n"
            "TeamIdentifier=not set\n"
        )

        signed = (
            "Signature size=9051\n"
            "Authority=Developer ID Application: Example Corp (ABCDE12345)\n"
            "TeamIdentifier=ABCDE12345\n"
        )
        with self.assertRaisesRegex(ValueError, "not ad-hoc"):
            require_macos_adhoc_signature(signed)

        with self.assertRaisesRegex(ValueError, "TeamIdentifier"):
            require_macos_adhoc_signature(
                "Signature=adhoc\nTeamIdentifier=ABCDE12345\n"
            )

        with self.assertRaisesRegex(ValueError, "signing authority"):
            require_macos_adhoc_signature(
                "Signature=adhoc\nTeamIdentifier=not set\nAuthority=Unexpected\n"
            )

    def test_each_platform_requires_its_complete_secret_set(self):
        with self.assertRaisesRegex(ValueError, "APPLE_API_KEY_P8_BASE64"):
            require_credentials("macos", {
                "APPLE_CERTIFICATE": "certificate",
                "APPLE_CERTIFICATE_PASSWORD": "password",
                "APPLE_API_ISSUER": "issuer",
                "APPLE_API_KEY": "key",
                "APPLE_TEAM_ID": "team",
            })
        with self.assertRaisesRegex(ValueError, "WINDOWS_CERTIFICATE_PASSWORD"):
            require_credentials("windows", {"WINDOWS_CERTIFICATE": "certificate"})

    def test_complete_credentials_are_accepted_without_returning_them(self):
        credentials = {
            "WINDOWS_CERTIFICATE": "private material",
            "WINDOWS_CERTIFICATE_PASSWORD": "secret",
            "WINDOWS_CERTIFICATE_THUMBPRINT": "a" * 40,
        }
        self.assertIsNone(require_credentials("windows", credentials))

    def test_windows_config_is_narrow_and_normalizes_thumbprint(self):
        config = windows_config("aa bb cc dd ee ff 00 11 22 33 44 55 66 77 88 99 aa bb cc dd")
        self.assertEqual(config, {
            "bundle": {
                "windows": {
                    "certificateThumbprint": "AABBCCDDEEFF00112233445566778899AABBCCDD",
                    "digestAlgorithm": "sha256",
                    "timestampUrl": DEFAULT_WINDOWS_TIMESTAMP_URL,
                }
            }
        })

    def test_bad_thumbprints_and_timestamp_urls_are_refused(self):
        for thumbprint in ["", "ab", "g" * 40, "a" * 41]:
            with self.subTest(thumbprint=thumbprint):
                with self.assertRaisesRegex(ValueError, "thumbprint"):
                    windows_config(thumbprint)
        for url in ["", "timestamp.example", "file:///tmp/server", "https://u:p@example.test"]:
            with self.subTest(url=url):
                with self.assertRaisesRegex(ValueError, "timestamp URL"):
                    windows_config("a" * 40, url)

    def test_generated_config_is_valid_json(self):
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / "tauri-signing.json"
            write_windows_config(path, "a" * 40, "https://timestamp.example.test")
            self.assertEqual(
                json.loads(path.read_text())["bundle"]["windows"]["digestAlgorithm"],
                "sha256",
            )


if __name__ == "__main__":
    unittest.main()
