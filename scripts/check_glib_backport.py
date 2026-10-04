"""Check the desktop backport against the authenticated published crate."""

import hashlib
import io
from pathlib import Path, PurePosixPath
import sys
import tarfile

CHECKSUM = "233daaf6e83ae6a12a52055f568f9d7cf4671dabb78ff9560ab6da230ce00ee5"
ROOT = Path(__file__).resolve().parent.parent / "desktop/vendor/glib-0.18.5"


def patched_source(data):
    for before, after in (
        (b"let p: *mut libc::c_char =", b"let mut p: *mut libc::c_char ="),
        (b"                &p,", b"                &mut p,"),
    ):
        if data.count(before) != 1:
            raise ValueError("upstream patch context is not unique")
        data = data.replace(before, after)
    return data


def verify(archive, root=ROOT):
    data = archive.read_bytes()
    if hashlib.sha256(data).hexdigest() != CHECKSUM:
        raise ValueError("original glib crate checksum mismatch")
    expected = set()
    with tarfile.open(fileobj=io.BytesIO(data), mode="r:gz") as source:
        for member in source.getmembers():
            if not member.isfile():
                raise ValueError("unexpected archive entry")
            parts = PurePosixPath(member.name).parts
            if parts[0] != "glib-0.18.5" or ".." in parts:
                raise ValueError("unexpected archive path")
            relative = Path(*parts[1:])
            expected.add(relative)
            content = source.extractfile(member).read()
            if relative.as_posix() == "src/variant_iter.rs":
                content = patched_source(content)
            local = root / relative
            if local.is_symlink() or local.read_bytes() != content:
                raise ValueError(f"unexpected vendor change: {relative}")
    actual = {p.relative_to(root) for p in root.rglob("*")
              if p.is_file() and "target" not in p.relative_to(root).parts}
    if actual != expected | {Path("Cargo.lock")}:
        raise ValueError("unexpected files in vendored crate")
    print("glib source matches the published crate plus the upstream two-line fix")


if __name__ == "__main__":
    verify(Path(sys.argv[1]))
