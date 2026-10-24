# Crypto losses by attack vector: sourced figures for the ClearSign benchmark

Compiled 2026-10-05. The same figures, one object per figure with a verbatim quote, are in `landscape.json` (263 entries).

## How to read this file

- Every figure is followed by its source URL and publication date.
- **[computed]** marks arithmetic done here on the source's own numbers. Nothing else is derived or estimated.
- **[secondary]** marks a news article relaying a primary source that could not be fetched (mostly X posts). The primary is named.
- **[reader]** marks a page read through the r.jina.ai reader because the site blocked direct fetches (Medium, hacken.io, fbi.gov). X posts marked "via fxtwitter" were read as exact JSON text from the api.fxtwitter.com mirror.
- **(chart)** marks a value printed on a chart in a PDF. PDF quotes were extracted with pdftotext.
- Quotes come from automated page extraction. Whitespace and table formatting may differ slightly from the page.
- **NOT FOUND** means the number was looked for and could not be found in an accessible source.
- The session's web-search budget ran out partway through, so later lookups were direct fetches of known or linked URLs. As a result, Scam Sniffer's monthly posts after January 2026 (published only on X) could not be located.

---

## 0. What the numbers say for ClearSign

1. **Retail signature phishing is real but shrank sharply.**
   - Scam Sniffer counted:
     - 2023: about $295M from about 324,000 victims (https://drops.scamsniffer.io/scam-sniffer-2023-crypto-phishing-scams-drain-300-million-from-320000-users/, 2024-01-01)
     - 2024: $494M from 332,000 victims (https://drops.scamsniffer.io/scam-sniffer-2024-web3-phishing-attacks-wallet-drainers-drain-494-million/, 2025-01-03)
     - 2025: $83.85M from 106,106 victims, down 83% (https://drops.scamsniffer.io/scam-sniffer-2025-crypto-phishing-losses-fall-83-to-84-million/, 2026-01-03)
   - CertiK's H1 2026 report puts wallet drainers at only $11.4M across 27 incidents (https://indd.adobe.com/view/publication/9f1c777d-8d9f-4b14-b417-e4a29ee5359a/931z/publication-web-resources/pdf/English.pdf, 2026-07-06).

2. **What retail victims sign falls into a handful of decodable call types.**
   - 2024 large cases (≥$1M), share of losses (Scam Sniffer 2024, URL above, 2025-01-03): Permit 56.7%, setOwner 31.9% (one $55.48M DSProxy owner change), Transfer 4.5%, increaseAllowance 3.5%, others 3.4%.
   - 2025 large cases (Scam Sniffer 2025, URL above, 2026-01-03):

     | Signature type | Cases | Losses |
     |---|---|---|
     | Permit/Permit2 | 3 | $8.72M (38%) |
     | Approve/increaseApproval | 3 | $5.62M |
     | Transfer | 2 | $4.87M |
     | EIP-7702 batch | 2 | $2.54M |
     | setApprovalForAll | 1 | $1.23M |

3. **Institutional signer deception is where most of the money is.**
   - Bybit: over $1.4B, after three signers approved a Safe transaction that switched to delegatecall and upgraded the wallet (https://coinacademy.fr/wp-content/uploads/2025/02/Bybit-Incident-Investigation-Report.pdf, 2025-02-24).
   - WazirX: over $230M (https://wazirx.com/blog/preliminary-report-cyber-attack-on-wazirx-multisig-wallet/, 2024-07-18).
   - Radiant: about $50M (https://medium.com/@RadiantCapital/radiant-post-mortem-fecd6cd38081, 2024-10-18) [reader].
   - Drift: about $285M, after at least two council members blind-signed (https://www.chainalysis.com/blog/lessons-from-the-drift-hack/, 2026-04-09).
   - Share of DeFiLlama's yearly totals from incidents classified here as signer-deceived (https://api.llama.fi/hacks, retrieved 2026-10-05) [computed]: 35.3% in 2024, 53.2% in 2025, 13.8% in 2026 to date.

4. **Most money is lost operationally, not to code bugs. But no firm separates deceived signers from stolen keys.**
   - TRM 2025: infrastructure attacks 76% ($2.2B, 45 incidents); code exploits 12.1% ($350M, 52 incidents) (https://www.trmlabs.com/reports-and-whitepapers/2026-crypto-crime-report, 2026-01-28).
   - Hacken Q2 2026: 88.3% from "compromised keys, signers, and infrastructure"; about 11% from smart contracts (https://hacken.io/insights/q2-2026-security-report/, 2026-07-22) [reader].
   - Chainalysis: private-key compromises were 43.8% of stolen funds in 2024 (https://www.chainalysis.com/blog/crypto-hacking-stolen-funds-2025/, 2024-12-19).
   - All three buckets mix stolen keys (out of scope for ClearSign) with deceived signers (in scope).

5. **The biggest "phishing" losses are key or account takeovers, not signing deception.** CertiK H1 2026 phishing totalled $366.3M (CertiK H1 2026 PDF, URL above, 2026-07-06). Within that:
   - social engineering: $310.1M across 4 incidents, including one $284.8M victim on 2026-01-10
   - address poisoning: $15.6M across 17 incidents
   - wallet drainers: $11.4M across 27 incidents

6. **Address poisoning is a large, separate problem.** The victim sends to a lookalike address they chose themselves.
   - $50M, December 2025 (https://www.blockaid.io/blog/address-poisoning-the-growing-threat-draining-millions-from-crypto-users, 2026-03-09)
   - $12.25M, January 2026 (https://decrypt.co/357450/signature-phishing-up-200-as-january-losses-pass-6m, 2026-02-09) [secondary]
   - At least $83.8M across 6,633 incidents on Ethereum and BSC, Jul 2022 to Jun 2024 (https://arxiv.org/abs/2501.16681, 2025-01-28)
   - Only 3 of 53 wallets warn when sending to a poisoned address (https://arxiv.org/abs/2508.12107, 2025-08-16).

7. **EIP-7702 "phishing" mostly does not involve a dapp-requested authorization.**
   - The two named cases checked on-chain ($146,551 on Ethereum; about $1.54M on Base) were ordinary type-2 transactions. Each was sent to the victim's own address and called `execute(bytes32,bytes)` on MetaMask's delegator, with a batch of approvals or transfers inside. Checked by RPC on 2026-10-05: https://etherscan.io/tx/0x1ddc8cecbcaad5396fdf59ff8cddd8edd15f82f1e26779e581b7a4785a5f5e06 and https://basescan.org/tx/0xa984840a3a6322fc0a51650a71870bc493bf1d7e1ca15945a82c37c603139bba.
   - Raw delegations to sweeper contracts are, per Wintermute, signed with keys that had already leaked (https://x.com/wintermute_t/status/1928501765865091400, 2025-05-30, via fxtwitter).
   - For ClearSign, this means the decoder must open the inner batch of a delegated-account `execute` call. Otherwise the display shows only a call to your own address.

8. **DeFiLlama's hack list does not cover the retail side** [computed]. None of the largest 2024–2026 retail cases appear in it: the $55M setOwner, the $32.5M September 2024 case, the May 2024 WBTC poisoning, and the $50M and $12.25M poisonings (https://api.llama.fi/hacks, retrieved 2026-10-05).

---

## 1. Retail wallet drainers and signature phishing

### 1.1 Totals (Scam Sniffer: EVM chains, phishing-site drainers only)

Scam Sniffer's stated scope: "Wallet drainer attacks via phishing websites only; excludes direct hacks, exchange compromises, smart contract exploits". Address poisoning is not included. (Scam Sniffer 2025 report, URL below.)

| Period | Losses | Victims | Largest single theft (what was signed) | Cases ≥$1M | Source (published) |
|---|---|---|---|---|---|
| 2023 | ~$295M | ~324,000 | $24.05M (Increase Allowance) | 13 | https://drops.scamsniffer.io/scam-sniffer-2023-crypto-phishing-scams-drain-300-million-from-320000-users/ (2024-01-01). The count of 13 comes from the 2025 report's comparison table. |
| H1 2024 | $314M | ~260,000 | $11M | 20 victims lost $58M | https://crypto.news/crypto-scammers-stole-nearly-60m-from-20-victims-in-h1-2024-data-shows/ (2024-07-05) [secondary; cites Scam Sniffer X thread https://twitter.com/realScamSniffer/status/1809089851859611908] |
| 2024 | $494M (+67%) | 332,000 addresses (+3.7%) | $55.48M (setOwner on a DSProxy; DAI) | 30, totalling $171M | https://drops.scamsniffer.io/scam-sniffer-2024-web3-phishing-attacks-wallet-drainers-drain-494-million/ (2025-01-03) |
| H1 2025 | $39.72M [computed, Q1+Q2] | 43,628 [computed] | $3.13M (increaseApproval, WBTC, May) | — | Scam Sniffer 2025 report (2026-01-03). SlowMist reproduces "~$39.73 million in losses across 43,628 victim addresses": https://slowmist.medium.com/slowmist-2025-mid-year-blockchain-security-and-aml-report-3dfc535971fb (2025-07-02) [reader] |
| 2025 | $83.85M (−83%) | 106,106 (−68%) | $6.5M (Permit; stETH + aEthWBTC; September) | 11, totalling $22.98M (27%) | https://drops.scamsniffer.io/scam-sniffer-2025-crypto-phishing-losses-fall-83-to-84-million/ (2026-01-03) |
| Jan 2026 | $6.27M (+207% vs Dec 2025) | 4,741 | $3.02M (permit + increaseAllowance; SLV/XAUt tokens) | two wallets ≈65% of losses | https://decrypt.co/357450/signature-phishing-up-200-as-january-losses-pass-6m (2026-02-09) and https://cointelegraph.com/news/over-62m-lost-address-poisoning-since-december-scam-sniffer (2026-02-09) [secondary; both cite Scam Sniffer X post https://twitter.com/realScamSniffer/status/2020343088523407836, 2026-02-08] |
| H1 2026 / full 2026 | NOT FOUND | NOT FOUND | — | — | drops.scamsniffer.io lists no 2026 report after the 2025 annual (checked 2026-10-05). Later monthly reports are posted on X and could not be located without search. |

Quarterly and monthly detail:

- **2024** (Scam Sniffer 2024 report, URL above, 2025-01-03)
  - Q1: $187.2M, 175,000 victims
  - March (peak month): $75.2M
  - Q2+Q3: $257M, 90,000 victims
  - Q4: $51M, 30,000 victims
  - Largest large-case months: $55.48M in August and $32.51M in September
  - The quarterly victim counts sum to 295,000, against 332,000 for the year [computed]. The report does not explain the difference.
- **Revisions to monthly reports**
  - The March 2024 monthly report gave $71M and a Q1 total of $173M (https://drops.scamsniffer.io/71-million-stolen-due-to-phishing-in-march/, 2024-04-02). The annual report later gives $75.2M and $187.2M.
  - January 2024: about $55M, about 40,000 victims, and $17M lost by the top 7 victims. All top-7 victims signed ERC20 Permit and/or increaseAllowance, mostly to CREATE2 addresses (https://drops.scamsniffer.io/scamsniffer-report-over-55-million-stolen-due-to-crypto-phishing-in-january/, 2024-02-09).
- **2025** (Scam Sniffer 2025 report, URL above, 2026-01-03)
  - Q1: $21.94M (22,654 victims)
  - Q2: $17.78M (20,974)
  - Q3: $31.04M (39,886); the yearly high
  - Q4: $13.09M (22,592)
  - Monthly range: $2.04M (December) to $12.17M (August)
  - August 2025: "$12.17M | 15,230 victims … Sharp escalation driven by EIP-7702 batch-signature scams and direct transfers to phishing contracts. 3 whale hits totaled $5.62M (46%)." (https://x.com/realscamsniffer/status/1964235400593178793, 2025-09-06, via fxtwitter)

### 1.2 What victims signed

**2023: the 13 largest victims** (table in the Scam Sniffer 2023 report, URL above, 2024-01-01; cumulative "$50 million")

| Signed item | Victims | Losses |
|---|---|---|
| On-chain approvals (Approve, Increase Allowance, Increase Approval) | 6 | $39.01M [computed] |
| Off-chain ERC-20 Permit or Uniswap Permit2 | 5 | $9.93M [computed] |
| Permit plus Approve | 1 | $1.49M |
| ClaimRewards | 1 | $1.03M |

- On-chain approvals were about 76% of the 13 victims' $51.46M table total [computed].
- The largest 2023 victim lost $24.05M after signing Increase Allowance. The case is from September 2023 and was attributed to MS Drainer, which stole about $58.98M from about 63,210 victims in nine months (https://drops.scamsniffer.io/from-google-to-x-ads-tracing-the-crypto-wallet-drainers-58-million-trail/, 2023-12-21).

**2024: large cases (≥$1M), share of losses** (Scam Sniffer 2024 report, 2025-01-03)

| Signature | Share |
|---|---|
| Permit | 56.7% |
| setOwner (Proxy ownership change) | 31.9% |
| Transfer | 4.5% |
| increaseAllowance | 3.5% |
| Others | 3.4% |

- 25 of the large cases (85.3%) were on Ethereum, with $152M lost.
- Assets in large cases: Staking & Restaking 40.9%; Stablecoin 33.5%; Aave collateral 10.7%; Pendle yield 9.3%.
- SlowMist's 2024 report repeats these shares: "Permit signatures remain the primary method for phishing attacks, accounting for 56.7%." and "In one incident in August, a setOwner phishing signature led to a victim losing $55 million in DAI." (https://slowmist.medium.com/analysis-of-the-2024-blockchain-security-and-anti-money-laundering-annual-report-phishing-and-scam-503f018c7c4c, 2025-01-10) [reader]

**2025: all 11 cases ≥$1M** (Scam Sniffer 2025 report, 2026-01-03)

| Amount | Asset | Month | Signed |
|---|---|---|---|
| $6.50M | stETH, aEthWBTC | Sep | Permit |
| $3.13M | WBTC | May | increaseApproval |
| $3.05M | aEthUSDT | Aug | Transfer |
| $1.82M | cUSDCv3 | Mar | Transfer |
| $1.54M | Multiple | Aug | EIP-7702 batch |
| $1.43M | Multiple | Apr | Approve |
| $1.23M | Uniswap V3 NFTs | Jul | setApprovalForAll |
| $1.22M | aPlaUSDT0 | Nov | Permit |
| $1.06M | sUSDf/USDe | Jul | Approve |
| $1.00M | Multiple | Aug | EIP-7702 batch |
| $1.00M | RLB | Jan | Uniswap Permit2 |

**January 2026** (Decrypt, 2026-02-09) [secondary]: $3.02M via "a permit and increaseAllowance attack"; $1.08M "drained via a permit attack".

**Signature types not broken out in the 2023–2025 Scam Sniffer reports:** eth_sign; Seaport, Blur or other marketplace orders; address poisoning. The MS Drainer report mentions "malicious signatures using Blur for phishing" as a paid add-on module, with no figure (Scam Sniffer MS Drainer report, 2023-12-21).

**Payload formats behind these labels.** This is interpretation, based on the call names above:
- setOwner on a DSProxy, transfer, approve, increaseAllowance and setApprovalForAll are ordinary on-chain transactions.
- Permit and Permit2 are off-chain EIP-712 typed-data signatures.
- The 2025 "EIP-7702 batch" cases were type-2 transactions calling the victim's own delegated account (§4.2).

### 1.3 Independent cross-checks on retail losses

These sources use different definitions; read each figure inside its own scope.

- **Chainalysis, approval phishing:** "$516.8 million" in 2022 and "$374.6 million in 2023 through November"; "approximately $1.0 billion … since … May 2021" (https://www.chainalysis.com/blog/approval-phishing-cryptocurrency-scams-2023/, 2023-12-14). This covers targeted, pig-butchering-style approval scams and overlaps partly with drainers.
- **Chainalysis, drainers in 2024:** "nearly 170% YoY revenue growth" (https://www.chainalysis.com/blog/2024-pig-butchering-scam-revenue-grows-yoy/, 2025-02-13).
- **Chainalysis, all personal-wallet compromises** (key theft and signing deception combined) (https://www.chainalysis.com/blog/crypto-hacking-stolen-funds-2026/, 2025-12-18):
  - 158,000 incidents in 2025 (54,000 in 2022)
  - at least 80,000 unique victims (40,000 in 2022)
  - $713M stolen in 2025, against $1.5B in 2024
  - share of all value stolen: 20% in 2025, 44% in 2024, 37% in 2025 excluding the Bybit effect
- **CertiK H1 2026, phishing sub-types** (CertiK H1 2026 PDF, 2026-07-06):
  - wallet drainers: 27 incidents, $11.4M
  - address poisoning: 17 incidents, $15.6M
  - wallet-compromise-type phishing: 11 incidents, $28,474,825
  - social engineering: 4 incidents, $310,107,510
- **Check Point, Inferno Drainer:** "over 30,000 users … losses exceeding $9 million" between September 2024 and March 2025; "over $250 million" stolen over its lifetime as of May 2024 (https://research.checkpoint.com/2025/inferno-drainer-reloaded-deep-dive-into-the-return-of-the-most-sophisticated-crypto-drainer/, 2025-05-07).
- **PeckShield 2025:** "Social Engineering (12%): User-targeted attacks via phishing, permit signature manipulation, & impersonation", out of more than $4.04B total (https://x.com/PeckShieldAlert/status/2010960699766563200 and infographic https://pbs.twimg.com/media/G-hdbzfbUAAm1W5.jpg, 2026-01-13).
- **Hacken:** its phishing figures are not independent. Its 2024 phishing figure of $607.5M "was primarily sourced from ScamSniffer reports" (https://assets.hacken.io/assets/2024/12/2024-Web3-Security-Report.pdf, 2024-12-24).

### 1.4 "Phishing" in industry reports is mostly not signature phishing

The largest items in the phishing buckets are seed, account or 2FA takeovers. For these ClearSign has nothing to review.

- **CertiK 2024** phishing: $1,050,129,498 across 296 incidents (https://www.certik.com/resources/blog/hack3d-the-web3-security-report-2024, 2025-01-02). The bucket includes:
  - DMM Bitcoin (~$304M), which CertiK called "likely the result of address poisoning". The FBI instead says attackers manipulated "a legitimate transaction request by a DMM employee"; see §5.
  - a $243M social-engineering theft from a Genesis creditor (fake Google/Gemini support; 2FA reset)
  - a $129M address poisoning, returned within an hour

  Source for these three: https://indd.adobe.com/view/publication/733ec833-8311-4c15-b27a-96561ccdef59/wiic/publication-web-resources/pdf/2024_Q4__Hack3d_Report.pdf (2025-01-02).
- **April 2025:** "a scammer tricked one person into handing over $330 million worth of Bitcoin" (https://storage.ghost.io/c/d1/14/d114cb3d-6b95-4710-8c36-fdcbff0ed991/content/files/2025/07/hacken-2025-h1-web3-security-report-1.pdf, 2025-07-24). CertiK's 2025 top 10 lists phishing victims of $330,700,000, $91,127,110 and $49,982,750.02 (https://indd.adobe.com/view/publication/d21da0b0-06c4-4f38-a82b-c7757971064b/jh85/publication-web-resources/pdf/2025-eoy-skynet-hack3d-report.pdf, 2025-12-23).
- **2026-01-10:** "an unidentified victim lost $284,785,689 across multiple chains in a social engineering attack". This was 82% of Q1 2026 phishing losses (CertiK H1 2026 PDF, 2026-07-06). Hacken describes it as "a single $282 million hardware-wallet social-engineering attack" (https://assets.hacken.io/assets/q1-2026-security-report.pdf, 2026-04-14).

### 1.5 Address poisoning (deceived about the counterparty)

| Figure | Source (published) |
|---|---|
| 270M on-chain attacks targeting 17M victims; 6,633 incidents; at least $83.8M lost. Ethereum and BSC, 2022-07-01 to 2024-06-30. Excludes the $68M WBTC case. | https://arxiv.org/abs/2501.16681 (2025-01-28; USENIX Security 2025) |
| One 2024 campaign: 82,031 seeded addresses; 2,774 victim addresses sent $69,720,993. "Nearly cost an unknown crypto whale $68 million in wrapped bitcoin (WBTC)" (May 2024; returned). Median 2024 campaign take about $400. | https://www.chainalysis.com/blog/address-poisoning-scam/ (2024-10-23) |
| $129M address poisoning, 2024-11-20, returned within an hour | CertiK 2024 PDF (2025-01-02) |
| 65.4M poisoning transactions flagged since January 2025; about 316,000 confirmed (about 1 in 200 attempts succeeds); attempts rose from 628,000 (Nov 2025) to 3.4M (Jan 2026); December 2025 victim sent 49,999,950 USDT; about 15,000 malicious Safe proxy addresses in one campaign | https://www.blockaid.io/blog/address-poisoning-the-growing-threat-draining-millions-from-crypto-users (2026-03-09) |
| $12.2M (≈$12.25M; 4,556 ETH) in January 2026 and $50M in December 2025, both from copying an address out of transaction history | https://cointelegraph.com/news/over-62m-lost-address-poisoning-since-december-scam-sniffer (2026-02-09) [secondary, Scam Sniffer] |
| Ethereum poisoning losses $4.9M in the 73 days before Fusaka (Sep 21 – Dec 2, 2025) vs $63.3M in the 73 days after (Dec 3, 2025 – Feb 13, 2026); dust transactions rose from 30,000 to 167,000 per day | https://sergeenkov.com/fusaka-poison-attacks-ethereum/ (2026-02-18; independent researcher) |
| CertiK H1 2026: 17 incidents, $15.6M | CertiK H1 2026 PDF (2026-07-06) |
| Hacken Q1 2026: one "$24 million address-poisoning theft" | https://assets.hacken.io/assets/q1-2026-security-report.pdf (2026-04-14) |
| Safe multisigs: "about 10 Safe wallets have lost $2.05 million in the past week"; the same attacker took "$5.05 million from 21 victims in the past 4 months" | https://drops.scamsniffer.io/multiple-safe-wallets-lose-2-million-to-address-poisoning-attacks/ (2023-12-03) |
| Only 3 of 53 popular Ethereum wallets warn when sending to a poisoned address; 16 were rated high risk | https://arxiv.org/abs/2508.12107 (2025-08-16) |

Yearly address-poisoning totals for 2024 and 2025 from Scam Sniffer, Chainalysis or CertiK: NOT FOUND.

### 1.6 Simulation alone is not a guarantee

In a "transaction simulation spoofing" case, the chain state changed about 30 seconds between the wallet's simulation and the user's signature. The victim lost "143.45 ETH (approximately $460,895)" (https://drops.scamsniffer.io/transaction-simulation-spoofing-a-new-threat-in-web3/, 2025-01-10). Tx hash given by Scam Sniffer: 0x014321fbace3c22ade53fd34a81981c92b499451e70bc840f567bc22c95de700.

---

## 2. Chainalysis

| Report (published) | Figure |
|---|---|
| 2024 Crypto Crime Report, hacking chapter (https://www.chainalysis.com/blog/crypto-hacking-stolen-funds-2024/, 2024-01-24) | 2023 stolen: $1.7B (−54.3%; 2022: $3.7B) across 231 incidents (later restated as 282). DeFi: $1.1B. DPRK: "slightly over $1.0 billion", 20 hacks (later revised to $660.50M). Halborn data on the top-50 DeFi hacks: compromised private keys 47.8% of losses (up from 22.0%); smart-contract bugs 18.2% (down from 47.0%); price manipulation about 20%. DPRK share in percent: NOT FOUND. CeFi/DeFi split: chart only, NOT FOUND in text. |
| 2024 mid-year update (https://www.chainalysis.com/blog/2024-crypto-crime-mid-year-update-part-1/, 2024-08-15) | January to end of July 2024: $1.58B stolen (same period 2023: $857M). Average per incident $10.6M (+79.46%). DMM ≈19% of value hacked. DPRK amount and private-key share: NOT FOUND. |
| 2025 Crypto Crime Report, hacking chapter (https://www.chainalysis.com/blog/crypto-hacking-stolen-funds-2025/, 2024-12-19) | 2024 stolen: $2.2B (+21.07%), 303 incidents. **Private key compromises: 43.8% of stolen crypto.** DPRK: $1.34B across 47 incidents, 61% of value and 20% of incidents. CeFi/DeFi by quarter: chart only (centralized services most targeted in Q2 and Q3). |
| 2025 mid-year update (https://www.chainalysis.com/blog/2025-crypto-crime-mid-year-update/, 2025-07-17) | More than $2.17B stolen from services in 2025 to date. Bybit ≈69% of service losses. Personal-wallet compromises 23.35% of all stolen funds to date. Personal-wallet victim counts: NOT FOUND. |
| Bybit post (https://www.chainalysis.com/blog/bybit-exchange-hack-february-2025-crypto-security-dprk/, 2025-02-24) | "During what appeared to be a routine transfer from Bybit's Ethereum cold wallet to a hot wallet, Bybit unknowingly signed the malicious transaction, enabling the attackers to move approximately 401,000 ETH". |
| 2026 Crypto Crime Report, hacking chapter (https://www.chainalysis.com/blog/crypto-hacking-stolen-funds-2026/, 2025-12-18) | 2025: more than $3.4B stolen, January to early December (Bybit $1.5B). DPRK at least $2.02B (+51%), 76% of service compromises. Top 3 hacks 69% of service losses. Personal wallets: 158,000 incidents; at least 80,000 victims; $713M (2024: $1.5B); 20% of value (2024: 44%). CeFi: attacks on "private key infrastructure and signing processes" were "88% of losses in Q1 2025", and attackers "trick legitimate signers into authorizing malicious transactions". Venus user: manipulated into "granting delegate status over a $13 million account". Private-key vs code split and number of 2025 service hacks: NOT FOUND. |
| 2026 mid-year update | NOT FOUND. Not published as of 2026-10-01 per Chainalysis's RSS feed and sitemap; /blog/2026-crypto-crime-mid-year-update/ returns 404. |
| 2026 incident posts | Drift: $285M; "at least two council members signed transactions they did not fully understand (a classic case of blind signing)" (https://www.chainalysis.com/blog/lessons-from-the-drift-hack/, 2026-04-09). KelpDAO: ~$292M via compromised RPC nodes and a 1-of-1 verifier; not a signer case (https://www.chainalysis.com/blog/kelpdao-bridge-exploit-april-2026/, 2026-04-23). Bitget: $387M; DPRK's 2026 total so far passes $1B (https://www.chainalysis.com/blog/387m-bitget-theft-2026/, 2026-10-01). Wrench attacks: $58M in 2025 and $30M in H1 2026 (https://www.chainalysis.com/blog/violent-crypto-wrench-attacks-2026/, 2026-08-06). |
| Scam revenue | 2024: at least $9.9B, later revised to $12B (https://www.chainalysis.com/blog/2024-pig-butchering-scam-revenue-grows-yoy/, 2025-02-13). 2025: at least $14B, projected above $17B (https://www.chainalysis.com/blog/crypto-scams-2026/, 2026-01-13). |

Revisions to flag:

- DPRK 2023 was cut from $1.0B to $660.50M. Chainalysis explains this on the 2024-12-19 page.
- The 2023 incident count went from 231 to 282, with no explanation.
- Chainalysis's 2024 total including personal wallets must exceed $2.2B: $1.5B taken from individuals was described as 44% of all stolen value. The revised 2024 total itself is NOT FOUND.

---

## 3. Security-firm reports: losses by vector

No firm has a category for "a legitimate signer was misled about what they signed". Deceived-signer cases sit inside buckets that also hold stolen keys and infrastructure breaches. The same incident is classified differently across firms and editions (§3.7).

### 3.1 CertiK (Hack3d)

| Period | Total | Phishing | Wallet / private-key compromise | Code vulnerability | Other | Source (published) |
|---|---|---|---|---|---|---|
| 2024 | $2,362,748,975.83 / 760 incidents (restated in 2025 as $2,446,285,251) | $1,050,129,498 / 296 ("nearly half" of value; 39.1% of incidents); $836,801,668 after $213,327,829 returned | Private key compromise: $855,385,570 / 65 | $170.9M / 218 (chart) | Access control $86.4M / 38; exit scam $85.4M / 91; price manipulation $50.4M / 33 (chart) | https://www.certik.com/resources/blog/hack3d-the-web3-security-report-2024 and PDF https://indd.adobe.com/view/publication/733ec833-8311-4c15-b27a-96561ccdef59/wiic/publication-web-resources/pdf/2024_Q4__Hack3d_Report.pdf (2025-01-02) |
| H1 2025 | $2,472,777,618 / 344 | $410,747,038 / 132 | Wallet compromise (incl. Bybit): $1,706,937,700 / 34 | Q2 only: $235,783,844 / 47 | — | https://www.certik.com/resources/blog/hack3d-the-web3-security-quarterly-report-q2-h1-2025 (2025-06-30) |
| 2025 | $3,352,850,816 / 630 | $722,885,398 / 248 (most incidents of any vector) | — (Bybit filed under "Supply Chain": $1,450,914,902 / 2) | $554,646,929 / 240 (>47% frozen or returned) | — | https://www.certik.com/resources/blog/hack3d-the-web3-security-report-2025 and PDF https://indd.adobe.com/view/publication/d21da0b0-06c4-4f38-a82b-c7757971064b/jh85/publication-web-resources/pdf/2025-eoy-skynet-hack3d-report.pdf (2025-12-23) |
| H1 2026 | $1,315,676,432 / 344 | $366,312,027 / 63 (incidents −52.3%, losses −10.8% vs H1 2025). Q1: $347,286,540 / 37 (68.3% of Q1). Q2: $19,025,487 / 26 | Wallet compromise: $444,531,691 / 33 | $151,591,472 / 204 | Kelp DAO RPC compromise $291.3M and Drift $285.3M ≈44% of H1 | https://www.certik.com/skynet-report/certik-hack3d-h1-2026-report and PDF https://indd.adobe.com/view/publication/9f1c777d-8d9f-4b14-b417-e4a29ee5359a/931z/publication-web-resources/pdf/English.pdf (2026-07-06) |

CertiK's own words:

- On Bybit: "the attack exploited the user interface of the Safe{Wallet} system, leading signers to unknowingly approve malicious transactions."
- On phishing: "Phishing attacks succeed through signature prompts disguised as routine approvals."
- On scope: the dataset "does not include pig butchering schemes, wrench attacks, high-pressure investment scams, or social engineering fraud conducted entirely off-chain".

All three quotes are from the CertiK 2025 PDF (2025-12-23). A Q3 2025 or Q3 2026 report: NOT FOUND.

### 3.2 TRM Labs

| Period | Total | Operational / infrastructure | Code | Source (published) |
|---|---|---|---|---|
| 2024 | $2.2B (+17%) | "Infrastructure attacks (primarily private key and seed phrase compromises) accounted for nearly 70% of stolen funds" | NOT FOUND | https://www.trmlabs.com/resources/blog/category-deep-dive-2-2-billion-was-stolen-in-crypto-related-hacks-in-2024 (2025-03-17) |
| H1 2025 | More than $2.1B, at least 75 hacks (later restated to ~$2.3B) | Infrastructure (keys, seed phrases, front-end compromises): more than 80% | Protocol exploits: 12% | https://www.trmlabs.com/resources/blog/h1-2025-crypto-hacks-and-exploits-a-new-record-amid-evolving-threats (2025-06-26) |
| 2025 | $2.87B, nearly 150 hacks (Bybit $1.46B = 51%); DPRK $1.92B | Infrastructure attacks: $2.2B (76%), 45 incidents | Code exploits $350M (12.1%), 52 incidents; protocol attacks $277M (9.6%), 25 incidents | https://www.trmlabs.com/reports-and-whitepapers/2026-crypto-crime-report (2026-01-28) |
| H1 2026 | ~$972M, 207 hacks; DPRK ~$643M (66%) | "Infrastructure and operational compromises accounted for approximately 76% of all funds stolen, despite representing only about 15% of incidents." | Smart-contract exploits: the majority of incidents, a small share of losses | https://www.trmlabs.com/resources/blog/h1-2026-crypto-hacks-reach-record-high-as-losses-fall-below-usd-1-billion (2026-07-01) |

TRM on Drift: signers were induced "into pre-signing transactions that appeared routine but carried hidden authorizations for critical admin actions" (https://www.trmlabs.com/resources/blog/north-korean-hackers-attack-drift-protocol-in-285-million-heist, 2026-04-02).

### 3.3 Hacken

| Period | Total | Access control / operational | Phishing | Smart contracts | Source (published) |
|---|---|---|---|---|---|
| 2024 | $2,914,629,674 | "nearly $1.7 billion" ($1.721B on chart), "78% of all crypto hacks" | $607.5M (sourced from ScamSniffer) | NOT re-verified here | https://assets.hacken.io/assets/2024/12/2024-Web3-Security-Report.pdf (2024-12-24) |
| H1 2025 | $3,093,946,000 | Vector split extracted from the PDF too garbled to quote | — | — | https://storage.ghost.io/c/d1/14/d114cb3d-6b95-4710-8c36-fdcbff0ed991/content/files/2025/07/hacken-2025-h1-web3-security-report-1.pdf (2025-07-24) |
| 2025 | $4,004,090,000 | $2,123,633,000 (53.0%) | $951,577,000 (23.8%) | $512,028,000 (12.8%); rug pulls $316,852,000 (7.9%) | https://assets.hacken.io/assets/Hacken-2025-Yearly-Security-Report.pdf (2025-12-29) |
| Q1 2026 | $482,661,580, 44 incidents | $71,900,000 | $306,000,000 (the $282M social-engineering case plus a $24M address poisoning) | $86,173,580 | https://assets.hacken.io/assets/q1-2026-security-report.pdf (2026-04-14) |
| Q2 2026 | $763.9M, 67 incidents | "88.3% of the total was traced to compromised keys, signers, and infrastructure" | NOT FOUND | "about 11% of the losses" | https://hacken.io/insights/q2-2026-security-report/ (2026-07-22) [reader] |

Hacken 2025 (PDF above): "In recent years, 100% of crypto thefts attributed to North Korean actors have relied on social engineering and advanced phishing rather than smart contract exploitation."

### 3.4 Immunefi

2024, from https://immunefi.com/blog/research/immunefi-crypto-losses-2024-report/ (2025-01-01):
- $1,495,487,055 across 232 incidents.
- Hacks $1,467,448,336 (192 incidents) vs fraud $28,038,719 (40 incidents).
- DeFi $769,287,055 (51.4%) vs CeFi $726,200,000 (48.6%).
- DMM Bitcoin plus WazirX: $540M, 36% of the year's losses.
- "Losses have primarily been driven by larger-scale exploits, often resulting from private key compromises."

Dollar figures by vector: NOT FOUND (Immunefi does not publish them). A 2025 annual "Crypto Losses" report: NOT FOUND. The research agent reported that the series ended in April 2025.

### 3.5 PeckShield (X posts and infographics)

- **2024:** "exceeded $3.01B" = "$2.15B stolen from crypto hacks and $834.5M stolen from scams" (https://x.com/PeckShieldAlert/status/1877258501623525797, 2025-01-09, via fxtwitter).
- **2025:** "exceeded $4.04B" = "$2.67B from crypto hacks" + "$1.37B from scams" (https://x.com/PeckShieldAlert/status/2010960699766563200, 2026-01-13, via fxtwitter).
  - Infographic split: Exploits 66% ("Smart contract vulnerabilities, protocol-level flaws, & private key compromises"); Scams 22%; Social Engineering 12% (https://pbs.twimg.com/media/G-hdbzfbUAAm1W5.jpg, 2026-01-13).
  - The infographic notes: "This chart only includes crypto hacks with losses greater than $100K & scams/phishing with losses greater than $10M."

### 3.6 SlowMist

SlowMist excludes individual users from its totals and gives causes as shares of incident counts.

| Period | Total | Notes | Source (published) |
|---|---|---|---|
| 2024 | $2.013B, 410 incidents | Contract vulnerabilities: 99 incidents, ~$214M. Second most frequent cause: account compromises. "known losses from phishing signature attacks have reached $790 million" | https://slowmist.medium.com/slowmist-2024-blockchain-security-and-anti-money-laundering-annual-report-d7fc94ccf624 (2025-01-04) [reader] |
| H1 2025 | ~$2.373B, 121 incidents | Account compromise 42 cases; contract vulnerabilities 35. Drainers (ScamSniffer data): ~$39.73M, 43,628 victims | https://slowmist.medium.com/slowmist-2025-mid-year-blockchain-security-and-aml-report-3dfc535971fb (2025-07-02) [reader] |
| 2025 | ~$2.935B, 200 incidents | Exchanges: 12 incidents, $1.809B (Bybit ~$1.46B). Contract vulnerabilities 61 incidents; compromised accounts 48 | https://slowmist.medium.com/2025-blockchain-security-and-aml-annual-report-9f85183d5461 (2025-12-30) [reader] |
| H1 2026 | ~$956M, 182 incidents | Losses by cause: supply chain ~$298M (Kelp DAO ~$292M); contract and logic ~$152M (85 incidents); private key and credential ~$130M (17 incidents) | https://slowmist.medium.com/slowmist-2026-mid-year-blockchain-security-and-aml-report-75e0862179ef (2026-07-07) [reader] |

### 3.7 Cross-firm comparison

**2025 totals.** The firms use different scopes:

| Firm | 2025 total |
|---|---|
| Chainalysis | >$3.4B (to early December) |
| CertiK | $3,352,850,816 |
| TRM | $2.87B |
| Hacken | $4,004,090,000 |
| PeckShield | >$4.04B (incl. scams) |
| SlowMist | ~$2.935B |

Sources are given in the sections above.

**Operational vs code shares for 2025 and 2026:**

| Report | Operational | Code |
|---|---|---|
| TRM 2025 | infrastructure 76% | code exploits 12.1% |
| TRM H1 2026 | 76% | — |
| Hacken 2025 | access control 53.0% + phishing 23.8% | smart contracts 12.8% |
| Hacken Q2 2026 | 88.3% | about 11% |
| PeckShield 2025 | — | "Exploits" 66% (mixes contract bugs and private keys) |

**How each firm files Bybit:**

| Firm | Category |
|---|---|
| CertiK | "wallet compromise" in H1 2025, then "Supply Chain" in the 2025 annual |
| TRM | infrastructure attack |
| Hacken | access control |
| SlowMist | Bybit is part of the "Exchange" total of $1.809B |

---

## 4. EIP-7702 delegation phishing since Pectra (mainnet activation 2025-05-07)

### 4.1 Figures

| Figure | Source (published) |
|---|---|
| "over 97% of all EIP-7702 delegations were authorized to multiple contracts using the same exact code. These are sweepers, used to automatically drain incoming ETH from compromised addresses." … "mostly delegate contracts designed to auto-sweep funds from EOAs with leaked private keys." (Ethereum, 2025-05-07 to 05-30) | https://x.com/wintermute_t/status/1928501765865091400 (2025-05-30, via fxtwitter) |
| 1,580,930 EIP-7702 activations on Ethereum since May 7; 768,275 (48%) tagged crime-related; 6,285 per day (0.37% of ETH transactions) | https://protos.com/48-of-ethereum-eip-7702-uses-linked-to-crime-says-wintermute/ (2025-09-22) [secondary; Wintermute Dune] |
| Seven EVM chains to 2025-07-15: 3,664,166 authorization transactions, of which 2,322,548 (63%) were associated with malicious contracts. Malicious contracts confirmed: 793 EOA-targeted, 124 contract-targeted, 7 composite (924 in all [computed]). Total asset loss $2,362,848. One contract family was authorized by more than 100,000 victim accounts. | https://www.usenix.org/system/files/conference/usenixsecurity26/sec26_prepub_huang-mingyuan.pdf (USENIX Security 2026 prepublication; exact date NOT FOUND) |
| More than 150k authorization and execution events, 26k addresses, hundreds of delegator contracts; authorizations "dominated by a small number of contract families linked to criminal activity" | https://arxiv.org/abs/2512.12174 (2025-12-13) |
| Two large EIP-7702 batch cases in August 2025: $1.54M and $1.00M ($2.54M total) | https://drops.scamsniffer.io/scam-sniffer-2025-crypto-phishing-losses-fall-83-to-84-million/ (2026-01-03) |
| "An address upgraded to EIP-7702 lost $146,551 through malicious batched transactions in phishing attack." | https://x.com/realScamSniffer/status/1926296681198326254 (2025-05-24, via fxtwitter) |
| "Someone lost ~$1.54M due to signing EIP-7702 phishing batch transactions." | https://x.com/realScamSniffer/status/1959423000752820374 (2025-08-24, via fxtwitter) |
| The May 2025 case was attributed to Inferno Drainer: "the delegated address is not a phishing address, but MetaMask: EIP-7702 Delegator 0x63c0c19a…E32B" | https://x.com/SlowMist_Team/status/1927279062822641772 (2025-05-27, via fxtwitter) |
| Adoption across chains: 63,328,609 live smart accounts; 261,266,473 authorizations; 109,861,042 set-code transactions | https://www.bundlebear.com/eip7702-overview/all (live dashboard, read 2026-10-05) |

### 4.2 Mechanism: what the victim actually signed (on-chain check, 2026-10-05)

- **$146,551 case**, Ethereum tx 0x1ddc8cecbcaad5396fdf59ff8cddd8edd15f82f1e26779e581b7a4785a5f5e06, block 22546176.
  - Type-2 transaction; from and to are both the victim, 0xc6d289d55fe64227a09e3120855ccba0d2e606dc.
  - Input selector 0xe9ae5c53 = `execute(bytes32,bytes)`. No authorization list.
  - The receipt shows 10 Approval events, all to spender 0x00008c22f9f6f3101533f520e229bbb54be90000.
  - The account's current code is `0xef0100‖63c0c19a282a1b52b07dd5a65b58948a07dae32b` (MetaMask delegator).
- **~$1.54M case**, Base tx 0xa984840a3a6322fc0a51650a71870bc493bf1d7e1ca15945a82c37c603139bba, block 34588154.
  - Type-2 self-call `execute(bytes32,bytes)` from 0x96892b8fbd24e0ac40bd98a304095a6c485a6a68.
  - 15 Transfer events, 0 approvals. Recipients include 0x0000b3812f3912259374da1329ca1eecef370000.
  - The account is delegated to the same MetaMask delegator.
- **Limit of the check:** archive code at the time of each attack could not be read. "latest" shows only today's delegation.

The specification says wallets must not let applications request authorizations: "There is no safe way to provide this interface. The code specified by an authorization has unrestricted access to the account and must always be closely audited by the wallet." (https://raw.githubusercontent.com/ethereum/EIPs/master/EIPS/eip-7702.md; EIP created 2024-05-07; status Final).

**Conflicting view.** Huang et al. write that malicious authorizations "typically occurred after users interacted with phishing websites or copied transaction data from untrusted channels" (USENIX paper, above). Wintermute attributes sweeper delegations to already-leaked keys (above).

**Not found:**
- any named 2026 EIP-7702 phishing case
- any wallet that lets dapps request raw authorizations
- Wintermute Dune figures after September 2025 (the dashboard renders only with JavaScript)

---

## 5. Multisig and Safe signer deception

### 5.1 Cases

| Case | Date | Loss | What happened / what was signed | Source (published) |
|---|---|---|---|---|
| Safe multisigs (address poisoning) | Nov 2023 | $2.05M (about 10 Safes, one week); $5.05M from 21 victims over 4 months | Lookalike CREATE2 addresses planted in Safe history; signers approved transfers to them | https://drops.scamsniffer.io/multiple-safe-wallets-lose-2-million-to-address-poisoning-attacks/ (2023-12-03) |
| DMM Bitcoin | late May 2024 | $308M (4,502.9 BTC) | A wallet-provider (Ginco) employee was compromised via a fake recruiter. The attackers "manipulate[d] a legitimate transaction request by a DMM employee" | https://www.fbi.gov/news/press-releases/fbi-dc3-and-npa-identification-of-north-korean-cyber-actors-tracked-as-tradertraitor-responsible-for-theft-of-308-million-from-bitcoindmmcom (2024-12-23) [reader] |
| WazirX | 2024-07-18 | more than $230M | 6 signatories (5 WazirX, 1 Liminal). "a discrepancy between the data displayed on Liminal's interface and the transaction's actual contents". The payload is suspected to have been replaced "to transfer wallet control to an attacker" | https://wazirx.com/blog/preliminary-report-cyber-attack-on-wazirx-multisig-wallet/ (2024-07-18) |
| Radiant Capital | 2024-10-16 | ~$50M | Malware on the devices of "at least three core contributors". The Safe front-end showed legitimate transactions while hardware wallets signed `transferOwnership`. "Each transaction was simulated for accuracy on Tenderly and individually reviewed by multiple developers at each signature stage" | https://medium.com/@RadiantCapital/radiant-post-mortem-fecd6cd38081 (2024-10-18) [reader] |
| Bybit | 2025-02-21 | more than $1.4B incl. 401,347 ETH (Verichains); ~$1.5B (Chainalysis) | Safe{Wallet} JavaScript tampered with via a compromised developer machine. Three signers, including the CEO, approved a transaction that "upgraded Bybit's multi-signature contract for Cold Wallet 1 … pointing to a malicious contract"; the injected code set operation=1 (delegatecall). Sygnia: "The Safe{Wallet} user interface displayed legitimate transaction data while malicious transactions were signed". Safe: "compromised Safe{Wallet} developer machine resulting in the proposal of a disguised malicious transaction" | https://coinacademy.fr/wp-content/uploads/2025/02/Bybit-Incident-Investigation-Report.pdf (2025-02-24; mirror of the Verichains report); https://www.sygnia.co/blog/sygnia-investigation-bybit-hack/ (2025-03-16); https://safefoundation.org/blog/safe-ecosystem-foundation-statement (2025-02-28) |
| SwissBorg / Kiln | 2025-09-08 | ~$42M (~192,600 SOL) | Kiln API compromised. An approved "standard unstaking transaction" concealed "eight authorization instructions" that transferred stake authority to the attacker | https://www.halborn.com/blog/post/explained-the-swissborg-hack-september-2025 (2025-09-15) |
| Venus Protocol user (single user, not a multisig) | 2025-09 | $13M account | Compromised Zoom client; user manipulated "into granting delegate status" | https://www.chainalysis.com/blog/crypto-hacking-stolen-funds-2026/ (2025-12-18) |
| Drift (Solana) | 2026-04-01 | ~$285M (DeFiLlama: $295M) | At least 2 of 5 Security Council signers blind-signed durable-nonce transactions that transferred admin control. Zero timelock | https://www.chainalysis.com/blog/lessons-from-the-drift-hack/ (2026-04-09); https://www.trmlabs.com/resources/blog/north-korean-hackers-attack-drift-protocol-in-285-million-heist (2026-04-02) |
| SUPERFORTUNE AI | 2026-05-27 | $15.18M | A multisig airdrop transfer's destination was replaced with a lookalike address despite "various address verification controls". Root cause unconfirmed | https://www.halborn.com/blog/post/explained-the-superfortune-ai-hack-may-2026 (2026-06-01) |
| Bitget (no human signer) | 2026-09-24 | $351.6M (TRM); ~$388M (Bitget) | A backend system was compromised. It "spoofed transaction data, and triggered that approval process", which was an automated approval. Keys were not stolen, per Bitget | https://www.trmlabs.com/resources/blog/bitget-loses-usd-3516-million-in-hot-wallet-breach-in-likely-north-korea-attack (2026-09-25) |

Cases the research agent checked and judged **not** to be signer deception (sources it cited: Halborn, SlowMist Hacked DB, Chainalysis, TRM; **not re-verified here**):
- UXLINK, 2025-09: keys stolen from Safe key holders' devices after deepfake social engineering
- Step Finance, 2026-01: executive devices compromised
- Infini, 2025-02: insider
- UPCX, 2025-04, and Zoth, 2025-03: admin keys
- Resolv, 2026-03: AWS KMS signing key
- KelpDAO, 2026-04: RPC/DVN
- Liquid Network, 2026-09: software bug

### 5.2 How often (from DeFiLlama data) [computed]

DeFiLlama labels only 6 of its 1,293 records "Signer Phishing" or "Blind Signing", totalling $1,687,090,000. They are Llamascape 2022, PeopleDAO 2023, WazirX 2024, Radiant 2024, Bybit 2025 and OlaXBT 2025 (https://api.llama.fi/hacks, retrieved 2026-10-05).

Other signer cases carry other labels:

| Case | DeFiLlama label |
|---|---|
| Drift | "Proxy Upgrade Hijack" |
| SwissBorg | "API Key Compromised" |
| SUPERFORTUNE AI | "Improper Access Control" |
| DMM Bitcoin | "Phishing" |

Re-classifying from the post-mortems above and using DeFiLlama's amounts:

| Year | Signer-deceived incidents | Signer-deceived total | DeFiLlama total | Share |
|---|---|---|---|---|
| 2024 | DMM, WazirX, Radiant | $589.9M | $1,669,697,215 | **35.3%** |
| 2025 | Bybit, SwissBorg, OlaXBT | $1,443.5M | $2,715,021,619 | **53.2%** |
| 2026 to 2026-10-01 | Drift, SUPERFORTUNE AI | $310.18M | $2,242,313,291 | **13.8%** |

Caveats:
- OlaXBT ($2M) rests on DeFiLlama's label alone; no details were found.
- DMM is a deceived-signer case only by the FBI's account.

### 5.3 Industry statements and statistics

- **Chainalysis** on centralized services: "Many attackers have developed methods to exploit third-party wallet integrations and trick legitimate signers into authorizing malicious transactions." Such attacks were 88% of losses in Q1 2025 (https://www.chainalysis.com/blog/crypto-hacking-stolen-funds-2026/, 2025-12-18).
- **Hacken Q2 2026:** "88.3% of the total was traced to compromised keys, signers, and infrastructure" (https://hacken.io/insights/q2-2026-security-report/, 2026-07-22) [reader].
- **Count or share of incidents involving blind signing:** NOT FOUND from any vendor. The research agent checked Ledger, Safe, Hypernative, Cyfrin, OpenZeppelin, Trail of Bits and SEAL. Chainalysis says only that blind signing "has resulted in billions of losses", with no figure (https://www.chainalysis.com/blog/transaction-signing-and-approval-hexagate-gatesigner/, 2025-06-17).

---

## 6. DeFiLlama coverage check [computed]

From https://api.llama.fi/hacks, retrieved 2026-10-05; 1,293 records, latest dated 2026-10-01.

| Year | Total | Incidents | Largest classes |
|---|---|---|---|
| 2023 | $1,626,253,369 | 197 | — |
| 2024 | $1,669,697,215 | 217 | Social Engineering 38.1%; Key Compromise 30.9% |
| 2025 | $2,715,021,619 | 148 | Social Engineering 56.9% ($1,546,200,000, 8 incidents, incl. Bybit $1.4B) |
| 2026 to 10-01 | $2,242,313,291 | 286 | Key Compromise 31.4%; Bridge & Cross-Chain 31.2%; Access Control 16.5% |

No DeFiLlama record matches the month and amount of these retail cases:
- the $55.48M setOwner theft (August 2024)
- the $32.51M case (September 2024)
- the WBTC poisoning (May 2024)
- the $50M poisoning (December 2025)
- the $12.25M poisoning (January 2026)

Since 2023 there is a single "Address Poisoning" record: Florence Finance, $1.45M, 2023-11-30. So Scam Sniffer's $494M (2024) and $83.85M (2025) are almost entirely additional to DeFiLlama's totals.

---

## 7. Not found, open gaps, conflicts

**Not found**

- **Scam Sniffer:**
  - an H1 2026 or 2026-to-date total
  - monthly reports after January 2026
  - a standalone H1 2025 report (figures here are derived from the annual report's quarters)
- **Chainalysis:**
  - a 2026 mid-year update
  - the DPRK share of 2023 as a percentage
  - CeFi/DeFi dollar values (charts only)
  - a private-key vs code split for 2025
- **CertiK:** a Q3 2025 standalone report; a Q3 2026 report; a CeFi/DeFi split.
- **Immunefi:** a 2025 annual report; per-vector dollar figures.
- **Hacken:** a 2024 smart-contract figure (not re-verified here), an H1 2025 vector split (PDF text garbled) and a Q2 2026 phishing figure.
- **TRM:** a 2024 code-exploit share.
- **Address poisoning:** yearly totals for 2024 and 2025 from the major firms.
- **Blind signing:** any published count or share of incidents.
- **EIP-7702:** any named 2026 phishing case.

**Conflicts**

- **DMM Bitcoin:** CertiK: "likely the result of address poisoning". FBI: manipulation of "a legitimate transaction request by a DMM employee".
- **Bybit:** $1.4B (Verichains, DeFiLlama), $1,447,063,421 (CertiK), $1.46B (TRM, SlowMist), ~$1.5B (Chainalysis, FBI).
- **Drift:** ~$285M (Chainalysis, TRM, CertiK) vs $295M (DeFiLlama).
- **Bitget:** $351.6M (TRM) vs ~$387–388M (Chainalysis, Bitget, SlowMist).
- **CertiK 2024 total:** $2,362,748,975.83 (Jan 2025) vs $2,446,285,251 (restated Dec 2025).
- **Scam Sniffer 2024:** monthly figures revised upward in the annual report (March $71M → $75.2M; Q1 $173M → $187.2M).
- **EIP-7702 sweeper delegations:** leaked keys (Wintermute) vs phishing sites (Huang et al.).
