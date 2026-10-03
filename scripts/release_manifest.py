"""Validate release versions and hash every downloadable artifact."""

import hashlib
import json
from pathlib import Path
import re
import sys
import tomllib


def check_version(root, tag, ref_type):
    if ref_type != "tag":
        return
    if not re.fullmatch(r"v\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?", tag):
        raise ValueError("release tags must use vMAJOR.MINOR.PATCH")
    expected = tag[1:]
    versions = {
        "desktop/package.json": json.loads((root / "desktop/package.json").read_text())["version"],
        "desktop/src-tauri/tauri.conf.json": json.loads((root / "desktop/src-tauri/tauri.conf.json").read_text())["version"],
        "desktop/src-tauri/Cargo.toml": tomllib.loads((root / "desktop/src-tauri/Cargo.toml").read_text())["package"]["version"],
        "signing-core/Cargo.toml": tomllib.loads((root / "signing-core/Cargo.toml").read_text())["workspace"]["package"]["version"],
    }
    for name, actual in versions.items():
        if actual != expected:
            raise ValueError(f"{name}: {actual} does not match {tag}")
    heading = next((line for line in (root / "CHANGELOG.md").read_text().splitlines()
                    if line.startswith(f"## {tag} ")), None)
    if heading is None:
        raise ValueError(f"CHANGELOG.md has no section for {tag}")
    if "unreleased" in heading.lower():
        raise ValueError(f"CHANGELOG.md still marks {tag} as unreleased")


def manifest(directory, version, commit):
    required = [
        "clearsign-aarch64-apple-darwin.tar.gz",
        "clearsign-x86_64-apple-darwin.tar.gz",
        "clearsign-x86_64-unknown-linux-musl.tar.gz",
        "clearsign-aarch64-unknown-linux-musl.tar.gz",
        "clearsign.html", "BINARY-SHA256",
    ]
    for name in required:
        if not (directory / name).is_file() or (directory / name).stat().st_size == 0:
            raise ValueError(f"missing or empty release artifact: {name}")
    for pattern in ["*.dmg", "*.msi", "*.exe", "*.deb", "*.AppImage"]:
        if not any(p.is_file() and p.stat().st_size for p in directory.glob(pattern)):
            raise ValueError(f"missing installer: {pattern}")
    assets = sorted(p for p in directory.iterdir() if p.is_file() and (
        p.name in required or p.suffix in {".dmg", ".msi", ".exe", ".deb", ".AppImage"}
    ))
    metadata = directory / "BUILD-METADATA.json"
    metadata.write_text(json.dumps({
        "version": version, "commit": commit,
        "artifacts": [p.name for p in assets],
        "reproducibleTarget": "aarch64-unknown-linux-musl",
        "codeSigned": False,
    }, indent=2) + "\n")
    lines = []
    for path in sorted([*assets, metadata]):
        with path.open("rb") as stream:
            digest = hashlib.file_digest(stream, "sha256").hexdigest()
        lines.append(f"{digest}  {path.name}\n")
    (directory / "SHA256SUMS").write_text("".join(lines))


if __name__ == "__main__":
    if sys.argv[1] == "check-version":
        check_version(Path(__file__).resolve().parent.parent, *sys.argv[2:])
    elif sys.argv[1] == "manifest":
        manifest(Path(sys.argv[2]), *sys.argv[3:])
    else:
        raise SystemExit("unknown release command")
