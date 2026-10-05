# Using ClearSign before you sign

**Who this is for:** anyone who approves transactions on a shared Safe — a fund,
a DAO treasury, a protocol multisig. You do not need any hardware, and you do not
need to change how you sign. This adds one step before you click approve.

**What it does:** reads the transaction you are about to approve and tells you
what those bytes actually do, from the bytes alone. It has no knowledge of your
wallet and no opinion from any service — including Safe's own. The decoder never
opens a socket; the one thing that reaches the network is the option to fetch a
queued transaction by its hash from Safe's service, and what comes back is
decoded from its bytes with its `dataDecoded` field ignored.

**What it does not do:** hold your keys, sign anything, or talk to a chain. It
reads and reports. Your existing hardware wallet still signs.

---

## The five-minute version

1. In Safe{Wallet}, open the queued transaction and copy its details as JSON.
   (Or fetch it: `curl "https://safe-transaction-mainnet.safe.global/api/v1/multisig-transactions/<safeTxHash>/"`.)
2. Save it as `tx.json`.
3. Run:

   ```sh
   clearsign safe-json tx.json --chain-id 1
   ```

4. Read what it says. Compare the **Safe transaction hash** it prints with the
   hash on your hardware wallet's screen, and with what the other signers see on
   their own machines.
5. Approve only if the hashes match everywhere and you intended everything it
   listed.

The exit code is there for scripts: `0` nothing alarming, `2` something could not
be decoded, `3` something CRITICAL.

## What you are looking for

A transaction that is what it claims looks like this:

```
Action ........................... ERC-20 transfer
Token contract ................... 0xA0b8 6991 c621 8b36 c1d1 9D4a 2e9E b0cE 3606 eB48
Recipient ........................ 0x7099 7970 C518 12dc 3A01 0C7d 01b5 0e0d 17dc 79C8
Amount (raw integer units) ....... 1_000_000
```

A transaction that is not looks like this — and this is the real one that took
about $1.5 billion out of Bybit in February 2025:

```
Operation ........................ DELEGATECALL
Code that will run as the Safe ... 0x9622 1423 681A 6d52 E184 D440 a8eF CEbB 105C 7242
Calldata selector ................ 0xa9059cbb

[CRITICAL] 1:SAFE_DELEGATECALL - DELEGATECALL runs the code at 0x9622…7242 with
full control over this Safe's storage, owners, modules and funds. Any function
name the calldata appears to have does NOT describe what that code does, so it
is deliberately not decoded.
```

The calldata there begins `0xa9059cbb`, which is `transfer(address,uint256)`.
Every tool that decodes calldata without looking at the operation showed Bybit's
signers a token transfer. It was not one. This tool refuses to name it, because
under DELEGATECALL the function name describes nothing.

## Things worth knowing

**The hash is the point.** Everything else is a convenience; the hash is what you
and your co-signers compare. If this tool's hash does not match your hardware
wallet's, stop. Something between you and the transaction is wrong.

**Your Safe's version changes the hash.** Safes on v1.1.x do not include the
chain ID in what they sign, which also means those signatures can be replayed on
other chains. The tool works this out from the file and says which it used; the
version is in Safe{Wallet} under Settings, and it is worth confirming once.

**It will not guess.** No chain ID, no Safe version, no hash to check against —
it stops and says so rather than showing you a confident answer about the wrong
thing.

**The `safeTxHash` in the file is never believed.** It is recomputed from the
fields. If the two disagree, the tool refuses and tells you, because the only
reason they would differ is that something is misrepresenting the transaction.

## Honest limits

- It has had **one** external security review. Ten problems were found and fixed,
  and the fixes have not been reviewed by anyone outside the project.
- It decodes a deliberately bounded set of things: ERC-20 transfers and
  approvals, Safe administration, MultiSend batches at Safe's published
  addresses, and since October 2026 the calls attacks have used: ownership,
  role and upgrade changes, token permissions beyond `approve`, and calls that
  carry other calls. Anything else is reported as *not understood* rather than
  guessed at. That is the design, but it means it will often tell you less
  than you want: most ordinary DeFi calls are still BLIND.
- The window reads Safe transactions only. EIP-712 typed data — permits,
  Permit2 and the like — can be reviewed with the command-line tool
  (`clearsign typed-data request.json`), not yet in the window, and nothing
  here signs it.
- The newest decoders, everything added in October 2026, have not had a human
  security review.
- Running it on your everyday laptop means trusting that laptop. It is a second
  opinion, not a secure device. Never give it a recovery phrase.

If it tells you something surprising about a transaction you were about to
approve, that is the moment it was built for. Please say so.
