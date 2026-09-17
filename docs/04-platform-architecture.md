# 04 — Platform architecture: an intelligent OS people can trust

**Status:** DRAFT, 16 Sep 2026. Written after the project owner set the ambition at a top-tier, general-audience operating system that handles highly complex work by breaking it into simple steps. **[needs sign-off]**

This document does not replace the version-1 signer scope in `02-v1-scope.md`. The signer ships first and becomes the security root of the platform described here.

---

## 1. The thesis

Every major operating system is converging on AI agents that act for the user: reading files, sending messages, making purchases, moving money. That creates a problem no incumbent has solved cleanly. **The agent is the new untrusted web interface.** A prompt-injected agent is exactly the Bybit attack, generalised to everything a computer can do.

Our position, built on what already works in this repository:

> **An operating system where intelligence proposes, and a human approves exactly what will happen, before anything irreversible happens.**

The same pipeline we built for transactions, applied to every sensitive action:

```
 goal ─► planner (AI, untrusted) ─► plan: small typed steps
                                        │
                                        ▼
                  authority engine: validate ─► trace data flow ─► classify risk
                                        │
                                        ▼
                  plain-language review, broken into simple chunks
                                        │
                                        ▼
                  human approves exact plan fingerprint + named risks
                                        │
                                        ▼
                  executor runs ONLY that plan, with only the capabilities it declared
```

"Breaks complexity into simple chunks" is not a presentation layer on top. It is the security model: a task the system cannot break into typed, reviewable steps is a task it will not perform silently.

## 2. Why this is winnable when "another general OS" is not

The failure study of 143 operating systems found brilliant systems die from no channel, no inherited software, and an owner who stops committing. This design answers each:

| Failure family | Answer |
|---|---|
| No inherited software | Existing Linux and Android applications run inside isolated compartments. We do not ask anyone to rewrite anything. |
| No channel | The authority engine and the signer are portable libraries. They can ship first inside other platforms, wallets and agent frameworks, before any device exists. |
| Perpetual rewrite | Borrow the kernel, the application runtimes and the cryptography. Write only the authority layer, the review experience and the signer. |
| Invisible security does not sell | The product is visible: a person watches a complex task become a short list of plain steps and approves it. |

## 3. Layers

| Layer | What it is | Borrow or build | Why |
|---|---|---|---|
| **L0 Hardware root** | Secure element or TPM, verified boot, the signing device | Borrow hardware; build the signer (done: `clearsign`, `clearsign-keys`) | Keys and approval must live below everything else |
| **L1 Kernel** | seL4 formally verified microkernel | **Borrow** (done: Microkit 2.3.0, boots in QEMU) | About ten thousand lines with a machine-checked proof. Nothing we could write would be as trustworthy |
| **L2 Compartments** | Isolated domains, each with explicit capabilities, in the style of Qubes but on a microkernel | Build on seL4 components (done: signer, wallet UI and Linux compartments, isolation demonstrated) | Isolation by default is what lets untrusted apps and agents coexist |
| **L3 Compatibility** | Linux guest compartments for desktop apps; an Android runtime compartment for mobile apps | **Borrow** (done: Linux guest via libvmm; GrapheneOS source verified and hardened_malloc deployed; Android compartment not yet) | This is where the millions of existing lines come from, and why users get their apps on day one |
| **L4 Authority engine** | Plans, capabilities, information-flow tracing, risk classification, approval binding, execution | **Build** (started: `authority` crate) | The product. No incumbent has it as an OS primitive |
| **L5 Intelligence** | On-device language models that turn a goal into a plan | Borrow models and runtimes; build the planner adapters | Models are commodities. What we control is that their output is treated as untrusted input |
| **L6 Experience** | Review screens, the approval flow, the everyday shell | Build | Where the "simple chunks" are felt |

## 4. The authority engine in detail

**Plans are data, not code.** A planner emits a plan: a small graph of typed steps such as read a file, send a message, make a web request, sign a transaction, delete, install, change a setting. Each step declares which earlier steps' outputs it uses.

**Plans cross a trust boundary as bytes.** The wire format is exactly the encoding the fingerprint is taken over, so what is transmitted, what is displayed and what is approved cannot diverge. The decoder runs on the deciding side, not the proposing side: bounded lengths, known tags only, UTF-8 text, no trailing bytes, and a decoded plan must re-encode to the input byte for byte.

**Validation.** Plans are rejected if they reference missing steps, contain cycles, or exceed fixed size limits. A malformed plan never reaches a person.

**Information-flow tracing.** Every step's output inherits the sensitivity of everything that flowed into it. If secret data can reach a network request or a message, that is flagged CRITICAL even when the individual steps look harmless. This is the direct defence against prompt injection that tries to leak data: "summarise my documents" followed quietly by "send the summary to this address".

**Risk classification.** Same four levels as the signer: INFO, WARNING, BLIND, CRITICAL. Irreversible actions, secret egress, system modification and funds movement are CRITICAL. Running arbitrary programs is BLIND, because what a program does cannot be read from the request. A transaction-signing step is reviewed by `clearsign`, and its findings are carried into the plan review.

**Approval binding.** A plan has a fingerprint computed from a canonical encoding of every field. Approval names that fingerprint and acknowledges exactly the set of BLIND and CRITICAL findings, each tied to its step.

**Execution.** The executor recomputes the fingerprint before running, refuses a plan that changed after approval, runs steps in dependency order, gives each step only its declared inputs, and stops at the first failure.

## 4a. Engine invariants, with tests

| ID | Invariant | Test |
|---|---|---|
| **AUTH-1** | What a step does is always described from its typed action; planner text is quoted, escaped and marked unverified | `mislabelled_destructive_step_is_described_by_its_action`, `untrusted_text_cannot_inject_terminal_or_bidi_control` |
| **AUTH-2** | Sensitive data is traced through every intermediate step, and reaching the network or a message is flagged | `secret_egress_is_caught_across_intermediate_steps`, `flow_merges_multiple_sources` |
| **AUTH-3** | Tracing is precise enough not to taint unrelated branches | `unrelated_branch_is_not_tainted` |
| **AUTH-4** | Approval requires exactly the step-scoped BLIND and CRITICAL acknowledgements | `approval_requires_exact_step_scoped_acknowledgements` |
| **AUTH-5** | Any change to the plan after approval, including a label, prevents execution before any step runs | `a_plan_changed_after_approval_never_runs` |
| **AUTH-6** | Steps receive only declared inputs, run in dependency order, and the first failure stops everything | `executes_in_order_with_only_declared_inputs`, `first_failure_stops_everything` |
| **AUTH-7** | Transaction steps get the full `clearsign` review | `transaction_steps_are_reviewed_by_clearsign` |

Status: implemented in `signing-core/crates/authority`, `no_std`, with the same no-panic lints as the signer.

## 5. Phased roadmap with gates

A phase starts only when the previous gate is met. This is the defence against the scope creep that killed Copland and the Hurd.

| Phase | Deliverable | Gate to move on |
|---|---|---|
| **1 — Now** | Signer v1 and the authority engine as portable libraries, tested and fuzzed | Signer v1 exit criteria met; authority engine invariants tested |
| **2** | Authority engine integrated with one real agent framework and one wallet, running on existing operating systems | **Half met, 17 Sep 2026.** The adapter exists: `authority-agent` turns tool-call proposals into typed plans, refuses tools this device does not have, classifies data by local policy rather than by what the planner claims, and runs only an approved plan through a capability-poor executor. The wallet side is the EIP-4527 QR path. Still needed for the gate: real users approving real plans, and outside review |
| **3** | seL4 compartment prototype in QEMU: signer compartment, one Linux guest, authority engine mediating between them | **Met, on the second attempt.** Claimed on 17 Sep 2026, withdrawn the same day after an external review pointed out that the review text on the console was printed by the guest, then earned back: the guest has no serial device at all. Its writes trap to the VMM, which relays them line by line behind a prefix it cannot forge; it has no receive path, so nothing can type into it. The harness asserts on `SIGNER\|` lines, which only the signer can produce, and separately that no guest line can begin with that prefix |
| **4** | Reference device and the everyday experience layer | External security audit passed; funding secured for a team |
| **5** | Android compatibility compartment, on-device models, developer platform | Independent developers shipping on it |

## 6. What this does not claim

- It is not a promise that one person can ship a general-audience OS. Phases 4 and 5 require a funded team, which the research says realistically comes from grants, a foundation with large donors, or hardware-backed venture funding.
- Line count is not a measure of progress. The code we write should stay small and heavily verified. The large bodies of code come from borrowed, proven components.
- An approval screen does not protect a user who approves without reading. It makes the risk visible and specific; it cannot make the decision.
