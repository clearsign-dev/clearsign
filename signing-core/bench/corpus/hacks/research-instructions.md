# Attack-path research: what actually happened in each incident

You are classifying real crypto hacks by ATTACK PATH, for a benchmark of a
transaction-review tool. Accuracy matters more than speed. Never invent facts,
never invent transaction hashes. "UNKNOWN" is an acceptable, honest answer.

## The tool being benchmarked (so you know what question matters)

ClearSign decodes, offline, exactly what a transaction or signed message will do
from its raw bytes, before a human signs it with their own key. It holds no
keys. It can only help when a LEGITIMATE KEY HOLDER WAS ASKED TO SIGN something
and did not understand what it really did. It cannot help when an attacker
already has the keys and signs for themselves, or when a smart-contract bug is
exploited with no victim signature at all.

## For each incident in your batch file, decide:

1. `vector` — exactly one of:
   - `SIGNING_DECEPTION_MULTISIG`: legitimate multisig signers approved a
     transaction whose real effect differed from what they were shown
     (compromised UI / frontend / signer machine, blind signing on a hardware
     wallet). Examples: Bybit 2025, WazirX 2024, Radiant 2024.
   - `SIGNING_DECEPTION_EOA`: a key holder (an individual, a team member, or a
     protocol's users) signed a transaction or off-chain message, served by a
     phishing site, compromised frontend, DNS/CDN hijack, injected or
     supply-chain script, or malicious dapp, that did something other than what
     they believed: token approvals, Permit/Permit2 signatures,
     setApprovalForAll, marketplace orders, native or token transfers to the
     attacker, EIP-7702 delegations, ownership or admin changes.
   - `DECEIVED_INTENT`: the transaction did exactly what its bytes said and a
     faithful display would have shown it, but the person was deceived about the
     counterparty or purpose (address poisoning, email/BEC address swap,
     poisoned payment list, fake token, impersonated counterparty).
   - `KEY_THEFT`: the attacker obtained private keys, seed phrases, MPC shares,
     API keys, signing servers or cloud credentials (malware, fake job offer,
     hot wallet server breach, weak/brute-forced key generation, leaked key,
     insider leak, SIM swap) and signed transactions THEMSELVES. No victim review
     step existed to intercept.
   - `CONTRACT_OR_PROTOCOL_BUG`: a code, logic, oracle, accounting, bridge
     verification or compiler flaw exploited without any victim signing anything.
   - `INSIDER_OR_RUG`: a privileged insider or the team deliberately took funds.
   - `GOVERNANCE`: a malicious proposal passed or executed through governance.
   - `UNKNOWN`: public information is insufficient to say.

   Be careful with labels like "private key compromised" or "phishing": phishing
   that STOLE A SEED or installed malware that exfiltrated keys is `KEY_THEFT`;
   phishing that got the victim to SIGN a malicious transaction is
   `SIGNING_DECEPTION_*`. If the attacker stole one signer's key but still
   needed other signers to approve a disguised transaction, that is
   `SIGNING_DECEPTION_MULTISIG`.

2. `signed_payload_format` — what the deceived victim actually signed, only for
   the two SIGNING_DECEPTION vectors and DECEIVED_INTENT; otherwise `NONE`.
   One of: `SAFE_TX` (Safe/Gnosis multisig), `EVM_TX` (ordinary Ethereum-style
   transaction on any EVM chain), `EIP712` (typed-data signature such as Permit,
   Permit2, Seaport/Wyvern orders), `PERSONAL_SIGN`, `EIP7702_AUTH`, `BITCOIN`,
   `SOLANA`, `TRON`, `OTHER_NONEVM`, `OTHER_MULTISIG` (non-Safe multisig on EVM),
   `UNKNOWN`.

3. `malicious_action` — short concrete label of what the signed thing did, e.g.
   "Safe delegatecall replacing implementation", "transferOwnership of
   LendingPoolAddressesProvider to attacker contract", "unlimited approve to
   attacker EOA", "increaseAllowance to attacker", "Permit2 PermitBatch to
   drainer", "setApprovalForAll to attacker", "ETH sent to attacker address",
   "upgradeToAndCall to malicious implementation". Empty string if not
   applicable.

4. `attack_tx` — list of `{"chain": "...", "hash": "0x..."}` for the malicious
   transaction(s) the deceived signer(s) approved, or for retail phishing one or
   two example victim transactions. ONLY include a hash you read in a source
   (block explorer link, post-mortem, security firm alert). Empty list otherwise.
   Do not guess.

5. `confidence` — `high` (post-mortem or multiple reputable sources agree),
   `medium` (one reputable source), `low` (inferred).

6. `sources` — 1 to 3 URLs you actually read or saw in search results
   (post-mortems, rekt.news, SlowMist, Halborn, CertiK, PeckShield, Chainalysis,
   Elliptic, TRM, ZachXBT, the project's own statement, reputable news).
   Posts on X/Twitter surfaced by search are acceptable sources.

7. `notes` — one or two sentences: what happened, in plain words.

## How to work

- Use WebSearch, then WebFetch on the best result when needed. x.com pages
  usually cannot be fetched; use search snippets, or security firms' blogs and
  news that quote them.
- Spend effort where it matters: an obvious contract exploit or an
  exchange hot-wallet server breach needs one confirming search. Anything that
  might involve signers or users being tricked into signing deserves a real look
  and an attempt to find the attack transaction hash.
- Several incidents are from 2026; do not assume you know them — search.
- Keep the `id` field from the batch file unchanged.

## Output

Write a JSON array to the result file named in your task, one object per input
incident, in the same order, with exactly these keys:
`id, name, vector, signed_payload_format, malicious_action, attack_tx,
confidence, sources, notes`.
Validate it parses (e.g. `python3 -c "import json;json.load(open(PATH))"`).
Then reply with a 5-line summary: counts per vector, and any incident where you
found the deception class but no transaction hash.
