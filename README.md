# Signing OS project

Working project for a privacy and security focused operating system, narrowed by research to its most defensible job: **letting people see exactly what they are about to sign, on a machine nothing else can reach.**

> **Status: early development. Unaudited. Not for real funds.**
>
> No part of this has been reviewed by anyone outside the project. It runs under
> emulation, with no hardware root of trust and no verified boot. The signing and
> seed commands are locked behind an environment variable for that reason. Never
> give it a recovery phrase that holds anything, and never use it to sign a
> transaction you care about.
>
> What *is* checked is written down in
> [docs/03-verification-status.md](docs/03-verification-status.md), including the
> things that are not: it is meant to be read by someone deciding whether to
> trust this, and it names the gaps rather than only the passes.

## Start here

| Read | Why |
|---|---|
| [docs/00-why-a-dedicated-os.md](docs/00-why-a-dedicated-os.md) | Who this is for, the guarantee, and decisions awaiting sign-off |
| [docs/01-threat-model.md](docs/01-threat-model.md) | Adversaries, trust boundary, and invariants INV-1 to INV-12 |
| [docs/02-v1-scope.md](docs/02-v1-scope.md) | Frozen version-1 scope, progress, and a proposed change |
| [docs/03-verification-status.md](docs/03-verification-status.md) | What has been proven, how, and what has not |
| [docs/04-platform-architecture.md](docs/04-platform-architecture.md) | The full platform: an intelligent OS where AI proposes and a person approves exactly what happens |
| [docs/05-review-package.md](docs/05-review-package.md) | For a security reviewer: the claims worth attacking, the trust boundaries, and what is already known to be missing |
| [docs/08-what-was-checked.md](docs/08-what-was-checked.md) | The last full verification pass: what was run, what it found |
| [docs/07-timeline.md](docs/07-timeline.md) | What happens next, when, and what it costs |
| [docs/06-using-it-before-you-sign.md](docs/06-using-it-before-you-sign.md) | **Start here if you sign transactions on a Safe.** What to run before you approve, and what to look for |
| [research/](research/) | Feasibility study and the analysis of 143 failed operating systems |

## Layout

```
docs/                     decision record, threat model, scope, verification status
research/                 research reports and evidence
signing-core/             Rust workspace, toolchain pinned
  crates/clearsign/           no_std decoder: shows what a transaction really does
  crates/clearsign-keys/      no_std seeds, derivation, review-bound signing
  crates/clearsign-cli/       command-line front end
  crates/clearsign-difftest/  test-only differential tests against alloy
  crates/authority/           no_std authority engine: plans, data-flow tracing, approval-bound execution
  crates/clearsign-ffi/       C interface so seL4 compartments can call the decoder
  fuzz/                       cargo-fuzz targets with security-property assertions
  scripts/reproducible-build.sh
platform/                 borrowed layers: seL4 compartments, Linux inside seL4, GrapheneOS (see platform/README.md)
  sel4/signer-system/         seL4 + untrusted wallet UI + bare-metal signer
  sel4/linux-signer/          seL4 + Linux guest + signer, with GrapheneOS hardened_malloc
  grapheneos/                 hardened_malloc build pinned to the verified GrapheneOS manifest
  android/build-host/         verified GrapheneOS sync and build pipeline for x86_64 Linux
vm/                       verified, no-network development VM (Alpine)
```

## Try it

```sh
cd signing-core
cargo test --workspace --release

# Review: safe on any computer. Exit code 3 means a CRITICAL finding.
cargo run -q -p clearsign-cli -- safe-tx --chain-id 1 \
  --safe 0x1Db92e2EeBC8E0c075a02BeA49a2935BcD2dFCF4 \
  --to 0x00000000000000000000000000000000DeaDBeef --nonce 71 --operation 1 \
  --data 0xa9059cbb000000000000000000000000000000000000000000000000000000000000dead0000000000000000000000000000000000000000000000000000000000000000
```

Read a signing request the way a real air-gapped device would, from QR codes a
wallet displays (MetaMask and Keystone speak this format):

```sh
# each line is one scanned ur: string, as many as the animation has
cargo run -q -p clearsign-cli -- qr-review scanned-codes.txt
```

Review what an AI agent wants to do before it does it:

```sh
cargo run -q -p authority-agent -- review crates/authority-agent/examples/injected-proposal.json
```

That proposal is an ordinary "summarise my notes" task with two injected steps.
Every step looks routine; the plan does not. Exit code 3, and the reason is
named: secret data would leave the device through a web request.

Signing commands exist for development and are locked behind an environment variable, because a recovery phrase typed into an everyday computer must be treated as exposed. Run `clearsign --help` for details.

## Licence

MIT OR Apache-2.0, pending sign-off in `docs/00`.
