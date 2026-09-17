# Why Brilliant Operating Systems Fail

**An analysis of 143 technically excellent operating systems that never reached the mainstream, and what their failures mean for building a privacy and signing OS**

**Prepared for:** a blockchain security researcher deciding what kind of operating system to build
**Date:** 16 September 2026
**Evidence:** 143 distinct systems from 1960s timesharing to 2020s phones, gathered by ten research agents, analysed by sixteen failure-family analysts and one synthesis pass. Every system's full record is in [os-failures-corpus.md](os-failures-corpus.md).

> **How much to trust this.** Unlike the strategy report, this study had no adversarial verification pass. Facts were checked against primary retrospectives and postmortems by the agents that gathered them, but no independent skeptic tried to refute them. Failure tags are ordered judgement calls, so every percentage below is approximate. The corpus is also selected on failure by design. It can say how brilliant systems die. It cannot say how many narrow security systems quietly succeeded.

---

## The answer in one paragraph

Brilliant operating systems almost never died of bad engineering. A technical shortfall was the decisive cause in about 5% of the corpus, and even there the engineers were usually good. What killed them, in rough order, was an owner whose money came from somewhere else, a paying niche that captured the roadmap, a price charged at the exact moment someone decides to adopt, and no route onto the machines people actually buy. A missing app ecosystem appears in two-thirds of the records, but it is usually how the system died rather than why. The finding that matters most for you is narrower and more uncomfortable. **In this entire history, strong security reached ordinary people only as a hidden default inside someone else's volume product. No standalone security operating system has ever done it.**

---

## 1. The distribution

Each system carries an ordered list of failure causes. The first is the decisive one.

| Decisive cause, grouped into families | Systems | Share |
|---|---|---|
| The owner's incentives were elsewhere | 31 | 22% |
| No channel onto the machines people buy | 19 | 13% |
| Captured by a paying niche | 18 | 13% |
| Charged at the point of adoption | 16 | 11% |
| Tied to a roadmap or funder it did not control | 16 | 11% |
| No inherited software | 14 | 10% |
| Out of time: perpetual rewrite or arriving too late | 13 | 9% |
| **Technical shortfall or developer cost** | **9** | **6%** |
| Licensing or legal friction in the window | 7 | 5% |

Now the same systems counted by every cause that contributed, not just the decisive one.

| Contributing cause | Share of systems |
|---|---|
| No app ecosystem | 67% |
| Strategic mismanagement | 55% |
| Niche capture | 51% |
| Business model failure | 48% |
| No hardware channel | 43% |
| Incumbent lock-in | 30% |
| Technical shortfall | 27% |

Read the two tables together. A missing app ecosystem shows up in 67% of cases but is decisive in only 7%. It is the common final symptom. The apps never came because the owner would not commit, because there was no channel, or because the price was wrong. Technical shortfall moves the other way: it contributed in about a quarter of cases but decided almost none of them.

---

## 2. The nine ways brilliant systems die

Each family below gives the mechanism, the cases that show it most clearly, and who escaped. The warning signs for every family are gathered in section 5 so you can check your own project against them.

### 2.1 The owner's incentives were elsewhere — 22%

**Mechanism.** The operating system is never beaten on engineering. Its fate is decided by a company that earns its money somewhere else: hardware margins, an existing platform it cannot endanger, an alliance, or a bigger strategic bet. The owner confines the system to protect that other business. Then an event outside the system forces a discontinuity, and the owner, who loses almost nothing from a bad call, walks away. Each platform reset also taxes the credibility of the next one, because developers learn to discount promises.

**Cases.**
- **Mica, Digital Equipment Corporation, 1988.** Dave Cutler's team built an advanced operating system for the PRISM chip. DEC settled an internal architecture fight by cancelling the chip rather than moving the OS to whichever architecture won. Cutler left for Microsoft and the design lineage became Windows NT. The best OS DEC had went to its competitor.
- **Solaris on x86, 1993 to 2010.** Sun had ZFS, DTrace and Zones years ahead of Linux, and chose not to release a cheap, well-supported Solaris for commodity PCs because it would undercut SPARC hardware margins. Scott McNealy later said so in almost exactly those words.
- **Maemo and MeeGo at Nokia.** Positioned as "an internet tablet, not a phone" so as not to compete with Symbian. The N9 shipped in 2011, widely praised, after Nokia had already committed the company to Windows Phone.

**Who escaped.** Only two routes appear. An owner willing to cannibalise its own core product, as Apple did with NeXT and Microsoft did with NT. Or a licence that let the code outlive its owner, as with illumos after Oracle closed OpenSolaris and LineageOS after Cyanogen Inc.

### 2.2 No channel onto the machines people buy — 13%

**Mechanism.** An operating system is not a product people buy. It arrives on a machine someone else makes, sells and supports. Without a dependable route onto those machines, an OS falls back on users downloading and installing it. That brings a driver burden that grows with the hardware market while the team stays the same size, and growth that is linear rather than stepped.

**Cases.**
- **BeOS, 1998 to 2001.** Be offered its operating system free to PC makers. Microsoft's licensing terms still kept it off preloads. By the corpus account, Hitachi shipped BeOS on a machine but hid the partition. Being better and free was not enough to reach the shelf.
- **NeXT.** About 50,000 machines shipped over the entire hardware life of the company, roughly one week of Apple's sales at the time. The operating system was superb and locked inside an expensive workstation with no volume channel.
- **Linux netbooks, 2007 to 2009.** Linux's only structural advantage was price. Microsoft extended cheap Windows XP licences for low-cost PCs in 2008 and cancelled that advantage with a single pricing decision.

**Who escaped.** Nobody escaped by writing better software. Linux entered through commodity servers and embedded boards, markets with no incumbent tying preload to its OS, and reached ordinary consumers only through Google's Android and ChromeOS preloads. QNX and VxWorks chose markets where the device maker, not the end user, picks the OS.

### 2.3 Captured by a paying niche — 13%

**Mechanism.** This is rarely a commercial failure. It is a failure to ever become general. A rare, deep strength attracts a few wealthy buyers such as defence, banks, carmakers or retailers. Their requirements become the roadmap. Certification freezes releases. A separate secure edition and a closed licence shut out outside developers. Lock-in revenue makes the dashboard look healthy while the general market is lost for good.

**Cases.**
- **SCOMP, later XTS-400.** Honeywell's Secure Communications Processor was the only operating system ever rated A1, the highest level under the US government's old security evaluation criteria. It never left defence.
- **Trusted Solaris, 1990 to 2006.** Labelled mandatory access control shipped as a separate edition for sixteen years, with essentially no outside software vendors.
- **Commercial seL4.** The first formally verified general-purpose kernel was kept proprietary around 2009 to 2012 and routed through Open Kernel Labs and then General Dynamics, a defence contractor, rather than opened with a broad community.

**Who escaped.** The analysts found no system in this family that escaped as a general platform. QNX, IBM i and NonStop are commercial successes, but all succeeded inside their niche and none left it. The only escapes came early or through a different vehicle. SELinux entered mainline Linux as an optional, default-off module and later became enforcing on Android. Trusted Solaris stayed a separate edition and died.

**This family matters most to you.** A narrow signing OS is choosing a niche deliberately. Survival inside it is plausible. Leaving it for the general public has no precedent.

### 2.4 Charged at the point of adoption — 11%

**Mechanism.** The system worked, but the vendor collected money on the exact item buyers use to decide: a per-copy royalty, custom hardware, a runtime fee, a subscription or a per-device source licence. A free or bundled substitute kept improving. The cheaper option won the installed base, complementary software followed it, and the revenue model blocked the fix until the market had tipped.

**Cases.**
- **CP/M-86 against PC DOS, 1982.** CP/M-86 launched at $240. PC DOS cost $40. The better-established system lost.
- **Symbolics Genera.** Lisp machines costing tens of thousands of dollars in custom hardware, against Unix workstations that got good enough.
- **Inferno.** Lucent sold per-device source licences to manufacturers in the same years Sun made Java free and put it in the dominant browser.

**Who escaped.** Every escape moved the charge away from the adoption point before the market tipped. Microsoft took a low-margin per-machine deal with IBM and earned on every PC shipped instead. Red Hat split into a paid enterprise product and a free community one. Sony's PlayStation undercut the 3DO's $699 launch price by subsidising hardware.

### 2.5 Tied to a roadmap or funder it did not control — 11%

**Mechanism.** Survival depends on someone else's story. For hardware, that story is a chip or board roadmap. For funding, it is a grant window, a parent's balance sheet or a single state customer. The quality of the operating system has no bearing on the event that ends it.

**Cases.**
- **HP-UX on Itanium.** Stranded on a chip HP itself co-created, after Oracle stopped Itanium development in 2011 and HP ruled out moving to x86.
- **Nemesis, Cambridge.** A whole operating system with no application base, built inside a fixed-term European research grant measured by papers. When the grant ended, nothing existed to carry it forward.
- **Sailfish OS, Jolla.** Revenue concentrated on a Russian licensee paying for sovereignty. That became untenable after February 2022.

**Who escaped.** Survivors kept the ability to move before they needed to. Apple kept an Intel build of Mac OS X compiling quietly for years under the codename Marklar before switching in 2005. Windows NT was portable from birth. seL4 survived the Australian government's cut to its lab because defence customers had independent money, and it later moved into a neutral foundation.

### 2.6 No inherited software — 10%

**Mechanism.** A new system's value to a user is roughly its own merits multiplied by the software it runs, and at launch that software is close to zero. Where this is genuinely the root cause, the system's best idea was built into how every program had to be written, so the benefit only appeared in software rewritten for it. That turns a technical advantage into a porting bill for every developer.

**Cases.**
- **Genode and Sculpt OS.** Eighteen years of steady quarterly releases of a genuinely advanced capability-based framework. Essentially no ordinary users.
- **Rhapsody, Apple, 1997 to 1998.** Existing Mac software ran in a compatibility box nicknamed "the penalty box." At its developer conference in May 1998 Apple announced Carbon, an admission that no major developer would rewrite.
- **Windows RT, 2012.** Carried the Windows name while blocking third-party desktop software. Microsoft took a $900 million write-down.

**Who escaped.** Every escape broke the loop from outside the system. Windows NT ran old DOS and Windows software from day one. Mac OS X shipped Carbon and Classic. GrapheneOS keeps the Android app ecosystem. Qubes runs ordinary virtual machines. Each put its new model underneath existing software rather than replacing it.

### 2.7 Out of time: perpetual rewrite, or arriving too late — 9%

**Mechanism.** The architecture delivered nothing until nearly all of it worked. A revenue stream elsewhere meant no forcing deadline, so every lesson justified another redesign while the window closed. The usual verdict, "too ambitious," is imprecise. The actual killer was stacking more than one research risk on the critical path, with no committed adopter holding a date.

**Cases.**
- **Copland, Apple, 1994 to 1996.** Scope grew live on stage. At the 1996 developer conference little was running, and features were added in response to audience complaints. Apple cancelled it and bought NeXT.
- **GNU Hurd.** The GNU project had a complete userland and no kernel, so almost any working free kernel would have been adopted. Linux arrived in 1991. The Hurd was not usable for years afterwards.
- **Coyotos against seL4.** Coyotos stacked a new kernel, a new programming language and a new proof method, and never shipped. seL4 took one research risk, the proof, using ordinary C and an existing prover, and published its verification in 2009.
- **BlackBerry 10.** Delayed twice while enterprise buyers moved to the iPhone. The company later took an inventory charge reported in the corpus at $934 million.

**Who escaped.** They separated shipping from redesign. Apple stopped writing a kernel from scratch and bought one that already shipped.

### 2.8 Licensing or legal friction in the window — 5%

**Mechanism.** What mattered was not how strong a legal claim was, but how long it lasted relative to the adoption window. Restrictive terms filtered out exactly the people who compound a platform: students, rival vendors and forkers.

**Cases.**
- **BSD and the USL v. BSDi lawsuit, 1992 to 1994.** It ended with a handful of files removed out of thousands, but it ran for about two years, exactly while Linux took the free Unix slot.
- **Plan 9, Bell Labs.** University-only distribution in 1992, then $350 commercial licences in 1995. A free release arrived in 2000, eight years after Linux. Plan 9 gave the world UTF-8 and the idea behind Linux namespaces and containers, and never became anyone's main system.
- **NeWS, Sun.** Technically better than the X Window System. Sun declined to release it on open terms in 1987 and 1988, and X11 won.

### 2.9 Technical shortfall or developer cost — 6%

**Mechanism.** The explanation people reach for first ranks near the bottom. Where it did operate, the engineering was usually good, but the founding premise charged a tax on every single operation, or protection was an allowlist that one demonstration could break, or the cost of the model landed on people other than its authors.

**Cases.**
- **Mach 3.0.** Every service behind validated message passing. The corpus reports inter-process communication around 50% slower. "Microkernels are slow" outlived Mach by a decade, long after L4 had fixed the problem.
- **Subgraph OS, 2017.** A privacy-hardened desktop whose protection relied on per-application sandbox profiles. In April 2017 Micah Lee demonstrated an exploit through the unsandboxed file manager. The project never left alpha.
- **SELinux strict policy in Fedora Core 2, 2004.** Complete confinement, at a cost paid by users and administrators rather than by the policy authors. Fedora switched to targeted policy in the next release.

**Who escaped.** L4 kept the microkernel thesis and redesigned the hot path until the tax fell by an order of magnitude. OpenBSD's pledge and unveil put the cost of confinement on the author of each program, who understands it best.

---

## 3. What the survivors actually had in common

The systems that broke through, Windows NT, Linux, Mac OS X and iOS, Android and ChromeOS, and the durable partial survivors, QNX, KaiOS, seL4, SELinux, GrapheneOS, Qubes and illumos, share five traits. Only the first is technical.

1. **Good enough, not best.** NT, Linux and Android all launched cruder than contemporaries such as Mica, Plan 9, Sprite, webOS and BeOS.
2. **A volume channel they owned or were given.** Apple owned its hardware. Google licensed Android free to handset makers and carriers. Microsoft had per-processor PC maker licences. KaiOS had Reliance Jio.
3. **An inherited ecosystem.** NT ran DOS and Windows software. Mac OS X had Classic and Carbon. Linux had the GNU userland. GrapheneOS keeps Android apps.
4. **Money that did not come from the point of adoption.** Search and advertising for Android, hardware margin for iOS, a royalty on every PC for NT, support contracts for Linux, a foundation plus defence contracts for seL4, many donors and grants for GrapheneOS and Qubes.
5. **Timing.** They arrived at a platform shift, or retreated to a market where they were not late.

**Among security systems specifically, the only ones that reached mass scale were components or defaults chosen by a platform owner.** SELinux reached billions of people when Google made it enforcing in Android 5.0. An L4-family kernel reached billions inside Qualcomm modem firmware and Apple's Secure Enclave. OpenSSH ships with every operating system. None of those people chose a security OS. They got security inside something they already used.

---

## 4. The hardest lessons for a technically excellent builder

**1. Past "good enough," better engineering buys almost no adoption.**
EROS beat Linux on inter-process communication and process creation in published benchmarks in 1999 and was abandoned. BeOS was better than Windows 98 for media and could not get preloaded even when free. Multics was profitable when Honeywell cancelled it in 1985. *The implication for you:* the work you are best at and enjoy most, kernel hardening, formal properties and clever isolation, is the part history rewards least.

**2. Invisible security does not sell as a product.**
SCOMP, Trusted Solaris, Qubes and Subgraph all stayed niche or died. Bromium's buyers could not see what they were paying for, and it was absorbed into HP. *The implication:* a dedicated signing OS is exactly the form factor this history says stays niche. If the general public is the goal, the stronger bet is a component or mode inside a platform people already have.

**3. Something worse, adopted inside the window, beats something better that arrives after it.**
Linux over BSD, X11 over NeWS, MS-DOS over CP/M-86, a free Java over Inferno. *The implication:* a beautiful from-scratch kernel that ships in 2029 may be worth less than a hardened, minimal build on a borrowed foundation that ships in 2027.

**4. In a security product, credibility is spent once.**
Subgraph died after one public exploit and never left alpha. Repeated platform resets at Microsoft and Nokia taught developers to discount every later promise. *The implication:* this collides with lesson 3. You can be crude in features. You can never be crude in the trust claim. Overclaiming security early is more dangerous than shipping late.

**5. Closed or single-maintainer systems die with their maintainer.**
SkyOS was one developer selling closed betas; it died in 2009 and its source was never released. Midori's architect later named not open-sourcing it as his regret. Firefox OS was open, was forked into KaiOS, and reached over a hundred million devices. *The implication:* a solo builder has a bus factor of one by definition. In a key-custody product, abandonment strands users holding secrets on an unpatched system. Openness, reproducible builds and shared control of signing keys are safety design, not community management.

**6. Borrow almost everything and take at most one research risk.**
seL4 took one risk and shipped. Coyotos took three and did not. Haiku borrowed FreeBSD network drivers and survives; SkyOS wrote its own and died. GrapheneOS hardens Android rather than replacing it. *The implication:* writing your own kernel, language or cryptography for a solo signing OS repeats the most consistently fatal pattern in the record. Put the originality into the one thing only you can do: the signing trust path and the threat model.

**7. Whoever controls your hardware or its boot policy controls whether you exist.**
HP-UX and Itanium, IRIX and SGI's firmware, Danger and T-Mobile, BeOS and Apple's clone programme closing in 1997. *The implication:* a reference device you control solves the driver problem and puts you inside someone else's bootloader, verified boot or secure-element policy. GrapheneOS's dependence on Pixel boot policy is the modern version of this exposure.

**8. Mainstream breakthrough has almost always been bought with distribution or money a solo builder does not have.**
Linux reached consumers only through Google. NeXTSTEP needed Apple's $429 million purchase. KaiOS needed Reliance Jio. Microsoft paid developers to build for Windows Phone and still failed without volume. The grassroots survivors, Haiku, OpenBSD, Qubes and 9front, became respected niches, not mainstream platforms.

---

## 5. What this means for your operating system

### It revises the earlier recommendation, and you should see exactly how

The strategy report concluded: build a narrow, single-purpose signing and vault OS rather than a general phone OS or an Android fork. This study agrees with half of that and challenges the other half.

**Where it agrees.** Narrow scope, borrowed foundations and one research risk are strongly supported. A general-purpose "privacy OS for everyone" would walk into almost every failure family at once.

**Where it challenges.** A standalone security OS is precisely the form factor that has never reached ordinary people. So two of your original goals are in direct tension:

- *"Something the general public would use and really admire."*
- *"My own operating system."*

History says you can probably have one. A durable, respected, trusted niche is achievable as your own OS. Reaching the general public almost certainly requires being carried inside a channel someone else already owns.

**The refined recommendation: build the signing core as a portable component first, and treat your own OS as one vehicle for it, not the product.** Design the trust path, transaction decoding and signing protocol as a library plus a protocol that could ship inside GrapheneOS, inside a Qubes domain, on existing open signer hardware, or on your own minimal OS. If the OS stalls, the idea survives. If a volume platform wants it, you are ready. This directly answers lessons 2, 5 and 8.

Before writing kernel code, answer one question in writing: *which user gets a guarantee from a dedicated OS that they could not get from an app plus hardware isolation on a platform they already own?* If the honest answer is "air-gapped, auditable, what-you-see-is-what-you-sign signing with no general-purpose attack surface," the OS is justified. If not, contribute the idea to an existing platform instead.

### Risks the history identifies for your specific plan

| Risk | Severity | Precedent | Mitigation |
|---|---|---|---|
| Hardware wallet makers already hold this niche with retail channels and years of trust, and platforms are absorbing key custody through passkeys and secure enclaves | **Fatal** | OS/2 and DR-DOS were better at the incumbent's own job and lost | Do not pitch "a better hardware wallet." Pick a job incumbents cannot structurally promise, such as fully open and reproducible signing with independent transaction decoding on a trusted display |
| One public break or key-loss incident ends the project | **Fatal** | Subgraph OS, 2017 | Crude features, uncompromising trust boundary. No network stack in the signer. Independent audit and a funded bug bounty before any real funds. Pre-audit builds testnet only |
| Bus factor of one, with signing keys and infrastructure held by one person | **Fatal** | SkyOS, AtheOS, Coyotos stalling when its lead left | Open source from the first commit. Reproducible builds. Release keys under 2-of-3 control with a trusted co-maintainer. A documented exit path using standard seed and transaction formats so users can leave if the project dies |
| Your VM-first development plan quietly undermines the security premise | **Serious** | Subgraph's protection collapsed on the path it did not cover | A signer in a VM on a compromised host protects nothing. Label the VM build as development only and enforce that in the interface, refusing real seeds without an explicit override |
| No distribution channel, so growth is linear self-install | **Serious** | Qubes, Redox, BeOS free to OEMs and still not preloaded | Support one or two reference targets where everything works. Never chase PC drivers. Seek one borrowed channel, such as a bundle with a multisig coordinator or a flashable image for existing signer hardware |
| Scope creep and stacked research risks | **Serious** | Copland; Coyotos against seL4 | Freeze version one in writing: key generation and storage, deterministic display of what is signed, standard transaction signing, air-gapped transport, backup and restore. Nothing else |
| Dependence on one hardware vendor's boot or secure-element policy | **Serious** | HP-UX on Itanium; BeOS and Apple's clone programme | Build and boot on two architectures in continuous integration at all times. Keep hardware specifics behind a narrow interface |
| Funding concentration, or capture by institutional custody buyers wanting frozen certified builds | **Serious** | Sailfish and one licensee; SCOMP and Trusted Solaris frozen by certification | Never charge for the OS. Earn from reference hardware, support and several grant sources, capping any one at about a third. Certified builds as pinned configurations of mainline, never a separate edition |
| Incompatibility with the wallets people already use | **Manageable** | Genode and EROS required rewriting everything | Treat existing watch-only wallet coordinators as your app ecosystem. Support the standard transaction and seed formats. Publish a measured compatibility matrix from day one |

### The one gap worth building for

The analysis points to a demand window that incumbents structurally struggle to close: **blind signing.** In February 2025 roughly $1.5 billion was taken from the Bybit exchange after the wallet interface its signers used was tampered with, and hardware signers approved transactions they could not independently verify. A signer that decodes and displays exactly what is being signed, from a fully open and reproducible stack, is a job no hardware wallet with a closed secure element can fully promise. Whether that window is genuinely open or already closing is one of the open tensions below.

---

## 6. Check your own project against the warning signs

These are observable symptoms the analysts identified for each failure family, written so you can test your project against them. Revisit this list every quarter.

**Owner and politics**
- Can you name a customer or revenue line that depends on this OS succeeding in its own right?
- Have there been two or more platform-level resets in three years: a framework swap, a sponsor swap or a kernel swap?
- If one named person stopped tomorrow, is there a second person with the authority and motive to keep it alive?

**Channel**
- Can you name the specific company whose factory, store or default image will put your OS on a device the buyer did not choose for its OS? If the answer is "users will download and install it," you are already in this failure mode.
- Is your install count growing in a straight line rather than in steps?
- Is most engineering time going to drivers, firmware quirks and suspend and resume rather than to what makes you different?

**Niche capture**
- Does more than about half your funding come from one sector or fewer than ten customers?
- Can you state a benefit an ordinary user notices within five minutes, or is the strength visible only in a specification or an audit?
- Does running it require your hardware or a separate secure edition, rather than being a mode of something mainstream?

**Price**
- Does your revenue arrive at the same moment and on the same item that decides adoption?
- Has the platform beneath you shipped even a crude version of your core differentiator for free?

**Roadmap and funding**
- Have you built and booted on a second architecture in the last twelve months?
- Does any single source provide more than about 60% of your funding?
- Would anyone outside the team notice within a month if you stopped?

**Software**
- Can you show the core benefit using software people already run, or does the first useful demo need something written specifically for you?
- Do you publish install or active-user numbers?

**Time**
- Can you name a subset a real user could run daily within six to twelve months?
- Has the schedule slipped twice without scope shrinking?
- Is there more than one open research question on the critical path?
- Is there a named adopter with a deadline who is hurt if you slip?

**Technical**
- Does your core security mechanism run on the hot path, per system call, per frame or per packet?
- Is protection an allowlist that grows with the ecosystem? Can you name the single component whose compromise defeats the whole model, and is it confined?
- Can a competent outsider get a working result from your public documentation in under an hour?

---

## 7. Where the evidence genuinely points both ways

**Ship crude and early, or protect one-strike credibility.** The corpus says a worse system inside the window beats a better one outside it. For a security product, one public break killed Subgraph. No survivor clearly resolves this for key custody. "Crude features, uncompromising trust boundary" is an inference, not a demonstrated pattern.

**Narrow scope, or niche capture.** A closed, first-party task set avoids the app ecosystem trap, and systems survived that way in cars, on televisions and in point-of-sale terminals. But no system that settled into a niche ever left it for the mainstream. The move that makes survival likely makes the general public unlikely.

**Own the hardware, or avoid being tied to it.** Without a controlled device you pay an endless driver tax, and Apple shows owning the device is the strongest channel. But NeXT, Symbolics, BeBox and the Jolla Tablet show that owning low-volume hardware is itself a common killer.

**Compatibility, or guarantee.** Inheriting existing software saved NT, Mac OS X, Qubes and GrapheneOS. But for a vault, every compatibility surface, a browser, arbitrary wallet apps, USB, widens the trusted computing base that is the reason the product exists.

**Open early, or sustainable.** Openness is the only hedge that survived an owner's cancellation. Yet open, free distribution left Mandriva, elementary OS and Redox short of revenue, and QNX's owners repeatedly found closing the source commercially rational.

**Component, or platform.** Components reach enormous scale: L4 in modems and secure enclaves, SELinux in Android, OpenSSH everywhere. But they earn little and carry no identity. Open Kernel Labs was sold to a defence contractor and its office closed. Being carried inside someone else's channel may maximise your impact while making any sustainable project around it harder.

**Is the window open or closed?** High-profile blind-signing losses suggest a new demand window for verifiable signing. But hardware wallets have held the category for about a decade, and platform vendors are absorbing key custody. The evidence fits a latecomer entering a tipped market as well as an entrant at a platform shift. History shows the two are only distinguishable in hindsight.

## 8. Independent analysis by the project owner, and how it compares

Antics wrote a separate, unaided analysis of OS failures — IBM OS/2, BeOS, Apple Copland and Solaris, plus a two-line general conclusion — without reading this study. It is recorded here because three of its four judgements match the corpus, one of them sharpens a cause this study had recorded but not named well, and its general conclusion independently isolates the two most common contributing causes across all 143 systems.

**The general conclusion: "No hardware. No Software."** Measured across the corpus, those are exactly the two largest contributing tags: no app ecosystem 96/143 (67%), no hardware channel 62/143 (43%). Nothing else in the tag set comes close as a *pair*. The refinement this study adds is that neither is usually the root: no channel and no software are most often the *consequence* of an owner whose incentives lay elsewhere (decisive in 22%) or a business model that charged at the point of adoption (11%). OS/2 had IBM's own factories and still was not preloaded; Be offered BeOS to OEMs for free and still was not preloaded. So the operative form of the rule for this project is: *assume you will get neither hardware nor software, and design a product that does not need either.*

**Case 1 — OS/2. "Compatibility; strategic mistakes in marketing."** Confirmed, and the compatibility half is the more interesting claim because it runs the opposite way to the obvious reading. OS/2's problem was not that compatibility was missing; Win-OS/2 ran Windows 3.1 applications better than Windows did. That success removed every economic reason for an ISV to write a native OS/2 version, so the platform never grew software of its own. The OS/2 Museum states it directly: excellent DOS/Windows compatibility "discouraged vendors from developing native OS/2 applications, since their DOS/Windows versions already reached OS/2 users." The corpus tagged this `compatibility gap`, which understates it — **compatibility can be a cause of death by success, not only by absence**, and that framing is adopted here from Antics's note. The marketing half is also supported but is downstream: Garry Norris's 1999 DOJ testimony shows Microsoft tied IBM's Windows licence terms to IBM's promotion of OS/2, and IBM's own PC division declined to preload consistently.

**Case 2 — BeOS. "Extreme speed; couldn't run users' complex applications; installing the OS yourself."** Confirmed, and the third item is the decisive one. "Installing the OS yourself" is the channel failure stated in user terms: Be negotiated preloads with Dell, Compaq, Micron and Hitachi; the confidential Windows OEM licence forbade a machine carrying Microsoft's OS from offering a non-Microsoft OS as a boot option; only Hitachi shipped, with the BeOS partition hidden and buyers left to build boot floppies. The corpus's judgement and Antics's agree: the technical superiority was real (BFS, the Media Kit, ~15-second boot) and irrelevant without a factory preload.

**Case 3 — Copland.** This is the strongest of the four and the closest to this study's own finding. The "renovating a house while people are still in it" image captures the bind precisely: total reinvention plus an absolute compatibility constraint. The specific mechanism Antics identifies — every feature depending on other unfinished features, teams working in parallel with no integration discipline, an OS that existed as prototypes that could not be combined — is what Gil Amelio himself described as "just a collection of separate pieces, each being worked on by a different team ... that were expected to magically come together somehow." Worth adding to the account: Amelio added microkernel multithreading to the feature list live on stage at WWDC in May 1996, two years in, because attendees complained; and there were four successive slips with no narrowing of scope. Antics's closing line — "attempted a total reinvention while simultaneously trying not to break anything, and the result was that nothing worked and everything broke anyway" — is the `perpetual rewrite` family in one sentence.

**Case 4 — Solaris.** Confirmed on every technical point (ZFS snapshots, integrity and pooling; Zones as containers a decade before Docker; SMP scaling; SPARC tie) and on the conclusion that it did not fail technically. One correction of emphasis: the corpus evidence puts the decisive act *earlier* than the Oracle acquisition. Sun subordinated x86 to SPARC margins from 1993 onward — the x86 port was unstable until 2.4 in 1994 and repeatedly deprioritised after — so the vacancy Linux filled was created by Sun's own pricing strategy, not by Linux's rise. McNealy said it himself: "If we'd have just decided to release Solaris on metal instead of shrink wrapped, Solaris on Intel would have been a wild hit and nobody would have done Linux." The Oracle acquisition (closed 27 Jan 2010) and the OpenSolaris shutdown (13 Aug 2010) finished a platform whose desktop position had been lost fifteen years earlier. So: right that acquisition and shifting priorities killed it; the underlying cause is `owner's incentives elsewhere` — Sun protecting hardware margins against its own software.

**What the four cases do not cover.** The corpus's other decisive families are absent from this sample, and each is a live risk for this project: charged at the point of adoption (11% — CP/M-86 at $240 against PC DOS at $40); captured by a paying niche (13% — QNX, Trusted Solaris, and every security system that found a defence or enterprise customer and stopped there); tied to a funder or roadmap not controlled (11%); and licensing or legal friction inside the adoption window (5% — Plan 9, and the BSD lawsuit that cost BSD the years Linux used). Adding those to the "No hardware, no software" rule gives the full check list in §6.
