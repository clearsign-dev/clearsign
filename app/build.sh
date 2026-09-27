#!/usr/bin/env bash
# Build the window: compile the reviewer to WebAssembly and inline it into one
# HTML file.
#
# One file, because the point is that someone can be handed it and open it. A
# page that fetches its WebAssembly alongside itself cannot be opened from a
# folder — browsers refuse the request — so the module is embedded, and what you
# give somebody is a single thing with nothing else to install and nothing to
# connect to.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/.." && pwd)"
OUT="$HERE/dist"
WASM_TARGET=wasm32-unknown-unknown

cd "$ROOT/signing-core"
rustup target add "$WASM_TARGET" >/dev/null 2>&1 || true
SOURCE_DATE_EPOCH=0 cargo build -q -p clearsign-wasm --release --locked --target "$WASM_TARGET"
WASM="$ROOT/signing-core/target/$WASM_TARGET/release/clearsign_wasm.wasm"
[[ -f "$WASM" ]] || { echo "the WebAssembly module was not built" >&2; exit 1; }

mkdir -p "$OUT"
python3 - "$WASM" "$HERE/index.template.html" \
    "$ROOT/signing-core/crates/clearsign-cli/tests/fixtures/bybit-safe-tx.json" \
    "$OUT/before-you-sign.html" <<'PY'
import base64, json, pathlib, sys

wasm, template, example, out = (pathlib.Path(p) for p in sys.argv[1:5])
page = template.read_text()
page = page.replace("__WASM_BASE64__", base64.b64encode(wasm.read_bytes()).decode())
# The example is embedded as a JSON string literal so it cannot break the script.
page = page.replace("__EXAMPLE_JSON__", json.dumps(example.read_text()))
assert "__WASM_BASE64__" not in page and "__EXAMPLE_JSON__" not in page, "a placeholder was left behind"
out.write_text(page)
print(f"{out}  ({out.stat().st_size // 1024} KB)")
PY

echo
echo "Open it, or send it to someone. Nothing else is needed:"
echo "  open $OUT/before-you-sign.html"
