#!/usr/bin/env bash
# Build the window, in both the forms it takes.
#
#   dist/clearsign.html   one file, nothing to install, opens from a folder
#   ../desktop/src/index.html   the front end of the desktop application
#
# Both are generated from one template, so the two cannot drift apart. They
# differ in exactly one place: where the review comes from. The standalone page
# carries the reviewer with it, compiled to WebAssembly; the application asks the
# Rust it is built from. Neither decodes anything in JavaScript.
set -euo pipefail
HERE="$(cd "$(dirname "$0")" && pwd)"
ROOT="$(cd "$HERE/.." && pwd)"
OUT="$HERE/dist"
DESKTOP_SRC="$ROOT/desktop/src"
WASM_TARGET=wasm32-unknown-unknown
EXAMPLE="$ROOT/signing-core/crates/clearsign-cli/tests/fixtures/bybit-safe-tx.json"

# Windows has Python but not always under the name `python3`.
PY=python3
command -v "$PY" >/dev/null 2>&1 || PY=python
command -v "$PY" >/dev/null 2>&1 || { echo "no python found" >&2; exit 1; }

cd "$ROOT/signing-core"
rustup target add "$WASM_TARGET" >/dev/null 2>&1 || true
SOURCE_DATE_EPOCH=0 cargo build -q -p clearsign-wasm --release --locked --target "$WASM_TARGET"
WASM="$ROOT/signing-core/target/$WASM_TARGET/release/clearsign_wasm.wasm"
[[ -f "$WASM" ]] || { echo "the WebAssembly module was not built" >&2; exit 1; }

mkdir -p "$OUT" "$DESKTOP_SRC"
"$PY" - "$HERE" "$WASM" "$EXAMPLE" "$OUT/clearsign.html" "$DESKTOP_SRC/index.html" <<'PY'
import base64, json, pathlib, sys

here, wasm, example, standalone_out, desktop_out = (pathlib.Path(p) for p in sys.argv[1:6])
template = (here / "index.template.html").read_text()
example_json = json.dumps(example.read_text())

def build(backend_js, out, title_suffix=""):
    page = template.replace("__REVIEW_BACKEND__", backend_js)
    page = page.replace("__EXAMPLE_JSON__", example_json)
    assert "__REVIEW_BACKEND__" not in page and "__EXAMPLE_JSON__" not in page
    assert "__WASM_BASE64__" not in page, "a placeholder was left behind"
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(page)
    print(f"{out}  ({out.stat().st_size // 1024} KB){title_suffix}")

wasm_backend = (here / "backend-wasm.js").read_text().replace(
    "__WASM_BASE64__", base64.b64encode(wasm.read_bytes()).decode()
)
build(wasm_backend, standalone_out, "  — one file, opens anywhere")
build((here / "backend-desktop.js").read_text(), desktop_out, "  — front end of the application")
PY

# Parsing is not working. A constant referenced in three places and declared in
# none passed every check this build had, and shipped.
if [[ "${SKIP_SMOKE:-0}" != "1" ]] && command -v node >/dev/null 2>&1; then
  echo
  node "$HERE/smoke-test.mjs" || {
    echo "the built page failed its own smoke test" >&2
    exit 1
  }
fi

echo
echo "The single file:   open $OUT/clearsign.html"
echo "The application:   cd $ROOT/desktop && npm run build"
