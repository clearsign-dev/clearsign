from pathlib import Path
import unittest

from release_signing import require_macos_adhoc_signature


class ReleaseSigningTests(unittest.TestCase):
    def test_unsigned_platform_checks_run_on_branch_candidates(self):
        workflow = (
            Path(__file__).resolve().parents[1] / ".github/workflows/release.yml"
        ).read_text(encoding="utf-8")
        self.assertIn("if: runner.os == 'macOS'", workflow)
        self.assertIn("if: runner.os == 'Windows'", workflow)
        self.assertIn("check-macos-adhoc", workflow)
        self.assertIn("signed releases are disabled in this repository workflow", workflow)
        self.assertNotIn("secrets.APPLE_", workflow)
        self.assertNotIn("secrets.WINDOWS_", workflow)

    def test_publication_token_is_confined_to_the_final_upload_step(self):
        candidate_workflow = (
            Path(__file__).resolve().parents[1] / ".github/workflows/release.yml"
        ).read_text(encoding="utf-8")
        publisher = (
            Path(__file__).resolve().parents[1]
            / ".github/workflows/publish-release.yml"
        ).read_text(encoding="utf-8")
        self.assertNotIn("GH_TOKEN:", candidate_workflow)
        self.assertEqual(publisher.count("GH_TOKEN:"), 1)
        self.assertNotIn("actions/checkout", publisher)
        self.assertNotIn("scripts/", publisher)
        self.assertIn("workflow_run:", publisher)
        self.assertIn("github.event.workflow_run.event == 'push'", publisher)
        self.assertIn('git/ref/tags/$tag"', publisher)
        self.assertIn('git/tags/$object_sha"', publisher)
        self.assertIn('[ "$object_type" = commit ]', publisher)
        self.assertIn('[ "$object_sha" = "$commit" ]', publisher)
        self.assertNotIn('repos/${GITHUB_REPOSITORY}/commits/$tag', publisher)
        self.assertIn('[ "$tag" = "$EXPECTED_TAG" ]', publisher)
        self.assertIn("/usr/bin/diff -u /tmp/expected-files /tmp/actual-files", publisher)
        self.assertIn('"${assets[@]}"', publisher)

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

if __name__ == "__main__":
    unittest.main()
