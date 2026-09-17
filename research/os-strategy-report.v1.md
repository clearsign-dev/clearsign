# Building Your Own Privacy OS: A Feasibility and Strategy Analysis

**Prepared for:** a solo blockchain security researcher evaluating whether and how to build a GrapheneOS-inspired operating system
**Date:** 16 September 2026
**Method:** 5-angle web research, 27 sources fetched, 134 falsifiable claims extracted with verbatim quotes

> **Verification status — read this first.** This report was produced by a research harness whose final step is a 3-vote adversarial verification of every claim. That step did not run: all 75 verifier agents failed on an API session limit. Every fact below is attributed to a source and carries a verbatim quote, but none has been independently cross-checked against a contradicting source. Treat single-source claims, self-reported figures, and one-sided accounts as provisional. Each section flags the weak ones.

---

## Executive summary

Three paths exist. Only one of them has ever been completed by a very small team, and it is not the one that looks most like GrapheneOS.

**Forking AOSP is closing, not opening.** Four independent changes in 2025-2026 compound against a new entrant: Google's Risk-Based Update System now ships source only for quarterly bulletins, AOSP source publishes just twice a year from 2026, AOSP 16 dropped Pixel device trees and driver binaries, and bootloader-unlock policy changes shrank the modder-accessible share of the smartphone market from 42.5% to roughly 7%. A fork started today has worse upstream access than GrapheneOS had in 2019, while GrapheneOS has about ten paid developers and a signed OEM partnership.

**The hardware gate is not a skill problem.** GrapheneOS requires verified boot with custom key enrollment, a StrongBox secure element with attest-key pinning, and ARM memory tagging. Only Google Pixels pass. Motorola signed a partnership in June 2025, is a top-ten OEM, and still is not expected to ship a qualifying device until 2027, blocked on Qualcomm memory-tagging silicon. No amount of individual effort moves that.

**"General public" and "privacy OS" have never overlapped at scale.** The largest custom-ROM base, LineageOS, is 4.25 million installs concentrated in Brazil and China, driven by refurbishers reselling old mid-range phones. Pixels are 0.9% of those installs. The privacy-motivated audience is an order of magnitude smaller and well-mapped. The two products that did move consumer numbers moved on repairability and on an airdrop, not on security.

**There is a real and specific niche, and it fits your discipline exactly.** Two incidents define it. Triada malware shipped preinstalled in the Android system partition, loaded into every process, swapping crypto addresses on a two-second clipboard poll. No wallet app can defend that layer. Separately, Coldcard wallets lost roughly $88.6 million because a build configuration defined a macro as zero and a library checked whether the macro existed rather than whether it was enabled, routing seed generation to a deterministic PRNG. That is a code-review and entropy-audit finding. It is what you already do.

**Recommendation: build a single-purpose signing and vault operating system, not a general-purpose phone OS.** Foundation built KeyOS, a Rust microkernel OS, in about three years, and now ships it on a $349 device with an SDK, a simulator, and an outside wallet team building on it. That path completed because the scope is small. It needs no GPU driver, no browser, and no Android app compatibility. Start entirely in virtual machines, stay device-agnostic through year one, and choose narrow auditable hardware afterward.

---

## 1. The three build paths, costed from real projects

### Path A — Fork and harden AOSP

**What it actually took GrapheneOS.** Daniel Micay founded it in late 2014 as a solo project. Twelve years later it self-reports about 20 active contributors, roughly 10 of them paid full-time, funded entirely by donations. The project's own history page notes that early progress was fast "because there was so much low hanging fruit to address and it wasn't yet trying to produce a highly robust, production quality OS." The formal funding vehicle, the GrapheneOS Foundation, was incorporated in Canada in March 2023, about eight and a half years after founding.

**The sponsor risk is not hypothetical.** A company incorporated in late 2015 became the primary sponsor and the OS was renamed CopperheadOS. In 2018 the CEO attempted a takeover, and by GrapheneOS's account seized the infrastructure and took roughly $300,000 in Bitcoin donations. Micay permanently deleted the release signing keys, which prevented the takeover but froze existing CopperheadOS builds so they could never be patched again. A suit for $400,000 in damages was filed in 2020. *This is GrapheneOS's one-sided account and should be read as such.*

**Funding that actually worked.** Not community donations. Large individual donors: $1,000,000 from Jack Dorsey's StartSmall per the Foundation's own claim, and two donations from Vitalik Buterin in 2023 totalling 205 ETH, worth roughly $410,000 at the time. **Note for your position specifically: the crypto community is already the funding base for privacy OS work.**

**Why a new fork is harder now than in 2019.**

| Change | Effect on a new fork |
|---|---|
| Risk-Based Update System | Monthly bulletins carry only actively-exploited or exploit-chain vulnerabilities; the majority ship quarterly in March, June, September, December |
| Source for monthly updates withheld | Google publishes source only for quarterly updates, so "most custom ROMs can't ship monthly updates anymore" |
| AOSP source cadence | From 2026, source publishes twice a year, Q2 and Q4 only |
| Private bulletin access | OEMs and chipset vendors get the private bulletin ~30 days early; independent forks get nothing |
| Bootloader unlock policy | Samsung, Xiaomi, ASUS and Realme changes cut modder-accessible market share from 42.5% to ~7% |

The July 2025 bulletin listed zero vulnerabilities, the first of 120 bulletins ever to do so. August listed six. The September quarterly listed 119. The cadence changed, not the bug count.

**Verdict:** viable only with a team and a hardware partner. Not a solo path in 2026.

### Path B — Harden an existing Linux base

**Qubes OS is the cautionary tale, and the numbers are specific.** Launched April 2010. Self-funded for about five years out of Invisible Things Lab's consulting revenue. Its founder's 2015 note on the first Open Technology Fund grant is worth quoting in full context: the award was $160,000, intended to cover a year of work by "3-4 skilled developers working full time," and she called it "very symbolic" by open-source standards. On community funding she was blunter: Bitcoin donations accumulated over nearly two years totalled about $800, "for which we could probably afford to pay for… 1-2 days of work a skilled system developer."

By November 2016 the project had received roughly $410,000 from OTF over the prior year. The team's assessment: "This was enough to survive and release new versions, but not enough to implement everything we've planned for." The concrete cost of that shortfall, in their own words: Qubes 4.0 repeatedly delayed, GNOME support never added, Live USB left in alpha and unmaintained, many bugs unfixed. An attempt to commercialize proprietary Windows AppVM support failed outright. They explicitly declined to sell dedicated Qubes laptops, citing the difficulty of making hardware trustworthy enough to merit their seal of approval.

**Adoption after sixteen years:** roughly 72,000 users in August 2026, from 56,947 unique clearnet IPv4 addresses plus an estimated 14,881 Tor users. The project is careful that this is a proxy, not a count. Growth has been steady and niche.

| Month | Estimated users |
|---|---|
| Aug 2015 | 5,600 |
| Aug 2016 | 13,900 |
| Aug 2020 | 30,200 |
| Aug 2024 | 48,900 |
| Aug 2025 | 57,600 |
| Aug 2026 | 71,800 |

One useful signal buried in that data: users migrate to new releases fast. In August 2026 the 4.3 bucket held 40,231 clearnet addresses against 16,546 for 4.2. A security-focused user base does update.

**Verdict:** the fastest path to something real people run, especially as a configuration layer on an existing immutable base rather than a new distribution. But Qubes proves that even with grant funding, a full security OS outruns a small team's capacity.

### Path C — From scratch, or on a microkernel

**Redox OS, ten years in.** Marked its 10th anniversary in 2025. As of December 2025 it still describes Wayland support, hardware-accelerated graphics, self-hosting and running on a smartphone as in-progress or first-attempt goals. Its first GPU driver, for Intel Tiger Lake and Kaby Lake, supports mode setting only with no acceleration; the desktop had relied on BIOS VESA and UEFI framebuffers for years. The USB SCSI driver was disabled for unreliability.

Funding in 2025: about $37,000 in cash, of which $17,000 came from community donations and $20,000 from the lead maintainer himself. The nonprofit says it is seeking larger donors "to help support full-time developers," which implies it currently funds none. More than 40 significant contributors, a four-person unpaid board.

**And the counterexample that changes the conclusion.** Foundation, maker of the Passport hardware wallet, built KeyOS as a Rust microkernel operating system over roughly three years, open-sourced it, and ships it with a post-quantum encrypted Bluetooth protocol running on a dedicated isolated Bluetooth chip. It raised $6.4 million in May 2026 for $16.5 million total. Passport Prime sells at $349, is manufactured in the USA, and reached general availability on 22 May 2026. Foundation publishes an SDK, documentation and a KeyOS simulator, and Cake Wallet, which it says has one million users, is the first outside team shipping on it.

**The difference between Redox and KeyOS is scope, not talent.** Redox is trying to be a general-purpose desktop OS and therefore needs a GPU driver, a window system, and a POSIX surface. KeyOS signs transactions. It needs none of those. That is the entire reason one took ten years without reaching its goals and the other shipped in three.

**Verdict:** from-scratch is viable if and only if the OS is single-purpose.

---

## 2. What the public actually adopts, and where the walls are

### Adoption reality

| Project | Figure | Basis |
|---|---|---|
| LineageOS | 4,254,349 active installs (Oct 2025) | Opt-out telemetry, undercount; developer notes fake accounts have inflated some builds |
| GrapheneOS | ~350,000-400,000 users (Apr 2026) | Self-reported from update-server logs; no telemetry exists |
| Qubes OS | ~72,000 (Aug 2026) | Unique IPv4 addresses hitting update servers |
| Fairphone | ~150,000 units sold in 2025, +41% YoY | Company impact report |

**The most important number in this report is a distribution, not a total.** LineageOS installs are 44.5% Brazil (1.88 million) and 24.1% China (1.02 million), against 3.6% for all of Europe (154,000). The author attributes the Brazilian concentration to businesses refurbishing and reselling old mid-range Motorola phones with LineageOS preinstalled; 99.8% of Moto G7 Play installs are in Brazil. Google devices account for 12.3% of official builds but 0.9% of active installs.

Read that carefully. The mass audience for alternative Android is people who want a cheap working phone, not a hardened one. The security-motivated audience is the 400,000 on GrapheneOS and the 72,000 on Qubes. Those are different markets and they do not convert into each other.

**The update gap is the strongest argument for doing this properly or not at all.** Only 25.9% of active LineageOS devices run a version still receiving security updates. The most popular version, 18.1, runs on 25.1% of devices and has not been updated since March 2024. An alternative OS without a seamless forced update channel is a net security liability.

### The walls you inherit on day one

**Google Pay will never work.** Host Card Emulation must be run by a service holding system-level privileges and NFC routing rights. Sandboxed, unprivileged Google Play cannot hold them. The reporting is unambiguous: "There is no version of this that works. There is no upcoming fix."

**Play Integrity moved to hardware attestation and closed the spoofing door.** On 3 December 2024 Google changed the technology behind the API on all Android 13+ devices to require hardware-backed signals via Platform Key Attestation, with automatic transition for all integrations in May 2025. The `meets-strong-integrity` verdict now requires a security update within the last year, and Google positions it explicitly for banking, government and enterprise apps gating money transfers. Google also states it will adjust verdicts server-side "when there is evidence of excessive activity or key compromise," which is precisely what kills leaked-keybox workarounds. Enhanced verdicts are returned only for apps installed by Google Play.

**Banking apps mostly do work, which is the good news.** A month-long daily-driver test on a Pixel 8 Pro found Chase, Amex, Discover, Wells Fargo, Navy Federal, HSBC, Barclays, Monzo and Starling all functional on GrapheneOS with sandboxed Google Play. Apps checking only for root pass; apps demanding Google's device certification fail. Android Auto has worked since late 2023.

### Design lessons that actually transfer

Fairphone is the clearest consumer signal in the data, and it is not a privacy signal. Nearly 150,000 units in 2025, up 41%, with more than 70% of Gen 6 buyers being first-time customers, and 40% of 2019-era Fairphone 3 units still in active use seven years later. It sells /e/OS editions as first-party SKUs. People bought it for repairability, longevity and sustainability. Privacy came along for the ride.

The lesson is uncomfortable but useful: **pick one concrete promise a person can verify for themselves, and be unambiguously the best at it.** "More private" is not that. "Your keys cannot leave this device, and here is how you check" is.

---

## 3. Device strategy: narrow wins, and the evidence is not close

### Why GrapheneOS is Pixel-only

Its published requirements list is long and every item is load-bearing. Abbreviated:

- Alternate OS support with full hardware security functionality
- Complete monthly bulletin patches with no regular delay beyond one week for firmware, drivers and HALs
- At least 5 years of device-support updates for phones, 7 for tablets
- Linux 6.1, 6.6 or 6.12 Generic Kernel Image support
- Hardware-accelerated virtualization, ideally pKVM
- Hardware memory tagging, ARM MTE or equivalent
- Verified boot with rollback protection, and relocking with a custom signing key
- StrongBox keystore in a secure element, with hardware key attestation and attest-key pinning
- Weaver throttling for disk-encryption key derivation, in the secure element
- 64-bit-only device support code, hardware-level USB disabling, JTAG inaccessible while locked

The project is explicit about the tradeoff: "Broad device support would imply mainly supporting very badly secured devices unable to support our features. It would also take a substantial amount of resources away from our work."

It supports 21 Pixel models, Pixel 6 through 10a plus Fold and Tablet, and recommends Pixel 8 and later for the 7-year support guarantee and ARMv9 memory tagging.

### What else qualifies: almost nothing

**Motorola** signed with the GrapheneOS Foundation at MWC 2026 on 2 March, the first major OEM partnership. The partnership began June 2025. Devices are not expected until 2027, delayed because GrapheneOS requires ARM memory tagging and Qualcomm is only fully implementing it with the Snapdragon 8 Elite Gen 5. As of January 2026 the GrapheneOS team said publicly that "Motorola's devices don't currently meet the requirements for GrapheneOS, but they're getting closer." Physical sensor kill switches are planned for the partner hardware.

**Fairphone 6** fails on the security element. GrapheneOS states it has no secure element, so without Weaver throttling a typical 6-to-8 digit PIN is trivially brute-forced and disk encryption protects only users with strong passphrases. It also states Fairphone skips monthly and quarterly updates, runs 1-2 months late on backports and a year or more late on yearly upgrades, that Fairphone 4 is on the end-of-life Linux 4.19 branch and Fairphone 5 on 5.4, and that as of January 2026 Fairphones were still on the initial Android 15 release. *These are GrapheneOS's assertions about a competitor and are exactly the kind of claim the missing verification step would have tested. Check them against Fairphone's release notes before relying on them.*

**Samsung** is considered disqualified: Knox and restrictive bootloaders are hostile to custom operating systems.

### Linux phones are not a path

| Device | Price | Fatal limitation |
|---|---|---|
| PinePhone Pro | $400 | Discontinued August 2025 for poor sales; driver support ended up worse than the cheaper original |
| PinePhone | $200 | 2019 silicon, "genuinely slow" by 2025, 3-5 hours battery, inconsistent MMS, no Android apps |
| Librem 5 | $699 | Three real electrical kill switches, but 3-5 hours of battery; 2017 campaign backers waited until 2020-2021 |

postmarketOS on used Android hardware is the cheap entry point, $50 to $300, with Fairphone 6 support landing on the device's release day. But those phones have no kill switches and no secure element.

### The desktop comparison

Qubes certification is the general-purpose analogue of GrapheneOS's device gate, and it is just as narrow: 11 models from 5 vendors, several of them refurbished ThinkPad X230 and T430 machines. It requires open-source boot firmware such as coreboot, with all System Management Mode code open-source, excepting authenticated CPU-vendor blobs. Vendors send two non-returnable units per configuration, keep that exact configuration on sale for a year, and pay a negotiated flat monthly fee. Certification explicitly does not mean secure end-to-end; the project disclaims responsibility for supply-chain tampering.

**Conclusion on device strategy: narrow, always.** But the right narrow target for you is not a phone you must persuade an OEM to build. It is a small, cheap, auditable board you choose yourself, after a year of validating the software in virtual machines.

---

## 4. The crypto-security niche: the strongest part of the case

### The attack that only an OS can stop

Triada arrived preinstalled on counterfeit Android phones sold at a discount, compromised somewhere in the supply chain so that "stores may not even suspect that they are selling smartphones with Triada." Kaspersky logged more than 4,500 infections worldwide.

The technical detail is the whole argument. A shared library is loaded into Zygote, the parent of every Android app process, by a tampered ahead-of-time compiled framework file in `/system/framework/`. From there it is copied into every process on the phone. One module polls the clipboard every two seconds and replaces any cryptocurrency address with an attacker's. Another payload is injected into wallet apps to substitute the destination address inside transaction fields. Roughly $270,000 was moved between June 2024 and March 2025.

No wallet application can defend against code running underneath it in every process. Verified boot over a signed, immutable system image is the mechanism that addresses this, and it is an operating-system property. Google's own response was that the affected devices were uncertified AOSP builds with no Play Protect test records, which frames certification and attestation as the mainstream answer and makes clear that an independent OS must supply its own equivalent.

### The failure that proves the niche is unsolved, and that you are the right person

On 30 July 2026 an attacker drained 1,196 Bitcoin addresses in 41 minutes, taking 1,082.65 BTC worth about $70.2 million. Galaxy Research later raised its observed estimate to 1,367.05 BTC, about $88.6 million, across 4,585 addresses in three waves.

The cause was not cryptography and not a broken air gap. Coldcard's production configuration defined `MICROPY_HW_ENABLE_RNG` as zero, because Coinkite supplies its own hardware-RNG wrapper. The libngu library checked whether that macro *existed* rather than whether it was *enabled*. Seed generation therefore fell back to MicroPython's deterministic Yasmarang PRNG, initialized from the chip's unique ID and timer registers and collecting no fresh entropy afterward. Effective entropy: roughly 40 bits on the Mk3 and about 72 bits on the Mk4, Mk5 and Q, against 128 bits for a proper 12-word BIP-39 seed.

Two things follow. First, a firmware patch cannot repair a key-generation flaw after the fact; restoring the old seed to fixed firmware carries the weakness forward, and owners must generate new seeds and move funds. Second, this class recurs: Coinspect's "Ill Bloom" research in early July 2026 found a separate weak-PRNG flaw in older software wallets tied to more than $5 million drained since May 2026.

*Caveat the attribution: Galaxy's link between the sweep and the firmware flaw rests on on-chain pattern analysis, and no public report has reconstructed a victim seed and matched it to a drained address.*

**A build-configuration error in a signing device's operating system cost users roughly $88.6 million.** Finding that class of bug before it ships is your profession. That is the differentiator, and it is not one a general-purpose OS developer has.

### What the market precedents say about demand

**Solana Seeker** is the volume data point: roughly 150,000 second-generation units shipped by late August 2025, against a total production run of 20,000 first-generation Saga phones. Price dropped from $999 at Saga launch to $500. Its Seed Vault stores keys in hardware and gates signing behind a fingerprint plus a physical side-button double-tap, which is the right interaction model.

But read the pitfalls honestly. The Saga initially failed with consumers and only sold out months later after a large price cut and the BONK airdrop, meaning token incentives drove demand, not security. And a reviewer with the device still got most communication and entertainment apps from Google Play and found the Solana dApp store thin beyond trading apps, concluding it is a "no-brainer" only for people already active in that ecosystem.

**Foundation** shows the better shape. It moved from a Bitcoin-only wallet to a broader consumer security device: Passport Prime bundles a Bitcoin wallet with FIDO security keys, 2FA storage, a secrets vault and 50GB of encrypted file storage, marketed under a trademarked "Human Authority Hardware" category aimed at approving AI-agent actions. A crypto-native security company concluded that mainstream appeal lies in general key and credential custody, not in Bitcoin alone. It is growing adoption by opening the OS to third-party wallet developers with an SDK and a simulator.

**That is the template.** Not a phone. A device and an OS whose single job is that a human being approves a high-stakes action on hardware no compromised software environment can reach.

---

## 5. Getting started, with your actual machine

### Your hardware, measured

| Property | Value |
|---|---|
| Architecture | arm64, Apple Silicon |
| RAM | 16 GB |
| Cores | 10 |
| Free disk | 590 GB |
| Installed | Docker, Homebrew |
| Not installed | QEMU, UTM, Lima, Podman, Nix |

### What that machine can and cannot do

**It cannot build AOSP or GrapheneOS, and not because of configuration.** Three documented blockers stack:

1. Google's AOSP documentation states Android OS development on macOS "isn't supported as of June 22, 2021 (Android 11)."
2. The required host is a 64-bit **x86** Linux system with glibc 2.17 or later. An ARM64 Linux VM on Apple Silicon is not the supported configuration. The concrete failure is that checked-in prebuilt host toolchains such as `prebuilts/go/linux-x86` are x86-64 binaries, so the build aborts with `Exec format error`. Soong's linux/arm64 support exists for cross-compiling arm64 host binaries *from* an x86-64 machine, which is a different thing from building on arm64.
3. RAM. AOSP documents a 64 GB minimum, with Google itself using 72-core machines with 64 GB. GrapheneOS requires at least 32 GiB, driven by link-time optimization peaks when linking Chromium and the kernel with LTO and control-flow integrity. You have 16 GB.

Storage would also bind: AOSP wants 400 GB (250 checkout plus 150 build), GrapheneOS wants 136 GiB for a standard sync plus 100 GiB for output. You have 590 GB free, so disk is survivable; RAM and architecture are not.

For scale: a full AOSP build is about 40 minutes on a 72-core, 64 GB machine, and about 6 hours on a 6-core machine with 64 GB.

**It can do everything the recommended path needs, plus real Android VM testing.** Cuttlefish, the AOSP virtual device, officially supports ARM64 hosts with the `aosp_cf_arm64_only_phone-userdebug` target on the `aosp-android-latest-release` branch, checked via `/dev/kvm`. Crucially, you do not need a local AOSP build to start: prebuilt device images and the matching `cvd-host_package.tar.gz` download from ci.android.com, launch with `HOME=$PWD ./bin/launch_cvd --daemon`, drive over adb, and display in a browser over WebRTC at `https://localhost:8443`. The host package must come from the same build as the images. The host path is Linux-only and hard-requires KVM, so this runs inside an ARM64 Linux VM, not on macOS directly.

For AOSP-flavoured experiments GrapheneOS recommends the `sdk_phone64_x86_64` emulator target, which needs no vendor files, while warning that emulator targets do not receive full monthly security updates and do not provide all baseline security features.

### Infrastructure that is permanent from day one

**Signing keys are effectively irreversible.** GrapheneOS's build documentation is explicit: keys must be generated to re-sign builds away from the public AOSP test keys, must be reused for every subsequent build, and "cannot be changed without flashing the generated factory images again which will perform a factory reset." The CopperheadOS episode is the proof: deleting the keys froze every existing build permanently. Generate them offline, with a written ceremony, before you have users.

**Reproducible builds are the mitigation for key theft, and the founders of Qubes said so in 2015.** Their assessment was that the binary build-and-distribution process was the weakest link, that building from source was the only current mitigation, and that deterministic builds were the prerequisite for a multi-signature release scheme that removes the single point of failure. Build reproducibly from the first commit; retrofitting it is painful.

**The update server is trivial; the discipline is not.** GrapheneOS uses a plain static web server hosting signed images plus channel metadata, with the URL configured in the Updater app. That is the easy part. The hard part is a patch cadence you can actually sustain, which the RBUS change has made materially harder for anyone outside the OEM partner list.

**Attestation is the expensive one, and it gates your core claim.** GrapheneOS's AttestationServer is MIT-licensed and self-hostable, but a self-hoster must fork it to change the domain, the Auditor app signing key and the app ID, and run a headless Java 25 runtime behind nginx with systemd and SQLite. The binding constraint is not the server: "Alternative operating systems need their verified boot key included in the Auditor app and Attestation Server… most alternative operating systems lack support for full verified boot and most devices don't support using verified boot with a custom key." The service currently verifies only 21 Pixel models, and GrapheneOS is the only alternative OS listed as verifiable.

One more number worth internalizing: that repository was created on 17 March 2018 and its most recent commit is dated 14 September 2026, with about 2,195 commits. That is eight and a half years of continuous maintenance for a single supporting component.

### Phased roadmap

**Months 0-3 — decide what you are defending, and touch all three paths**

- Write the threat model document first, before any code. Name the adversary, the assets, and what you explicitly do not defend against. Every later decision follows from it.
- Install QEMU and UTM. Your machine has neither. Stand up an ARM64 Linux guest.
- Run Cuttlefish in that guest from prebuilt ci.android.com images. Learn the Android security model by operating it, not by reading about it.
- Boot Redox from its published harddrive images and work through an seL4 example in QEMU. Two weekends will tell you more about the microkernel path than two months of reading.
- Reproduce the Coldcard entropy bug in a simulator and write it up. This is your credential, it is publishable, and it costs nothing but time.
- Do not buy a build machine. Do not register a domain. Do not announce anything.

**Months 3-12 — build the narrow thing**

- Fix the scope: an OS whose only job is generating, storing and using signing keys, with a human-approval step on hardware. No browser, no GPU, no app store.
- Deterministic builds from commit one, with published build instructions anyone can follow to reproduce your binary.
- Entropy and key generation as the audited core. Given the Coldcard failure mode, make the entropy path the most reviewed code in the tree, and make dice-roll or other user-supplied entropy a first-class input rather than an expert option.
- Transport by QR or USB CDC only. No network stack is a feature, and it removes an enormous class of bugs.
- Generate signing keys in an offline ceremony. Document it publicly.
- Stand up the static update server with signed channel metadata.
- Publish the threat model and invite people to break it. Your existing security reputation is the distribution channel.

**Year 2 and beyond — hardware, attestation, funding**

- Choose hardware you can audit rather than hardware you must negotiate for. A fixed, cheap board you fully control beats a phone whose firmware you cannot see.
- Add attestation only when you control a device with verified boot and custom key enrollment. Until then, say plainly that you cannot attest, rather than implying you can.
- Consider the Foundation model: open the OS as a platform with an SDK and a simulator so wallet teams build on it. Cake Wallet brought a million users to KeyOS.
- Funding, ordered by what the evidence shows works: grants of the OTF kind at $160,000 to $410,000 a year; a foundation with large individual donors, which in GrapheneOS's case came substantially from the crypto community; or venture funding attached to a hardware product, as with Foundation's $16.5 million. Community donations are not a funding model. Qubes raised $800 in two years.

### A second, faster track worth running in parallel

If you want something people use inside six months rather than two years, ship a hardened, immutable Linux **configuration** for crypto operations rather than a new distribution. Build on an existing atomic base, add Qubes-style separation between a network-facing qube and an offline vault, and specialize it for key handling and transaction review. You inherit the driver stack and the upstream security cadence, which are the two things that sank every project in section 1. Be honest in the naming that it is a configuration layer, not a new OS. It builds an audience and a track record while the signing OS matures.

### What not to do

Do not start a new AOSP fork intending to compete with GrapheneOS. You would be starting with worse upstream source access than they had in 2019, against ten paid developers with twelve years of accumulated hardening, a signed OEM partnership, and the only alternative-OS verified boot key enrolled in a working attestation service, on a hardware platform you cannot influence, with a build machine you do not own.

---

## Open questions this research did not settle

1. **Are GrapheneOS's claims about Fairphone accurate?** The secure-element and update-cadence assertions come from GrapheneOS's own forum and concern a competitor. They need checking against Fairphone's release notes, Qualcomm's chipset documentation, and kernel.org LTS dates.
2. **What did the Motorola partnership actually commit to?** Reporting spans a confirmed MWC 2026 announcement and an earlier article whose identification of Motorola was the author's inference. Device tiers, timing and whether GrapheneOS ships preinstalled are unresolved.
3. **Is the Coldcard attribution sound?** Galaxy's link rests on on-chain pattern analysis with no reconstructed seed matched to a drained address. The bug is documented; the $88.6 million figure is inferred.
4. **What does a signing-OS audience actually pay?** Passport Prime at $349 and Seeker at $500 bracket the range, but neither separates buyers who wanted security from buyers who wanted an airdrop or a brand.
5. **How much does attestation cost to run independently?** The server is self-hostable and the code is public, but nothing in these sources gives an operating cost or the effort to get a verified boot key enrolled for a new OS.

---

## Source list

**Project economics and funding**
- https://grapheneos.org/history/
- https://discuss.grapheneos.org/d/34369-original-grapheneos-responses-to-wired-fact-checker
- https://en.wikipedia.org/wiki/GrapheneOS
- https://blog.invisiblethings.org/2015/06/04/otf-funding-announcement.html
- https://www.qubes-os.org/news/2016/11/30/qubes-commercialization/
- https://www.redox-os.org/news/this-month-251231/

**Adoption and usability**
- https://www.techtimes.com/articles/322270/20260730/grapheneos-passes-daily-driver-test-most-apps-work-google-pay-permanently-blocked.htm
- https://android-developers.googleblog.com/2024/12/making-play-integrity-api-faster-resilient-private.html
- https://amosbbatto.wordpress.com/2025/11/02/lineageos-statistics/
- https://www.fairphone.com/stories/summing-up-our-2025-impact-report
- https://doc.qubes-os.org/en/latest/introduction/statistics.html

**Device strategy**
- https://grapheneos.org/faq
- https://discuss.grapheneos.org/d/24134-devices-lacking-standard-privacysecurity-patches-and-protections-arent-private
- https://www.howtogeek.com/grapheneos-is-coming-to-non-pixel-phones-thanks-to-motorola/
- https://piunikaweb.com/2026/01/26/grapheneos-hint-motorola-future-non-pixel-hardware-partner/
- https://stateofsurveillance.org/guides/advanced/open-source-phones/
- https://doc.qubes-os.org/en/latest/user/hardware/certified-hardware/certified-hardware.html

**Crypto-security niche**
- https://decrypt.co/336582/solana-seeker-review-more-measured-crypto-phone
- https://www.globenewswire.com/news-release/2026/05/22/3300208/0/en/foundation-announces-6-4m-round-and-availability-of-passport-prime-the-first-human-authority-hardware-device.html
- https://thehackernews.com/2025/04/triada-malware-preloaded-on-counterfeit.html
- https://thehackernews.com/2026/08/coldcard-hardware-wallet-flaw-linked-to.html

**Build and infrastructure**
- https://source.android.com/docs/setup/start/requirements
- https://source.android.com/docs/devices/cuttlefish/get-started
- https://grapheneos.org/build
- https://github.com/GrapheneOS/AttestationServer
- https://groups.google.com/g/android-building/c/G01E2O9egKw
- https://www.androidauthority.com/android-risk-based-security-updates-3597466/
