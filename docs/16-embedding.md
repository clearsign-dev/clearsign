# Embedding ClearSign

ClearSign is useful as a component only if every host receives the same verdict
from the same reviewed bytes. The host owns transport and presentation. The Rust
engine owns parsing, hash computation, findings, severity and required
acknowledgements.

## Implemented interfaces

| Interface | Intended host | Status |
|---|---|---|
| `clearsign-api` | Rust applications and services | Implemented. Typed Review API v1 response |
| `clearsign-wasm` | Offline browser reviewer | Implemented. Length-prefixed UTF-8 JSON from the same API |
| `clearsign-native` | iOS, Android, wallets and custody software | Implemented C ABI. Caller-owned buffers; no allocator crosses the boundary |
| Tauri command | macOS, Windows and Linux desktop app | Implemented and routed through `clearsign-api` |
| Expo native module | iOS and Android reviewer app | Source implemented. Native CI and physical-device release testing are required before publication |

The response carries `apiVersion: 1`, an `ok` flag, the aggregate severity,
locally computed signing hash, Safe domain evidence, sections, findings and
required acknowledgements. Refused input uses the same envelope with
`ok: false`; absence of a result is never interpreted as clear.

## Native C boundary

```c
uint32_t clearsign_api_version(void);
size_t clearsign_max_record_bytes(void);

intptr_t clearsign_review_safe_json(
    const uint8_t *input,
    size_t input_length,
    uint64_t chain_id,
    uint32_t version,
    uint8_t *output,
    size_t output_capacity
);
```

Call once with a null output pointer and zero capacity to obtain the exact
required length, allocate in the host runtime, then call again. A non-negative
return is the UTF-8 JSON byte count. Negative values are boundary errors. Check
`clearsign_max_record_bytes()` before copying input into a native runtime; the C
entry point also rejects a larger `input_length` before reading the pointer.

`chain_id` zero means the record must carry the chain. `version` zero means the
included Safe hash must settle the domain, `1` selects the legacy v1.1.x/v1.2.0
domain and `3` selects v1.3.0 and later.

## Integration requirements

An embedding is not complete merely because it renders the JSON:

1. Preserve the original transaction bytes or signed fields until approval.
2. Display the locally computed hash and compare it with the signing device.
3. Treat `BLIND`, `CRITICAL` and refusals as blocking unless an explicit policy
   says otherwise; never collapse a missing native module into a clear result.
4. Invalidate the result whenever input, chain or Safe domain changes.
5. Keep each finding's returned `number` beside that exact finding. It is the
   canonical identifier used by `mustAcknowledge`; array position or code alone
   is not an approval identifier.
6. Keep service descriptions, token metadata and simulation output visually and
   structurally separate from facts derived from signed bytes.
7. Test the actual wallet version, signing mode and device firmware. ClearSign
   has not certified a Safe, MetaMask, custody platform or hardware-wallet
   integration.

For firmware or a dedicated signer, use the existing `no_std` core rather than
the JSON application boundary where possible. The seL4 framing in
`clearsign-ffi` is deliberately separate from this host API because it has a
fixed arena and a different request protocol.

## What this does not provide

The API does not identify deployed contract bytecode, simulate future chain
state, attest the host display, bind a separate wallet to the reviewed hash, or
turn unsupported chains into supported ones. Bitcoin, Solana and chain-specific
non-EVM transaction formats remain outside the current engine.
