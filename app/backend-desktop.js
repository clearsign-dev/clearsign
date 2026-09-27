// Desktop build: the reviewer is the application itself. Every decision about
// what a transaction does is made in Rust, by the same crates the command-line
// tool uses; this file only carries the question across and brings the answer
// back. There is no decoding in this window.
const invoke = window.__TAURI__.core.invoke;

async function review(json, chainId, version) {
  try {
    return await invoke("review_transaction", {
      jsonText: json,
      chainId: chainId || 0,
      version,
    });
  } catch (e) {
    return { ok: false, message: String(e) };
  }
}
