# Test requests

Produced by Keystone's `@keystonehq/bc-ur-registry-eth` 0.22.1 with
`@ngraveio/bc-ur` — the library MetaMask's air-gapped flow uses — not by this
project. The transaction inside is an unsigned EIP-1559 ERC-20 transfer built by
Foundry's `cast to-rlp`. The commands are in
`signing-core/crates/clearsign-qr/tests/vectors/README.md`.

The request names account 0 of Foundry's public Anvil test phrase as its expected
signer, and carries that wallet's BIP-32 master fingerprint (`16a93ed0`), so the
device can check the request was built for the wallet it holds:

    0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266

`request-single.ur` is the whole request in one QR code.
`request-animated.ur` is the same request across five QR codes.
