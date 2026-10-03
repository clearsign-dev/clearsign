import hashlib
import json
from pathlib import Path
import tempfile
import unittest

from release_manifest import check_version, manifest


class ReleaseTests(unittest.TestCase):
    def test_manifest_covers_installers_page_archives_and_metadata(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            names = [f"clearsign-{target}.tar.gz" for target in (
                "aarch64-apple-darwin", "x86_64-apple-darwin",
                "x86_64-unknown-linux-musl", "aarch64-unknown-linux-musl",
            )] + ["clearsign.html", "BINARY-SHA256"] + [
                f"ClearSign.{ext}" for ext in ("dmg", "msi", "exe", "deb", "AppImage")
            ]
            for name in names:
                (root / name).write_bytes(name.encode())
            manifest(root, "0.1.2", "a" * 40)
            entries = dict(line.split("  ", 1)[::-1] for line in (root / "SHA256SUMS").read_text().splitlines())
            self.assertEqual(set(entries), set(names) | {"BUILD-METADATA.json"})
            for name, digest in entries.items():
                self.assertEqual(digest, hashlib.sha256((root / name).read_bytes()).hexdigest())
            self.assertEqual(json.loads((root / "BUILD-METADATA.json").read_text())["commit"], "a" * 40)
            (root / "ClearSign.msi").unlink()
            with self.assertRaises(ValueError):
                manifest(root, "0.1.2", "a" * 40)

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
            (root / "desktop/src-tauri/Cargo.toml").write_text('[package]\nversion = "0.1.2"\n')
            (root / "signing-core/Cargo.toml").write_text('[workspace.package]\nversion = "0.1.2"\n')
            (root / "CHANGELOG.md").write_text('## v0.1.2 - 2026-10-03\n')
            check_version(root, "v0.1.2", "tag")
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
