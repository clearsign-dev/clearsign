# Where these came from

`bybit-safe-tx.json` is the real transaction that took about $1.5 billion out of
Bybit on 21 February 2025, fetched from Safe's own production Transaction
Service:

```sh
curl "https://safe-transaction-mainnet.safe.global/api/v1/safes/0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4/multisig-transactions/?limit=1"
```

Nothing in it is synthetic. It is the shape a real signer is looking at when they
are about to approve something, which is why it is the fixture: the calldata
reads as an ordinary ERC-20 transfer of zero tokens, and `operation` is 1, so
what actually ran was the attacker's code with the Safe's full authority.

The file keeps only the fields that are signed, plus `safeTxHash` — which the
tool recomputes rather than believes — and a couple of fields for context.
