"""Fail-closed helpers for release signing configuration.

This module validates credential presence without printing credential values and
creates the temporary Tauri override used for Windows Authenticode signing.
"""

import json
import os
from pathlib import Path
import re
import sys
from urllib.parse import urlsplit


REQUIRED_CREDENTIALS = {
    "macos": (
        "APPLE_CERTIFICATE",
        "APPLE_CERTIFICATE_PASSWORD",
        "APPLE_API_ISSUER",
        "APPLE_API_KEY",
        "APPLE_API_KEY_P8_BASE64",
        "APPLE_TEAM_ID",
    ),
    "windows": (
        "WINDOWS_CERTIFICATE",
        "WINDOWS_CERTIFICATE_PASSWORD",
        "WINDOWS_CERTIFICATE_THUMBPRINT",
    ),
}

DEFAULT_WINDOWS_TIMESTAMP_URL = "http://timestamp.digicert.com"


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


def require_credentials(platform, environ=None):
    environ = os.environ if environ is None else environ
    try:
        required = REQUIRED_CREDENTIALS[platform]
    except KeyError as error:
        raise ValueError(f"unsupported signing platform: {platform}") from error
    missing = [name for name in required if not environ.get(name, "").strip()]
    if missing:
        raise ValueError(
            f"{platform} release signing credentials are incomplete: "
            + ", ".join(missing)
        )


def windows_config(thumbprint, timestamp_url=DEFAULT_WINDOWS_TIMESTAMP_URL):
    compact_thumbprint = re.sub(r"\s", "", thumbprint).upper()
    if not re.fullmatch(r"[0-9A-F]{40}", compact_thumbprint):
        raise ValueError("Windows certificate thumbprint must be 40 hexadecimal characters")
    parsed = urlsplit(timestamp_url)
    if parsed.scheme not in {"http", "https"} or not parsed.netloc:
        raise ValueError("Windows timestamp URL must be an absolute HTTP(S) URL")
    if parsed.username or parsed.password or parsed.fragment:
        raise ValueError("Windows timestamp URL must not contain credentials or a fragment")
    return {
        "bundle": {
            "windows": {
                "certificateThumbprint": compact_thumbprint,
                "digestAlgorithm": "sha256",
                "timestampUrl": timestamp_url,
            }
        }
    }


def write_windows_config(path, thumbprint, timestamp_url):
    output = Path(path)
    output.write_text(
        json.dumps(windows_config(thumbprint, timestamp_url), indent=2) + "\n",
        encoding="utf-8",
    )


def main(argv):
    if len(argv) == 2 and argv[0] == "check-env":
        require_credentials(argv[1])
        return
    if len(argv) in {3, 4} and argv[0] == "windows-config":
        timestamp_url = argv[3] if len(argv) == 4 else DEFAULT_WINDOWS_TIMESTAMP_URL
        write_windows_config(argv[1], argv[2], timestamp_url)
        return
    if len(argv) == 2 and argv[0] == "check-macos-adhoc":
        require_macos_adhoc_signature(
            Path(argv[1]).read_text(encoding="utf-8", errors="replace")
        )
        return
    raise SystemExit(
        "usage: release_signing.py check-env PLATFORM | "
        "windows-config OUTPUT THUMBPRINT [TIMESTAMP_URL] | "
        "check-macos-adhoc DETAILS_FILE"
    )


if __name__ == "__main__":
    try:
        main(sys.argv[1:])
    except ValueError as error:
        raise SystemExit(str(error)) from error
