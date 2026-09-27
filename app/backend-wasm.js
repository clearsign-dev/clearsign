// Standalone build: the reviewer travels with the page, compiled to WebAssembly
// from the same Rust the desktop application runs natively.
const WASM_BASE64 = "__WASM_BASE64__";

function bytesFromBase64(b64) {
  const bin = atob(b64);
  const out = new Uint8Array(bin.length);
  for (let i = 0; i < bin.length; i++) out[i] = bin.charCodeAt(i);
  return out;
}

const { instance } = await WebAssembly.instantiate(bytesFromBase64(WASM_BASE64), {});
const { cs_alloc, cs_free, cs_review, memory } = instance.exports;

/** Hand the reviewer a file and get back its finished verdict. */
async function review(json, chainId, version) {
  const bytes = new TextEncoder().encode(json);
  const ptr = cs_alloc(bytes.length);
  if (!ptr) return { ok: false, message: "out of memory" };
  new Uint8Array(memory.buffer, ptr, bytes.length).set(bytes);
  const out = cs_review(ptr, bytes.length, BigInt(chainId || 0), version);
  cs_free(ptr, bytes.length);
  if (!out) return { ok: false, message: "the reviewer could not answer" };
  const len = new DataView(memory.buffer).getUint32(out, true);
  const text = new TextDecoder().decode(new Uint8Array(memory.buffer, out + 4, len));
  cs_free(out, len + 4);
  return JSON.parse(text);
}

/** What this build is, for the line at the foot of the window. */
function describeBuild() {
  const line = document.getElementById("version-line");
  if (line) line.textContent = "Standalone page — the reviewer travels with it, compiled to WebAssembly from the same Rust the application runs.";
}
