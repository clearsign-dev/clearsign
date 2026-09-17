#!/usr/bin/env bash
# Download the GrapheneOS source tree for one release, verifying the signed manifest tag
# first. The manifest pins all 1,057 projects to commit hashes, so verifying the tag
# verifies the whole tree. The signing key fingerprint is pinned here, not trusted on
# first download: SHA256:AhgHif0mei+9aNyKLfMZBh2yptHdw/aN7Tlh/j2eFwM (checked 16 Sep 2026).
set -euo pipefail
PINS="$(cd "$(dirname "$0")/../../.." && pwd)/config/pins.env"
[[ -f "$PINS" ]] && source "$PINS"
TAG="${TAG:-${GRAPHENEOS_RELEASE:?set TAG or provide config/pins.env}}"
DIR="${1:-$HOME/grapheneos-$TAG}"
PINNED_FP="${GRAPHENEOS_SSH_KEY_FP:-SHA256:AhgHif0mei+9aNyKLfMZBh2yptHdw/aN7Tlh/j2eFwM}"

mkdir -p "$DIR" && cd "$DIR"
curl -fsSL https://grapheneos.org/allowed_signers -o .allowed_signers
fp=$(awk '{print $2" "$3}' .allowed_signers | ssh-keygen -lf - | awk '{print $2}')
[[ "$fp" == "$PINNED_FP" ]] || { echo "GrapheneOS signing key changed: $fp (pinned $PINNED_FP). Stop and investigate." >&2; exit 1; }

repo init -u https://github.com/GrapheneOS/platform_manifest.git -b "refs/tags/$TAG"
( cd .repo/manifests
  git config gpg.ssh.allowedSignersFile "$DIR/.allowed_signers"
  git verify-tag "$(git describe)" )
echo "manifest tag $TAG verified; syncing"
repo sync -j8
