# Documentation

These are working documents, kept in the order they were decided rather than
tidied afterwards. The numbering is the sequence, not a ranking.

## If you are deciding whether to trust this

- **[03 — Verification status](03-verification-status.md)** — what has been
  demonstrated, how, and what has not. Written to be read by someone deciding
  whether to rely on this, so it names the gaps rather than only the passes.
- **[09 — Against real transactions](09-against-real-transactions.md)** — the
  reviewer run over 238 real Safe transactions, what it said about each, and the
  one finding that run produced about the reviewer itself.
- **[08 — What was checked](08-what-was-checked.md)** — the last full
  verification pass: what was run, and what it found.

## If you approve transactions on a Safe

- **[06 — Using it before you sign](06-using-it-before-you-sign.md)** — what to
  run before you approve, and what to look for. Start here.

## If you are here to attack it

- **[05 — Review package](05-review-package.md)** — the claims worth attacking,
  the trust boundaries, and what is already known to be missing.
- **[01 — Threat model](01-threat-model.md)** — adversaries, the trust boundary,
  and the twelve invariants everything else is built to hold.

## If you want to know why it is built this way

- **[00 — Why a dedicated operating system](00-why-a-dedicated-os.md)** — who
  this is for, the guarantee, and the test the project had to pass before any
  kernel code was written.
- **[02 — Version 1 scope](02-v1-scope.md)** — what is in, what is deliberately
  out, and the rule that anything added must be paid for by something removed.
- **[04 — Platform architecture](04-platform-architecture.md)** — the longer arc:
  an operating system where intelligence proposes and a person approves exactly
  what will happen.
- **[07 — Timeline](07-timeline.md)** — what happens next, when, and what it
  costs. Dates are targets, not promises.

## Elsewhere

[research/](../research/) holds the feasibility study and the analysis of 143
failed operating systems that set this project's scope. The website lives in its
own repository, [clearsign.dev](https://github.com/clearsign-dev/clearsign.dev).
