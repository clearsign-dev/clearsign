"""Validate release versions and hash every downloadable artifact."""

import hashlib
import json
from pathlib import Path
import re
import sys
import tomllib


def check_version(root, tag, ref_type):
    if ref_type == "tag" and not re.fullmatch(r"v\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?", tag):
        raise ValueError("release tags must use vMAJOR.MINOR.PATCH")
    package = json.loads((root / "desktop/package.json").read_text())
    lock = json.loads((root / "desktop/package-lock.json").read_text())
    expected = tag[1:] if ref_type == "tag" else package["version"]
    versions = {
        "desktop/package.json": package["version"],
        "desktop/package-lock.json": lock["version"],
        "desktop/package-lock.json root package": lock["packages"][""]["version"],
        "desktop/src-tauri/tauri.conf.json": json.loads((root / "desktop/src-tauri/tauri.conf.json").read_text())["version"],
        "desktop/src-tauri/Cargo.toml": tomllib.loads((root / "desktop/src-tauri/Cargo.toml").read_text())["package"]["version"],
        "signing-core/Cargo.toml": tomllib.loads((root / "signing-core/Cargo.toml").read_text())["workspace"]["package"]["version"],
    }
    for name, actual in versions.items():
        if actual != expected:
            raise ValueError(f"{name}: {actual} does not match {expected}")
    if ref_type != "tag":
        return
    heading = next((line for line in (root / "CHANGELOG.md").read_text().splitlines()
                    if line.startswith(f"## {tag} ")), None)
    if heading is None:
        raise ValueError(f"CHANGELOG.md has no section for {tag}")
    if "unreleased" in heading.lower():
        raise ValueError(f"CHANGELOG.md still marks {tag} as unreleased")


def required_assets(version):
    if not re.fullmatch(r"\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?", version):
        raise ValueError("invalid release version")
    return [
        "clearsign-aarch64-apple-darwin.tar.gz",
        "clearsign-x86_64-apple-darwin.tar.gz",
        "clearsign-x86_64-unknown-linux-musl.tar.gz",
        "clearsign-aarch64-unknown-linux-musl.tar.gz",
        "clearsign.html", "BINARY-SHA256",
        f"ClearSign_{version}_aarch64.dmg",
        f"ClearSign_{version}_x64_en-US.msi",
        f"ClearSign_{version}_x64-setup.exe",
        f"ClearSign_{version}_amd64.deb",
        f"ClearSign_{version}_amd64.AppImage",
    ]


def validate_assets(directory, version):
    required = required_assets(version)
    for name in required:
        path = directory / name
        if path.is_symlink() or not path.is_file() or path.stat().st_size == 0:
            raise ValueError(f"missing or empty release artifact: {name}")
    # Per-archive sidecars are CI output, not published release assets.
    allowed = set(required) | {"SHA256SUMS", "BUILD-METADATA.json"} | {
        name + ".sha256" for name in required if name.endswith(".tar.gz")
    }
    unexpected = {p.name for p in directory.iterdir()} - allowed
    if unexpected:
        raise ValueError(f"unexpected release artifacts: {sorted(unexpected)}")
    return sorted(required)


def manifest(directory, version, commit):
    assets = validate_assets(directory, version)
    metadata = directory / "BUILD-METADATA.json"
    metadata.write_text(json.dumps({
        "version": version, "commit": commit,
        "artifacts": assets,
        "reproducibleTarget": "aarch64-unknown-linux-musl",
        "codeSigned": False,
    }, indent=2) + "\n")
    lines = []
    for name in sorted([*assets, metadata.name]):
        path = directory / name
        with path.open("rb") as stream:
            digest = hashlib.file_digest(stream, "sha256").hexdigest()
        lines.append(f"{digest}  {path.name}\n")
    (directory / "SHA256SUMS").write_text("".join(lines))


def verify(directory, version, commit):
    """Check the candidate's existing manifest without rewriting its evidence."""
    assets = validate_assets(directory, version)
    expected_names = set(assets) | {"BUILD-METADATA.json"}
    entries = {}
    for line in (directory / "SHA256SUMS").read_text().splitlines():
        match = re.fullmatch(r"([0-9a-f]{64})  (.+)", line)
        if not match or match[2] not in expected_names or match[2] in entries:
            raise ValueError("invalid, duplicate or unexpected checksum entry")
        entries[match[2]] = match[1]
    if set(entries) != expected_names:
        raise ValueError("checksum manifest is incomplete")
    for name, expected in entries.items():
        path = directory / name
        if path.is_symlink():
            raise ValueError(f"symlink in release: {name}")
        with path.open("rb") as stream:
            actual = hashlib.file_digest(stream, "sha256").hexdigest()
        if actual != expected:
            raise ValueError(f"checksum mismatch: {name}")
    metadata = json.loads((directory / "BUILD-METADATA.json").read_text())
    if (metadata.get("version") != version or metadata.get("commit") != commit
            or metadata.get("artifacts") != assets):
        raise ValueError("candidate metadata does not match the release")


if __name__ == "__main__":
    if sys.argv[1] == "check-version":
        check_version(Path(__file__).resolve().parent.parent, *sys.argv[2:])
    elif sys.argv[1] == "manifest":
        manifest(Path(sys.argv[2]), *sys.argv[3:])
    elif sys.argv[1] == "verify":
        verify(Path(sys.argv[2]), *sys.argv[3:])
    else:
        raise SystemExit("unknown release command")
