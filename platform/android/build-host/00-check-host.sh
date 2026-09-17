#!/usr/bin/env bash
# Refuse to start on a machine that cannot build GrapheneOS, per grapheneos.org/build:
# x86_64 Linux, 32 GiB+ memory, 136 GiB+ for a synced tree plus 100 GiB+ for a build.
set -euo pipefail
fail() { echo "HOST CHECK FAILED: $*" >&2; exit 1; }
[[ "$(uname -s)" == "Linux" ]] || fail "need Linux, found $(uname -s)"
[[ "$(uname -m)" == "x86_64" ]] || fail "need x86_64, found $(uname -m)"
mem_kib=$(awk '/MemTotal/ {print $2}' /proc/meminfo)
(( mem_kib >= 32 * 1024 * 1024 )) || fail "need 32 GiB+ RAM, found $((mem_kib / 1024 / 1024)) GiB"
free_gib=$(df -BG --output=avail "${1:-$PWD}" | tail -1 | tr -dc 0-9)
(( free_gib >= 250 )) || fail "need 250 GiB+ free, found ${free_gib} GiB"
echo "host OK: $(uname -m) Linux, $((mem_kib / 1024 / 1024)) GiB RAM, ${free_gib} GiB free"
