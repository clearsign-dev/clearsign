# Pilot evaluation record

Use one copy per team and tested release. This is a proposed evaluation protocol,
not evidence that a pilot has happened. Do not submit keys, seed phrases, private
transaction records or customer identities to public issues.

## Scope and agreement

Start with Safe transaction review in observation mode alongside existing
approval controls. ClearSign does not simulate arbitrary contract execution,
verify the code deployed at a destination, or force a separate wallet to sign
the reviewed hash. Unsupported input is not a successful review.

Record before testing:

| Item | Team's record |
| --- | --- |
| Evaluation owner and dates | Not recorded |
| Release, source commit and downloaded asset checksum | Not recorded |
| OS version, architecture and installer format | Not recorded |
| Safe version, chain ID and transaction workflow | Not recorded |
| Wallet model, firmware and signing mode | Not recorded |
| Who owns approval risk and the purchasing budget | Not recorded |
| Permission to retain redacted results and retention period | Not recorded |

Use a disposable test Safe or public historical fixtures. Keep real funds and
real signing keys out of development signers. Agree who can stop the evaluation
and how issues will be reported before starting.

## Acceptance cases

Run each case on the downloaded application, not a development preview. Record
pass, fail or unsupported, the expected outcome, and the evidence location.

| Case | Expected result |
| --- | --- |
| Install, launch, close and reopen | The correct release version is displayed; no developer tools are required |
| Known ordinary transaction | Decoded fields and signing digest agree with independently computed expectations |
| Historical Bybit fixture | DELEGATECALL is visible and the locally computed Safe hash matches the fixture |
| Change input after review | The previous verdict and hash disappear |
| Clear during a delayed fetch | A late response cannot restore the old review |
| Malformed, oversized or duplicate-key JSON | Refused with a useful explanation; no stale verdict |
| Hash-selected fetch returns a different transaction | Refused, not substituted silently |
| Unknown selector or unsupported typed data | BLIND or refused, not described as safe |
| Offline review of a saved fixture | Review completes without requesting remote transaction data |
| Cross-check the wallet's proposed signing digest | Exact agreement under the documented signing mode, or an explicit unsupported result |

The historical fixture is
`signing-core/crates/clearsign-cli/tests/fixtures/bybit-safe-tx.json`. Its expected
Safe hash is
`0xb3476d061aeb8fc1d605a873c483a2402d88a68a9cdd1a8b47655dd55ba004f8`.
Do not infer that recognizing this attack detects every malicious transaction.

If the wallet cannot display the relevant digest, mark the binding step
unsupported. A visually similar summary is not a digest comparison. Test
deliberately changed fixtures without submitting signatures or moving funds.

## Results and stopping rules

Record transaction count, supported count, refusal reasons, median review time,
installation failures, repeated warnings and completed digest comparisons. Keep
denominators: ten successful supported reviews out of a hundred attempted
transactions is not 100% workflow coverage. Separate transaction findings from
standing Safe configuration warnings.

Stop on any unexplained digest disagreement, stale verdict, unsupported input
presented as understood, or discrepancy between the reviewed bytes and the
wallet's approval. Preserve a redacted reproducer and report security issues
through [private advisories](https://github.com/clearsign-dev/clearsign/security/advisories/new).
Use [GitHub issues](https://github.com/clearsign-dev/clearsign/issues) for ordinary
support, without confidential data. Do not continue with real funds to establish
whether a suspected bug is exploitable.

## Decision after four weeks

Ask whether the team returned without prompting, whether the check changed an
approval decision, which unsupported workflows blocked use, and what additional
work the check imposed. Record who would pay, the budget process and any written
commitment separately from compliments, download counts or expressions of interest.

Proceed toward a supported integration only when recurring use and a specific
buyer justify its maintenance cost. Consider an embedded library when teams
value the decoder but will not adopt a separate review step. Publish aggregate
results only with consent. An investor discussion should distinguish tested
capabilities, unresolved risks, proposed pilots and actual customer commitments.

## Reference methods

- [Safe's transaction hash API](https://docs.safe.global/reference-sdk-protocol-kit/transactions/gettransactionhash)
  provides an independent implementation to compare with, not an authority that
  can replace local recomputation.
- [Safe signature guidance](https://docs.safe.global/sdk/protocol-kit/guides/signatures/transactions)
  and [EIP-712](https://eips.ethereum.org/EIPS/eip-712) describe signature and
  domain distinctions that must be resolved for the chosen wallet workflow.
- [GrapheneOS device requirements](https://grapheneos.org/faq#future-devices)
  illustrate the separate hardware, firmware, verified-boot and maintenance
  commitments required for a physical security appliance. Desktop tests do not
  establish these properties for ClearSign's experimental platform.
