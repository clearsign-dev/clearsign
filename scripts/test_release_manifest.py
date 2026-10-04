import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from release_manifest import check_version, manifest, required_assets, verify


class ReleaseTests(unittest.TestCase):
    def make_candidate(self, root):
        names = required_assets("0.1.2")
        for name in names:
            (root / name).write_bytes(name.encode())
        manifest(root, "0.1.2", "a" * 40)
        return names

    def test_manifest_covers_installers_page_archives_and_metadata(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            names = self.make_candidate(root)
            entries = dict(line.split("  ", 1)[::-1] for line in (root / "SHA256SUMS").read_text().splitlines())
            self.assertEqual(set(entries), set(names) | {"BUILD-METADATA.json"})
            for name, digest in entries.items():
                self.assertEqual(digest, hashlib.sha256((root / name).read_bytes()).hexdigest())
            self.assertEqual(json.loads((root / "BUILD-METADATA.json").read_text())["commit"], "a" * 40)
            verify(root, "0.1.2", "a" * 40)
            (root / "ClearSign_0.1.2_x64_en-US.msi").unlink()
            with self.assertRaises(ValueError):
                manifest(root, "0.1.2", "a" * 40)

    def test_old_or_extra_installer_is_refused(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self.make_candidate(root)
            (root / "ClearSign_0.1.0_x64-setup.exe").write_bytes(b"old installer")
            with self.assertRaisesRegex(ValueError, "unexpected"):
                manifest(root, "0.1.2", "a" * 40)
            (root / "ClearSign_0.1.2_x64-setup.exe").unlink()
            with self.assertRaisesRegex(ValueError, "missing"):
                manifest(root, "0.1.2", "a" * 40)

    def test_verification_does_not_rewrite_corrupted_artifacts(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self.make_candidate(root)
            before = (root / "SHA256SUMS").read_bytes()
            (root / "clearsign.html").write_bytes(b"changed after validation")
            with self.assertRaisesRegex(ValueError, "checksum mismatch"):
                verify(root, "0.1.2", "a" * 40)
            self.assertEqual(before, (root / "SHA256SUMS").read_bytes())

    def test_wrong_source_commit_is_refused(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self.make_candidate(root)
            with self.assertRaisesRegex(ValueError, "metadata"):
                verify(root, "0.1.2", "b" * 40)

    def test_bad_checksum_entries_are_refused(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            self.make_candidate(root)
            path = root / "SHA256SUMS"
            lines = path.read_text().splitlines(keepends=True)
            for content in ["".join(lines[1:]), "".join(lines + lines[:1]),
                            "0" * 64 + "  ../outside\n", "not a checksum\n"]:
                with self.subTest(content=content[:80]):
                    path.write_text(content)
                    with self.assertRaises(ValueError):
                        verify(root, "0.1.2", "a" * 40)

    def test_missing_artifacts_fail_closed(self):
        with tempfile.TemporaryDirectory() as tmp:
            with self.assertRaises(ValueError):
                manifest(Path(tmp), "0.1.2", "a" * 40)

    def test_malformed_tag_is_refused(self):
        with self.assertRaises(ValueError):
            check_version(Path('.'), "vnot-a-version", "tag")

    def test_versions_and_changelog_must_agree(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "desktop/src-tauri").mkdir(parents=True)
            (root / "signing-core").mkdir()
            for name in ["desktop/package.json", "desktop/src-tauri/tauri.conf.json"]:
                (root / name).write_text('{"version":"0.1.2"}')
            lock = {"version": "0.1.2", "packages": {"": {"version": "0.1.2"}}}
            (root / "desktop/package-lock.json").write_text(json.dumps(lock))
            (root / "desktop/src-tauri/Cargo.toml").write_text('[package]\nversion = "0.1.2"\n')
            (root / "signing-core/Cargo.toml").write_text('[workspace.package]\nversion = "0.1.2"\n')
            (root / "CHANGELOG.md").write_text('## v0.1.2 - 2026-10-03\n')
            check_version(root, "v0.1.2", "tag")
            check_version(root, "feature-branch", "branch")
            lock["packages"][""]["version"] = "0.1.0"
            (root / "desktop/package-lock.json").write_text(json.dumps(lock))
            with self.assertRaisesRegex(ValueError, "root package"):
                check_version(root, "feature-branch", "branch")
            lock["packages"][""]["version"] = "0.1.2"
            (root / "desktop/package-lock.json").write_text(json.dumps(lock))
            with self.assertRaises(ValueError):
                check_version(root, "v0.1.3", "tag")
            (root / "CHANGELOG.md").write_text('## v0.1.2 - Unreleased\n')
            with self.assertRaises(ValueError):
                check_version(root, "v0.1.2", "tag")
            (root / "CHANGELOG.md").write_text('## v0.1.1 - 2026-09-29\n')
            with self.assertRaises(ValueError):
                check_version(root, "v0.1.2", "tag")


if __name__ == "__main__":
    unittest.main()
