import tempfile
from pathlib import Path
import unittest

from check_glib_backport import patched_source, verify


class BackportTests(unittest.TestCase):
    def test_only_the_two_upstream_changes_are_applied(self):
        source = b"let p: *mut libc::c_char = std::ptr::null_mut();\n                &p,\n"
        self.assertEqual(patched_source(source),
                         b"let mut p: *mut libc::c_char = std::ptr::null_mut();\n                &mut p,\n")

    def test_missing_or_duplicate_patch_context_is_refused(self):
        for source in (b"", b"let p: *mut libc::c_char =" * 2):
            with self.assertRaises(ValueError):
                patched_source(source)

    def test_changed_archive_is_refused_before_parsing(self):
        with tempfile.TemporaryDirectory() as directory:
            archive = Path(directory) / "glib.crate"
            archive.write_bytes(b"not the upstream crate")
            with self.assertRaisesRegex(ValueError, "checksum"):
                verify(archive)


if __name__ == "__main__":
    unittest.main()
