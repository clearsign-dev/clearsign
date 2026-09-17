#!/usr/bin/env bash
# Build the GrapheneOS emulator target, the development target GrapheneOS recommends
# (sdk_phone64_x86_64, userdebug). It needs no vendor files. Emulator targets do not
# receive full monthly security updates or all baseline security features: development only.
set -euo pipefail
DIR="${1:?path to synced tree}"
cd "$DIR"
set +u
source build/envsetup.sh
lunch sdk_phone64_x86_64-cur-userdebug
set -u
m
echo "built. Launch with: emulator   (from this shell, after envsetup and lunch)"
