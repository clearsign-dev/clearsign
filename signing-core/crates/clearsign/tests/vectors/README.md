# How the test vectors were generated

Every expected value in `../vectors.rs` comes from Foundry's `cast` 1.7.1, an implementation independent of clearsign. Regenerate them with the commands below and compare. **Paste tool output verbatim; never retype or re-split hex by hand.** Two test inputs built by hand during development were wrong, and in one case the decoder was right to reject them.

```sh
# Selectors
cast sig "execTransaction(address,uint256,bytes,uint8,uint256,uint256,uint256,address,address,bytes)"

# Type hashes (also confirmed against safe-global/safe-smart-account tags v1.1.1, v1.3.0, v1.4.1)
cast keccak "SafeTx(address to,uint256 value,bytes data,uint8 operation,uint256 safeTxGas,uint256 baseGas,uint256 gasPrice,address gasToken,address refundReceiver,uint256 nonce)"
cast keccak "EIP712Domain(uint256 chainId,address verifyingContract)"
cast keccak "EIP712Domain(address verifyingContract)"

# Safe transaction hash, v1.3.0+ domain
SAFE=0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4
TYPEHASH=0xbb8310d486368db6bd6f849402fdd73ad53d316b5a4b2644ad6efe0f941286d8
DT=0x47e79534a245952e8b16893a336b85a3d9ea9fa8c573f3d803afb92a79469218
DATA=$(cast calldata "transfer(address,uint256)" 0x70997970C51812dc3A010C7d01b50e0d17dc79C8 1000000000)
SH=$(cast keccak $(cast abi-encode "f(bytes32,address,uint256,bytes32,uint8,uint256,uint256,uint256,address,address,uint256)" \
     $TYPEHASH 0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48 0 $(cast keccak $DATA) 0 0 0 0 \
     0x0000000000000000000000000000000000000000 0x0000000000000000000000000000000000000000 42))
DS=$(cast keccak $(cast abi-encode "f(bytes32,uint256,address)" $DT 1 $SAFE))
cast keccak 0x1901${DS#0x}${SH#0x}      # vector A: 0x62a251bf...

# Legacy (v1.1.x) domain: no chain ID
DS=$(cast keccak $(cast abi-encode "f(bytes32,address)" 0x035aff83d86937d35b32e04f0ddc6ff469290eef2f1b692d8a815c89404d4749 $SAFE))

# Unsigned EIP-1559 transaction bytes and signing hash
R=$(cast to-rlp '["0x01","0x07","0x3b9aca00","0x06fc23ac00","0x0186a0","0xA0b8...eB48","0x","<calldata>",[]]')
TX=0x02${R#0x}
cast keccak $TX
```

The delegatecall vector is modelled on the February 2025 Bybit pattern but uses a synthetic target address, `0x00000000000000000000000000000000DeaDBeef`.

## MultiSend batch vectors (`tests/multisend.rs`)

The packed argument is built with `cast` and never by hand. Each element is
`operation(1) ‖ to(20) ‖ value(32) ‖ dataLength(32) ‖ data`, concatenated:

```sh
USDC=0xA0b86991c6218b36c1d19D4a2e9Eb0cE3606eB48
u256() { cast to-uint256 "$1" | tr -d '\n' | sed 's/^0x//'; }
pack() { # operation to value calldata
  printf "%02x" "$1"; printf "%s" "${2#0x}" | tr 'A-F' 'a-f'
  u256 "$3"; u256 $(( ${#4} / 2 - 1 )); printf "%s" "${4#0x}"
}
d1=$(cast calldata "transfer(address,uint256)" 0x00000000000000000000000000000000DeaDBeef 1500000)
d2=$(cast calldata "approve(address,uint256)" 0x1111111111111111111111111111111111111111 0)

# BATCH_TWO_CALLS
cast calldata "multiSend(bytes)" "0x$(pack 0 $USDC 0 $d1)$(pack 0 $USDC 0 $d2)"

# BATCH_WITH_INNER_DELEGATECALL — second element's operation byte is 1
cast calldata "multiSend(bytes)" "0x$(pack 0 $USDC 0 $d1)$(pack 1 0x00000000000000000000000000000000DeaDBeef 0 $d2)"
```

The pinned deployment addresses in `src/multisend.rs` are not from memory: they
were fetched from `safe-global/safe-deployments`
(`src/assets/<version>/multi_send{,_call_only}.json`) on 17 Sep 2026, and each
one's bytes are checked against its published EIP-55 string by
`every_pinned_address_matches_its_published_checksum`.

```sh
curl -fsSL https://raw.githubusercontent.com/safe-global/safe-deployments/main/src/assets/v1.4.1/multi_send.json \
  | python3 -c 'import json,sys; d=json.load(sys.stdin); print({k: v["address"] for k,v in d["deployments"].items()})'
```
