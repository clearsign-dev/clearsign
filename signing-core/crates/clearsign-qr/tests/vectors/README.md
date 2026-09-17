# Where the QR test vectors come from

Nothing in these tests was produced by this crate.

## Specification vectors — `tests/spec_vectors.rs`

Copied from the Blockchain Commons research papers, which carry them as
reference-implementation test cases:

| What | Source |
|---|---|
| Bytewords word list, example encoding, CRC-32 | BCR-2020-012, "Word List" and "Example/Test Vector" |
| UR string form, single and multipart | BCR-2020-005, "UR Encoding" |
| Xoshiro256** outputs, random sampler, degree chooser, Fisher-Yates shuffle, fragment lengths, message partitioning, part CBOR | BCR-2024-001, the test vector blocks in each section |

```sh
curl -fsSL -O https://raw.githubusercontent.com/BlockchainCommons/Research/master/papers/bcr-2020-005-ur.md
curl -fsSL -O https://raw.githubusercontent.com/BlockchainCommons/Research/master/papers/bcr-2020-012-bytewords.md
curl -fsSL -O https://raw.githubusercontent.com/BlockchainCommons/Research/master/papers/bcr-2024-001-multipart-ur.md
```

The 20 published parts for `makeMessage(256)` are fed to our decoder with two
plain fragments withheld, so the fountain mixing is actually exercised rather
than the parts simply being concatenated.

## Interoperability vectors — `tests/reference_impl.rs` and the CLI tests

Produced by Keystone's implementation of EIP-4527, the same library MetaMask's
air-gapped flow uses, running under Node:

```sh
npm install @keystonehq/bc-ur-registry-eth@0.22.1 @ngraveio/bc-ur
```

```js
import { EthSignRequest, DataType, ETHSignature } from '@keystonehq/bc-ur-registry-eth';
import { UREncoder } from '@ngraveio/bc-ur';

const req = EthSignRequest.constructETHRequest(
  Buffer.from(unsignedTxHex.replace(/^0x/, ''), 'hex'),
  DataType.typedTransaction,
  "m/44'/60'/0'/0/0",
  "f23f9fd2",                               // source fingerprint
  "9b1deb4d-3b7d-4bad-9bdd-2b0d7b3dcb6d",   // request id
  1,                                        // chain id
  "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266",
  "metamask",
);
console.log(new UREncoder(req.toUR(), 1000).nextPart());   // one QR code
const enc = new UREncoder(req.toUR(), 40);                 // animated
for (let i = 0; i < 14; i++) console.log(enc.nextPart());
```

The transaction inside the request came from `cast`:

```sh
TX=$(cast to-rlp '["0x01","0x2a","0x3b9aca00","0x06fc23ac00","0x5208","0xa0b86991c6218b36c1d19d4a2e9eb0ce3606eb48","0x","0xa9059cbb…","[]"]')
echo "0x02${TX#0x}"
```

The reply is checked in both directions:

```sh
# our signature over the request's digest equals Foundry's
cast wallet sign --mnemonic "$TEST_PHRASE" --no-hash 0x2f128c7140e99f9c0880913191b070a078fdf92e216d1df89ff7f7b65e4a45da

# and Keystone's decoder reads the eth-signature UR we emit
node -e '…ETHSignature.fromCBOR(URDecoder…resultUR().cbor)…'
```

The test phrase is Foundry's public Anvil phrase. Never use a real one.
