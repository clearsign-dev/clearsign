"""Classify identity-free macOS signatures for evaluation builds.

Publisher credentials deliberately have no helper or execution path in this
repository. Stable signing belongs in a separately administered service.
"""

from pathlib import Path
import sys


def require_macos_adhoc_signature(details):
    """Accept only a macOS ad-hoc signature with no publisher identity."""
    fields = {}
    authorities = []
    for raw_line in details.splitlines():
        line = raw_line.strip()
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        if key == "Authority":
            authorities.append(value)
        else:
            fields[key] = value

    if fields.get("Signature") != "adhoc":
        raise ValueError("macOS evaluation app is not ad-hoc signed")
    if fields.get("TeamIdentifier") != "not set":
        raise ValueError("macOS evaluation app contains a TeamIdentifier")
    if authorities:
        raise ValueError("macOS evaluation app contains a signing authority")


def main(argv):
    if len(argv) == 2 and argv[0] == "check-macos-adhoc":
        require_macos_adhoc_signature(
            Path(argv[1]).read_text(encoding="utf-8", errors="replace")
        )
        return
    raise SystemExit(
        "usage: release_signing.py check-macos-adhoc DETAILS_FILE"
    )


if __name__ == "__main__":
    try:
        main(sys.argv[1:])
    except ValueError as error:
        raise SystemExit(str(error)) from error
