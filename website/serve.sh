#!/bin/sh
# Serve the site locally. No build step — this is the whole toolchain.
set -eu
port="${1:-8777}"
cd "$(dirname "$0")"
echo "http://127.0.0.1:$port"
exec python3 -m http.server "$port" --bind 127.0.0.1
