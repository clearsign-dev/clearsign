# Corpus: 143 brilliant operating systems that never reached the mainstream

Gathered 16 September 2026 by ten research agents, one per era or category, checking facts against retrospectives, postmortems, oral histories and court records. The raw run produced 151 records; systems gathered by more than one agent are merged here, leaving 143 distinct systems. Failure tags are ordered judgement calls, so treat every share as approximate.

Companion to [os-failures-analysis.md](os-failures-analysis.md), which explains the patterns.

## Decisive cause, by family

| Family | Systems | Share |
|---|---|---|
| The owner's incentives were elsewhere | 31 | 22% |
| Captured by a paying niche | 18 | 13% |
| Charged at the point of adoption | 16 | 11% |
| No channel onto the machines people buy | 19 | 13% |
| Tied to a roadmap or funder it did not control | 16 | 11% |
| No inherited software | 14 | 10% |
| Out of time: perpetual rewrite or too late | 13 | 9% |
| Licensing or legal friction in the window | 7 | 5% |
| Technical shortfall or developer cost | 9 | 6% |

## Every cause that contributed

A system usually carries several tags, so these add to far more than 100%.

| Contributing cause | Systems | Share |
|---|---|---|
| no app ecosystem | 96 | 67% |
| strategic mismanagement | 78 | 55% |
| niche capture | 73 | 51% |
| business model failure | 68 | 48% |
| no hardware channel | 62 | 43% |
| incumbent lockin | 43 | 30% |
| technical shortfall | 39 | 27% |
| hardware tied to dying platform | 36 | 25% |
| internal politics | 35 | 24% |
| compatibility gap | 35 | 24% |
| too late to market | 34 | 24% |
| funding collapse | 28 | 20% |
| licensing or legal | 25 | 17% |
| perpetual rewrite | 23 | 16% |
| acquired and killed | 22 | 15% |
| poor developer experience | 18 | 13% |

---

## The owner's incentives were elsewhere (31)

### A/UX
*1988–1995 · Apple Computer*

**What was brilliant.** A/UX 3.0 demonstrated in November 1991 the exact architecture Apple would not ship for another decade: the complete Macintosh System 7 environment, Finder included, running as ordinary processes on a real standards-compliant Unix, with Macintosh applications, Unix command shells and X11 clients coexisting on one desktop simultaneously. Mac applications got cooperative access to the Toolbox through a token-passing scheme in shared memory while the Unix kernel underneath provided preemption and memory protection — structurally the same Classic-on-Unix arrangement Apple shipped in Mac OS X in 2001. The base was SVR2.2 with SVR3, SVR4 and 4.2/4.3BSD features, POSIX and SVID compliant, and it reached C2 security certification in April 1992.

**What happened.** 1.0 February 1988; 1.1 1989; 2.0 mid-1990; 3.0 November 1991; final release 3.1.1 in 1995. Never ported to PowerPC — Apple placed its Unix-on-PowerPC bet on the AIM alliance's PowerOpen/AIX instead, which was itself discontinued in 1995 — and A/UX 4.0 was cancelled. Apple's Unix strategy restarted from zero with the NeXT purchase announced 20 December 1996; Mac OS X Server 1.0 shipped 16 March 1999, more than seven years after A/UX 3.0 had shown the same idea working on shipping hardware.

**Why it failed.** Apple treated A/UX as a federal-procurement checkbox rather than as the future of the Macintosh, so when the 68k line ended there was no internal product whose roadmap depended on it and nobody would fund the PowerPC port — it was killed by indifference rather than by a decision.

**What might have changed it.** If Apple had ported A/UX to PowerPC in 1993–94 and designated it the basis of the modern Mac OS — real Unix underneath, System 7 on top — Copland is unnecessary, the Taligent and NeXT detours never happen, and Apple ships the Mac OS X architecture roughly seven years early on technology it already owned.

**Evidence.** A/UX 3.0 ran the System 7.0.1 Finder as a Unix process alongside CommandShell and an X server, using token-passing to give Mac applications cooperative Toolbox access — which is the Classic environment, built and shipping in 1991. The C2 certification in April 1992 identifies the customer Apple was actually selling to: government procurement, not Macintosh users. Apple's own subsequent behaviour is the strongest evidence of the misjudgement: having cancelled A/UX 4.0 and let the PowerOpen/AIX path die in 1995, Apple spent $400M-plus in December 1996 to buy an outside company in order to obtain the Unix-underneath-a-Mac-UI architecture it had already built, shipped and abandoned.

Tags: strategic mismanagement, hardware tied to dying platform, niche capture, no app ecosystem, business model failure

Sources:
- https://en.wikipedia.org/wiki/A/UX
- https://gunkies.org/wiki/A/UX
- https://apple.fandom.com/wiki/A/UX
- https://eclecticlight.co/2024/12/07/a-brief-history-of-mac-servers/

### AmigaOS (Exec / Intuition / AmigaDOS)
*1985–1994 (Commodore era); maintained commercially as 3.2 / 4.1 into 2025 · Commodore-Amiga, Inc. — Exec microkernel by Carl Sassenrath; AmigaDOS derived from MetaComCo's TRIPOS (BCPL) by Tim King; Intuition by RJ Mical*

**What was brilliant.** A real message-passing microkernel with prioritised preemptive round-robin scheduling on a 7.16 MHz 68000 with no MMU — Sassenrath says the first machine processed on the order of 10,000 messages per second. The whole OS (scheduler, dynamic shared libraries, device drivers, graphics, GUI, filesystem) fitted in a 256 KB Kickstart ROM. Shared libraries were reached through negative-offset jump tables off a library base pointer, so any function — even ROM-resident — could be patched at runtime. Autoconfig on the Zorro bus gave the Amiga automatic address/resource assignment for expansion cards in 1985, years before PCI did the same on PCs. ARexx (Hawes, 1987; bundled from OS 2.0 in 1990) made every application scriptable via named message ports — an inter-application scripting bus. Datatypes (3.0, 1992) let any application open any format the system had a datatype for. Preemptive multitasking shipped a decade before Windows 95 and sixteen years before Mac OS X.

**What happened.** Commodore announced voluntary liquidation on 29 April 1994 after an $8.2M US quarterly loss. Assets passed to Escom AG (1995), Gateway 2000 (1997), Amiga Inc. (1999); AmigaOS 4 was contracted to Hyperion Entertainment (2001) and shipped PowerPC-only in 2004. The IP remains fragmented between Amiga Inc., Cloanto and Hyperion and has been repeatedly litigated. AmigaOS 3.2 (May 2021) and 4.1 Final Edition updates (18 October 2025) still ship to a hobby market.

**Peak adoption.** ~4.85 million Amigas (Amiga Format, June 1993) to ~4.91 million (Stuart Brown's audit); ~1.5M UK, ~1.5M Germany, ~700,000 North America

**Why it failed.** Commodore's owner-management extracted cash instead of funding the chipset and OS roadmap, so the hardware AmigaOS was welded to stopped advancing after 1990 — and because the OS was never ported off that hardware or licensed to clone makers, it died with the company rather than outliving it.

**What might have changed it.** Had Thomas Rattigan not been forced out on 22 April 1987 — he had returned Commodore to profit and had a US marketing plan — or had AmigaOS been ported to a commodity 68030/x86 platform and licensed around 1990, the OS survives the hardware. The Amiga's professional-video niche (NewTek Video Toaster) proves there was a business independent of the consumer box.

**Evidence.** Forbes analyst Evan McGlinn attributed the collapse to 'the absentee-landlord management style of globe-trotting chairman and chief executive Irving Gould.' Commodore executives continued to appear on the Philadelphia Inquirer's highest-paid list while the firm bled; Commodore UK MD David Pleasance called a major product initiative a 'complete and utter screw-up.' Sassenrath's own account is that the message-passing design was a necessity of fitting 'dozens of internal libraries and devices... graphics, sound, GUI, floppy disc, file systems' into ROM. AmigaOS never gained memory protection, which capped its credibility for professional work even as it won the video/DTV niche.

Tags: strategic mismanagement, no hardware channel, business model failure, internal politics, technical shortfall, niche capture

Sources:
- https://en.wikipedia.org/wiki/AmigaOS
- https://en.wikipedia.org/wiki/Amiga
- https://en.wikipedia.org/wiki/Commodore_International
- https://en.wikipedia.org/wiki/Exec_(Amiga)
- https://www.generationamiga.com/2026/05/28/carl-sassenrath-the-engineer-behind-the-amigas-revolutionary-multitasking-kernel/

### Android Things (Project Brillo)
*2015–2022 · Google*

**What was brilliant.** The only serious attempt to give IoT devices the thing they most conspicuously lack — a vendor-managed security update pipeline. Google stripped the phone-specific layers out of the Android framework, added Peripheral I/O APIs so Java and Kotlin developers could drive GPIO, I²C, SPI, PWM and UART lines directly from managed code, and then took over the operating system itself: Google built, signed and delivered OTA updates to every Android Things device for three years, from a console where an OEM assembled a build against Google-maintained board support packages for certified NXP, Qualcomm and MediaTek system-on-modules. The division of labour was genuinely new — the OEM shipped hardware and an application, Google shipped and patched the OS. Brillo, its predecessor, targeted devices with as little as 32–64 MB of RAM, and Weave supplied a common device-to-cloud schema and provisioning protocol.

**What happened.** Announced as Brillo at Google I/O in May 2015; renamed Android Things in December 2016; version 1.0 general availability in May 2018 with a three-year update commitment. Shipping consumer products were confined to smart displays and speakers on Qualcomm's 'Home Hub' platform — Lenovo Smart Display, LG XBOOM AI ThinQ, JBL Link View. In February 2019 Google dropped support for resource-constrained hardware and restricted the platform to OEMs building smart speakers and displays, ending the hobbyist and general-IoT story. The console stopped accepting new projects on 5 January 2021 and was shut down entirely on 5 January 2022, deleting build configurations and factory images.

**Why it failed.** Google's three-year managed-update promise meant Google carried the cost and the liability of every device an OEM shipped, which forced it to restrict the platform to a short list of expensive certified system-on-modules — pricing it out of the cheap, high-volume, wildly heterogeneous hardware that actually constitutes the IoT — and once the platform had been narrowed to smart displays, Google could serve those with ordinary Android and had no remaining reason to keep it.

**What might have changed it.** If Google had shipped Android Things as an unmanaged, self-certifiable OS runnable on commodity boards in the Raspberry Pi and ESP32 price band, with managed updates as an optional paid tier rather than a mandatory Google obligation, it would have captured the developer base that Arduino, ESP-IDF and Raspberry Pi OS took instead — and IoT security would look materially different.

**Evidence.** Brillo's original target was 32–64 MB of RAM; the February 2019 refocus dropped low-power hardware entirely and moved to 'smartphone-class devices', which is the moment the platform stopped being an IoT OS. After that refocus the only hardware permitted was commercial SoMs from NXP, Qualcomm and MediaTek 'available to specific OEM partners building smart speakers and smart displays'. The console shutdown on 5 January 2022 permanently deleted customers' build configurations and factory images — the update pipeline that was the product's entire reason to exist was itself unmaintained.

Tags: strategic mismanagement, business model failure, no hardware channel, technical shortfall

Sources:
- https://en.wikipedia.org/wiki/Android_Things
- https://9to5google.com/2020/12/17/android-things-shutdown/
- https://www.androidauthority.com/google-android-things-shutdown-1186745/
- https://www.xda-developers.com/google-android-things-shut-down-2021/
- https://www.microej.com/news/what-happened-to-android-things-why-did-google-kill-its-iot-os/

### Apple GS/OS
*1988–1993 (GS/OS in System Software 4.0, 1988; System 6.0, March 1992; 6.0.1, March 1993) · Apple Computer — Apple II Systems Software group*

**What was brilliant.** The first fully 16-bit native 65816 operating system, replacing ProDOS 16's 8-bit shim and delivering markedly faster disk access, loading and screen work as a result. Its defining feature was File System Translators — pluggable modules that let any application read ProDOS, Macintosh HFS, MS-DOS FAT, ISO 9660/High Sierra, Apple DOS 3.3 and Apple Pascal volumes transparently, with no application awareness. That is a pluggable VFS layer on a consumer 8/16-bit machine in 1988, years before it became normal on desktop systems. It combined this with hot-installable device drivers (printer, modem, network) that did not require replacing the OS, a resource fork, a Finder, loadable fonts and a system loader handling relocatable segmented 65816 code — all at 2.8 MHz.

**What happened.** Apple held the IIGS's 65C816 at 2.8 MHz for its entire six-year production run despite the part being certified to 4 MHz and 5–14 MHz versions being available. Apple's own developer technical support advised developers not to write new Apple II or IIGS applications. The finished ROM 04 'Mark Twain' upgrade was pulled from Apple's September 1991 satellite broadcast at the last minute, which pushed System 6.0 out to March 1992. Production ceased 4 December 1992; System 6.0.1 (March 1993) shipped without the promised Ethernet support and was the end.

**Peak adoption.** about 1.25 million Apple IIGS units

**Why it failed.** Apple deliberately subordinated the IIGS to the Macintosh — capping the clock at 2.8 MHz, killing the ROM 04 upgrade, and steering developers away — so GS/OS was starved of the hardware performance and application base it needed by its own vendor, not by any competitor.

**What might have changed it.** Shipping the ROM 04 machine in late 1991 with a faster 65816, and giving the line an executive champion, plausibly buys GS/OS two or three more years and a coherent low-cost colour-GUI position beneath the Macintosh LC — the exact slot Apple was trying to protect.

**Evidence.** Apple representatives justified ending IIGS development on the grounds that further enhancements 'would take away sales of the Macintosh LC, the new consumer color computer.' Apple's DTS staff specifically recommended that new applications not be created for the Apple II or IIGS. The apple2history account of the ROM 04 cancellation notes the project 'did not have someone in a position of power at the company who would champion the machine.' The 65C816 was certified to 4 MHz and shipped at 2.8 MHz for six years.

Tags: internal politics, strategic mismanagement, hardware tied to dying platform, no app ecosystem, niche capture

Sources:
- https://www.apple2history.org/history/ah11/
- https://en.wikipedia.org/wiki/Apple_IIGS
- https://en.wikipedia.org/wiki/Apple_GS/OS
- https://dfarq.homeip.net/apple-iigs-released-september-1986/
- https://6502disassembly.com/a2-gsos/

### Atari 8-bit OS (CIO / SIO ROM)
*1979–1992 (Atari 400/800 through XE/XEGS) · Atari, Inc. home computer division; the SIO peripheral bus designed by Joe Decuir*

**What was brilliant.** In 10 KB of ROM the 400/800 OS implemented CIO, a fully device-independent named-device I/O layer: a program opened 'E:', 'S:', 'P:' or 'D1:FILE' through one call interface, and the handler table was replaceable, so code written against the screen editor worked unchanged against a printer or a disk. Beneath it, SIO was a daisy-chained serial peripheral bus with checksummed frames and a five-byte command packet carrying a device ID, and peripherals could upload their own ROM-based drivers to the host at boot — genuine plug-and-play peripheral autoconfiguration in 1979, fourteen years before ISA Plug and Play. SIO's designer Joe Decuir went on to co-design USB and credits SIO as its basis; Microsoft and others cited SIO as prior art when a patent assertion entity sued the USB Implementers Forum.

**What happened.** Warner sold Atari's home computer division to Jack Tramiel in July 1984. Under Tramiel the 8-bit line was kept alive only as a price-fighter (XL, XE, XEGS) while all engineering went to the ST; Atari Corporation formally dropped all remaining support for the 8-bit computers at the beginning of 1992.

**Peak adoption.** approximately 4 million units across the 400/800/XL/XE line (widely cited, not audited), against 12.5–17 million Commodore 64s

**Why it failed.** The OS lost on the price of the box it was welded to: Warner positioned the 400/800 as premium hardware while Commodore, which owned its own chip fabs, drove the C64 to $199 — vertical integration beat architecture, and no I/O abstraction could offset a 2:1 price gap.

**What might have changed it.** Matching Commodore's vertical cost structure (or publishing full hardware and OS internals at launch rather than withholding technical documentation into 1982, which delayed third-party software) plausibly makes CIO/SIO the 8-bit architecture that wins, since it was the better-engineered of the two by a wide margin.

**Evidence.** SIO's driver-upload and daisy-chain design in 1979 predates comparable PC plug-and-play by well over a decade, and Decuir's USB lineage claim is corroborated by the USB-IF prior-art defence. Atari then inflicted its own compatibility gap: OS revisions A and B on the 400/800 and the incompatible XL/XE ROMs broke existing software and cost the platform developer goodwill.

Tags: strategic mismanagement, business model failure, no hardware channel, hardware tied to dying platform, compatibility gap

Sources:
- https://en.wikipedia.org/wiki/Atari_8-bit_computers
- https://en.wikipedia.org/wiki/Atari_SIO
- https://atari-owner.com/club/articles/atari-8-bit-units-sold.23/

### Chorus / ChorusOS
*1979–1986 research at INRIA; Chorus Systèmes SA 1986–1997; ChorusOS at Sun 1997–~2001; Jaluna 2002, VirtualLogix 2006, Red Bend 2010 · Hubert Zimmermann and Michel Gien; research begun at INRIA in 1979, commercialised via Chorus Systèmes SA (founded 1986, Saint-Quentin-en-Yvelines, France)*

**What was brilliant.** Chorus was the only microkernel of its generation that shipped in revenue-bearing production systems at scale, and it did so while holding hard real-time deadlines and binary UNIX compatibility simultaneously. CHORUS/MiX ran System V.4 as a set of user-space subsystem servers over a nucleus whose IPC was location-transparent across a network, so a distributed telecom switch looked like one machine. Critically, it offered a pragmatic escape hatch Mach never did: 'supervisor actors' could be loaded into the kernel's address space for latency-critical paths, letting engineers trade isolation for determinism per-component rather than per-system. That is why it ended up under telephone exchanges, where a missed deadline is a contractual failure.

**What happened.** By 1997 Chorus Systèmes had roughly $10M in revenue and customers including Alcatel-Alsthom, Lucent Technologies, Matra and Motorola, with earlier deployments at GEC Plessey Telecommunications (System X digital switching), Unisys, Acorn and ICL. Sun Microsystems announced the acquisition in September 1997 and closed it on 21 October 1997 for $26.5 million, folding the technology into its Embedded Systems Software group as ChorusOS and targeting it at JavaOS and set-top boxes. Sun's embedded and JavaOS strategy collapsed; ChorusOS was open-sourced and then discontinued, with Oracle later dropping it entirely. The founders left and re-founded the business twice to keep the code alive: Jaluna (August 2002), renamed VirtualLogix (September 2006), acquired by Red Bend Software (September 2010).

**Peak adoption.** $10M annual revenue at the time of acquisition (1997), up from $6.5M in 1990; ~30 employees in 1989. Deployed in telecom switching infrastructure at Alcatel, Lucent, Matra, Motorola, GEC Plessey. No unit or installed-base figure is documented.

**Why it failed.** Chorus sold itself to a workstation company that wanted the microkernel as a Java delivery vehicle rather than as an operating-system business; Sun's JavaOS and embedded strategy died within four years and took a profitable, field-proven microkernel product line with it.

**What might have changed it.** Stay independent, or sell to a telecom or automotive buyer instead of Sun. Chorus' 1997 customer list — Alcatel, Lucent, Motorola, Matra — was already the exact customer base that, over the following decade, made QNX and Wind River into the embedded RTOS incumbents. The technology was not the constraint; the acquirer's strategy was.

**Evidence.** Acquisition announced September 1997, closed 21 October 1997, price $26.5 million. Revenue trajectory $6.5M (1990) → $10M (1997). Ownership mid-1991: founders and employees 63%, Innovacom 16%. The strongest evidence that the technology was not at fault is that the same founders had to restart the business twice (Jaluna 2002, VirtualLogix 2006) simply to keep shipping a product Sun had abandoned, and that product line survived two further acquisitions.

Tags: acquired and killed, strategic mismanagement, business model failure, niche capture

Sources:
- https://en.wikipedia.org/wiki/Chorus_Syst%C3%A8mes_SA
- https://en.wikipedia.org/wiki/ChorusOS
- https://techmonitor.ai/techonology/sun_betaresearch_in_chorusos_set_top_box_deal
- https://en.wikipedia.org/wiki/VirtualLogix
- https://en.wikipedia.org/wiki/Red_Bend_Software

### ChromeOS on tablets (Andromeda, Acer Chromebook Tab 10, Pixel Slate)
*2015–2019 (convergence conceded to Android, 2025) · Google*

**What was brilliant.** ChromeOS itself is an excellent OS design — verified boot with a TPM-rooted chain and two-slot A/B images, automatic background updates with rollback, a read-only rootfs, per-origin sandboxing — and by 2018 it was running three userspaces on one kernel simultaneously. ARC++ ran Android in a container sharing the ChromeOS kernel through namespaces rather than a VM, so Android apps got near-native performance and real GPU access. Crostini ran a full Debian userspace inside a crosvm/KVM virtual machine, with a virtio-wl Wayland proxy so Linux GUI applications composited directly into the ChromeOS window manager, security boundary intact. Hosting Android and Linux natively and securely on one kernel with a shared compositor is real systems work, and the Pixel Slate hardware (3000×2000 display, Titan C security chip) was first rate.

**What happened.** The Wall Street Journal reported in October 2015 that ChromeOS would be folded into Android by 2017; Google publicly denied it. The internal merge, Andromeda, and its 'Bison' laptop were cancelled around June 2017. The tablet push instead ran through Acer's Chromebook Tab 10 (March 2018) and Google's own Pixel Slate (9 October 2018, $599–$1,599), which reviewers — including friendly ones — found laggy and unfinished in tablet mode. On 20 June 2019 hardware SVP Rick Osterloh confirmed Google had cancelled two in-development ChromeOS tablets and was exiting first-party tablets entirely; Pixel Slate listings were removed on 20 January 2021. Google then shipped an Android tablet (Pixel Tablet, 2023), and in July 2025 confirmed that ChromeOS will merge into Android, with Android-based 'Aluminium OS' announced in September 2025 for 2026 — the 2015 WSJ story arriving a decade late and pointed the other way.

**Peak adoption.** ChromeOS overall reached roughly 10.8% of worldwide end-user computer shipments in 2020, outselling macOS — but that is the laptop business. ChromeOS tablets specifically never had disclosed sales and held effectively no share; Google discontinued the category within eight months of the Pixel Slate's launch.

**Why it failed.** ChromeOS's touch story depended on Android applications running in a container built for a laptop window manager, so tablet ChromeOS inherited Android's tablet-app drought and Chrome's mouse-first input model at the same time — and each time Google faced the resulting two-operating-system problem it cancelled the hardware rather than finishing the convergence, in 2017 and again in 2019, before conceding in 2025 that the merge should have run in the opposite direction.

**What might have changed it.** If Google had shipped Andromeda in 2017 — one OS with Android's application catalogue and ChromeOS's update and security model — it would have had in 2017 the convergence platform it then spent eight years deciding to build anyway, and the iPad would have faced a credible competitor while the tablet market was still forming.

**Evidence.** Three separate reversals on the same question: WSJ reports the merge (Oct 2015) and Google denies it; Andromeda and Bison cancelled (June 2017); Osterloh confirms cancellation of two unreleased tablets and exit from first-party tablets (20 June 2019), with tablet staff moved to the Pixelbook laptop team; then Google confirms the ChromeOS-into-Android merge (July 2025) and announces Aluminium OS at the Snapdragon Summit (September 2025). Pixel Slate reviews converged on the same defect — praise for hardware and Chrome, complaints about tablet-mode lag affecting all ChromeOS devices, not just this one.

Tags: strategic mismanagement, internal politics, perpetual rewrite, no app ecosystem, too late to market

Sources:
- https://en.wikipedia.org/wiki/ChromeOS
- https://en.wikipedia.org/wiki/Pixel_Slate
- https://www.digitaltrends.com/computing/google-cancelling-andromeda-chrome-os-android-mashup/
- https://www.digitaltrends.com/phones/google-ends-production-of-new-chrome-os-tablets/
- https://hothardware.com/news/google-pixel-slate-successor-cancelled

### CyanogenMod / Cyanogen OS
*2009–2016 (continues as LineageOS) · Stefanie Jane (who founded the project as Steve Kondik) and the CyanogenMod community; commercialised by Cyanogen Inc. (Kirt McMaster, CEO) from 2013*

**What was brilliant.** The largest sustained third-party operating-system distribution in history, and in practice Android's external R&D lab. Per-application permission revocation shipped as Privacy Guard in 2013 and landed in AOSP as runtime permissions in Android 6.0 (2015); quick-settings notification toggles, the theme engine, the DSP equaliser, lock-screen profiles and FLAC support all came from CyanogenMod first. The harder achievement was structural: maintaining one common Android tree across hundreds of SoCs with no vendor co-operation, publishing device trees and reverse-engineered HAL shims for vendor blobs, and keeping thousands of OEM-abandoned handsets patched and current for years after their manufacturers stopped — the update problem Google is still trying to solve with Treble and Mainline.

**What happened.** Google issued a cease-and-desist in September 2009 over bundled Google Apps; the resolution — unbundling proprietary apps into a separately flashed 'gapps' package — is still how every Android distribution works. Cyanogen Inc. founded September 2013 with $7M Series A (Benchmark); $80M Series C in March 2015 (Premji Invest, Twitter Ventures, Qualcomm, Telefónica, Rupert Murdoch) at roughly $1B valuation, ~$110–115M raised in total; Microsoft invested and struck a services deal in January/April 2015. Cyanogen OS shipped on Oppo N1, OnePlus One, Micromax Yu, Wileyfox, BQ, Lenovo ZUK and Alcatel. About 30 of 136 staff cut in July 2016, Seattle office closed; McMaster out in October 2016, the founder in November. All services and nightlies ended 31 December 2016; the community forked LineageOS on 24 December 2016. The shell became Cyngn, an autonomous-driving company.

**Peak adoption.** Over 50 million users reported in March 2015 (Forbes), derived from opt-in telemetry and therefore a floor, not a ceiling.

**Why it failed.** Cyanogen's commercial thesis required OEMs to ship Android without Google Mobile Services, but no OEM could sell a phone without Play Services in any market that mattered — so the company had to ship Google-certified builds while telling the press it was destroying Google, and then traded its only real asset (neutrality and developer goodwill) for a single territorial exclusive with Micromax that legally blocked its best-selling partner.

**What might have changed it.** If Cyanogen had sold itself as a neutral, non-exclusive OEM Android layer — 'Android without the OEM skin, still Play-certified' — and never signed a territorial exclusive, both OnePlus and Micromax would have shipped Cyanogen OS through 2015–2016 and the company would have had a real device base instead of a lawsuit and two lost partners.

**Evidence.** McMaster's public positioning — 'I'm the CEO of Cyanogen. We're attempting to take Android away from Google', and the 'bullet to the head' line — was made while the company's shipping products depended on Google certification. The Micromax exclusive covering India, Bangladesh, Sri Lanka, Nepal, Pakistan and Myanmar produced a Delhi High Court injunction on 16 December 2014 barring OnePlus One sales in India (lifted on appeal within days, but the OnePlus relationship was over). The founder's own post-mortem: 'My co-founder apparently became unhappy with running the business and not owning the vision. This is when the "bullet to the head" and other misguided media nonsense started, and the bad business deals were signed.'

Tags: strategic mismanagement, internal politics, business model failure, incumbent lockin, licensing or legal

Sources:
- https://en.wikipedia.org/wiki/CyanogenMod
- https://en.wikipedia.org/wiki/Cyanogen_Inc.
- https://www.androidauthority.com/steve-kondik-cyanogen-inc-kirk-mcmaster-statement-733274/
- https://www.forbes.com/sites/miguelhelft/2015/03/23/meet-cyanogen-the-startup-that-wants-to-steal-android-from-google-2/
- https://www.androidauthority.com/oneplus-one-india-575171/

### DYNIX and DYNIX/ptx
*1984–2002 (DYNIX 1984; DYNIX 3.0 May 1987; DYNIX/ptx from c. 1992; NUMA-Q 1996; IBM acquired Sequent July 1999; discontinued May 2002) · Sequent Computer Systems — founded 1983 in Beaverton, Oregon by seventeen engineers and executives who left Intel after the iAPX 432 was cancelled*

**What was brilliant.** Sequent solved symmetric multiprocessing years before the rest of the industry had to. The Balance 8000/21000 (1984–86) ran up to 12 and then 30 NS32032 processors; the Symmetry line scaled 386, 486 and Pentium SMP into the 30-CPU range when the mainstream argument was whether two processors were worth the complexity. DYNIX/ptx — a merge of System V onto the BSD-derived DYNIX base — is where the hard problems of parallel Unix were actually solved: fine-grained kernel locking, per-processor data structures, and decisively read-copy-update, invented at Sequent by Paul McKenney and John Slingwine, a synchronisation mechanism in which readers proceed with zero locking and zero atomic operations. NUMA-Q (1996, internally 'STiNG') assembled quad-Pentium SMP bricks into cache-coherent NUMA systems, with DYNIX/ptx scheduling and placing memory across the topology. Sequent owned the high end of the Unix database market in partnership with Oracle.

**What happened.** IBM acquired Sequent in July 1999 for $810M, intending to merge DYNIX/ptx with AIX and UnixWare under Project Monterey (with SCO and Intel). Monterey was quietly abandoned around 2001 as Linux took the volume; IBM stopped selling Sequent-heritage NUMA-Q hardware and DYNIX/ptx by May 2002 on declining sales. IBM then contributed Sequent's NUMA and RCU work into the Linux kernel, where RCU is now used in tens of thousands of call sites. SCO responded in spring 2003 by revoking IBM's Dynix licence and alleging IBM had leaked 148 files and 168,276 lines of Dynix code into Linux — a central claim of SCO v. IBM.

**Why it failed.** DYNIX/ptx was a high-end server Unix with no desktop, no volume channel and no consumer presence, and IBM bought Sequent for its NUMA engineers and patents rather than for its operating system — so the OS was cancelled within three years while its best ideas were transplanted into AIX and, decisively, into Linux.

**What might have changed it.** Sequent stays independent, or IBM keeps DYNIX/ptx as its x86 Unix instead of defending AIX on Power. The engineering was never the problem: the survival of RCU and NUMA scheduling as core Linux infrastructure proves the work was a decade ahead and the product decision was political.

**Evidence.** Sequent founded 1983 by seventeen Intel departures; Balance up to 30 processors from 1984; NUMA-Q shipped 1996. IBM acquisition July 1999 for $810M; Project Monterey abandoned c. 2001; products discontinued by May 2002 'due to declining sales.' IBM contributed both NUMA and RCU code from Dynix to Linux. SCO's spring 2003 claim: 148 files and 168,276 lines of Dynix code allegedly contributed to Linux.

Tags: acquired and killed, niche capture, hardware tied to dying platform, internal politics, too late to market

Sources:
- https://en.wikipedia.org/wiki/Sequent_Computer_Systems
- https://en.wikipedia.org/wiki/DYNIX
- https://www.infoworld.com/article/2224418/sco-terminates-ibm-s-sequent-os-license.html
- https://www.informationweek.com/software-services/sco-pulls-the-plug-on-sequent

### Fuchsia / Zircon
*2016–present · Google (Zircon kernel derived from Travis Geiselbrecht's Little Kernel; Brian Swetland, Chris McKillop and other ex-Android/ex-Be/ex-Danger engineers)*

**What was brilliant.** The most rigorous capability-secure operating system anyone has actually shipped to consumers. Zircon exposes kernel objects — channels, VMOs, jobs, processes, ports — reachable only through unforgeable handles; there is no ambient authority, no global filesystem, no root user, and resources are passed by handle rather than by name. Every component receives a private namespace assembled by the component framework from an explicit manifest, so 'what can this program touch' is a static, auditable declaration rather than a runtime accident. FIDL provides a versioned, language-neutral IPC ABI, which is what makes the real payoff possible: the driver framework runs drivers as ordinary userspace components against a stable ABI, so a driver crash is not a kernel panic and the kernel can be updated independently of every driver on the device. A stable driver ABI is precisely what Linux has deliberately refused to provide and is the root cause of Android's update problem — which is exactly why Google built this.

**What happened.** Appeared on GitHub in August 2016 with no announcement; fuchsia.dev opened 1 July 2019; open governance December 2020. First consumer shipment May 2021, as an in-place OS swap beneath the first-generation Google Nest Hub's existing Cast UI — users noticed nothing — completed across the fleet in August 2021, extended to the second-generation Nest Hub in May 2023. January 2023 layoffs hit 16% of the Fuchsia team. Releases continue (F30 April 2026, F31 July 2026), but Google now describes it as 'one of Google's experiments around new operating system concepts', and its stated desktop and tablet future is Android-based: ChromeOS/Android convergence confirmed July 2025 and 'Aluminium OS' announced at the Snapdragon Summit in September 2025, both built on Linux-based Android, not Fuchsia. The Android-compatibility path (Starnix, a Linux syscall translation layer running Android/Linux binaries as Fuchsia components) has never shipped in a consumer product.

**Peak adoption.** Google Nest Hub first and second generation only. Google has never disclosed Nest Hub unit numbers, and Fuchsia's consumer role is an invisible substrate under someone else's UI rather than a platform anyone develops for.

**Why it failed.** Fuchsia has no application story of its own and no organisational mandate to replace Android or ChromeOS, so after a decade and hundreds of engineers its entire consumer footprint is silently hosting a smart display's pre-existing Cast interface — an operating system that solves Android's deepest structural problems inside a company that has decided to solve them inside Android instead.

**What might have changed it.** If Google had committed Fuchsia to one flagship consumer product with its own developer story — the Nest/Chromecast line as a real app platform, or the tablet/desktop convergence effort that instead became Android-based Aluminium OS — it would have had the application pressure and the executive sponsorship that turn a research kernel into a platform, rather than a beautiful substrate with no tenants.

**Evidence.** Google's own documentation states the model plainly: applications 'have no ambient authority' and 'Fuchsia has no global file system; instead, each program is given its own local namespace.' The only consumer deployments in nine years are Nest Hub generations 1 and 2 (May/August 2021, May 2023), both of which kept the existing Cast UI so the OS change was invisible. January 2023 layoffs cut 16% of the team. Google's framing — 'one of Google's experiments around new operating system concepts' — is not the language used about a platform with a roadmap. The 2025 ChromeOS/Android merge and Aluminium OS are both Linux/Android-based, which forecloses the convergence role Fuchsia was widely assumed to be built for.

Tags: strategic mismanagement, internal politics, no app ecosystem, niche capture, perpetual rewrite

Sources:
- https://en.wikipedia.org/wiki/Fuchsia_(operating_system)
- https://fuchsia.dev/fuchsia-src/concepts/principles/secure
- https://fuchsia.dev/fuchsia-src/concepts/components/v2/introduction
- https://fuchsia.dev/fuchsia-src/concepts/components/v2/starnix
- https://9to5google.com/2022/07/15/android-removes-fuchsia-code-starnix/

### Interlisp-D / Medley (Xerox D-machines)
*circa 1979–mid 1990s; Envos spun out 1987; open-source Medley Interlisp revival from 2021 · Xerox PARC, then Xerox AI Systems, then the Envos spin-out (1987), then Venue*

**What was brilliant.** Interlisp-D was not an application on an OS — it was the OS, window system, network stack and IDE of the Xerox Dolphin/Dorado/Dandelion/Daybreak workstations, all in Lisp, all live. Its residential model treated the development session itself as a first-class object: DWIM ('Do What I Mean') repaired typos and misspelled identifiers in place; the Programmer's Assistant recorded and could undo or replay the history of the session, including edits; Masterscope maintained a queryable database of the program's own cross-references so you could ask which functions bind a variable or call through a given path and then edit the answers. It was the first bitmapped, multi-window, mouse-driven Lisp environment, and it won the 1992 ACM Software System Award 'for pioneering work in programming environments.'

**What happened.** Xerox divested rather than invested: the AI Systems division was spun out as Envos in 1987, and Envos 'failed almost immediately,' with assets passing to the much smaller Venue. Development tapered through the 1990s and stopped. In 2021 many of the original PARC developers restarted it as the open Medley Interlisp Project, which now runs on modern hardware and in the browser — a preservation effort, not a platform.

**Why it failed.** Xerox classified Interlisp-D as an AI product to be divested rather than as a general software platform, and the 1987 spin-out into undercapitalized Envos destroyed the only organization with the money to port it off Xerox's dying proprietary hardware before the AI market collapsed.

**What might have changed it.** Port Interlisp-D to commodity Unix workstations while it was still inside Xerox, instead of spinning out Envos in 1987. The environment's value was in the software — DWIM, Masterscope, the Programmer's Assistant — none of which needed D-machine hardware; Lucid and Franz proved there was a Unix Lisp market at exactly that moment.

**Evidence.** 'In 1987, XAIS was spun off into Envos Corporation, which failed almost immediately.' Ownership chain: Xerox PARC → Xerox AI Systems → Envos → Venue. 1992 ACM Software System Award. Revival by original developers began in 2021.

Tags: strategic mismanagement, funding collapse, hardware tied to dying platform, business model failure, niche capture

Sources:
- https://interlisp.org/history/
- https://en.wikipedia.org/wiki/Interlisp
- https://www.theregister.com/2023/11/23/medley_interlisp_revival/
- https://en.wikipedia.org/wiki/Lisp_machine

### Maemo
*2005–2011 (OS2005 on the Nokia 770 in Nov 2005; Maemo 5 'Fremantle' on the N900 in Nov 2009; final Maemo 5 update Nov 2011; Maemo Leste community fork since 2017) · Nokia (Maemo Devices / Nokia Research Center)*

**What was brilliant.** Maemo was a real Debian system on a phone, not a Linux-derived phone OS. It used dpkg and APT against real repositories, X11, GTK+ with the Hildon mobile widget framework, BusyBox, a root shell and an on-device toolchain — which meant porting desktop free software to an N900 was frequently just a recompile. Maemo 4 'Diablo' (June 2008) shipped Seamless Software Update, so the device could be upgraded incrementally over the air instead of being reflashed, years before that was normal. On the N900 it did genuine preemptive multitasking with a live zoomable window-thumbnail switcher, a Mozilla-based MicroB browser with full Adobe Flash, and a cellular stack exposed as an ordinary Linux userspace service — a phone whose telephony was just another D-Bus service rather than a privileged black box.

**What happened.** Only four retail devices ever ran it: Nokia 770 (2005), N800 (2007), N810 (2007), N900 (2009). Nokia deliberately positioned each as an 'Internet tablet' or 'mobile computer' rather than as a phone line, so Maemo could never be seen internally to compete with Symbian. Then at Mobile World Congress in February 2010 — three months after the N900 shipped — Nokia merged Maemo into MeeGo with Intel and switched the application framework from GTK+/Hildon to Qt, orphaning the entire Maemo app ecosystem before it had formed. The N900's actual successor, the N9, ran a different stack (Harmattan). Nokia's final Maemo 5 update came in November 2011. Maemo Leste, a Devuan-based community continuation, has kept the lineage running on N900 and Motorola Droid 4 hardware since 2017.

**Why it failed.** Nokia structurally prevented Maemo from becoming a product line, because a successful Maemo would have cannibalised Symbian inside a company whose entire organisation was built around Symbian — so it got four experimental devices and no marketing, and then had its application framework replaced at the exact moment it acquired its first real phone, destroying the developer base twice before it could form once.

**What might have changed it.** If the N900 had been launched in 2009 as the N-series flagship with Symbian-scale marketing and a public five-year commitment to one application framework, Maemo had the technical substance to be the third ecosystem — 2009 was the last year the window was open, before Android's app catalogue closed it.

**Evidence.** Four retail devices in six years is itself the evidence of a project never given a channel. The framework switch is documented: MWC February 2010, three months after the N900 shipped, Maemo was merged with Moblin into MeeGo and the toolkit moved from GTK+/Hildon to Qt. The OSnews retrospective frames the N900 as 'the future that wasn't' and attributes the outcome to 'endless mismanagement' at Nokia rather than to any technical deficiency, noting that the window for a non-Android, non-iOS platform 'was closing rapidly.'

Tags: strategic mismanagement, internal politics, perpetual rewrite, no app ecosystem, no hardware channel, niche capture

Sources:
- https://en.wikipedia.org/wiki/Maemo
- https://www.osnews.com/story/133160/the-nokia-n900-the-future-that-wasnt/
- https://www.linux-magazine.com/Online/News/Nokia-N900-Internet-Tablet-with-Maemo-Linux
- https://leste.maemo.org/Leste_FAQ
- https://www.engadget.com/2010-02-15-meego-nokia-and-intel-merge-maemo-and-moblin.html

### MeeGo (and MeeGo 1.2 Harmattan)
*2010–2011 (announced at MWC 15 Feb 2010; Nokia N9 released 21 Sept 2011; Intel abandoned the project 28 Sept 2011) · Intel and Nokia, hosted by the Linux Foundation*

**What was brilliant.** Harmattan on the N9 is still the most-admired shipping mobile UI that lost. It removed the home button entirely and navigated by a single edge swipe between three fixed views — running apps, the app grid, and a notifications/feeds pane — a gesture model Apple adopted in the iPhone X in 2017 and Android adopted in 2019, six to eight years later. It did unlimited real preemptive multitasking with live window thumbnails when iOS had none and Android's was crude. Underneath was a clean, modular Linux stack whose components the industry then adopted independently of MeeGo: oFono for telephony, ConnMan for connectivity, Tracker for indexed metadata, PulseAudio for audio, and Qt/QML as the declarative UI language — QML remains one of the best-designed declarative UI toolkits ever shipped, and is still the UI layer of Sailfish, many automotive systems and KDE Plasma.

**What happened.** Nokia's board chose Windows Phone on 11 February 2011, three days after the Elop memo leaked and seven months before the N9 was due. The N9 shipped on 21 September 2011 to excellent reviews and was deliberately kept out of Nokia's largest markets — it was never officially sold in the US, UK, Germany, France, Italy or Spain. Elop told the press Nokia would not build a second MeeGo device even if the N9 was a success. Intel abandoned MeeGo for Tizen on 28 September 2011. The Nokia engineers who built Harmattan left and founded Jolla; the Mer fork of the MeeGo core became the basis of Sailfish OS. Only three devices of note ever shipped MeeGo: the N9, the WeTab tablet and the Asus Eee PC X101.

**Peak adoption.** Nokia never disclosed N9 sales; contemporaneous analyst estimates put it at roughly 1.5–2 million units through Q4 2011 — against the roughly 100 million Symbian smartphones Nokia sold in 2010. One MeeGo phone shipped, worldwide, ever.

**Why it failed.** MeeGo was cancelled by its own sponsor before it shipped: Nokia committed in February 2011 to a platform that did not yet exist on its hardware while the platform it already had was seven months from retail, so the N9 launched as a product whose maker had an active commercial interest in its failure — a restricted-market launch with no marketing and a public commitment never to make a second one.

**What might have changed it.** If Nokia had launched the N9 in its major markets and committed to one further MeeGo device in 2012 — running MeeGo and Windows Phone in parallel for two years, exactly as Samsung ran Android and Bada in parallel — it would have kept the engineering team that instead walked out and founded Jolla, and would have had a fallback when Windows Phone stalled permanently at 3%.

**Evidence.** The dates do the work: memo leaked 8 Feb 2011, Microsoft partnership announced 11 Feb 2011, N9 shipped 21 Sept 2011, Intel exited 28 Sept 2011. Elop's own memo said 'at this rate, by the end of 2011, we might have only one MeeGo product in the market' — which describes a resourcing decision he controlled, not an external constraint. The popular claim that 'MeeGo was late and unfinished' is not supported: Harmattan shipped on schedule and reviewed better than the Lumia 800, which used the same chassis and launched six weeks later. Independent confirmation of the design's quality is that the three-view edge-swipe model and the button-free navigation were later adopted by both Apple and Google.

Tags: strategic mismanagement, internal politics, no hardware channel, too late to market, no app ecosystem, perpetual rewrite

Sources:
- https://en.wikipedia.org/wiki/MeeGo
- https://www.engadget.com/2011-02-08-nokia-ceo-stephen-elop-rallies-troops-in-brutally-honest-burnin.html
- https://mobile-review.com/articles/2011/nokia-n9-meego-en.shtml
- https://www.slashgear.com/nokia-silent-on-n9-sales-as-meego-kept-out-of-spotlight-26210826/
- https://www.osnews.com/story/25569/nokia-n9-outselling-lumia/

### Mica
*1985–1988 · Digital Equipment Corporation, DEC West (Seattle) — Dave Cutler*

**What was brilliant.** Mica defined the structure the industry still runs on. It separated a small kernel from a layered executive; introduced an Object Manager abstracting every system data structure behind typed handles with uniform security and lifetime management; had native multithreading and symmetric multiprocessing; and, most importantly, ran multiple operating-system personalities — VMS and ULTRIX system-call and library environments — over one common kernel, on a RISC architecture (PRISM) whose instruction set was co-designed with the compilers. Cutler then built the same architecture at Microsoft, which is why Windows NT has a kernel, an executive, an object manager and environment subsystems.

**What happened.** Begun around 1985 at DEC West. The full VMS/ULTRIX interchangeability goal proved impossible and the target was cut back to a standalone PRISM ULTRIX. DEC cancelled PRISM in the summer of 1988 in favour of MIPS-based systems — after the microPrism ALU design had completed in April 1988 and samples had been fabricated — and Mica died with the chip. In autumn 1988 Nathan Myhrvold introduced Cutler to Bill Gates; Cutler left with roughly two dozen DEC engineers and shipped Windows NT 3.1 in July 1993.

**Peak adoption.** None — cancelled before completion.

**Why it failed.** Mica was cancelled because DEC would not let a new operating system threaten VMS, its hardware-attached revenue base, and resolved a fight between two internal RISC programmes by killing the processor rather than choosing between architectures — the OS died as collateral damage of a chip decision made for reasons that had nothing to do with its merits.

**What might have changed it.** If DEC had shipped PRISM and Mica around 1989 instead of buying in MIPS systems, DEC — not Microsoft — owns the portable, multi-personality operating system that takes the workstation and server market through the 1990s, and Cutler never leaves Maynard.

**Evidence.** Cutler in 2023: 'MICA was wildly ambitious...at the level of ambition of Multics.' DEC cancelled PRISM in summer 1988 despite the microPrism ALU design being complete in April 1988 and samples fabricated — a cancellation of working silicon. Windows NT's design is Mica's design, carried by the same architect: multiple OS APIs on a common kernel, kernel/executive separation, Object Manager, multithreading and SMP. The popular claim that 'Windows NT is VMS re-implemented' overstates the case — what moved to Redmond was the architecture and the architects, not code — and DEC's own resolution was commercial: the 1995 DEC/Microsoft alliance that put Windows NT on Alpha.

Tags: strategic mismanagement, internal politics, hardware tied to dying platform, perpetual rewrite

Sources:
- https://en.wikipedia.org/wiki/DEC_MICA
- https://en.wikipedia.org/wiki/DEC_PRISM
- https://neilrieck.net/docs/dave_cutler-prism-mica-emerald-etc.html
- https://neilrieck.net/docs/Windows-NT_is_VMS_re-implemented.html

### Midori
*circa 2008–2015 (Joe Duffy on the team 2009, transitions 2012–2014, project ended 2015); over eight years of development · Microsoft — an incubation grown out of MSR's Singularity, with Joe Duffy leading languages and compilers*

**What was brilliant.** Midori is the only managed operating system that closed the performance argument with C++ at whole-system scale, and it did so with a co-designed language, compiler, runtime and OS. Everything — drivers, the domain kernel, the browser, all user code — was written in an ahead-of-time compiled, memory-safe C# dialect (M#) with capability-based security in which object references are the only authority. Its error model separated recoverable typed exceptions from fail-fast bugs and measured 'roughly 7% smaller and 4% faster on some key benchmarks' than the return-code equivalent. Its async model made every asynchronous activity explicit with one message loop per process by default. The compiler did aggressive whole-program work: bounds-check elimination, devirtualization, reflection removal worth at least 30% of image size, a LINQ refactor that saved over 100 MB across the whole OS image, and eleven distinct garbage collectors evaluated. Compilation throughput went from 40x slower than C# to 3x for debug and 5x for optimized builds. It ran real production traffic.

**What happened.** Wound down between 2012 and 2015 and the team dispersed into Windows, .NET and Azure. Duffy: 'decisions around the destiny of Midori's core technology weren't entirely technology-driven,' and 'Midori happened before the OSS renaissance at Microsoft, and so it never saw the light of day.' He records regret that 'we didn't OSS it from the start.' The ideas resurfaced piecemeal: async/await, Span<T> and ref-safety, .NET Native/CoreRT AOT, the nullable-reference-type work, and Project Verona.

**Why it failed.** Midori had to displace Windows to matter, and no amount of measured superiority could make Microsoft trade a compatibility franchise worth billions a year for a from-scratch OS that ran none of its existing software — the deciding constraint was organizational, which is exactly what the project's own engineering leadership said afterwards.

**What might have changed it.** Open-source it from the start, which is Duffy's own stated regret. A public M# compiler, error model and async runtime in 2010 would have had an external constituency and an external codebase; instead eight years of work survived only as ideas that had to be re-derived one feature at a time inside C# and .NET.

**Evidence.** Duffy: 'Our one production workload – taking Speech Recognition traffic for Bing.com – actually saw significant reductions in latency and improvements in throughput as a result,' and Cortana's DNN speech recognition 'could have never reached their latency targets were it not for this overall parallelism model.' Microsoft Research's own page: Midori 'briefly powered Microsoft's natural language search service for the West Coast and Asia regions.' Duffy on the ending: 'decisions around the destiny of Midori's core technology weren't entirely technology-driven'; 'Midori happened before the OSS renaissance at Microsoft, and so it never saw the light of day.' Error-model result: 'the exceptions-based system ended up being roughly 7% smaller and 4% faster on some key benchmarks.'

Tags: internal politics, strategic mismanagement, compatibility gap, business model failure, incumbent lockin, no app ecosystem

Sources:
- https://joeduffyblog.com/2015/11/03/blogging-about-midori/
- https://raw.githubusercontent.com/joeduffy/joeduffy.github.io/master/_posts/2015-11-03-blogging-about-midori.md
- https://joeduffyblog.com/2015/12/19/safe-native-code/
- https://joeduffyblog.com/2015/11/19/asynchronous-everything/
- https://joeduffyblog.com/2016/02/07/the-error-model/

### Nokia Meltemi
*circa 2010–2012 (cancelled 14 June 2012; never publicly announced by Nokia, never shipped a device) · Nokia — Mary McDowell's Mobile Phones organisation, engineered by the Linux teams in Ulm and Tampere that had built Maemo and MeeGo*

**What was brilliant.** Meltemi was Linux with Qt aimed at the price point Android could not reach in 2012 — sub-$100 handsets with tens of megabytes of RAM — and the strategic insight behind it was the most valuable unclaimed position in the industry. Nokia was still the world's largest handset maker by volume, shipping hundreds of millions of Series 40 feature phones a year into exactly the emerging markets that would supply the next billion smartphone users, and Series 40 was a dead-end proprietary Java platform that could never acquire a real app ecosystem. Meltemi would have given that installed base a modern kernel, a modern toolkit and — decisively — Qt application compatibility with what Nokia was simultaneously pushing on Symbian and MeeGo, collapsing Nokia's fragmented developer story into one framework across its entire range from $50 to $600. Contemporary reporting described the engineering as well advanced.

**What happened.** Killed on 14 June 2012 as part of the restructuring that cut a further 10,000 jobs, closed the Salo plant in Finland and shut R&D sites in Ulm, Germany and Burnaby, Canada. On the analyst call Elop refused even to confirm the project's name, saying he had 'never talked publicly about a development project by that name' while acknowledging Nokia was ending some development projects. Mary McDowell, who ran the low-end phone business, left in the same reorganisation. Nokia stayed on Series 40 and then Asha, sold the devices business to Microsoft in 2013, and Microsoft killed Asha in 2014. The market Meltemi was designed for was taken first by sub-$100 Android and then, precisely as Meltemi had envisaged, by a Linux-plus-web feature-phone OS: KaiOS, which shipped over 100 million devices by May 2019 and attracted a $22 million investment from Google.

**Why it failed.** Meltemi was cancelled for political rather than technical reasons: having bet the company on Windows Phone in February 2011, Nokia could not simultaneously justify funding a fourth in-house platform, so it abandoned development in the one segment where it was still the outright global market leader in order to protect a strategy in a segment where it had 3% share.

**What might have changed it.** If Nokia had shipped Meltemi in 2012–2013 across its Series 40 volume base, it would have owned the feature-phone-to-smartphone transition in India, Africa and Southeast Asia — the exact position KaiOS then took with over 100 million units and Google and Reliance Jio money behind it — and would have had a revenue engine structurally independent of whether Windows Phone succeeded.

**Evidence.** The cancellation is documented for 14 June 2012 alongside the 10,000-job cut and the closure of Ulm and Burnaby R&D, with Elop conspicuously declining to name the project. Mary McDowell's simultaneous departure removes the executive sponsor. The strongest evidence that the thesis was correct rather than the execution wrong is KaiOS: the same bet — a Linux-based, low-footprint, app-capable OS for sub-$50 feature phones — reached 100 million devices within four years of Meltemi's cancellation, in the very markets Nokia was leading when it walked away.

Tags: internal politics, strategic mismanagement, funding collapse, no hardware channel, too late to market, perpetual rewrite

Sources:
- https://allthingsd.com/20120614/nokia-to-end-meltemi-effort-for-low-end-smartphones
- https://www.engadget.com/2012/06/14/nokia-reportedly-scraps-meltemi/
- https://www.gottabemobile.com/nokia-cancels-plans-for-another-linux-based-platform/
- https://m.gsmarena.com/newscomm-4393.php
- https://en.wikipedia.org/wiki/KaiOS

### OKL4 / Open Kernel Labs
*2006–2012 as an independent company; OKL4 still shipping as a General Dynamics product · Gernot Heiser and Steve Subar, spun out of NICTA (Sydney), headquartered in Chicago*

**What was brilliant.** The commercial proof that an academically rigorous microkernel could hold the hardest real-time envelope in consumer electronics. OKL4 ran as the protected-mode RTOS beneath Qualcomm's wireless modem firmware — the baseband, where a missed deadline drops a call — on hundreds of millions of handsets. It then became the OKL4 Microvisor, which para-virtualized a full Linux/Android stack and a modem stack side by side on a single ARM core with no hardware virtualization support at all, years before ARM shipped virtualization extensions. Apple arrived independently at the same conclusion: the Secure Enclave Processor in every A7-and-later Apple device runs sepOS, which Black Hat researchers reverse-engineered and documented as 'Based on Darbat/L4-embedded (ARMv7)' with 'custom modifications by Apple'. Two of the largest silicon vendors on earth chose an L4 for their most safety- and security-critical code paths.

**What happened.** NICTA announced Qualcomm's adoption in November 2005, with handsets shipping from late 2006. OK Labs announced passing 1.5 billion cumulative device shipments in January 2012. Heiser and co-founder Ben Leslie had already left in June 2010, after which the company was put up for sale; General Dynamics acquired it in August 2012 'for a price that gave our investors more or less their money back'. Common shareholders received nothing. OKL4 continues as a General Dynamics defense hypervisor, with the company now claiming over 2 billion cumulative deployments.

**Peak adoption.** More than 1.5 billion cumulative device shipments announced January 2012, predominantly Qualcomm wireless modem chips. General Dynamics now claims deployment in over 2 billion devices. Separately, L4-derived sepOS runs on every Apple device with an A7 or later SoC.

**Why it failed.** OK Labs shipped on more devices than almost any operating system in history and still could not build a business, because it sold an invisible component at a per-unit price the handset bill of materials would not bear, and its management chased the mass-market smartphone volume story instead of the automotive and defense markets where isolation is actually a line item someone pays for.

**What might have changed it.** Heiser's own counterfactual is automotive. OK Labs spent 'two full years' negotiating with OpenSynergy, who then partnered with Sysgo instead: 'we managed to create a competitor out of nothing in a space we could (and should) have owned!' That market subsequently produced QNX's 255-million-vehicle position and NIO's seL4-based SkyOS. A second counterfactual: the company owned seL4 and never productized it — per Heiser, the CEO 'never understood seL4 and its potential'.

**Evidence.** Heiser's eleven-part public postmortem is unusually specific. On the business model, the pitch he criticises: '>1 billion devices sold worldwide each year, if we can get on 20% of them and get $1 per unit, we're rich.' On his own market read: 'I could not see a significant value-add for our technology in mobile,' with the addressable market 'not exceeding $100M/a in this space.' On execution: 'only a single quarter where company revenue and booking performance had matched the goals.' On the exit: General Dynamics, August 2012, price returning investors 'more or less their money back'; common shareholders got nothing; Heiser was offered '$1000 for signing away some irrelevant rights, which I declined', while the CEO took 'about $900k in a combination of completion and retention bonuses.' He also records a legal exposure: porting Qualcomm's AMSS modem stack without a source licence. Apple SEP lineage confirmed verbatim in Tarjei Mandt et al., Black Hat USA 2016: 'SEPOS — Based on Darbat/L4-embedded (ARMv7) — Custom modifications by Apple'.

Tags: strategic mismanagement, business model failure, internal politics, licensing or legal, acquired and killed

Sources:
- https://microkerneldude.org/2014/10/23/ok-labs-story-9-the-end-game/
- https://microkerneldude.org/2014/10/16/ok-labs-story-8-competitors-and-markets/
- https://en.wikipedia.org/wiki/L4_microkernel_family
- https://blackhat.com/docs/us-16/materials/us-16-Mandt-Demystifying-The-Secure-Enclave-Processor.pdf
- https://gdmissionsystems.com/products/cross-domain-solutions/hypervisor

### OSF/1 → Digital UNIX → Tru64 UNIX
*OSF founded May 1988; OSF/1 released Dec 1990; DEC OSF/1 1.0 Jan 1992, AXP 1.2 March 1993; Digital UNIX 1995; Tru64 1998; final 5.1B-6 Oct 2010; support ended 31 Dec 2012 · Open Software Foundation — the 'Gang of Seven': Apollo, Groupe Bull, DEC, HP, IBM, Nixdorf, Siemens — and, in practice, Digital Equipment Corporation*

**What was brilliant.** The first commercially significant 64-bit-clean Unix, shipping on Alpha in March 1993 — five years before 64-bit Unix became mainstream and eleven before x86-64. Built on CMU's Mach kernel (2.5 plus much of the 4.3-Reno BSD kernel, then Mach 3.0 as a true microkernel from version 1.3) specifically so it would be free of AT&T intellectual property. AdvFS was a journaled filesystem with online defragmentation, online resize and filesets, backed up from a split mirror while live. TruCluster Server (5.0, 1999) delivered a genuine single-system-image cluster: a cluster-wide filesystem, a cluster-wide device namespace and one root visible identically from every member — a capability Linux still lacks. Memory Channel gave microsecond-class cluster interconnect. It held a large share of the TPC and SPEC records of its era.

**What happened.** OSF's political purpose collapsed before its technical one. AT&T sold Unix to Novell in June 1993 and Unix International dissolved; by 1994 the Open Software Foundation 'ceased funding of research and development of OSF/1.' HP withdrew its PA-RISC port for lack of hardware and software support; IBM used OSF/1 only as the basis of AIX/ESA on mainframes; Apple abandoned plans to base A/UX 4.0 on it. Only DEC shipped it at volume. Compaq bought DEC in 1998 and HP bought Compaq in 2002; HP then announced it would use Veritas VxFS instead of AdvFS and abandon Tru64's advanced features, folding what it could into HP-UX, and released AdvFS source to open source in 2008. Support ended 31 December 2012. OSF merged with X/Open into The Open Group in February 1996.

**Why it failed.** OSF/1 was created as a political counterweight to the AT&T–Sun alliance rather than as a product any founder's business actually depended on, so when the political threat evaporated in 1993–94 every founder except DEC walked away — and DEC's version then died with Alpha when Compaq and HP retired the architecture.

**What might have changed it.** If HP and IBM had actually shipped OSF/1 as their primary Unix instead of using OSF as negotiating leverage, there would have been one binary-compatible Unix spanning DEC, HP and IBM hardware facing Windows NT 3.1 in 1993. That is the single clearest 'what if' of the Unix wars: the fragmentation NT exploited was a choice made in boardrooms, not a technical necessity.

**Evidence.** OSF was first proposed by DEC's Armando Stettner at an invitation-only meeting in January 1988 and founded in May 1988 explicitly in response to AT&T–Sun. OSF/1 released Dec 1990; HP withdrew its PA-RISC port; by 1994 OSF ceased funding OSF/1 R&D. DEC OSF/1 AXP 1.2 shipped March 1993 as a full 64-bit implementation. TruCluster with a cluster-wide filesystem arrived in 5.0 (1999). HP chose Veritas over AdvFS post-2002 and open-sourced AdvFS in 2008; support ended 31 Dec 2012. Of OSF's five original deliverables only Motif (IEEE P1295, 1994) and DCE achieved meaningful adoption.

Tags: internal politics, hardware tied to dying platform, acquired and killed, too late to market, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/OSF/1
- https://en.wikipedia.org/wiki/Open_Software_Foundation
- https://en.wikipedia.org/wiki/Tru64_UNIX
- https://en.wikipedia.org/wiki/Unix_wars

### Sinclair QDOS
*1984–1986 at Sinclair; derivatives (Minerva, SMS2, SMSQ/E) to the present · Sinclair Research Ltd; designed and written by Tony Tebby*

**What was brilliant.** A preemptive multitasking, multi-job 68008 operating system in 48 KB of ROM on a machine that launched at £399 on 12 January 1984 — the first genuinely affordable personal computer with real multitasking, arriving three years before the Macintosh got even cooperative MultiFinder. Its extensible redirectable I/O system made devices and filesystems pluggable by name at runtime, and job management, memory allocation and scheduling were exposed through a documented trap interface. SuperBASIC, in the same ROM, was a structured BASIC with procedures, functions and parameter passing that doubled as the system's command interpreter — far ahead of the line-numbered BASICs on competing machines. Running from ROM meant instant boot with all RAM left for applications.

**What happened.** Sinclair took orders in January 1984 and shipped months late; the OS needed 48 KB of ROM against the 32 KB the board provided, so early machines shipped with an external ROM cartridge hanging out of the back. Tony Tebby resigned from Sinclair in 1984 in protest at the premature launch. Amstrad bought Sinclair's computer business in April 1986 and discontinued the QL immediately. Tebby reimplemented the system as SMS2 and then SMSQ/E — which, pointedly, ran on Atari ST hardware — and it is still maintained; Laurence Reeves's Minerva replaced the original ROM. QDOS technology also shipped in the ICL One Per Desk, Merlin Tonto and Telecom Australia ComputerPhone office terminals.

**Peak adoption.** about 150,000 Sinclair QL units

**Why it failed.** Sinclair shipped a genuinely advanced OS on hardware nobody could trust — Microdrive tape loops as the only mass storage and a ROM that visibly did not fit the machine — so the QL's reputation collapsed within six months and no serious application base ever formed, which is the one thing an OS cannot survive without.

**What might have changed it.** Holding the launch until the ROM fitted on the board and shipping with a floppy drive instead of Microdrives — the two faults every retrospective identifies — plausibly makes the QL the cheap 68000 multitasking machine of 1984–85, a full year ahead of both the Atari ST and the Amiga.

**Evidence.** Launched 12 January 1984 at £399, discontinued April 1986 after roughly 150,000 units. Tebby, the OS's author, left the company in 1984 over the premature launch — the strongest available internal testimony that the failure was one of shipping discipline, not design. That the OS was good enough for ICL to build the One Per Desk office terminal on it, and good enough that Tebby's own successor ran on a competitor's hardware for another two decades, is the counter-evidence to the popular verdict that the QL was simply bad.

Tags: strategic mismanagement, no app ecosystem, technical shortfall, no hardware channel, acquired and killed

Sources:
- https://en.wikipedia.org/wiki/Sinclair_QDOS
- https://en.wikipedia.org/wiki/Sinclair_QL
- https://en.wikipedia.org/wiki/Tony_Tebby
- https://www.theregister.com/on-prem/2024/01/16/the_sinclair_qls_legacy_at_40/
- https://en.wikipedia.org/wiki/SMSQ/E

### Solaris (as a desktop and workstation platform)
*1992–2011 as a desktop platform (Solaris 2.0 June 1992 → Solaris 11 Nov 2011); Oracle Solaris 11.4 (Aug 2018) still maintained for servers · Sun Microsystems, later Oracle*

**What was brilliant.** First-rate engineering that mostly arrived after the desktop was already lost. A fully preemptible kernel with turnstiles and priority inheritance, scheduler classes with real-time support, and single-image scalability into the hundreds of CPUs. Doors IPC. The Service Management Facility replaced init scripts with a dependency graph and fault-managed restart, years before systemd. Solaris 10 (31 Jan 2005) shipped three things still being copied: Zones — OS-level containers a decade before Docker; DTrace — safe, production-grade dynamic instrumentation of a live kernel and userland with genuinely zero disabled-probe cost; and ZFS — pooled storage with end-to-end checksums, copy-on-write snapshots and self-healing that collapsed the volume-manager/filesystem split. DTrace and ZFS are still being ported into other operating systems twenty years on.

**What happened.** Sun subordinated x86 to SPARC margins for a decade; the x86 port was unstable until 2.4 (1994) and was repeatedly deprioritised thereafter. The desktop churned through OpenWindows → CDE (bundled from Solaris 2.6) → the GNOME-based Java Desktop System, always a step behind. OpenSolaris opened on 14 June 2005 under the CDDL and was shut by Oracle on 13 Aug 2010 by internal memo; the illumos fork had been announced ten days earlier, on 3 Aug 2010. Oracle restricted Solaris downloads to 90-day trials, laid off most of the Solaris team in Sept 2017, and shipped 11.4 in Aug 2018 as the last release, supported as a maintenance annuity to the 2030s.

**Peak adoption.** about 0.78% of combined server-and-desktop OS share as of 2025; Solaris never registered a meaningful desktop share. Company context: Sun's peak revenue was $13.8B (FY2007–08) with roughly 38,600 employees in 2006; Oracle bought Sun for $7.4B gross ($5.6B net), closing 27 Jan 2010.

**Why it failed.** Sun tied Solaris to SPARC hardware margins and treated commodity x86 as a threat to its own revenue, so the single move that could have made Solaris a mainstream platform — a free, well-driven, well-supported x86 edition in the mid-1990s — was blocked by Sun's business model, and Linux permanently occupied that position instead.

**What might have changed it.** Scott McNealy named it himself: 'If we'd have just decided to release Solaris on metal instead of shrink wrapped, Solaris on Intel would have been a wild hit and nobody would have done Linux.' A free, binary, broadly driver-compatible Solaris/x86 in 1994–96 is the one changed decision.

**Evidence.** McNealy's quote above. Jonathan Schwartz, then EVP: 'One of the biggest mistakes we made a few years back was not supporting Solaris x86.' Solaris 2.1 for x86 shipped June 1993 but was not stable until 2.4 in 1994. Oracle's 13 Aug 2010 memo from Mike Shapiro, Bill Nesheim and Chris Armes: 'We will no longer distribute source code for the entirety of the Solaris operating system in real-time while it is developed.' illumos was announced 3 Aug 2010. Solaris' 2025 share stands at roughly 0.78%. Note that the common claim that 'Linux killed Solaris' inverts the sequence — Sun's refusal to compete on x86 in 1994–99 created the vacancy Linux filled.

Tags: strategic mismanagement, no hardware channel, incumbent lockin, too late to market, licensing or legal, niche capture

Sources:
- https://en.wikipedia.org/wiki/Oracle_Solaris
- https://softpanorama.org/Solaris/solaris_history.shtml
- https://www.ssdnodes.com/learn/history-of-solaris-and-illumos
- https://www.osnews.com/story/4583/sun-ashamed-of-solaris-x86-past/
- https://en.wikipedia.org/wiki/Sun_Microsystems

### Solus
*2015-present (as Evolve OS / Solus; the separate SolusOS 2013 project folded) · Ikey Doherty; later co-led by Joshua Strobl and others*

**What was brilliant.** An independent distribution built from scratch rather than derived from Debian, Fedora or Arch. It had a curated rolling-release model, a stateless design separating vendor and user configuration, the eopkg package manager, and the Budgie desktop, which spread beyond Solus to Ubuntu Budgie and other distributions and now lives on under the Buddies of Budgie organisation.

**What happened.** Doherty went silent in mid-2018, infrastructure bills went unpaid, and in November 2018 he stepped away and transferred rights to the team. Co-lead Strobl left on 1 January 2022 and took Budgie to a new organisation. On 17 January 2023 a hardware failure caused about three months of outage, because build infrastructure hosted at RIT was accessible to only one person. In April 2023 it relaunched with Doherty and Strobl back and a plan to rebase on Serpent OS. Solus 4.4 shipped in July 2023.

**Why it failed.** Solus's infrastructure, accounts and technical direction each depended on one or two individuals with no institutional funding. Every personal departure or hardware fault became an existential outage, which destroyed the reliability ordinary users need from an OS.

**What might have changed it.** Put infrastructure, domains and finances under a foundation with shared credentials and sponsored hosting around 2016-2017, when Solus was gaining popularity, so that no one person leaving could take the project offline.

**Evidence.** FOSS Force (2023): Doherty's 2018 exit 'left infrastructure unpaid; the team lost website access for months' after his message 'am very very sick atm. all will be paid up for the next 30 days'. Strobl on leaving: issues were 'not being addressed... it's been just deflected.' The January 2023 outage lasted about three months because on-premises infrastructure was accessible to one individual. The 2023 relaunch rebases on yet another new distribution (Serpent OS).

Tags: internal politics, funding collapse, strategic mismanagement, niche capture, perpetual rewrite

Sources:
- https://fossforce.com/2023/07/solus-is-back-but-can-it-survive-its-troubled-past/
- https://getsol.us/2023/04/18/a-new-voyage/
- https://news.itsfoss.com/solus-revival/
- https://news.itsfoss.com/solus-co-lead-resign-budgie-serpent/
- https://9to5linux.com/solus-linux-to-be-rebased-on-serpent-os

### Spring
*1987 (Sun/AT&T agreement) – mid-1990s; complete working system 1993; research release 1994 · Sun Microsystems Laboratories — Graham Hamilton, Panos Kougiouris, Yousef Khalidi, Michael Nelson, Sanjay Radia and colleagues*

**What was brilliant.** Spring made interface-first, object-oriented operating-system design actually fast, which nobody else managed. Every service was defined in a real interface definition language with exceptions, subtyping and namespaces — the direct ancestor of CORBA IDL. The nucleus implemented 'doors': synchronous, direct call/return cross-address-space invocation that transferred the caller's thread into the server rather than queuing an asynchronous message, measured at roughly 11 µs on a SPARCstation 2 when Mach's port-based IPC was several times that. Its virtual memory system cleanly separated address spaces, memory objects and pagers, so a file, a device and a network page were all the same kind of object and could be shared naturally across programs and machines. A unified naming service let arbitrary object graphs, not just files, live in one hierarchy. And it ran unmodified SunOS binaries on top of all this through a UNIX emulation subsystem, so the compatibility story was solved rather than deferred.

**What happened.** Sun shipped a research release in 1994 under a non-commercial licence ($75 to universities, $750 to commercial institutions) and never productized it. The team dispersed into other Sun initiatives — most consequentially Java, where Graham Hamilton became a Java architect. The technology was then harvested piecemeal: doors went into Solaris 2.5 as an undocumented internal interface and became a documented feature in Solaris 2.6; Spring's IDL fed CORBA IDL; its naming, caching file-server and virtual memory ideas fed Solaris; and several interface conventions fed the Java class libraries.

**Why it failed.** Sun had just forced its entire installed base through the painful SunOS 4 to Solaris 2 migration and could not credibly ask for a second operating-system transition; when Java arrived in 1995 the company's platform ambition moved up a layer to the language and the JVM, and Spring's engineers moved with it.

**What might have changed it.** If Spring had been framed from the start as the next Solaris kernel, with the UNIX emulation subsystem as the default personality rather than as a compatibility demo in a separate research OS, its doors, VM and naming work would have shipped as a Solaris release. Sun took precisely that path with doors alone, into Solaris 2.5 and 2.6, and it worked — which shows the transfer mechanism existed and only the ambition was missing.

**Evidence.** Spring originates in a 1987 Sun/AT&T agreement, alongside the System V / BSD merger, to 'reimplement UNIX in an object-oriented fashion'. Complete working system by 1993; research release 1994 priced at $75 (universities) / $750 (commercial). Doors measured at roughly 11 µs on a SPARCstation 2. Doors entered Solaris 2.5 undocumented and were documented in Solaris 2.6. Spring's IDL was adopted as CORBA IDL. The decisive fact is that Sun demonstrably wanted the technology — it took the parts it could retrofit — but would not take the system, which points to transition cost and internal priority rather than to any defect in the design.

Tags: strategic mismanagement, business model failure, no app ecosystem, niche capture, internal politics, too late to market

Sources:
- https://en.wikipedia.org/wiki/Spring_(operating_system)
- https://www.usenix.org/conference/usenix-summer-1993-technical-conference/spring-nucleus-microkernel-objects
- https://www.techmonitor.ai/technology/sunlabs_describes_how_its_spring_operating_system_will_treat_unix_other_environments_as_objects
- https://archiveos.org/spring/
- https://sites.cc.gatech.edu/classes/AY2009/cs4210_fall/papers/smli_tr-93-14.pdf

### Star Trek (Macintosh System 7 on x86)
*1992–1993 · Apple Computer with Novell and Intel (engineering led from Apple; Fred Monroe, Fred Huxham and others)*

**What was brilliant.** A small team got System 7.1 booting and running the complete Macintosh GUI on a stock Intel 486 PC in under ten months, by writing a dynamically loadable 32-bit protected-mode kernel called Vladivar hosted on DR DOS and porting the Mac Toolbox to x86. On screen the result was indistinguishable from a real Macintosh. The platform-abstraction work was solid enough that Apple reused it for the 68k-to-PowerPC migration, and Novell shipped parts of the underlying DOS work in DR DOS 7.0 in 1994.

**What happened.** Officially begun 14 February 1992 with John Sculley's backing at Apple, Novell as the partner and Intel's Andy Grove supporting it. The team hit its 31 October 1992 milestone (and collected the promised bonuses), and demonstrated System 7.1 running on a 486 on 4 December 1992. Cancelled in mid-1993 after Sculley was replaced by Michael Spindler, who committed Apple entirely to PowerPC and the IBM alliance. Apple did not ship Mac OS on Intel until 2006 — thirteen years later, using an OS it had to buy in the interim.

**Peak adoption.** None — prototype only, never a product.

**Why it failed.** Star Trek died because it attacked Apple's own hardware margins and contradicted the PowerPC commitment Apple had just made to IBM and Motorola, so it lost the internal argument the instant its executive sponsor left the company — the technology was never evaluated by a customer.

**What might have changed it.** If Apple had shipped Mac OS on x86 as a licensed software product in 1993 while keeping premium Macs as its hardware line — precisely the arrangement it executed successfully in 2006 — it enters the volume PC market when the competition is Windows 3.1 and OS/2, before Windows 95 consolidates the desktop.

**Evidence.** Fred Monroe on the milestone push: 'We worked like dogs. It was some of the most fun I've had working.' The team met the 31 October 1992 deadline and demonstrated on 4 December 1992. The popular retelling — that Apple could have shipped Mac OS on PCs immediately and did not — overstates it: existing Macintosh applications did not run on the port, every one would have needed recompilation for x86, so the product required an ISV porting campaign Apple had neither planned nor budgeted. What Apple actually killed was a credible eighteen-month path to a second platform, and it killed it for internal-alliance reasons rather than on the merits of the demo.

Tags: internal politics, strategic mismanagement, no app ecosystem, hardware tied to dying platform

Sources:
- https://en.wikipedia.org/wiki/Star_Trek_project
- https://lowendmac.com/2014/star-trek-apples-first-mac-os-on-intel-project/
- https://www.cultofmac.com/516787/mac-os-runs-intel-pc-tiah/
- https://wiki.preterhuman.net/Apple_Star_Trek

### Sun Java Desktop System
*2003-2005 (Linux edition); continued on Solaris 10/OpenSolaris until about 2010 · Sun Microsystems (Project Mad Hatter)*

**What was brilliant.** The first complete, enterprise-managed Linux desktop from a tier-one systems vendor. It combined a GNOME 2 desktop (Blueprint theme), StarOffice 7, Mozilla, Evolution, Java and Gaim on SuSE Linux, with centralised configuration management. Sun offered simple per-employee subscription pricing ($100 per employee, $50 with Java Enterprise System) and paired it with Project Looking Glass 3D desktop research.

**What happened.** Launched in 2003. In November 2003 Sun signed a China Standard Software Co. deal targeting 500,000-1,000,000 desktops a year and 200 million eventually. Novell's acquisition of SuSE (announced November 2003) put Sun's base distribution under a competitor. The Linux edition was shelved in 2005 after Sun open-sourced Solaris; JDS Release 3 continued as a Solaris 10 desktop option and ended with OpenSolaris, which Oracle closed after buying Sun.

**Why it failed.** Sun used the Linux desktop as a weapon in its server and Java strategy rather than as a product in its own right. It built it on a distribution a rival then bought, and dropped it as soon as its Solaris strategy changed.

**What might have changed it.** Sun committing JDS to a single platform it controlled from day one (Solaris x86 or its own Linux base), with sustained investment in Office-format fidelity for StarOffice, and treating China and government deals as a long-term product line rather than a 2003 marketing coup.

**Evidence.** Sun's marketing manager Peder Ulander in 2003: 'This really puts us in the leadership role in the Linux desktop market... Red Hat has all but washed its hands of Linux on the desktop.' Pricing at $100 per employee was announced on 3 December 2003. The popular memory of a 200-million-seat China win is misleading: that was a stated target. By July 2005 Sun had shelved Linux JDS sales and repositioned JDS inside Solaris for developers, and the Linux version ended after Solaris was open-sourced in 2005.

Tags: strategic mismanagement, incumbent lockin, compatibility gap, no hardware channel, acquired and killed

Sources:
- https://www.informationweek.com/software-services/sun-strikes-huge-linux-desktop-deal-with-china
- https://www.computerworld.com/article/2573654/sun-pushes-java-software-with-new-pricing.html
- https://en.wikipedia.org/wiki/Java_Desktop_System
- https://lwn.net/Articles/59321/
- https://adtmag.com/articles/2003/11/18/adt-at-comdex-chinese-firm-buys-into-java-desktop-system.aspx

### SunOS 4.x
*1982–1994 (SunOS 4.0 Dec 1988; 4.1.4 Nov 1994 final; shipping ended 27 Dec 1998; support ended 30 Sept 2003) · Sun Microsystems — Bill Joy and the Berkeley CSRG lineage*

**What was brilliant.** SunOS is where a large share of what we now call 'Unix' was actually built. SunOS 2.0 (May 1985) shipped NFS, the VFS/vnode layer, RPC/XDR and the YP directory — the vnode abstraction is still the internal shape of every Unix filesystem layer, and NFS's protocol was published rather than hoarded. SunOS 4.0 (Dec 1988) unified the buffer cache with the virtual memory system so mmap and the page cache became one mechanism, the design every modern Unix subsequently copied, and shipped the first widely deployed shared libraries and dynamic linker in Unix. It was fast, internally coherent, and remembered with more affection than any of its successors.

**What happened.** On 4 Sept 1991 Sun announced SunOS 4 would be replaced by an SVR4-derived system marketed as Solaris 2, a consequence of the 1987 AT&T–Sun alliance. Sun retroactively renamed the still-shipping SunOS 4.1.1 'Solaris 1.0', which confused customers further. Solaris 2.0 shipped June 1992 and was not widely considered stable until 2.5 in 1995. SunOS 4.1.4 (Nov 1994) was the last release; Sun shipped 4.1.3/4.1.4 until 27 Dec 1998 and supported them to 30 Sept 2003. Sun switched sides in the Unix war and made its own customers pay the porting cost.

**Why it failed.** Sun killed its own best-loved operating system for alliance politics rather than engineering reasons — the AT&T deal required SVR4 adoption — and the forced kernel replacement imposed a full porting cycle on customers and ISVs in 1992–95, converting Sun's single largest software advantage into a migration tax at exactly the moment Windows NT was being built.

**What might have changed it.** Evolve SunOS 4 incrementally and add SVR4 compatibility as a personality rather than replacing the kernel wholesale. The blast radius is larger than Sun: the AT&T–Sun alliance is what triggered the founding of OSF in May 1988 and split the industry. No SVR4 deal means no OSF, no OSF/1, and a far less fragmented Unix facing NT in 1993.

**Evidence.** Announcement date 4 Sept 1991; Solaris 2.0 shipped June 1992; contemporaneous FAQs treat 2.5 (1995) as the first genuinely stable release. The retroactive 'Solaris 1.0' renaming of SunOS 4.1.1 is documented. AT&T–Sun collaboration began in 1987; OSF was proposed by DEC's Armando Stettner at an invitation-only meeting in January 1988 and founded in May 1988 explicitly in response to it.

Tags: strategic mismanagement, compatibility gap, internal politics, niche capture

Sources:
- https://en.wikipedia.org/wiki/SunOS
- https://en.wikipedia.org/wiki/Oracle_Solaris
- https://en.wikipedia.org/wiki/Unix_wars
- https://en.wikipedia.org/wiki/Open_Software_Foundation
- https://softpanorama.org/Solaris/solaris_history.shtml

### Taligent (TalOS and CommonPoint)
*1992–1998 · Taligent Inc. — joint venture of Apple and IBM, later with Hewlett-Packard*

**What was brilliant.** The most complete application framework ever attempted: around a hundred interlocking frameworks and thousands of C++ classes covering text, resolution- and device-independent 2D/3D graphics, compound documents, printing, internationalization, collaboration and persistence, designed so applications were assembled by subclassing framework behaviour rather than written against a procedural API. The internationalization work was genuinely world-class and survives today as IBM's ICU, the Unicode library underneath Java, Android, macOS and most of the web.

**What happened.** Incorporated 2 March 1992; HP bought 15% in January 1994. TalOS, the native operating system, was cancelled in May 1995 — before Windows 95 shipped. CommonPoint, the framework runtime, was released 28 July 1995 for AIX at $1,500 runtime or $5,900 with SDK, plus $1,800 for the required Cset++ compiler, and needed 18 MB of RAM overhead. On 19 December 1995 Apple and HP withdrew, nearly 200 of 375 staff were laid off, and Taligent became a wholly owned IBM subsidiary. Dissolved January 1998; the technology was absorbed into IBM's Java class libraries and ICU.

**Why it failed.** Taligent was killed by its own shareholders rather than by a competitor: the joint venture existed so Apple and IBM could each outflank Microsoft, but as soon as each owner's internal OS plans diverged — Copland at Apple, Workplace OS at IBM — no owner was obliged to adopt what all three were jointly funding, so TalOS died in May 1995 with no host platform and no customer.

**What might have changed it.** If Taligent had been a product group inside one company with one shipping host operating system it was required to target, the frameworks would have had a platform and a forcing function; the three-way JV structure guaranteed that shipping was nobody's job and that any owner could defect without consequence.

**Evidence.** Kernel manager Stephen Kurtzman: 'The frameworks were so powerful that you could write any program in three lines of code, but it would take you 6 months to figure out what those three lines were.' InfoWorld to CEO Joe Guglielmi: 'Corporate users don't generally understand what CommonPoint is for.' TalOS was cancelled in May 1995 — three months before Windows 95 shipped — which means it was abandoned internally, not beaten in the market. The pricing is its own evidence of a failed business model: $5,900 plus $1,800 for a compiler, against a free Win32 SDK, for a framework whose stated learning curve was months.

Tags: internal politics, poor developer experience, perpetual rewrite, too late to market, business model failure, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/Taligent
- https://tedium.co/2019/02/28/ibm-workplace-os-taligent-history/
- https://www.techmonitor.ai/technology/ibm_finally_absorbs_taligent_firing_190_apple_and_hewlett_keep_rights_to_technology_stockpile/
- https://rip.so/taligent.html

### TENEX / TOPS-20
*1969-1988 (TENEX at BBN 1969; TOPS-20 shipped January 1976; 36-bit line cancelled 1983; final DEC-20 support ended late 1980s) · Bolt Beranek and Newman (Dan Murphy, Dan Bobrow et al.); productized by Digital Equipment Corporation*

**What was brilliant.** Demand-paged virtual memory in which 'a process address space contains no real storage, it is merely a set of 512 windows (mappings) to storage'. It had copy-on-write pages, file-to-memory mapping, and the JSYS system call interface, with a compatibility layer (PA1050) that ran TOPS-10 binaries. Escape-key command recognition and '?' context help are the direct ancestors of tab completion. Ray Tomlinson's first networked email (1972) ran on TENEX.

**What happened.** DEC adopted TENEX as TOPS-20 for the KL10 and DECSYSTEM-20 (Murphy joined DEC January 1973; shipped 1976). Internal cancellations followed: Dolphin in early 1979, Minnow shelved. In 1983 DEC cancelled the whole PDP-10 line, including the Jupiter follow-on processor, and customers were migrated to VAX/VMS.

**Why it failed.** DEC committed its future to VAX, and TOPS-20 was hand-coded in 36-bit assembly. When DEC decided to leave the 36-bit business, the OS could not migrate to another architecture.

**What might have changed it.** Murphy's own answer: 'If I could have done just one thing differently...it would be to have coded it in a higher level language.' A portable TOPS-20 could have moved to VAX or another architecture the way Unix did.

**Evidence.** Murphy, 'Origins and Development of TOPS-20': by 1979 management had a 'going-out-of-business mindset' toward 36-bit, and the one-OS TOPS-36 plan was dropped. The popular account says Jupiter was 'in good shape' when killed. Murphy, the TOPS-20 lead, says instead that feature creep and weak instruction performance left Jupiter 'mired in weekly schedule replans and indecisiveness'. The cancellation reflected real project trouble plus VAX politics, not only politics.

Tags: internal politics, hardware tied to dying platform, strategic mismanagement, compatibility gap

Sources:
- https://opost.com/tenex/hbook.html
- https://en.wikipedia.org/wiki/TOPS-20
- https://en.wikipedia.org/wiki/TENEX_(operating_system)
- https://en.wikipedia.org/wiki/PDP-10

### Ultrix
*1984–1995 (Ultrix-32 in June 1984; version 4.5 in 1995, then Y2K patches only) · Digital Equipment Corporation, Unix Engineering Group (Armando Stettner, Bill Doll and colleagues), Nashua/Merrimack, New Hampshire*

**What was brilliant.** The best-integrated 4.2/4.3BSD in the industry running on the best minicomputer and RISC hardware of its day. DECnet, LAT and TCP/IP coexisting properly in one kernel; System V IPC (named pipes, messages, semaphores, shared memory) grafted cleanly onto a BSD base; symmetric multiprocessing; and documentation and engineering discipline that most Unix vendors of the era could not match. Ultrix on the VAXstation and the MIPS-based DECstation 5000 was the standard academic and research platform for most of a decade, and a very large fraction of early Internet protocol work, X Window development and Project Athena infrastructure ran on it.

**What happened.** DEC never actually wanted to sell it. Ultrix was a defensive product, approved by Ken Olsen — who publicly called Unix 'snake oil' from 1987 — only after IBM announced a native Unix of its own. It never supported shared libraries, dynamically linked executables or memory-mapped files, which by the early 1990s was a serious competitive deficiency against SunOS 4 and its peers. DEC put its strategic weight behind OSF/1 on Alpha from 1992 and let Ultrix expire at version 4.5 in 1995 with only Y2K fixes afterwards. DEC thereby abandoned its Unix franchise twice: once by starving Ultrix, once by discarding it.

**Why it failed.** DEC ran Ultrix as a hedge to protect VMS rather than as a business, so it was deliberately under-invested in exactly the capabilities that decided workstation competitiveness — shared libraries, dynamic linking and mmap — and then DEC threw away the accumulated Unix franchise a second time by switching to OSF/1 on a new architecture.

**What might have changed it.** DEC commits to Ultrix as a strategic product in 1985–87 instead of defending VMS, and ships shared libraries and mmap by 1989. DEC had better hardware, a larger channel and a bigger installed base than Sun in 1986; it declined to be Sun on purpose.

**Evidence.** Mike Humphries of Oracle, in a Computer History Museum oral history recorded 12 June 2007, on DEC's New Hampshire Unix group: its 'real purpose was to persuade customers to stay with VMS, and only sell Unix to those that insisted on it.' Olsen described UNIX as 'snake oil' in public appearances from 1987. Ultrix 'never supported shared libraries or dynamically linked executables' and lacked memory-mapped file support — 'a particular deficiency with Ultrix in comparison to its competitors in the early 1990s.' The project was initiated reactively, after IBM announced plans for a native Unix product.

Tags: internal politics, strategic mismanagement, technical shortfall, hardware tied to dying platform, niche capture

Sources:
- https://en.wikipedia.org/wiki/Ultrix
- https://en.wikipedia.org/wiki/Ken_Olsen
- https://en.wikipedia.org/wiki/Tru64_UNIX

### UnixWare (SVR4.2 'Destiny')
*1992–2001 as a mainstream attempt (Univel formed Dec 1991; UnixWare 1.0 Nov 1992; sold to SCO 6 Dec 1995; to Caldera 2001) · Univel — the Unix System Laboratories / Novell joint venture, formed Dec 1991; later Novell, SCO, Caldera*

**What was brilliant.** On paper, SVR4.2 was the reunification of Unix: the AT&T System V and Berkeley lineages genuinely merged into one kernel, with a real-time scheduler, dynamic linking, STREAMS, the Veritas filesystem and a bundled desktop. UnixWare's own distinctive contribution was deep NetWare integration — native NetWare Core Protocol and IPX, and NDS directory integration, making a Unix server a first-class citizen of the dominant corporate network of the era. UnixWare 2.x shipped a genuinely good fully preemptible multithreaded SMP kernel, and UnixWare 7 (1998), the convergence of UnixWare 2 and OpenServer 5, was a solid SVR5 with clustering and large-memory support.

**What happened.** The vehicle of Unix's single largest strategic failure. Novell announced the purchase of USL on 21 Dec 1992 for roughly $335M in stock, closing June 1993, and thereby acquired all Unix copyrights, trademarks and licensing contracts — and discovered it had bought a royalty business with no desktop channel. Novell gave the UNIX trademark away to X/Open in October 1993. It completed the sale of the UnixWare business to The Santa Cruz Operation on 6 Dec 1995 for about 6.1 million SCO shares (roughly 17% of SCO) plus a royalty stream capped at $84M net present value expiring by 2002. SCO's Unix division passed to Caldera in 2001; Caldera became The SCO Group and spent 2003–2010 suing IBM, Novell and Linux users over code it had bought — and lost, when the courts confirmed Novell, not SCO, had retained the Unix copyrights.

**Peak adoption.** about 35,000 copies of UnixWare 1.0 sold. That single number is the epitaph of the Unix desktop.

**Why it failed.** Novell bought Unix to fight Microsoft on the desktop while having nothing at all to sell against the Windows 3.1 application library, and then priced and channelled UnixWare like a server licence — so a technically unified Unix arriving in November 1992, four months before Windows NT 3.1, was already irrelevant for want of applications.

**What might have changed it.** AT&T and Sun never fragment Unix in 1987–88, so a single binary-compatible SVR4 with a real ISV base exists by 1990 and the industry meets NT as one platform rather than a dozen. Failing that: Novell open-sources UnixWare in 1994 rather than selling it — roughly what Caldera eventually did with ancient Unix sources in 2002, eight years too late.

**Evidence.** About 35,000 copies of UnixWare 1.0 sold. Novell paid ~$335M for USL (announced 21 Dec 1992, closed June 1993) and recovered ~6.1M SCO shares plus capped $84M NPV royalties on 6 Dec 1995. USL itself had ~500 employees and ~$100M annual revenue in 1991. The concrete cost of fragmentation is documented in ISV terms: Informix shipped software for over 100 Unix systems, 'altogether over 1,000 versions of their products'; Ingres supported more than 40. One contemporary analyst on the March 1993 COSE truce: 'Two Unixes are a lot better than 225 — which is what we have had until now.'

Tags: strategic mismanagement, no app ecosystem, business model failure, too late to market, licensing or legal, incumbent lockin

Sources:
- https://en.wikipedia.org/wiki/UnixWare
- https://en.wikipedia.org/wiki/Unix_System_Laboratories
- https://www.novell.com/news/press/archive/1995/12/pr95274.html
- https://en.wikipedia.org/wiki/Univel
- https://en.wikipedia.org/wiki/Unix_wars

### Windows Mobile (Pocket PC / Windows CE)
*1996–2010 (Windows CE 1.0 Nov 1996; Pocket PC 2000 on 19 April 2000; Windows Mobile 6.5 May 2009; superseded by Windows Phone 7 in Oct 2010) · Microsoft*

**What was brilliant.** Windows CE underneath was a genuinely excellent small real-time OS, written from scratch rather than cut down from NT: hard real-time with bounded interrupt latency and priority inheritance, fully componentized so an OEM could compose a custom OS image from a catalogue of hundreds of components using Platform Builder, and processor-agnostic in a way nothing else was — the same OS shipped on ARM, MIPS, SH3/SH4 and x86. CE 6.0 (2006) re-architected the kernel to raise the limits from 32 concurrent processes with 32MB of virtual address space each to 32,000 processes with 2GB each. On top of it, Windows Mobile gave enterprises the one thing no competitor had: a Win32 and .NET Compact Framework programming model that millions of desktop developers already knew, with Visual Studio as the IDE, plus ActiveSync and Exchange ActiveSync push mail — a protocol so good that Apple licensed it and it still synchronises mail on iPhones today.

**What happened.** Peaked around 2007 with roughly 42% of the US smartphone market; Gartner reported global share falling from about 12% to 8% within a year after 2007. HTC alone manufactured 80% of the roughly 50 million Windows Mobile devices shipped through February 2009. Microsoft's answer to the iPhone was a three-year non-response, then a clean break: Windows Phone 7 (Oct 2010) abandoned the Windows Mobile shell, app model and driver model entirely, invalidating every existing app and every OEM's driver investment at once. Share fell 27% (2008) → 15% (2009) → 7% (2010) → 3% (2011). The kernel itself survived far longer than the phone platform: Windows Embedded Compact shipped in industrial handhelds, point-of-sale terminals and automotive head units (Ford SYNC) for another decade, with the final release in 2013 and support ending in October 2023.

**Peak adoption.** ~42% of the US smartphone market in 2007; ~50 million cumulative Windows Mobile devices shipped through February 2009, of which HTC built 80%.

**Why it failed.** Microsoft built a stylus-driven desktop metaphor — Start menu, scrollbars, close boxes — on top of a good real-time kernel, and when capacitive multitouch made that shell obsolete it chose a clean architectural break that discarded its entire app catalogue, driver ecosystem and OEM tooling, converting a 42%-US-share incumbency into a standing start against Android.

**What might have changed it.** HTC proved with TouchFLO 3D and Sense that a finger-first capacitive shell could be layered on Windows Mobile 6.x with full binary app compatibility. If Microsoft had shipped that itself in 2008 as the official platform while building the NT-based successor behind it, it would have kept its installed base and its OEM channel through the 2009–2010 window in which every one of those OEMs instead moved to Android.

**Evidence.** The share series (42% US in 2007 → 27% → 15% → 7% → 3%) and HTC's 80% of ~50m units are the documented figures. The decisive mechanism is the WM→WP7 discontinuity: no Windows Mobile application ran on Windows Phone 7, and no Windows Mobile device could be upgraded to it, so in October 2010 Microsoft's app catalogue went to zero on the same day its OEMs' driver work went to zero. Note a common error in secondary sources: Windows Phone 7 did NOT use the NT kernel — it ran on Windows CE 6.0 R3. The NT transition came with Windows Phone 8 in 2012, which is a separate break.

Tags: strategic mismanagement, poor developer experience, compatibility gap, too late to market, niche capture, incumbent lockin

Sources:
- https://en.wikipedia.org/wiki/Windows_Mobile
- https://en.wikipedia.org/wiki/Windows_Embedded_Compact
- https://en.wikipedia.org/wiki/Pocket_PC_2000
- https://www.howtogeek.com/703977/what-was-windows-ce-and-why-did-people-use-it/
- https://www.gartner.com/en/documents/619509

### Xenix
*1980–1996 (announced 25 Aug 1980; first commercial shipment Jan 1981; last SCO release System V 2.3.4 in Aug 1991; support ended 1 Jan 1996) · Microsoft (licensed Version 7 Unix from AT&T in 1978–79), with The Santa Cruz Operation as second source from Dec 1981 and owner from 1987*

**What was brilliant.** The system that proved Unix belonged on microcomputers, and the hardest systems engineering Microsoft ever did before NT. Making a minicomputer operating system fit 16-bit micros meant a segmented-memory port to the 8086 and then a genuine protected-mode 80286 port using the MMU, at a time when almost nobody had made 286 protected mode work in a shipping product. SCO's Xenix System V/386 in 1987 was the first 32-bit operating system commercially available for the x86 architecture — before OS/2, before Windows NT, and four years before Linux. It brought real multiuser, multitasking, networked Unix to machines costing a few thousand dollars, running production accounting and database workloads in small businesses.

**What happened.** Microsoft was the dominant microcomputer Unix vendor through the mid-1980s and then walked away. SCO became second source in Dec 1981 and bought US distribution rights in 1984. In 1987 Microsoft transferred Xenix ownership to SCO while retaining just under 20% of the company (a $20M equity investment is documented in 1989). Microsoft redirected the same people and ambitions into OS/2 with IBM, then into Windows NT. SCO shipped the final Xenix release in Aug 1991, moved customers to SCO UNIX and OpenServer from 1992, and ended Xenix support on 1 Jan 1996 — the same year Microsoft finally stopped running its own internal email on Xenix and moved to Exchange Server.

**Peak adoption.** roughly 60,000 Xenix systems deployed worldwide by 1987. SCO's share of x86 Unix rose from 12% in early 1986 to over 40% by the end of fiscal 1987, with SCO Xenix revenue doubling annually from 1984 to 1987 to reach $27.1M in FY1987. Separately, AT&T reported around 500,000 Unix licences worldwide by 1988, of which Xenix developers were about half — a licence-count measure, not an installed-system count.

**Why it failed.** Microsoft abandoned the leading microcomputer Unix by deliberate choice: it could never own Unix, because AT&T did and began marketing System V directly after the 1984 divestiture, and Microsoft's entire business model required owning the platform — so it traded Xenix to SCO for equity and rebuilt the same ambition as a product it could control.

**What might have changed it.** AT&T licenses Unix to Microsoft on ownership terms, or the consent decree resolves differently. A Microsoft that owned its own Unix in 1985 — already the largest distributor of Unix by installations — is a straightforwardly different industry. Note that the folk history that 'Microsoft never cared about Unix' is simply wrong: Microsoft shipped Unix for a decade, led the segment, and ran its own company on it until 1996.

**Evidence.** Microsoft licensed V7 Unix in 1978–79 and announced Xenix on 25 Aug 1980, first shipping Jan 1981. ~60,000 systems deployed by 1987; SCO share 12% (early 1986) → over 40% (end FY1987); SCO Xenix revenue $27.1M in FY1987 after doubling annually since 1984. Ownership transferred to SCO in 1987 with Microsoft keeping just under 20%; $20M investment in 1989. SCO Xenix System V/386 (1987) was the first 32-bit OS available for x86. Xenix ran Microsoft's internal email until 1996. Final Xenix release Aug 1991; support ended 1 Jan 1996.

Tags: strategic mismanagement, business model failure, licensing or legal, acquired and killed, no hardware channel

Sources:
- https://en.wikipedia.org/wiki/Xenix
- https://grokipedia.com/page/Xenix
- https://www.os2museum.com/wp/ibm-pc-xenix/
- https://microsoft.fandom.com/wiki/Xenix

## Captured by a paying niche (18)

### Acorn MOS
*1981–1986 (MOS 0.10 through MOS 5; BBC Micro discontinued 1994) · Acorn Computers, Cambridge — the same team, Sophie Wilson and Steve Furber, that went on to design the ARM*

**What was brilliant.** A complete, fully vectored operating system in 16 KiB of 6502 ROM. Every entry point (OSBYTE, OSWORD, OSWRCH, OSFILE, OSFSC) ran through a RAM vector table, so any call could be intercepted and replaced by a paged ROM or a running program. The filing system was pluggable behind one interface — DFS, ADFS, ROM FS and Econet NFS were interchangeable, so a machine could boot and run entirely from a network file server. Sideways ROM banking gave 16 KiB paged expansion ROMs, each with allocated private workspace and a service-call protocol, a genuinely clean extension mechanism. The Tube interface let a second processor (6502, Z80, 32016 — and, crucially, the first ARM evaluation system) execute user code in its own memory map while the host kept servicing I/O, with the OS mediating between the two address spaces. Econet networked whole schools at 100 kbit/s from 1981.

**What happened.** MOS 3.x for the BBC Master (1986) was the last 8-bit version. Its architecture — star commands, VDU control codes, vectored calls, filing-system abstraction — was carried directly into Arthur and then RISC OS on the ARM-based Archimedes from 1987. The BBC Micro was discontinued in 1994 and Acorn's education base then eroded to the PC.

**Peak adoption.** over 1.5 million BBC Micros sold against an initial forecast of about 12,000; Acorn claimed 85% of British school computers by October 1984, delivering 40,000 machines a month, and about 80% of UK schools had one by 1985

**Why it failed.** The BBC Computer Literacy Project contract that made Acorn dominant also confined it: MOS shipped only inside a BBC-branded education machine priced where no mass home buyer would follow, so the OS owned one country's classrooms and essentially no other market — and when schools standardised on the PC, it had nowhere to go.

**What might have changed it.** A cheap, MOS-compatible consumer machine sold internationally at Commodore prices during the 1982–83 peak converts a captive education base into a general platform. The Electron was that machine, but it arrived in 1983 and was crippled by Acorn's Christmas 1983 supply failure, which nearly bankrupted the company.

**Evidence.** 1.5M units against a 12,000-unit forecast, and 85% UK schools share by October 1984, establish both the technical appeal and the geographic confinement. The Tube hosted the first ARM development system — the OS architecture directly enabled the birth of the ARM, which outlived everything else in this list by orders of magnitude, while the OS itself stayed inside British classrooms.

Tags: niche capture, no hardware channel, business model failure, incumbent lockin

Sources:
- https://en.wikipedia.org/wiki/Acorn_MOS
- https://en.wikipedia.org/wiki/BBC_Micro
- https://en.wikipedia.org/wiki/Acorn_Computers

### Burroughs MCP (B5000 lineage, Unisys ClearPath MCP)
*1961-present (B5000 MCP 1961; B6500/B6700 MCP late 1960s; Unisys since 1986; software-only on x86 since 2016) · Burroughs Corporation (Bob Barton's B5000 team); Unisys after the 1986 Burroughs-Sperry merger*

**What was brilliant.** Often called the first OS written entirely in a high-level language (ESPOL, an ALGOL 60 extension), on hardware designed around high-level languages. It used tagged 48-bit words that separate descriptors, operands and code, and hardware stacks. It was among the first commercial systems with segmented virtual memory (the Ferranti Atlas was contemporary) and with multiprocessor OS support. Burroughs shipped MCP source to customers, who could modify and recompile it.

**What happened.** Burroughs merged with Sperry to form Unisys in 1986, and the MCP line continued as A Series, then ClearPath NX and Libra. Proprietary hardware production ended in the early 2010s. Since 2016 MCP runs under emulation on commodity Intel x86, under VMware or in Azure, and Unisys still publishes MCP release roadmaps (Releases 21-22, 2023). It survives mainly in banking and government.

**Why it failed.** MCP was inseparable from Burroughs' proprietary stack architecture. Burroughs was a second-tier 'BUNCH' vendor that could not overcome IBM System/360 lock-in, so MCP ended up as a loyal, long-lived niche for existing customers instead of a broad platform.

**What might have changed it.** If Burroughs had licensed the B5000/B6500 architecture and MCP widely in the 1960s-70s, instead of keeping it a single-vendor product against IBM, its language-oriented protected design could have been a second industry standard.

**Evidence.** Wikipedia/Burroughs MCP: 'the first OS written exclusively in a high-level language'. Hardware production 'ceased in the early 2010s, and the operating system is now run under emulation.' In 2016 Unisys released a hardware-independent MCP running 'on any Intel x86 server under VMware ESXi'. The 'first commercial virtual memory' claim is contested because of the Ferranti Atlas (1962), so it should be read as among the first.

Tags: niche capture, hardware tied to dying platform, incumbent lockin, compatibility gap

Sources:
- https://en.wikipedia.org/wiki/Burroughs_MCP
- https://en.wikipedia.org/wiki/Burroughs_Large_Systems
- https://dl.acm.org/doi/pdf/10.1145/641542.641543
- https://www.theregister.com/2016/04/28/unisys_releases_clearpath_for_vms_or_x86/
- https://www.unisys.com/siteassets/microsites/clearpath-future-matters/cpf-mcp-product-roadmap-2023.pdf

### Commercial seL4 (OK Labs/OKL4, Cog Systems, Ghost Locomotion, HENSOLDT TRENTOS, Kry10)
*2006-present (OK Labs founded 2006; seL4 functional correctness proof completed 29 July 2009; DARPA HACMS 2012-2017; open-sourced 29 July 2014; seL4 Foundation founded 7 April 2020) · NICTA/CSIRO Data61 Trustworthy Systems and UNSW Sydney -- Gerwin Klein, Gernot Heiser, June Andronick; commercialised by Open Kernel Labs, then General Dynamics, Cog Systems, DornerWorks, HENSOLDT Cyber, Kry10 and Ghost Locomotion*

**What was brilliant.** seL4 is the first general-purpose OS kernel with a machine-checked proof that its C implementation refines an abstract specification, extended by 2013 to proofs of integrity, confidentiality and authority confinement, and to binary-level correctness of the compiled code -- which removes the C compiler from the trusted computing base, something no other production kernel can claim. It is also the fastest microkernel in the world by IPC latency, so the assurance costs nothing at runtime. In DARPA HACMS a professional Red Team given full access to a partition of Boeing's Unmanned Little Bird helicopter could not compromise the flight mission. The proof is 8,700 lines of C verified at a cost of roughly 20 person-years, about $350 per source line, against 6 person-years to build the comparable and entirely unverified L4Ka::Pistachio.

**What happened.** The kernel succeeded completely; every business built on it failed to reach a general-purpose market. OK Labs shipped L4-derived kernels on over 1.5 billion devices -- later cited as 2 billion -- mostly invisibly, inside Qualcomm phone basebands, and was sold to General Dynamics in September 2012; GD closed the former OK Labs Sydney engineering office in February 2014, and the surviving engineers reformed as Cog Systems. Cog Systems got its D4-Secure HTC One A9 onto the NSA CSfC list in 2017 and has since been acquired by Riverside Research, a defence nonprofit. Founding Foundation member Ghost Locomotion/Ghost Autonomy raised about $220M, took a $5M investment from OpenAI, and shut down on 3 April 2024. Current named deployments -- NIO's ONVO L60 with SkyOS-M, MEP's SureVoice air-traffic and maritime voice control, DornerWorks tooling, Kry10's KOS -- are embedded, automotive and defence, not general-purpose computing.

**Peak adoption.** OKL4 -- the unverified L4 predecessor, not seL4 -- reached over 1.5 billion and later 2 billion devices, almost entirely as a baseband hypervisor invisible to users. Verified seL4 itself has no published general-purpose install figure; the Foundation concedes 'while seL4 is open source, many products or developments on it are not or cannot be public.' Named seL4 volume deployment: NIO ONVO L60 vehicles, announced at the seL4 Summit 2024, with no unit numbers disclosed.

**Why it failed.** A verified microkernel is a component, not a product -- it has no drivers, no file system, no network stack and no user interface -- so every company that tried to sell it had to fund an entire operating system on top at its own expense, and the only customers who would pay for that were defence and embedded integrators building one device each.

**What might have changed it.** If the verified kernel had shipped alongside a verified, reusable OS personality -- drivers, a network stack, a POSIX layer -- rather than a kernel plus a proof, integrators would not each have had to rebuild the userland from nothing. That is precisely what the Trustworthy Systems group is now attempting with LionsOS, fifteen years after the proof was finished, which is itself an admission of what the missing piece was.

**Evidence.** Gernot Heiser's own retrospectives are the primary source and they are unusually frank. On the economics: 'technology like seL4 isn't trivial to commercialise, it requires a big investment.' On the outcome of the NICTA commercialisation strategy: 'The only sensible alternative (both then and with the benefit of hindsight) would have been to open-source seL4, as we had done with L4 earlier.' And on why they sold to General Dynamics: 'GD has the resources to do this, and is active in the right markets, so has the distribution channels' -- eighteen months later GD shut the Sydney office. The commercial record is the argument: the world's only formally verified kernel produced one 1.5-billion-device deployment of its unverified predecessor hidden inside basebands, one $220M startup that shut down in 2024, one company absorbed by a defence nonprofit, one closed engineering office, and a handful of vehicle and air-traffic-control deployments. Proof of correctness turns out to be orthogonal to every variable that actually determines adoption.

Tags: niche capture, no app ecosystem, business model failure, funding collapse, poor developer experience

Sources:
- https://sel4.systems/About/history.html
- https://sel4.systems/use.html
- https://www.sigops.org/s/conferences/sosp/2009/papers/klein-sosp09.pdf
- https://cacm.acm.org/research/sel4-formal-verification-of-an-operating-system-kernel/
- https://microkerneldude.org/2012/10/02/giving-it-away-part-2-on-microkernels-and-the-national-interes/

### Concurrent CP/M → Concurrent DOS → Multiuser DOS / REAL-32 (and FlexOS)
*1982–1990s; VAR derivatives sold into the 2000s · Digital Research, Inc. — project manager Kathryn Strutynski, designer Francis R. Holsworth*

**What was brilliant.** Concurrent CP/M-86 3.0 (late 1982) gave an IBM PC genuine multitasking of four programs with virtual consoles — three years before Windows 1.0 and eleven before Windows NT. Concurrent DOS 286 (1985) was a complete rewrite in C doing protected-mode multitasking on the 80286. Concurrent DOS 386 (1987) exploited the 80386's virtual-8086 mode to run unmodified DOS applications for multiple users on serial terminals, with full preemption and per-user screen contexts. FlexOS, the real-time sibling renamed 1 October 1986, was a modular message-passing real-time multiuser kernel for industrial control and retail.

**What happened.** Novell acquired DRI in July 1991 and abandoned Multiuser DOS in 1992; three VARs — DataPac Australasia, Concurrent Controls and Intelligent Micro Software — licensed the source and shipped derivatives for another decade (IMS REAL/32 through 7.95; CCI Multiuser DOS R18, 21 April 2005). FlexOS was sold to Integrated Systems Inc. in July 1994, but IBM had already based its 4690 Operating System on FlexOS release 2.32; 4690 OS ran retail point-of-sale terminals worldwide for three decades, passing to Toshiba Global Commerce Solutions with IBM's 2012 retail-business sale and now supported only under legacy service contracts.

**Why it failed.** Concurrent DOS needed roughly 200 KB of conventional memory and degraded proportionally with each added task, so on the 640 KB commodity PCs of the 1980s the multiuser capability cost more than single-user buyers would pay — the technology only paid for itself in dedicated verticals, which is precisely where it ended up and stayed.

**What might have changed it.** Positioning Concurrent DOS 386 in 1986–87 as a low-overhead single-user 386 DOS multitasker — the slot Windows/386 and DESQview took — rather than as a multiuser terminal host, plausibly makes it the 386 DOS multitasker instead of a vertical-market product.

**Evidence.** BYTE (1988) found Concurrent DOS 386 'substantially compatible' with MS-DOS and measured video I/O about twice as fast as under DOS, but recorded performance decreasing proportionately with the number of simultaneous applications and noted the higher RAM requirement as the adoption limit. Novell's 1992 abandonment forced the three-way VAR source licence. IBM's 4690 OS was built on FlexOS 2.32 and remained in supermarkets for thirty years — the OS succeeded commercially, just never as a personal-computer OS.

Tags: niche capture, technical shortfall, business model failure, incumbent lockin, acquired and killed, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/Concurrent_DOS
- https://en.wikipedia.org/wiki/FlexOS
- https://en.wikipedia.org/wiki/4690_Operating_System
- https://www.techmonitor.ai/technology/ibm_retains_flexos_in_new_4690_point_of_sale_line/

### Cosmos (C# Open Source Managed Operating System)
*Conceived as CHAOS in 1995; launched as Cosmos in 2007; latest packaged release 21 November 2022; low-level work continuing into 2026 · Chad Z. Hower ('Kudzu') and Matthijs ter Woord, with an open-source community; BSD licensed*

**What was brilliant.** IL2CPU is the real achievement: a working ahead-of-time compiler that takes .NET CIL and emits bootable x86 machine code with no runtime underneath it, paired with X#, a small assembly-level language for the parts C# cannot express, and a plug system for substituting native implementations of base-class-library methods. The result integrates into Visual Studio so that pressing F5 boots your operating system in a VM with a source-level debugger attached over serial or network — the best developer experience any bare-metal project has ever offered, and a clean demonstration that a managed language needs no host runtime to reach the metal.

**What happened.** Honest framing: Cosmos is not an operating system and does not claim to be one — it is an 'operating system construction kit.' The latest packaged Userkit is release 20221121; development continues slowly, with work since late 2024 to port the plug system onto .NET's native AOT compiler. It has produced no general-purpose OS that anyone deploys, and its realized role is education and hobby OS development.

**Why it failed.** Cosmos solved the compiler problem and stopped there — it hands you a bootable C# binary but no driver set, no memory-protected multitasking and no applications — so every user must build an entire OS themselves, which confines it structurally to teaching rather than platform status.

**What might have changed it.** Pair IL2CPU with one opinionated, complete reference operating system — drivers, networking, a package story, a shipping desktop — instead of a construction kit. Redox shows what happens when a managed-language OS project commits to a single finished target; Cosmos has arguably the better compiler and no product.

**Evidence.** Cosmos is described by the project and by Wikipedia as 'a toolkit for building GUI and command-line based operating systems,' with IL2CPU compiling CIL to native x86 and X# supplying assembly-level code; Visual Studio 2022 integration with debugging over serial or network; last packaged release 20221121; origin as Chad Hower's CHAOS concept in 1995, formally launched as Cosmos in 2007.

Tags: niche capture, technical shortfall, no app ecosystem, no hardware channel, business model failure

Sources:
- https://en.wikipedia.org/wiki/Cosmos_(operating_system)
- https://github.com/CosmosOS/IL2CPU
- https://cosmosos.github.io/articles/Compiler/il2cpu.html
- https://www.osnews.com/story/19246/cosmos-one-of-the-open-source-c-kernels/

### IBM System/38 CPF and AS/400 OS/400 (IBM i)
*1978-present (System/38 announced October 1978, shipped 1979-1980; AS/400 1988; renamed i5/OS and then IBM i) · IBM Rochester (Frank Soltis, Glenn Henry et al.), a descendant of the cancelled Future Systems project*

**What was brilliant.** A single-level store in which memory and disk form one 48-bit (later 64-bit) address space, plus a high-level Machine Interface (TIMI). Programs compile to the MI and are translated to native code, so customer binaries survived the 1995 CISC-to-PowerPC switch without recompilation. It is object-based, with a relational database built into the OS. System/38 used hardware capability-based addressing, which OS/400 later removed.

**What happened.** System/38 sold about 20,000 units in five years and was replaced in 1988 by the AS/400, which merged it with the System/36 line. AS/400 was a large commercial success (about 500,000 shipped by 1997) but stayed a midrange business-server niche. It continues as IBM i on Power Systems.

**Peak adoption.** System/38: about 20,000 units in its first five years. AS/400: about 111,000 installations by end of 1990, about 250,000 by 1994, and about 500,000 shipped by 1997. IBM i today: about 120,000 unique customers, about 30,000 of them current (IT Jungle, 2025).

**Why it failed.** IBM positioned System/38 and AS/400 as proprietary turnkey business servers sold through its midrange channel, and never as a general-purpose platform for developers or consumers. They dominated small-business back offices and never reached broader markets.

**What might have changed it.** IBM could have offered the single-level-store and Machine Interface design as the base of its workstation and PC line in the late 1980s, instead of AIX and OS/2. That would have exposed it to general developers.

**Evidence.** Wikipedia/System/38: 'IBM sold approximately 20,000 System/38s during its first five years.' AS/400 reached about 500,000 systems by 1997. IT Jungle counts about 120,000 IBM i customers today. The popular story that 'AS/400 is dead' is wrong: this is a commercially successful system confined to a niche, not a failure. The architecture came from IBM's Future Systems project, formally cancelled in February 1975.

Tags: niche capture, incumbent lockin, hardware tied to dying platform, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/IBM_System/38
- https://en.wikipedia.org/wiki/IBM_AS/400
- https://en.wikipedia.org/wiki/Control_Program_Facility
- https://en.wikipedia.org/wiki/IBM_Future_Systems_project
- https://www.itjungle.com/2025/02/17/state-of-the-power-systems-base-2025-the-systems/

### MSX-DOS / MSX-DOS 2
*1984–1993 (MSX-DOS 1.00, June 1984; 2.20, July 1988; 2.31, Dec 1991; turboR production ended 1993) · Microsoft Japan with ASCII Corporation (Kazuhiko Nishi); original Z80 MSX-DOS written by Tim Paterson, author of 86-DOS; I/O system by ASCII's Jay Suzuki*

**What was brilliant.** The only DOS that is simultaneously CP/M-80 binary compatible and MS-DOS FAT compatible: it ran the entire CP/M-80 application library unmodified while reading and writing FAT12 disks an IBM PC could read. Paterson built a Z80 emulator running under MS-DOS in order to develop it. MSX-DOS 2 (July 1988) added hierarchical subdirectories, environment strings, I/O redirection, file timestamping from the MSX2 real-time clock, and driver-level support for the MSX memory mapper addressing up to 16 MB — a modern DOS on a Z80. Around it, MSX was the first serious attempt at a written hardware standard for home computers, with a BIOS abstraction that let one binary run across machines from Sony, Panasonic, Philips, Yamaha, Toshiba, Canon, Daewoo and Goldstar.

**What happened.** MSX never established itself in the United States — only Spectravideo and Yamaha briefly sold MSX machines there — or in the UK. MSX2+ (1988) and MSX turboR (1990) were Japan-only, and turboR production ended in 1993. The standard retreated into strong regional positions and then expired commercially. Konamiman's Nextor, built on the original MSX-DOS 2 source, still extends it for the retro community.

**Peak adoption.** disputed: 9 million units worldwide including 7 million in Japan by one widely quoted account, against ASCII founder Kazuhiko Nishi's own figure of 3 million in Japan and 1 million overseas. MSX was the leading home computer in South Korea, Brazil and Argentina, strong in the Netherlands and Spain, and used to teach informatics in Soviet and Cuban schools

**Why it failed.** MSX standardised on the Z80 and the TMS9918 video chip and launched on 21 October 1983, by which time the Commodore 64 was already selling in the tens of millions at a lower price with better graphics and sound — so the standard arrived technically behind in the only two markets, the US and UK, large enough to make it a world standard.

**What might have changed it.** Launching in 1982 rather than late 1983, or specifying MSX2-class video (or a 68000) as the baseline rather than as a 1985 upgrade, gives the standard a genuine advantage over the C64 instead of parity at best — and a written multi-vendor standard with a real hardware lead is exactly the thing that becomes a world platform.

**Evidence.** MSX launched 21 October 1983; the C64 sold 12.5–17 million units. US presence was limited to two vendors. That Nishi's own total (about 4 million) is less than half the commonly repeated 9 million is itself evidence that MSX's global reach is routinely overstated — the platform's real success was regional, which is the definition of niche capture.

Tags: niche capture, no hardware channel, too late to market, technical shortfall, business model failure

Sources:
- https://en.wikipedia.org/wiki/MSX
- https://en.wikipedia.org/wiki/MSX-DOS
- https://www.msx.org/wiki/MSX-DOS_2
- https://www.pcgamer.com/the-bright-life-of-the-msx-japans-underdog-pc/

### Nokia Series 40 and the Nokia Asha platform
*1999–2014 · Nokia (Asha platform built on technology from Smarterphone, acquired January 2012; UI design led by Peter Skillman, inherited from MeeGo Harmattan)*

**What was brilliant.** Series 40 is arguably the most efficient consumer operating system ever shipped in volume: a hard-real-time, single-address-space, non-multitasking system running an entire phone — GSM/GPRS stack, UI, Java ME runtime, browser, camera, messaging — in a few hundred kilobytes of RAM on an MMU-less ARM7, booting in about a second and delivering week-long battery life at a bill of materials under $30. The Asha platform (2013) rebuilt the UI on Smarterphone's stack with interaction design inherited from MeeGo Harmattan: full-screen gesture navigation with no home button, bezel-swipe back, and 'Fastlane', a chronological activity stream that predates the mainstream adoption of both ideas — on hardware costing under $100. Nokia Xpress Browser did server-side rendering and compression of pages (the same architecture as WebTV's proxies and Opera Mini), cutting data use by up to 90% and making the web usable on 2G in markets where data was sold by the megabyte.

**What happened.** First Series 40 device was the Nokia 7110 (1999). Nokia announced the 1.5 billionth Series 40 handset on 25 January 2012 — an Asha 303 sold in São Paulo — while selling roughly twelve per second. The Asha platform launched with the Asha 501 on 9 May 2013, followed by the Asha 500/502/503 at Nokia World in October 2013 and the Asha 230 in February 2014. After Microsoft closed its acquisition of Nokia's devices business, it announced on 17 July 2014 that it would cease all development of the Asha, Series 40 and Nokia X ranges in favour of Lumia Windows Phone products, phasing out production over the following 18 months.

**Peak adoption.** More than 1.5 billion Series 40 handsets sold cumulatively as of 25 January 2012 — one of the highest-volume software platforms ever deployed on any hardware.

**Why it failed.** Series 40 sold in the billions but Nokia deliberately kept it beneath its smartphone lines to protect Symbian, so it was never given a native SDK, a real app store or true multitasking; by the time Asha tried to make it a smartphone platform in 2013, Android had already reached the same price point with a genuine application catalogue, and Microsoft — which had bought Nokia in order to sell Windows Phone — killed the whole line five months after closing.

**What might have changed it.** If Nokia had given Series 40 a native SDK and a store around 2008 instead of defending Symbian's position above it, the billion-unit emerging-market channel it already owned would have become an application platform before sub-$100 Android arrived — the exact market Android then took.

**Evidence.** The popular framing that Series 40 'failed' is wrong: commercially it is one of the best-selling platforms in history at 1.5 billion units. What failed is its status as a platform — the Asha platform shipped without true multitasking, with third-party development limited to Java ME and Gecko-rendered web apps in Nokia Xpress, and with no native API. Microsoft's 17 July 2014 statement ended all Asha, Series 40 and Nokia X development outright in favour of Lumia, retiring a line that was still selling in volume.

Tags: niche capture, acquired and killed, no app ecosystem, strategic mismanagement, technical shortfall

Sources:
- https://en.wikipedia.org/wiki/Nokia_Asha_platform
- https://en.wikipedia.org/wiki/Nokia_Asha_series
- https://thenextweb.com/news/nokia-has-now-shipped-1-5-billion-series-40-handsets-sells-12-of-them-every-second
- https://www.engadget.com/2012-01-25-nokia-s40-sales-reach-1-5-billion.html
- https://www.fool.com/investing/general/2014/07/23/why-did-microsoft-kill-off-nokia-x-and-asha.aspx

### PolyXene
*2002-present (DGA feasibility study launched 2002; development contract to Bertin Technologies 2004; Common Criteria EAL5 certificate for v1.1 awarded 29 September 2009; pilot deployment at the French armed forces staff headquarters announced June 2014) · Bertin Technologies / Bertin IT, France, under the DGA's SINAPSE programme, certified by ANSSI*

**What was brilliant.** One of very few hypervisors anywhere certified at Common Criteria EAL5 -- a level requiring semiformal design and a structured, analysed implementation, not merely testing -- and among the only ones to reach it as a fielded multilevel product rather than a research artefact. It runs several guest operating systems (Linux, Windows) at different classification levels on one physical workstation, with strong compartmentalization and controlled, auditable transfer between compartments, so a single PC can be simultaneously attached to the internet, an intranet, a NATO coalition network and a national classified network without risk of cross-contamination. It is Europe's answer to the MILS separation-kernel concept, and unlike most of them it was both certified and actually deployed.

**What happened.** It never left the sovereign-defence niche, and was never positioned to. Funded by the DGA, certified by ANSSI, sold through Bertin's defence business to the French state. Bertin IT's product line subsequently moved toward cross-domain appliances -- removable-media decontamination stations, network guards and diodes -- with PolyXene as the internal substrate rather than as a platform third parties build on.

**Why it failed.** PolyXene was specified by, certified for, funded by and sold to a single national defence ministry, so its requirements, its export status and its entire distribution channel were determined by one customer whose needs are structurally opposed to mainstream adoption.

**What might have changed it.** If the EAL5 separation kernel had been marketed as a general commercial MILS hypervisor to European industry -- the route Green Hills INTEGRITY and LynxSecure took in the United States into avionics, automotive and industrial control -- it could have reached markets where separation has genuine commercial demand and recurring volume. Sovereign funding bought the certification and simultaneously foreclosed the market.

**Evidence.** PolyXene is the clearest case in this slice of provable security that was actually proven, actually certified by a national authority, actually fielded -- and still reached essentially nobody, because the funding model determined the market. The EAL5 certificate was presented by ANSSI's deputy director general jointly to Bertin and the DGA in September 2009, which tells you the customer and the vendor and the certifier were all arms of the same state. Five years later the publicly announced milestone was still a pilot deployment at one headquarters. The lesson generalises across the category: state-funded high-assurance systems get the assurance and lose the market, because the customer that pays for the proof is also the customer that restricts who else may have it.

Tags: niche capture, no app ecosystem, licensing or legal, no hardware channel, business model failure

Sources:
- https://www.ssi.gouv.fr/publication/remise-dun-certificat-eal-5-a-bertin-technologie-pour-son-logiciel-polyxene/
- https://www.cio-online.com/actualites/lire-la-dga-se-dote-d-un-hyperviseur-certifie-cc-eal5-2440.html
- https://www.globalsecuritymag.fr/PolyXene-une-solution-de-securite,20140616,45735.html
- https://theatrum-belli.com/cyberdefense-bertin-it-deploie-une-plateforme-pilote-de-securisation-de-linformation-a-letat-major-des-armees/
- https://bertin.fr/medias/polyxener-la-solution-logicielle-de-tres-haute-securite-developpee-par-bertin-it

### Purism PureOS (with Librem hardware)
*2014-present · Todd Weaver, Purism SPC*

**What was brilliant.** An FSF-endorsed Debian-based OS shipped on vertically integrated hardware with coreboot, a neutralised and disabled Intel Management Engine, and physical kill switches for camera, microphone, Wi-Fi, Bluetooth and modem. Purism wrote the Phosh mobile shell and the libhandy adaptive-GTK library, which grew into GNOME's libadwaita. That made one GNOME app codebase work on phone and desktop, and Phosh became a standard shell across mobile Linux distributions.

**What happened.** Laptop crowdfunding from 2014. The Librem 5 phone campaign (August 2017) reached its $1.5M goal. Launch slipped from January 2019 through dev kits in 2019-2020; mass production shipped on 18 November 2020. The price rose from $599 to $1,199 (November 2021) and $1,299 (2022), later cut to $999. There were public complaints of refunds refused or taking years. It continues as a niche privacy brand.

**Why it failed.** Its free-software-only rules exclude proprietary firmware, drivers and apps, which limits hardware choice and app compatibility. Tiny-volume vertical hardware brings multi-year delays and prices of $1,000 or more. Together these limit it to ideologically committed buyers.

**What might have changed it.** Separate PureOS from Purism hardware and ship it as a privacy-hardened, broadly compatible distribution (allowing firmware blobs and Android-app compatibility), taking volume from existing OEM hardware instead of building its own phone.

**Evidence.** Librem 5 went from 2017 pre-orders at $599 to shipping in late 2020 at a list price that later reached $1,299, which Purism blamed on supply chains. A 2019 pre-order customer reported taking almost six years to get a refund after Purism changed its no-questions refund policy. FSF endorsement requires no nonfree software, which excludes mainstream proprietary apps.

Tags: niche capture, no app ecosystem, business model failure, technical shortfall, strategic mismanagement

Sources:
- https://en.wikipedia.org/wiki/Librem_5
- https://puri.sm/posts/librem-5-price-increase/
- https://liliputing.com/purisms-librem-5-linux-smartphone-costs-1199-after-the-latest-price-hike/
- https://battlepenguin.com/tech/the-purism-libre-5-and-the-six-year-refund/
- https://lwn.net/Articles/841105/

### QNX (as a general-purpose / consumer OS)
*1980–present; the consumer and desktop attempts run roughly 1999–2016 · Gordon Bell and Dan Dodge, University of Waterloo students, who founded Quantum Software Systems on 30 March 1980 (renamed QNX Software Systems); later Harman International, then BlackBerry*

**What was brilliant.** The one microkernel that was demonstrably not slow, and the only one to prove it to consumers. Neutrino's procnto kernel contains only CPU scheduling, interprocess communication, interrupt redirection and timers; filesystems, the TCP/IP stack, the graphics stack, USB and every device driver are ordinary user processes that can be killed and restarted while the system keeps running. Its synchronous message-passing IPC carries priority inheritance, so a low-priority driver cannot priority-invert a real-time client — the property that makes hard deadlines provable rather than hoped for. And it was small enough that in 1999 QNX shipped a bootable 1.44 MB floppy containing the operating system, the Photon microGUI, a TCP/IP stack, the Voyager HTML 3 browser, a graphical text editor and a web server. Nothing before or since has fit a working networked graphical desktop in 1.44 MB.

**What happened.** The floppy demo (1999) was followed by a genuine consumer push: a free-for-non-commercial-use QNX Realtime Platform / QNX 6 desktop with Photon and ports of common UNIX software. It never obtained retail distribution or an application base, and the company retreated to embedded licensing. Harman International acquired QNX Software Systems in 2004. RIM acquired it from Harman on 12 April 2010, and on the day the deal was announced 'QNX source code access was restricted from the public and hobbyists.' RIM built the PlayBook (2011) and BlackBerry 10 (January 2013) on it; BB10 collapsed to 0.0% of smartphone share, BlackBerry exited handset hardware on 28 September 2016, and decommissioned the legacy BlackBerry OS infrastructure on 4 January 2022. QNX itself thrives — in cars.

**Peak adoption.** Automotive: more than 255 million vehicles embedding QNX as of October 2024 per TechInsights, up 20 million year-on-year and 80 million since 2020. Consumer: BlackBerry sold 207,900 BB10-running devices in Q4 2016, a 0.048% global smartphone share per Gartner. Desktop QNX: undocumented, negligible.

**Why it failed.** QNX's technical superiority lay entirely in properties ordinary users never pay for — determinism, fault isolation, footprint — and no application ecosystem existed to convert them into visible benefit; when it finally got a consumer chance, it arrived as the operating system of a handset vendor already in terminal decline.

**What might have changed it.** If QNX had kept the Realtime Platform source open and freely redistributable after 2001 instead of re-closing it (and closing it again on the day of the RIM deal in 2010), it would have acquired the one asset it never had — a self-sustaining developer community porting applications — at precisely the moment Linux was assembling one. QNX had the better kernel in 1999 and lost on distribution, not on engineering.

**Evidence.** The 1999 demo disk fit 'the POSIX-compliant QNX 4 OS, a full graphical user interface, graphical text editor, TCP/IP networking, web browser and web server' on a single 1.44 MB floppy, running on a 386 with 8 MB RAM and no hard disk. Source access was restricted to the public and hobbyists on the same day the RIM acquisition was announced (April 2010). By Q4 2016 BlackBerry's own-OS share was 0.0% (207,900 units); hardware development ceased 28 September 2016. Meanwhile QNX reached >255 million vehicles by October 2024 with BMW, Bosch, Continental, Geely, Honda, Mercedes-Benz, Toyota, Volkswagen and Volvo as customers — a textbook niche capture, not a technical failure.

Tags: niche capture, no app ecosystem, licensing or legal, hardware tied to dying platform

Sources:
- https://en.wikipedia.org/wiki/QNX
- https://winworldpc.com/product/qnx/144mb-demo
- https://www.osnews.com/story/27413/qnx-14-mb-floppy-disk-demo/
- https://toastytech.com/guis/qnxdemo.html
- https://www.automotiveworld.com/news-releases/blackberry-qnx-embedded-technology-powers-255-million-vehicles-on-the-road-today/

### RISC OS (Arthur)
*1987–present (Arthur 1.20, 25 Sept 1987; RISC OS 2, April 1989; RISC OS 5.30, April 2024) · Acorn Computers; now RISC OS Open / RISC OS Developments*

**What was brilliant.** A fully modular OS in ROM in which every subsystem is a relocatable, replaceable module reached by software interrupt, so the system could be extended or patched without a new ROM — rolling OS updates in 1989. Its Font Manager provided system-wide anti-aliased scalable outline fonts with sub-pixel rendering and small-size caching from RISC OS 2 (April 1989) — contemporaneous with Adobe Type Manager on the Mac but built into the OS rather than sold as an add-on, and three years before Windows got TrueType. The three-button mouse model (Select/Menu/Adjust) put every menu on the pointer under the Menu button, eliminating the trip to a menu bar; and drag-and-drop save — dragging the file icon out of a Save box into a directory window or straight into another application — replaced the modal file dialog with a spatial operation many users still consider superior to what displaced it. Booting from ROM made cold start near-instantaneous and OS corruption impossible.

**What happened.** Acorn cancelled the Phoebe/Risc PC 2 and closed its workstation division in 1998, halted work in all areas except set-top boxes in January 1999 and renamed itself Element 14. Rights fragmented: RISCOS Ltd licensed desktop rights in March 1999 (RISC OS 4, July 1999); the set-top branch went to Pace Micro Technology; Castle Technology bought the head licence from Pace in July 2003 and had already shipped 32-bit RISC OS 5 in October 2002. RISC OS Developments acquired RISC OS 5 rights and re-licensed it under Apache 2.0 in October 2018. It survives on the Raspberry Pi and ARM development boards.

**Peak adoption.** shipped on over 500,000 systems by 1996; the A3000 took 37% of the UK schools market over a nine-month period in 1991, and by end-1991 Archimedes machines were roughly 15% of the 500,000+ computers installed in British schools

**Why it failed.** RISC OS was never decoupled from Acorn's own low-volume hardware, so when Acorn's workstation business closed in 1998 the OS lost its only channel — and by then its cooperative, single-address-space, unprotected design could not be credibly sold against protected-memory preemptive systems in any case.

**What might have changed it.** Porting RISC OS to commodity ARM hardware around 1995 and licensing it into the handheld and set-top OEM base that Acorn's own ARM spin-out had created — while adding memory protection and preemption — gives the OS a life independent of Acorn's defeat in the PC market. The Newton and the Psion Series 5 demonstrate that an ARM handheld OS market existed and was winnable.

**Evidence.** Arthur 1.2 shipped with roughly 100 documented bugs, and developers withheld software until RISC OS 2 stabilised the platform in April 1989 — the OS lost its launch window to a rushed first release. Acorn's education share did not convert: 37% of UK school purchases in 1991 but only ~15% of the installed base. RISC OS still uses cooperative multitasking in 2024, thirty-five years on.

Tags: niche capture, no app ecosystem, no hardware channel, technical shortfall, funding collapse, strategic mismanagement

Sources:
- https://en.wikipedia.org/wiki/RISC_OS
- https://en.wikipedia.org/wiki/History_of_RISC_OS
- https://en.wikipedia.org/wiki/Acorn_Archimedes
- https://www.theregister.com/2022/06/21/risc_os_35/

### SCOMP / STOP (Honeywell Secure Communications Processor, later BAE XTS-400)
*1976-present (project begun mid-1970s under Air Force ESD; Class A1 rating for STOP 2.1 in 1984; XTS-200/300/400 line; XTS-400 v6.0 Dec 2003; EAL5+ Mar 2005 and Jul 2008; still sold as STOP OS inside BAE guards) · Honeywell's government contracting organisation, under Air Force Electronic Systems Division and MITRE direction growing out of Roger Schell's security kernel programme; later Wang Government Services, Getronics, then BAE Systems*

**What was brilliant.** The only general-purpose operating system ever to complete a TCSEC Class A1 evaluation as a product: formal top-level specification in a machine-checkable specification language, a demonstrated correspondence from that specification down to the code, and full covert channel analysis. Crucially the assurance was not purely in software -- the Security Protection Module was a dedicated hardware board that performed descriptor validation and ring crossing, so mediation could not be bypassed even by kernel bugs in the wrong layer. The successor XTS-400 carried the same four-ring, mandatory-label, least-privilege kernel onto commodity Intel x86 while exposing a Linux-compatible API, and reached EAL5+ twice.

**What happened.** Never became a platform. It was a government appliance from the start and stayed one, changing corporate owner three times (Honeywell to Wang to Getronics to BAE). It survives today only as the substrate of BAE's XTS Guard 7 cross-domain guard and XTS Diode, and as an OEM component. Lipner's 2015 history calls it the last one standing: 'As of 2014, it is still possible to buy an updated SCOMP (now labeled STOP-OS and sold by BAE systems), but that product is the only survivor.'

**Why it failed.** A1 assurance requires freezing a system and proving it, which is economically incompatible with the rate of change a general-purpose platform must sustain; that arithmetic permanently confined SCOMP to small, high-price, slow-moving government appliance markets where no third-party application ecosystem could ever form.

**What might have changed it.** If the US government had made B2-or-above a procurement requirement for classified processing, the demand would have existed to amortise the assurance cost. Instead the NCSC did the opposite: faced with a nearly empty high-assurance Evaluated Products List, it launched 'C2 by '92', creating mandatory demand for the weakest rating and letting the high-assurance market evaporate.

**Evidence.** Lipner documents the decisive policy inversion directly: 'Perhaps as a result of the paucity of high-class evaluated products ... the late 1980s saw a shift in focus by the NCSC with a new slogan: "C2 by '92."' He also records why the assurance bought no commercial traction: 'by 1990 it had become evident that commercial customers did not require and would not use mandatory security controls,' and the Naval Research Laboratory's Military Message Experiment 'demonstrated that a system based on the Bell-LaPadula model was almost unusable.' His verdict on the whole programme: 'If the objective of the Orange Book and the NCSC was to create a rich supply of high-assurance systems that incorporated mandatory security controls, it is hard to find that the result was anything but failure.'

Tags: niche capture, business model failure, no app ecosystem, incumbent lockin, licensing or legal

Sources:
- https://www.stevelipner.org/links/resources/The%20Birth%20and%20Death%20of%20the%20Orange%20Book.pdf
- https://en.wikipedia.org/wiki/XTS-400
- https://www.baesystems.com/en/product/stop-os
- https://commoncriteriaportal.org/files/epfiles/crp242.pdf
- https://apps.dtic.mil/sti/citations/ADA229523

### seL4
*2006–present; functional correctness proof completed 29 July 2009; open-sourced 29 July 2014; seL4 Foundation launched 7 April 2020 · NICTA, later CSIRO's Data61, and UNSW Sydney — Gerwin Klein, Gernot Heiser, Kevin Elphinstone, June Andronick and colleagues; now stewarded by the seL4 Foundation*

**What was brilliant.** The first general-purpose operating-system kernel with a machine-checked proof of functional correctness running from an abstract specification down to its C implementation — roughly 8,700 lines of C and 600 lines of assembler — later extended to the compiled binary and to integrity, confidentiality and authority-confinement properties. It is also the first protected kernel in the literature with a complete and sound worst-case execution time analysis, so its timing is provable rather than measured. And the verification cost nothing in speed: one-way IPC of 0.09 µs on Haswell and 0.32 µs on Cortex-A9, right at the hardware limit. The deepest design achievement is the resource model: all kernel memory is explicitly accounted to user-level 'Untyped' capabilities and retyped under user control, so the kernel cannot be exhausted by a malicious task and spatial isolation is a theorem rather than an assurance.

**What happened.** Open-sourced in July 2014 (GPLv2 kernel and proofs, BSD libraries and tools). Won the ACM SIGOPS Hall of Fame Award in 2019 and the ACM Software System Award in 2023. The seL4 Foundation launched on 7 April 2020 under the Linux Foundation and is now transitioning to an independent Swiss association; members include Apple, NIO, RTX, DornerWorks, Kry10, Proofcraft, Gapfruit, ETH Zurich and UNSW. Its one large-scale consumer design win is NIO's seL4-based SkyOS, announced October 2023 for mass-produced electric vehicles from 2024. The company built expressly to commercialise this lineage, Open Kernel Labs, went under in 2012 without ever productizing it. Sixteen years after the proof, no consumer general-purpose system runs seL4.

**Why it failed.** A proof stops at the kernel boundary, and everything a user actually runs — drivers, filesystems, network stacks, a browser — lives above that boundary as unverified user-level code somebody must still write; so seL4's verification confers almost no end-user-visible benefit unless an entire verified system is built on top of it, and nobody has funded that.

**What might have changed it.** If the verified kernel had shipped in 2009 alongside a reusable, largely verified user-level system — the drivers, VMM, filesystem and network stack that the Genode, CAmkES and Microkit work only made practical a decade later — seL4 could have become the default trusted execution environment and embedded hypervisor of the 2010s. That ground went instead to ARM TrustZone and KVM, neither of which offers any comparable guarantee.

**Evidence.** Functional correctness proof completed 29 July 2009 over ~8,700 lines of C and 600 lines of assembler; Klein et al. report roughly 20 person-years and 200,000+ lines of Isabelle/HOL proof for the original effort, with the full proof base later exceeding 500,000 lines against ~10,000 lines of code. Klein et al.'s cost estimate is ~$400 per line of code, against roughly $1,000 per line for conventionally developed high-assurance systems — i.e. formal verification was cheaper, which makes the adoption gap a market fact rather than a cost fact. Published IPC: 0.09 µs (301 cycles) on a 3.4 GHz Haswell, 0.32 µs on a 1 GHz Cortex-A9 — no measurable verification penalty. seL4 Foundation launched 7 April 2020 under the Linux Foundation.

Tags: niche capture, no app ecosystem, poor developer experience, business model failure

Sources:
- https://sel4.systems/
- https://en.wikipedia.org/wiki/SeL4
- https://cacm.acm.org/research/sel4-formal-verification-of-an-operating-system-kernel/
- https://trustworthy.systems/publications/nicta_full_text/1817.pdf
- https://sel4.systems/Foundation/Membership/

### Squeak (and SqueakNOS/Etoys as an environment)
*1995–1996 origin at Apple; first public release 1996; still maintained in 2026 · Alan Kay, Dan Ingalls, Ted Kaehler, John Maloney and Scott Wallace at Apple Computer; later Walt Disney Imagineering, HP Labs and SAP*

**What was brilliant.** Squeak closed the last gap in a managed system: its virtual machine — garbage collector, object memory, primitives, JIT — is itself written in Smalltalk (a restricted subset, Slang) and mechanically translated to C, and there is a full VM simulator running inside the image. That means the entire runtime beneath your program is debuggable, profileable and modifiable from inside the running program, and the system can regenerate the VM it runs on. Morphic gave direct-manipulation live objects with no compile/run distinction; Etoys turned that into a tile-based authoring environment children could actually use.

**What happened.** Alive but permanently niche. Its successful descendants left it: Scratch's first implementation was Squeak, and Scratch 3.0 (2019) was rewritten in JavaScript; Etoys shipped on OLPC XO laptops and faded with them; Croquet/Open Cobalt wound down; Pharo forked off and now carries most of the active development. Licensing was a decade-long drag — the non-free Apple Squeak License was only replaced by APSL, then by MIT/Apache with Squeak 4.0 in March 2010, keeping Squeak out of mainstream Linux distributions through its most important years.

**Why it failed.** The image — one live blob that is simultaneously program, data, IDE and process state — is structurally incompatible with the file-and-diff toolchain that every deployment, packaging and version-control system in the industry assumes, so even the projects Squeak incubated migrated off it as soon as they needed to ship to ordinary users.

**What might have changed it.** Release under an OSI-approved license in 1996 rather than 2006–2010, and give the image a first-class source-file and version-control representation from the start. Squeak had the education market's best technology and lost the position to a JavaScript rewrite of its own child; a shippable, diffable, freely redistributable Squeak would have kept Scratch and OLPC on it.

**Evidence.** Squeak's VM is written in Slang and translated to C, with a VM simulator running inside Squeak. Scratch's first version was implemented in Squeak; Scratch 3.0 (2019) is JavaScript. Squeak 4.0 (March 2010) was the first release under MIT/Apache, after a multi-year contributor relicensing effort away from the Apple Squeak License.

Tags: niche capture, licensing or legal, compatibility gap, no app ecosystem, business model failure

Sources:
- https://en.wikipedia.org/wiki/Squeak_(programming_language)
- https://wiki.squeak.org/squeak/389
- https://wiki.laptop.org/go/Squeak
- https://wiki.squeak.org/squeak/3139

### Tandem NonStop Guardian (NonStop Kernel / NonStop OS)
*1976-present (T/16 shipped to Citibank May 1976; OSS POSIX personality 1995; Itanium 2005; x86 2014) · Tandem Computers (Jimmy Treybig, Mike Green, James Katzman); later Compaq, HP, HPE*

**What was brilliant.** A shared-nothing, message-passing OS with no shared memory, so a fault in one CPU cannot corrupt another. Process pairs keep checkpointed backup processes that take over on failure without losing state, and no single hardware or software fault stops the system. Jim Gray's Tandem report 'Why Do Computers Stop' (TR-85.7, 1985) was the founding empirical study of production failures and introduced persistent process pairs combined with transactions. NonStop SQL was an early massively parallel SQL database.

**What happened.** Inc. magazine ranked Tandem the fastest-growing US public company in 1983. It fell behind the 1990s move to open systems. Compaq bought it for about $3.0 billion, announced 23 June 1997, and it passed to HP in 2002 and HPE in 2015. It still runs ATM networks, payment switches, and exchanges.

**Peak adoption.** Tandem 1996 revenue of $1.9 billion. HP claimed that in 2002 NonStop handled 95% of worldwide stock transactions and two-thirds of credit card transactions (a vendor figure cited by Wikipedia, not independently verified).

**Why it failed.** Guardian's fault tolerance needed Tandem's proprietary, expensive hardware and a programming model built on process pairs. That left it worth paying for only in transaction processing where downtime costs millions, and it lost ground as cheaper open Unix systems and clusters became good enough for everyone else.

**What might have changed it.** If Tandem had moved NonStop onto commodity open hardware and exposed it as a standard Unix API in the late 1980s, instead of in 1995 and 2014, it could have become the general high-availability server OS instead of a payments niche.

**Evidence.** Business Standard, 1997: Tandem, 'with 1996 revenues of $1.9 billion', 'had begun a strong rebound... after falling behind in the trend toward open systems based on industry-standard chips and software'. Treybig 'remained CEO... until a downturn in 1996.' Wikipedia notes Stratus offered 'simpler hardware-only redundancy at lower costs'. The x86 port came only in 2014. The term acquired-and-killed is partial here: the product survives under HPE, but Tandem as an independent company ended in 1997.

Tags: niche capture, incumbent lockin, strategic mismanagement, acquired and killed

Sources:
- https://en.wikipedia.org/wiki/Tandem_Computers
- https://www.business-standard.com/article/specials/compaq-buys-tandem-for-3-billion-197062501099_1.html
- https://www.forbes.com/1997/06/24/tandem.html
- https://pages.cs.wisc.edu/~remzi/Classes/739/Fall2015/Papers/gray-why-do-computers-stop-85.pdf
- https://www.allthingsdistributed.com/2017/03/why-do-computers-fail.html

### Trusted Solaris / Solaris Trusted Extensions
*1990-2006 as a standalone product (SunOS MLS 1.0 in 1990; SunOS CMW 1992; Trusted Solaris 1.x through 8; EAL4+ in 2004; discontinuation announced 2005; folded into Solaris 10 11/06 as Trusted Extensions in 2006, which persists in Oracle Solaris 11) · Sun Microsystems; the Trusted Extensions architecture largely the work of Glenn Faden and Casper Dik*

**What was brilliant.** The only mainstream commercial Unix that pushed mandatory labels all the way to the graphical desktop and made it usable. Trusted CDE and later Trusted JDS (a multilevel GNOME) gave every window a classification stripe, made cut-and-paste and drag-and-drop between differently-labelled windows a mediated operation requiring explicit downgrade confirmation, and extended labelling through ZFS datasets, NFS mounts, printing and the network stack (CIPSO). It shipped fine-grained process privileges replacing the root monolith years before Linux capabilities were practical, plus RBAC. In 2004 Trusted Solaris 8 was certified EAL4+ against CAPP, RBACPP and LSPP simultaneously. Trusted Extensions then achieved the genuinely rare engineering feat of delivering full MLS as a configuration option of a stock commercial kernel rather than as a forked one -- no separate binary, no separate release train.

**What happened.** Sun announced in 2005 that the standalone Trusted Solaris product would be discontinued, shipping its functionality instead inside Solaris 10 11/06 as Solaris Trusted Extensions. The technology survived; its host did not. Solaris has been in decline since the Oracle acquisition and the 2017 dismantling of the Solaris organisation, and is now effectively in extended support. In 2019 the third-party integrator Dynamic Systems hired Glenn Faden, the trusted desktop's own architect, specifically to keep the product alive for the remaining customers -- a precise marker of an orphaned platform.

**Why it failed.** Labelled MLS solves a problem that essentially only defence and intelligence customers have, so Trusted Solaris's addressable market was capped at a few thousand seats from the outset, and it ultimately died with its host platform rather than on its own merits.

**What might have changed it.** If Sun had shipped labelling as an opt-in, default-off feature of ordinary Solaris a decade earlier -- the Trusted Extensions model delivered in 1996 rather than 2006 -- the labelled desktop would have ridden the commercial Solaris installed base and had ten years to find non-defence uses, instead of spending its whole productive life quarantined in a separate, separately-priced, separately-certified SKU that no ISV would target.

**Evidence.** The decisive structural fact is the separate-SKU decision: for sixteen years the secure version was a different operating system with a modified kernel, which meant a different release cadence, different certified hardware, different patches, and no application vendor willing to qualify against it. Sun's own 2005 announcement conceded the model had failed by merging it back into base Solaris -- Wikipedia records that with Trusted Extensions 'it is no longer necessary to have a different release with a modified kernel for labeled security environments.' By then Solaris itself was losing the datacentre to Linux, so the fix arrived on a sinking platform. Lipner, writing from the vendor side, groups Trusted Solaris with SELinux as technically capable mandatory-control products whose usage never became widespread outside national security.

Tags: niche capture, no app ecosystem, hardware tied to dying platform, business model failure, incumbent lockin

Sources:
- https://en.wikipedia.org/wiki/Trusted_Solaris
- https://en.wikipedia.org/wiki/Solaris_Trusted_Extensions
- https://www.route-fifty.com/cybersecurity/2005/10/sun-to-discontinue-trusted-solaris/306923/
- https://www.dynamicsystemsinc.com/history-of-trusted-extensions-desktop-ted/
- https://www.oracle.com/solaris/technologies/trusted-extensions-jsp.html

### VMS / OpenVMS
*1977-present (VAX/VMS 1977; Alpha 1992; Itanium 2001-2005; x86-64 via VSI, V9.2 in 2022) · Digital Equipment Corporation (Dave Cutler, Dick Hustvedt, Peter Lipman et al.); later Compaq, HP, VMS Software Inc.*

**What was brilliant.** VAXcluster (announced May 1983, shipped in VMS V4.0 in 1984): a shared-everything cluster with a distributed lock manager, where several machines act as one system and some sites reported uptimes measured in years. It also had file versioning in RMS, a strong privilege and ACL security model, and asynchronous system traps (ASTs). Its kernel design passed via Cutler into Windows NT.

**What happened.** Renamed OpenVMS in 1992. It followed DEC to Compaq (1998) and HP (2002). In June 2013 HP announced no port to the Itanium 9500 generation, with support ending in 2020. In 2014 VMS Software Inc. licensed it and ported it to x86-64 (V9.2, 2022). It still runs in banks, exchanges, rail, and manufacturing process control.

**Peak adoption.** Roughly half a million VMS systems in operation worldwide in the 1990s and 2000s (Wikipedia figure; no primary DEC count found).

**Why it failed.** DEC kept VMS proprietary and tied to its own hardware just as open Unix and then Windows NT commoditized servers. Each later owner treated it as a legacy asset, most visibly HP by pinning it to Itanium. It lost the general server market and survived only where uptime lock-in mattered.

**What might have changed it.** DEC could have licensed VMS to other hardware vendors, or ported it to commodity x86, in the early 1990s instead of in 2014-2022, while it still had half a million systems and a clustering lead over Unix and NT.

**Evidence.** HP's 2013 roadmap ended pre-8.4 support in 2015, Alpha support in 2016 and Itanium support in 2020, and refused a port to new Itanium chips. The Register: 'HP never really promoted its acquisition and OpenVMS suffered from a lack of development.' The x86 port came only after a third party, VSI, took over in 2014, about 25 years after x86 servers took the market.

Tags: niche capture, strategic mismanagement, incumbent lockin, hardware tied to dying platform, acquired and killed

Sources:
- https://en.wikipedia.org/wiki/OpenVMS
- https://en.wikipedia.org/wiki/VMScluster
- https://www.theregister.com/on-prem/2013/06/10/windows-nt-grandaddy-openvms-taken-out-back-single-gunshot-heard/1290732
- https://www.computerworld.com/article/1525025/hp-gives-openvms-new-life-3.html
- https://cstan.io/en/post/2020/05/totgesagte-leben-laenger-openvms-erscheint-fuer-x86/

## Charged at the point of adoption (16)

### 3DO Portfolio OS (Opera platform)
*1991–1996 (console 1993–1996) · The 3DO Company / New Technology Group — OS architected by RJ Mical, hardware with Dave Needle (both ex-Amiga, ex-Atari Lynx)*

**What was brilliant.** A preemptively multitasking, memory-protected 32-bit operating system inside a 1993 games console with 2 MB of RAM. Portfolio had user/supervisor separation, MMU-enforced process isolation, automatic resource tracking (memory and I/O handles released when a process dies), message-based IPC, dynamically loadable device drivers and 'folios' (loadable service modules), a custom filesystem and extensible global error codes. Mical and Needle built it explicitly to fix what they had got wrong on the Amiga — which had no memory protection and no resource tracking. Contemporary consoles had no OS at all: the SNES and Genesis were bare metal, the PlayStation shipped a thin BIOS/library. Because Portfolio abstracted the hardware behind a driver model, the same game binaries ran across independently manufactured hardware from Panasonic, Sanyo, GoldStar and Creative — a console with a genuine HAL, years before anyone else tried it.

**What happened.** Launched 4 October 1993 (Panasonic FZ-1) at $699.99; only ~30,000 units had shipped by mid-November 1993. Cut to $499 after roughly six months and $299 by late 1994. 3DO exited hardware in 1996; the M2 successor was sold to Matsushita/Panasonic and never shipped as a console. The 3DO Company became a game publisher and filed for bankruptcy in 2003. Portfolio's source has since been released publicly (trapexit/portfolio_os).

**Peak adoption.** ~2 million consoles sold across all licensees over the platform's lifetime.

**Why it failed.** The licensing model made the hardware structurally incapable of being sold at or below cost — every licensee had to earn margin on the box itself — so 3DO entered at $699 against $199–$299 competitors and could never close the price gap fast enough to attract the exclusive software that actually sells consoles.

**What might have changed it.** If 3DO had taken the razor-and-blades position itself — subsidise the hardware, fund it with software royalties at Nintendo/Sega rates rather than $3 per disc — the machine could have launched near $399, and Portfolio's real architectural lead would have had a market to matter in.

**Evidence.** Trip Hawkins' own post-mortem: for a console to succeed 'it needed a single strong company to take the lead in marketing, hardware, and software.' 3DO charged publishers only $3 per disc, far below Nintendo's and Sega's rates, so software royalties could not subsidise hardware; licensees therefore priced for hardware profit. Launch price $699.99 versus Genesis/SNES at a third of that; ~30,000 units shipped in the first six weeks; ~2 million lifetime against tens of millions for its competitors.

Tags: business model failure, strategic mismanagement, no hardware channel, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/3DO_Interactive_Multiplayer
- https://3dodev.com/software/operating_systems
- https://github.com/trapexit/portfolio_os
- https://en.wikipedia.org/wiki/RJ_Mical
- https://users.polytech.unice.fr/~buffa/videogames/3do_faq2.4.html

### BeIA (Be Internet Appliance) and the Sony eVilla
*2000–2001 · Be Incorporated (Jean-Louis Gassée, Steve Sakoman)*

**What was brilliant.** BeIA inherited BeOS's two genuine architectural achievements and made them fit an appliance. First, pervasive multithreading: the BeOS API forced applications into multiple threads, giving every window its own thread, so the interface physically could not be blocked by background work — an appliance that never stalls. Second, BFS, a 64-bit journaling filesystem with indexed attributes and live queries, which is database-like metadata in a filesystem years before anyone else shipped it. To these Be added a Compressed File System and CELF (compressed ELF) executables using dictionary-based compression of opcodes, so the entire OS plus an Opera 4 browser, a Java runtime, Flash and RealPlayer fit in 24 MB of flash and booted in seconds on a 266 MHz Geode. In 2001 nothing else booted like a television and stayed responsive under load.

**What happened.** Be pivoted from the desktop to internet appliances in 2000 after failing to sell BeOS against Windows. BeIA shipped in the Compaq iPAQ IA-1, the DT Research DT-300 webpad, the Proview iPAD and the FIC Genesis 2000, and most visibly the Sony eVilla — a 31.5-pound portrait-orientation 15-inch Trinitron web terminal, 266 MHz Geode GX1, 64 MB DRAM, 24 MB flash, 56K modem, no hard disk — released 14 June 2001 at $499 plus $21.95/month. Sony discontinued the eVilla on 13 September 2001, less than three months after launch, recalled all units and refunded every buyer and every subscription. Be sold its assets to Palm in 2001 (reported at roughly $11M in stock) and dissolved; the residual entity's antitrust suit against Microsoft settled in 2003.

**Why it failed.** Be bet the remaining company on the internet-appliance category at the exact moment that category collapsed — a $499 single-purpose web terminal plus $22 a month could not survive against a sub-$600 PC that did everything — and because Be licensed to OEMs rather than shipping its own device, its survival depended on a Sony product it did not control and which Sony recalled within 90 days.

**What might have changed it.** If Be had open-sourced BeOS in 1999–2000, while it still had developer mindshare and the best multimedia stack in the industry, instead of spending its remaining cash chasing OEM appliance design wins, the technology would have had a life independent of Sony's product decisions — which is exactly what the Haiku project then had to reconstruct from nothing over the following two decades.

**Evidence.** Sony's own statement on discontinuing the eVilla after less than three months: the 'product did not meet our expectations, it did not operate as planned.' Every unit was recalled with full hardware and subscription refunds. The category itself was tiny — roughly 150,000 internet appliances shipped in the US in the preceding year — and consumers could not justify a $499 fixed-function terminal against a general-purpose PC at a comparable price. BeIA could not repair Be's finances, and Be's assets went to Palm in 2001.

Tags: business model failure, hardware tied to dying platform, no hardware channel, funding collapse, no app ecosystem, too late to market, acquired and killed

Sources:
- https://en.wikipedia.org/wiki/Sony_eVilla
- https://en.wikipedia.org/wiki/Be_Inc.
- https://www.theregister.com/2001/08/16/palm_buys/
- https://www.computerworld.com/article/1339972/palm-buys-assets-of-multimedia-os-developer.html
- https://en.wikipedia.org/wiki/BeIA

### Boxee (Boxee Box, Boxee TV)
*2008–2015 · Boxee Inc. (Avner Ronen), forked from XBMC*

**What was brilliant.** Boxee took XBMC — already the best media-centre codebase in existence, born on the original Xbox — and rebuilt it around two ideas nobody else had in 2008: a social layer (a feed of what friends were watching, with ratings and recommendations, years before 'social TV' was a phrase) and an open application model, the AppBox, where apps written in Python and HTML could be published by anyone onto a genuine 10-foot interface. The Boxee Box (D-Link DSM-380, 10 November 2010, ~$199, Intel CE4110, 1 GB RAM) paired that software with unusually good industrial design — a canted cube, and a two-sided remote carrying a full QWERTY keyboard on its reverse. It played essentially every container and codec, including the ones licensed services would not touch, and unified local network playback with internet video in one interface. Boxee TV (2012) then added an over-the-air tuner with unlimited cloud DVR storage — the correct product several years before anyone shipped it at scale.

**What happened.** First public alpha 16 June 2008; public beta 7 January 2010; Boxee Box November 2010; Iomega TV with Boxee announced January 2011. Final desktop release 1.5 on 26 December 2011, with all desktop versions discontinued at the end of 2012 as the company bet everything on hardware. Boxee TV launched November 2012 at $99 plus $14.99/month for 'No Limits DVR', available in only eight metropolitan areas. Samsung acquired Boxee in July 2013 for a reported ~$30 million — widely reported as a sale at a loss and effectively an acqui-hire — shut the Boxee cloud DVR on 10 July 2013, and had turned off the remaining Boxee servers by 2015, disabling the boxes' core functionality. Samsung shipped no Boxee-derived product; it went to Tizen instead.

**Why it failed.** Boxee's best features were precisely the ones rights-holders would not tolerate — play any file from any source, and record broadcast television to unlimited cloud storage — so every route to scale ran through licensing deals it was never going to get, and the company then killed its own free cross-platform software distribution to protect hardware margins it never earned, leaving an acqui-hire as the only available exit.

**What might have changed it.** If Boxee had kept the free desktop application alive as its distribution engine instead of discontinuing every desktop version at the end of 2012 to defend a $199 box, it would have retained the installed base that Plex — the other XBMC-derived company, which kept its free clients and monetised the server — converted into a durable business over exactly the same period.

**Evidence.** Boxee discontinued all desktop versions at the end of 2012 after the 1.5 release of December 2011, abandoning its only zero-cost distribution channel. Boxee TV's cloud DVR was limited to eight metros and $14.99/month, constrained by the rights position rather than by engineering. Samsung's acquisition in July 2013 was reported at ~$30M and characterised in contemporaneous coverage as a sale at a loss; the cloud DVR was shut within days of the deal, on 10 July 2013, with users losing all recordings, and the remaining back-end servers were off by 2015.

Tags: business model failure, acquired and killed, licensing or legal, incumbent lockin, no hardware channel

Sources:
- https://en.wikipedia.org/wiki/Boxee
- https://en.wikipedia.org/wiki/Boxee_Box
- https://linuxgizmos.com/boxee-sells-itself-to-samsung-at-a-loss/
- https://variety.com/2013/digital/news/samsung-to-shut-down-boxees-cloud-dvr-service-on-july-10-1200508013/
- https://www.engadget.com/2010-09-13-boxee-box-ditches-nvidias-tegra-2-for-intel-ce4100-pre-orders.html

### Bromium vSentry (micro-virtualization / Microvisor)
*2010-2019 (founded 2010; emerged from stealth 2012; vSentry 1.0 September 2012; LAVA 2014; vSentry 3.0 December 2015; acquired by HP 19 September 2019) · Bromium Inc., Cupertino -- Gaurav Banga with Simon Crosby and Ian Pratt, the latter two being the creators of the Xen hypervisor and founders of XenSource*

**What was brilliant.** The Microvisor was a late-loading, Xen-derived type-1 hypervisor that inserted itself beneath an already-running Windows installation and then hardware-isolated each individual task rather than each machine: every browser tab, every email attachment, every downloaded document got its own micro-VM, built copy-on-write from a pristine gold template using Intel VT-x and EPT, created in milliseconds and destroyed when the task ended. This inverted the endpoint security model -- nothing had to be detected, because malware simply executed inside a container that evaporated, while LAVA introspected the live attack and emitted STIX-formatted threat intelligence about what it had tried to do. It is the only commercial product that made hardware-enforced isolation genuinely invisible to end users, who kept using Windows exactly as before.

**What happened.** Raised $115.7M over seven rounds from Andreessen Horowitz, Ignition, Lightspeed, Highland, Intel Capital and Meritech. Founding CEO Gaurav Banga departed in May 2015 and Ian Pratt took over; a 2016 fundraising attempt failed and the valuation was reported to have been cut roughly in half. HP began reselling the product as HP Sure Click in 2017 and acquired the company outright on 19 September 2019 for undisclosed terms. The technology survives as a bundled feature of HP business PCs under HP Wolf Security -- absorbed into one OEM's hardware differentiation rather than surviving as a platform anyone else can buy.

**Why it failed.** Bromium sold a per-seat security add-on that required specific CPU virtualization features and competed against detection products that were cheaper to deploy and easier for a CISO to justify, so it could never build a standalone business and ended as a bundled differentiator for a single PC vendor.

**What might have changed it.** If Microsoft had not built equivalent hypervisor-based isolation into Windows itself -- virtualization-based security and the Hyper-V-isolated browser -- Bromium would have had years of uncontested positioning in which to convert its technical lead into revenue. The platform owner gave the category away for free, which is the standard way a security startup selling a missing OS feature dies.

**Evidence.** The founders were not the problem: Crosby and Pratt had already built Xen and sold XenSource to Citrix, so this was the most technically credible team in client virtualization. The pattern is visible in the sequence -- $115.7M raised, founding CEO out in May 2015, a failed raise and reported halving of valuation in 2016, HP reselling from 2017, acquisition at undisclosed (and therefore almost certainly unimpressive) terms in September 2019. An isolation product that genuinely works and still cannot be sold standalone is strong evidence that the binding constraint on security-by-isolation is commercial, not technical: enterprises buy detection because it produces alerts that justify the budget line, while prevention that silently works produces nothing to show.

Tags: business model failure, incumbent lockin, no hardware channel, acquired and killed, niche capture

Sources:
- https://en.wikipedia.org/wiki/Bromium
- https://press.hp.com/us/en/press-releases/2019/hp-announces-acquisition-of-bromium.html
- https://www.crunchbase.com/organization/bromium
- https://www.cbinsights.com/company/bromium/financials
- https://www.theregister.com/2019/09/20/hp_acquires_bromium/

### CP/M-86
*1981–1985 (Nov 1981 for IBM Displaywriter; IBM PC version 5 April 1982) · Digital Research, Inc. (Gary Kildall)*

**What was brilliant.** CP/M-86's .CMD executable format was relocatable with an eight-group segment model, letting programs load anywhere and cleanly separate code, data and stack segments — a better fit for the 8086's segmented architecture than MS-DOS's .COM/.EXE scheme and better memory management than PC DOS 1.0. It preserved CP/M-80's BDOS call numbering and command set, so the very large 8-bit CP/M application and developer base could port mechanically. Its parent, CP/M, is the system that made the operating system a portable product independent of hardware — the BDOS/BIOS split every later microcomputer OS copied.

**What happened.** IBM shipped CP/M-86 for the PC on 5 April 1982 at $240 against PC DOS's $40 and it was crushed. DRI cut the end-user price to $60 by early 1983, far too late. Version 1.1 (March 1983) added hard-disk support. The line was folded into Concurrent CP/M-86 3.0 and thence Concurrent DOS. Digital Research itself was sold to Novell in July 1991.

**Peak adoption.** a contemporaneous survey found 96.3% of IBM PCs shipped with DOS against 3.4% with CP/M-86; CP/M-86 did win non-IBM design wins on the DEC Rainbow, Victor 9000/Sirius 1, Apricot PC, CompuPro 816 and Siemens machines

**Why it failed.** IBM's six-to-one retail price spread made CP/M-86 economically irrational for buyers of the machine that defined the market, so application developers followed DOS — and once a software network effect starts, no amount of technical merit reverses it.

**What might have changed it.** Closing an IBM deal in 1980 on a per-machine flat fee — the term DRI refused, insisting on royalties — makes CP/M-86 the bundled PC OS and renders Microsoft's purchase of 86-DOS from Seattle Computer Products irrelevant. Failing that, matching $40 at the PC's launch instead of cutting to $60 in 1983 was the decisive lever.

**Evidence.** The popular story that Kildall lost IBM by going flying is not supported: Dorothy Kildall handled DRI's contract negotiations and the sticking point was IBM's insistence on a one-time fee rather than royalties. Kildall later found that 86-DOS's API function calls matched the CP/M Interface Guide and threatened suit; the resolution was IBM agreeing to offer CP/M-86 — at a price that guaranteed it would lose. The 96.3%/3.4% split is the measured outcome of that pricing, not of any technical comparison.

Tags: business model failure, strategic mismanagement, too late to market, incumbent lockin, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/CP/M-86
- https://en.wikipedia.org/wiki/CP/M
- https://www.howtogeek.com/718124/what-was-cpm-and-why-did-it-lose-to-ms-dos/
- https://computerhistory.org/blog/gary-kildall-40th-anniversary-of-the-birth-of-the-pc-operating-system/

### elementary OS
*2011-present · Daniel (now Danielle) Foré and Cassidy James Blaede, elementary LLC/Inc.*

**What was brilliant.** The most design-led Linux desktop. Pantheon and the Granite/Vala toolkit enforce a written Human Interface Guidelines document across the system and third-party apps. AppCenter was the first distro app store with built-in pay-what-you-want payments to independent developers, and from OS 6 (2021) it moved to a curated Flatpak repository with sandbox permissions.

**What happened.** Funded by pay-what-you-want downloads. OS 6 and 6.1 (2021) sold 'far worse than expected'. In January 2022 the company cut salaries and eliminated employee healthcare. Co-founder Cassidy James resigned on 31 March 2022 after a dispute over dividing roughly $26,000 in company funds, and joined the Endless OS Foundation. Foré continued alone; OS 7 (2023) and OS 8 (2024) shipped.

**Why it failed.** Optional payments for a freely downloadable OS and apps brought in small, volatile revenue: only about 1% of AppCenter downloaders paid. It never paid for more than a handful of staff, and the funding squeeze broke the founding partnership.

**What might have changed it.** A durable OEM preload deal (a hardware partner paying a per-unit licence) or institutional sponsorship, giving recurring revenue independent of individual goodwill payments.

**Evidence.** Cassidy James, 'Farewell, elementary': every release sold better than the last 'until OS 6 and 6.1 which performed far worse than expected', because people were 'less likely to pay an optional amount to download an operating system when they could just get it for free'. In January 2022 the company was 'cutting salaries and completely eliminating healthcare'. The elementary blog reported AppCenter apps paid by only about 1% of downloaders, for example about $200 over 8 months for one app. Company funds were about $26,000 at the time of the split (FOSS Force).

Tags: business model failure, funding collapse, no hardware channel, internal politics, no app ecosystem

Sources:
- https://cassidyjames.com/blog/farewell-elementary/
- https://fossforce.com/2022/04/elementary-os-faces-uncertain-future-after-co-founder-split/
- https://blog.elementary.io/about-appcenter-payments/
- https://www.theregister.com/software/2021/08/12/elementary-os-6-odin-released-on-a-pay-what-you-want-basis/299000
- https://news.itsfoss.com/elementary-co-founder-joins-endless-os/

### Google Glass / Glass OS
*2012–2023 · Google X (Babak Parviz, Thad Starner, Sebastian Thrun)*

**What was brilliant.** Glass solved the two hardest problems in head-worn displays with real engineering. The display was an LCoS microdisplay with field-sequential colour LED illumination projected through a prism beam-splitter into the wearer's upper peripheral field, producing a 640×360 image perceived as a 25-inch screen at eight feet, at very low power, in a 43-gram device. Audio used a bone-conduction transducer — no earbud, essentially inaudible to bystanders. The software model was equally considered: Glass OS (Android-derived) replaced the app grid with a horizontal timeline of cards, and the Mirror API let developers push cards from a server without writing or shipping any on-device code at all — an architecturally correct interaction model for a glanceable, hands-free display, which Google then abandoned by opening the full Android SDK.

**What happened.** Sold to developers from 27 June 2012 at $1,500; shipped from 16 April 2013 to roughly 8,000 '#ifihadglass' Explorers; opened to the US public 15 April 2014 and the UK in June 2014 at £1,000. Explorer Edition production ended 15 January 2015 after a sustained privacy backlash — 'Glasshole' entered common usage, and bars, cinemas and casinos banned the device. The project moved into Alphabet's X and returned as Glass Enterprise Edition (2017) and Enterprise Edition 2 (May 2019) for manufacturing, logistics and field service. Google ended Glass Enterprise sales on 15 March 2023 with support through 15 September 2023 and 'no software updates from Google are planned' — a permanent retreat to niche, then out of it.

**Peak adoption.** ~8,000 Explorer Edition selectees in 2013 plus roughly 2,000 developers from the 2012 I/O pre-sale. Google never disclosed total Explorer or Enterprise unit sales; independent estimates put cumulative shipments in the tens of thousands.

**Why it failed.** Google sold an unfinished research prototype at $1,500 as a consumer fashion object, which put an always-on camera on strangers' faces years before either the social norms or the on-device compute existed to justify it — so the product was defined in public by the privacy backlash rather than by any use case, and Google retreated to the factory floor, where the camera is a feature and the wearer is paid to wear it.

**What might have changed it.** If Glass had launched first as the enterprise tool it eventually became — hands-free work instructions for technicians and warehouse pickers, no consumer sale, no fashion campaign, no $1,500 public beta — it would have built a defensible market before the word 'Glasshole' existed, which is precisely the path Vuzix and RealWear then took successfully.

**Evidence.** The Explorer programme was a paid public beta at $1,500, distributed via a social-media contest to ~8,000 people, with shipping beginning April 2013 — a research prototype sold at a premium to consumers. Consumer sales ended January 2015 after under two years. The enterprise pivot lasted six years (EE 2017, EE2 May 2019) and also ended, with sales stopping 15 March 2023 and support 15 September 2023. The Mirror API — the server-push card model that made Glass coherent — was superseded by the full Android SDK, which is when Glassware stopped being designed for a glanceable display.

Tags: business model failure, no app ecosystem, niche capture, strategic mismanagement, technical shortfall

Sources:
- https://en.wikipedia.org/wiki/Google_Glass
- https://time.com/3669927/google-glass-explorer-program-ends/
- https://9to5google.com/2023/03/15/google-glass-enterprise-edition-discontinued/
- https://support.google.com/glass-enterprise/customer/answer/13417888?hl=en
- https://www.theregister.com/2023/03/16/google_enterprise_glass_discontinued/

### Inferno and Limbo
*1995/1996–2000 at Bell Labs/Lucent; Vita Nuova Holdings 2000–2022; free-software releases from 2004, MIT-licensed 2021 · Bell Labs Computing Science Research Center — Sean Dorward, Rob Pike, David Presotto, Dennis Ritchie, Howard Trickey and Phil Winterbottom*

**What was brilliant.** Inferno combined three things no other system put together. Limbo is a type-safe, garbage-collected language with CSP concurrency built in — spawned threads, typed channels and an alt statement — and it is the direct ancestor of Go's goroutines, channels and select (Pike and Winterbottom designed both). Dis is a register-based bytecode VM deliberately designed so that on-the-fly compilation to native code is cheap, giving performance 'approaching that of compiled C' — a better JIT target than the stack-based JVM shipping at the same moment. And Plan 9's per-process namespaces with the Styx/9P protocol meant any remote resource — another machine's display, a modem, a compute farm — mounts into your namespace as a file tree, so the identical program runs local or distributed with no code change. All of it ran useful applications in 1 MB of RAM with no memory-management unit, on Intel, SPARC, MIPS, ARM, HP-PA and PowerPC, natively or hosted.

**What happened.** Lucent stood up an Inferno Business Unit of about 20 people in 1997, 'initially offered to sell source code licenses of Inferno but found few buyers,' and 'did little marketing and missed the importance of the Internet.' Lucent shipped it internally in the VPN Firewall Brick and the PathStar phone switch. The business unit closed in 2000, within three years of founding, and the rights were sold to Vita Nuova Holdings of York, England. Fourth edition went to free-software licenses in 2004 and to MIT in 2021; Vita Nuova was effectively defunct after 2022. Limbo's ideas reached the mainstream a decade later inside Go.

**Why it failed.** Lucent priced Inferno as a per-device source licence sold to consumer-electronics OEMs at the exact moment Sun was giving Java away behind a global marketing campaign; a technically superior VM lost to a free one with a developer community, because the licence, not the technology, was what an OEM evaluated first.

**What might have changed it.** Release Inferno free with a paid support-and-porting model in 1996–97 — the posture Vita Nuova only reached in 2004, by which time the market had gone. Limbo's channels and namespaces were better than Java's threads and sockets; free distribution in 1996 would have contested exactly the ground Java took, a decade before Go had to reintroduce the same concurrency model.

**Evidence.** Dorward et al., Bell Labs Technical Journal: Inferno 'runs useful applications stand-alone on machines with as little as 1 MB of memory, and does not require memory-mapping hardware,' with Dis JIT output 'approaching that of compiled C.' Lucent licensed Java from Sun and announced all Inferno devices would run Java; a Java-to-Dis translator and a 1998 Limbo programming contest still produced no customers. The ~20-person Inferno Business Unit closed in 2000; rights sold to Vita Nuova the same year.

Tags: business model failure, strategic mismanagement, no app ecosystem, too late to market, niche capture, funding collapse

Sources:
- https://en.wikipedia.org/wiki/Inferno_(operating_system)
- https://en.wikipedia.org/wiki/Limbo_(programming_language)
- https://www.vitanuova.com/inferno/papers/bltj.html
- https://github.com/inferno-os/inferno-os

### Mandrake Linux / Mandriva
*1998-2015 (company); distribution continues via Mageia, OpenMandriva, ROSA · Gaël Duval, MandrakeSoft (Paris)*

**What was brilliant.** The distribution that made Linux installable by non-experts. It was Red Hat plus KDE with Pentium-optimised packages, then added the DrakX graphical installer with partition resizing, the drakconf control centre, and urpmi, which resolved RPM dependencies automatically years before that was common on RPM systems.

**What happened.** IPO on the Paris Marché Libre in 2001. Filed for cessation of payments on 13 January 2003 and left receivership on 30 March 2004 with a plan to repay €4.1M over nine years. Renamed Mandriva in April 2005 after buying Conectiva and a trademark dispute with Hearst over 'Mandrake'. Laid off founder Duval in 2006. Rescued by Russian fund NGI in 2010, when the community forked Mageia. Placed in administrative receivership in early 2015 and liquidated on 22 May 2015.

**Peak adoption.** 5 million users claimed in a 2009 company overview (company claim, not audited)

**Why it failed.** The company earned money from boxed sets and subscriptions to a desktop OS that users could download for free. It never had OEM preload or enterprise support revenue at scale, so it lurched from one insolvency to the next despite having a popular product.

**What might have changed it.** Pivot early (around 2001-2003) to enterprise and server support contracts, as Red Hat did, using the user-friendly desktop as the funnel instead of the product being sold.

**Evidence.** Two insolvency proceedings twelve years apart (January 2003 and May 2015). Q3 2008 revenue was only €1.04M with a €0.64M operating loss, despite the claim of millions of users. Cost-cutting in 2006 removed the founder himself. The community's reaction to the 2010 restructuring was the Mageia fork, which drew away the developer base.

Tags: business model failure, funding collapse, strategic mismanagement, licensing or legal, internal politics

Sources:
- https://en.wikipedia.org/wiki/Mandriva
- https://www.theregister.com/2004/03/31/mandrakesoft_exits_bankruptcy_protection/
- https://lwn.net/Articles/76190/
- https://lwn.net/Articles/307694/
- https://www.linux.com/news/mandrake-founder-gael-duval-sue-mandriva-over-firing/

### Microsoft SPOT / MSN Direct (.NET Micro Framework)
*2002–2012 · Microsoft Research / Smart Personal Objects Technology group, with National Semiconductor and SCA Data Systems*

**What was brilliant.** Two real firsts. First, a managed-code runtime — a stripped-down CLR that became the .NET Micro Framework — running on a custom National Semiconductor SoC (ARM7 core, on-die ROM and SRAM, 100 MHz RF receiver) inside a wristwatch in 2003. Developers wrote watch applications in C# and debugged them in Visual Studio; nothing else in embedded offered that in 2004, and it worked without an MMU or a conventional OS underneath. Second, DirectBand: rather than wait for cellular data, Microsoft leased the 67.65 kHz FM subcarrier from broadcasters in over 100 North American cities and datacast roughly 12 kbit/s net of forward error correction per tower — over 100 MB per day per city — with per-device addressing, so a watch could passively receive personalised calendar entries, messages, news and traffic at very low power. That is a working nationwide low-power wide-area network a decade before LoRa and NB-IoT, built on 1950s broadcast infrastructure.

**What happened.** Announced by Bill Gates at CES in January 2003; MSN Direct launched June 2003. Watches from Fossil and Suunto in 2004, then Tissot (touchscreen) and Swatch; the platform extended to Oregon Scientific weather stations (2006), Melitta coffee makers, and Garmin and other GPS units for live traffic (2007) — the traffic business outlived the watches. Watch production ended in 2008; Microsoft announced the shutdown in 2009 and DirectBand transmissions ceased on 1 January 2012. The .NET Micro Framework was later open-sourced under Apache 2.0 and survives as the community nanoFramework.

**Why it failed.** SPOT was a receive-only device on a proprietary one-way broadcast network sold by subscription, so it could never do the two things that make a wrist device worth wearing — respond and sync — and when the smartphone arrived with two-way IP already paid for inside an existing data plan, both the capability gap and the $59-a-year price became indefensible at once.

**What might have changed it.** If Microsoft had used Bluetooth to a phone for the uplink — the architecture Pebble and Apple Watch later used — instead of building a one-way national FM datacast network, the same silicon, the same C# runtime and the same Visual Studio tooling would have been a viable smartwatch platform five years before Pebble's Kickstarter.

**Evidence.** Microsoft's own stated reason for shutting MSN Direct was 'decreased demand for the service' and 'the emergence of more efficient and popular forms of data distribution, such as Wi-Fi.' DirectBand was one-way by construction: ~12 kbit/s down, no uplink, North America only. Watch production ended in 2008, four years after launch and a year after the iPhone; transmissions ended 1 January 2012. The technology's real legacy is the .NET Micro Framework SDK, announced at Embedded World on 13 February 2007, which outlived the product it was built for.

Tags: business model failure, technical shortfall, no app ecosystem, no hardware channel

Sources:
- https://en.wikipedia.org/wiki/Microsoft_SPOT
- https://en.wikipedia.org/wiki/DirectBand
- https://en.wikipedia.org/wiki/MSN_Direct
- https://en.wikipedia.org/wiki/.NET_Micro_Framework
- https://www.radioworld.com/news-and-business/microsoft-will-shut-down-msn-direct

### Smalltalk-80 / ParcPlace VisualWorks
*1980 (Smalltalk-80 release) – 1999 (ObjectShare delisted and dissolved); ParcPlace Systems founded 1987 · Xerox PARC Learning Research Group (Alan Kay, Dan Ingalls, Adele Goldberg, Peter Deutsch, David Ungar and others); commercialized by ParcPlace Systems, Digitalk, and Tektronix*

**What was brilliant.** Smalltalk-80 was the first complete self-describing object system: classes, the compiler, the scheduler, the debugger and the exception mechanism are all ordinary objects inside one live image, and the debugger lets you edit a method inside a suspended stack frame and resume execution from that frame. It is also the origin of essentially all modern managed-runtime technology — Ungar's generation scavenging (1984) is the ancestor of every generational GC, and Deutsch and Schiffman's 1984 dynamic translation with inline caches is the ancestor of every JIT, running through Self and Strongtalk into Java HotSpot, V8 and .NET. The commercial VisualWorks image was genuinely portable across Unix, Windows and Mac at a time when nothing else was.

**What happened.** Positioned in the early 1990s in the trade press as the COBOL successor for enterprise development. IBM entered with VisualAge Smalltalk in 1995 and ParcPlace merged with Digitalk the same year out of mutual fear of IBM. The market evaporated in 1996 when the Web and Sun's Java campaign arrived simultaneously. Renamed ObjectShare in 1997; VisualWorks sold to Cincom in 1999; ObjectShare delisted from NASDAQ and dissolved in 1999. VisualAge Smalltalk went to Instantiations. Hundreds of legacy enterprise Smalltalk applications still run in production.

**Why it failed.** Smalltalk vendors charged per-seat and per-deployment for a closed image that could not interoperate with anything outside itself, and were mutually incompatible with each other; the moment Sun gave away a nearly-as-capable object language with C-like syntax and a free runtime, the price of the language advantage exceeded its value and the market cleared in about six months.

**What might have changed it.** Standardize one portable image and binary format across ParcPlace, Digitalk and IBM and make the deployment runtime free before 1995, competing on tools rather than taxing deployment. The 1996 browser and Java opening existed largely because Smalltalk had made itself expensive and non-interoperable; a free runtime plus real interop would have left Java a much smaller gap to walk through.

**Evidence.** Gilad Bracha's summary of the value proposition, quoted by Wirfs-Brock: 'Pay a lot of money to be locked in to slow software that exposes your IP, looks weird on screen and cannot interact well with anything else.' Wirfs-Brock: 'The Smalltalk companies were completely unprepared in 1996 to deal with the sudden emergence of the web browser platform,' and vendor images differed as much as 'Windows, Solaris, Linux and Mac OS X,' not as compiler dialects. Timeline: 1995 ParcPlace–Digitalk merger, 1997 ObjectShare rename, 1999 sale to Cincom and dissolution.

Tags: business model failure, compatibility gap, strategic mismanagement, too late to market, no app ecosystem

Sources:
- https://wirfs-brock.com/allen/posts/914
- https://en.wikipedia.org/wiki/Smalltalk
- https://www.cincom.com/blog/smalltalk/smalltalks-past/

### Sony mylo (COM-1 / COM-2, Qtopia Linux)
*2006–2010 · Sony Electronics, on Trolltech's Qtopia (Qt Extended) Linux stack*

**What was brilliant.** The first mass-market consumer device built on the premise that Wi-Fi alone is enough: a pocket communicator with no cellular radio, no carrier and no monthly bill, running a full embedded Linux stack with Trolltech's Qtopia application framework, a real browser (Opera on COM-1, NetFront with Flash Lite on COM-2), Skype VoIP over Wi-Fi, and multi-protocol IM across Google Talk, Yahoo! Messenger and later AIM — shipped in September 2006, a year before the iPhone and iPod touch. The COM-2 carried an 800×480 3.5-inch touchscreen, higher resolution than the contemporary iPhone's 480×320, and Sony opened it to third-party Qtopia applications. Its real significance is architectural: it is the clearest pre-iPhone statement that the pocket computer would be a Wi-Fi Linux device with an application framework rather than a phone with extras.

**What happened.** COM-1 released 15 September 2006 at $349.99 (Freescale i.MX21 ARM, 2.4-inch 320×240, 1 GB flash, 802.11b). COM-2 announced at CES and released 25 January 2008 at $299.99, later cut to $199 (3.5-inch 800×480 touchscreen, 802.11b/g, camera). Sales were poor, no third model was made, and the line was discontinued by 2010. Its software foundation died alongside it: Nokia acquired Trolltech in 2008 and discontinued Qtopia/Qt Extended in 2009.

**Why it failed.** The mylo's whole premise required a dense grid of free public Wi-Fi that simply did not exist in 2006, so its realistic coverage was the home and the dorm — where a laptop already did everything better — and Sony never built a developer programme or store to give anyone a reason to carry a second device once they were outside those two places.

**What might have changed it.** If Sony had shipped a real SDK and storefront for Qtopia on the COM-1 in 2006 and connected it to the assets Sony already owned — PSP's game library and the PlayStation Network identity — the mylo would have been a credible pre-iPhone application platform instead of a Wi-Fi-only instant-messaging terminal.

**Evidence.** The COM-2, the model Sony actually pushed, shipped in January 2008 — six months after the iPod touch — at a higher price with a worse ecosystem and a browser with no application store behind it, and was discounted to $199 and withdrawn. The device targeted 18–24-year-olds explicitly by avoiding cellular costs, which is a pricing argument, not a capability argument, and it collapsed the moment carriers bundled data. Qtopia, the framework Sony bet on, was itself discontinued in 2009 after Nokia's acquisition of Trolltech, leaving the platform with no upstream.

Tags: business model failure, no app ecosystem, no hardware channel, too late to market

Sources:
- https://en.wikipedia.org/wiki/Mylo_(Sony)
- https://www.linuxlookup.com/2006/aug/30/sony_mylo_built_on_qtopia_linux
- https://www.engadget.com/2006-08-10-more-mylo-deets-emerge-linux-is-under-the-hood.html
- https://pocketables.com/2008/02/sony-mylo-at-a.html
- https://en.wikipedia.org/wiki/Greenphone

### Symbolics Genera (Lisp Machine OS)
*1980–1993 as a product line; Symbolics Inc. founded 9 April 1980, Chapter 11 in early 1993, original entity defunct 1996; Portable Genera still maintained (2.0.6, 17 August 2024) · Symbolics, Inc. — a 20-founder spinout of the MIT AI Lab (Russell Noftsker, Tom Knight, David Moon, Dan Weinreb, Howard Cannon, Jack Holloway, Mike McMahon and others), built on the MIT CADR lineage*

**What was brilliant.** Genera is still the most complete realization of a single-address-space, single-language, fully live system: well over half a million lines of Lisp (Wikipedia's Genera article puts the codebase above a million) in which the OS, compiler, editor, debugger, network stack and documentation system are all one mutable image. Dynamic Windows (Genera 7, 1986) made output presentations typed — every glyph on screen retained a pointer to the object that produced it and was therefore mouse-sensitive and re-usable as input, the direct ancestor of CLIM. The condition system with named restarts let you repair a running program from the debugger and continue from the failing frame; every function could be recompiled incrementally into the live system; Document Examiner was a shipping hypertext documentation browser years before the Web. Underneath, the hardware was co-designed with the language: tagged 36-bit (later 40-bit) words with hardware type dispatch and GC support, culminating in Ivory, a 390,000-transistor single-chip Lisp processor addressing 16 GB.

**What happened.** Revenue peaked in fiscal 1986 and collapsed with the AI winter and DARPA's retreat from expert systems. Weinreb's own account describes long-term leases on big new offices and a new factory 'anticipating growth that did not come.' Hardware development effectively ceased around 1990; Chapter 11 was filed in early 1993; the original company was defunct by 1996. Genera was unbundled far too late: Open Genera for DEC Alpha in 1993 (2.0 in 1998), and Portable Genera for x86-64/Arm64/Apple Silicon only in 2021.

**Peak adoption.** Fewer than 7,000 Lisp machines of all makes were sold industry-wide by 1988. Symbolics revenue is reported by secondary sources as $101.6M (1986), $82.1M (1987), $55.6M (1988); Weinreb independently confirms the shape — a rise through 1986 then falling revenue and negative earnings 1987–1989 — without giving dollar figures.

**Why it failed.** Genera's advantages were inseparable from $70,000–$100,000 of custom tagged hardware, so when commodity RISC workstations under $20,000 running Lucid or Franz Common Lisp became good enough around 1987, the software's value could not carry the hardware's price — and Symbolics had structured itself so that it could not sell the software without the machine.

**What might have changed it.** Unbundle Genera as a software product on Sun and DEC Unix workstations in 1986–87 instead of 1993. Symbolics eventually proved the port was feasible (Open Genera on Alpha); doing it while there was still an installed base and a cash cushion would have let the software franchise outlive the hardware business, which is exactly the move Lucid and Franz made profitably with plain Common Lisp.

**Evidence.** Weinreb (Symbolics co-founder): 'Once funds were available, Symbolics was spending money like a lottery winner'; on the MacIvory, 'a Symbolics Ivory chip … that plugged into the NuBus on a Macintosh (oops, not the leading platform)'; on the successor board, 'the DEC Alpha architecture (oops, killed by HP/Compaq, should have used the Intel).' Internal conflict between Noftsker and CEO Brian Sear over hardware-versus-software strategy is cited in the company history. Industry-wide sales under 7,000 units by 1988 against millions of PCs and Unix workstations.

Tags: business model failure, funding collapse, hardware tied to dying platform, strategic mismanagement, internal politics, niche capture

Sources:
- https://danluu.com/symbolics-lisp-machines/
- https://en.wikipedia.org/wiki/Symbolics
- https://en.wikipedia.org/wiki/Genera_(operating_system)
- https://en.wikipedia.org/wiki/Lisp_machine
- https://positronia.com/the-rise-and-fall-of-lisp-machines

### TI Explorer / Explorer II / microExplorer
*1983–early 1990s (design licensed from LMI in 1983; Explorer II with the custom Lisp chip announced February 1987; microExplorer circa 1988) · Texas Instruments, on a design licensed from Lisp Machines Inc. (itself derived from the MIT CADR) plus the Nu Machine platform bought from Western Digital in 1983*

**What was brilliant.** The Explorer II put an entire Lisp machine on one 553,000-transistor custom 'MegaChip' processor — 64-bit microcode executing tagged, CDR-coded 32-bit data with hardware type dispatch — the first single-chip symbolic processor of its class. The microExplorer then folded that whole Lisp machine onto a NuBus card inside an Apple Macintosh II, so a Lisp machine and a Mac shared one screen, one keyboard and one file system: a genuine attempt to get a language-based OS into a mass-market channel rather than a $100,000 box. TI also pioneered NuBus itself on the Explorer family before Apple adopted it for the Mac II.

**What happened.** Explorers carried real production work — SPIKE, the Hubble Space Telescope observation scheduler, was built on them. The AI winter after 1987 removed the customer base (LMI, TI's own design source, went bankrupt in 1987), and TI wound the Explorer line down in the early 1990s; the exact exit date is not well documented. The Explorer system source was eventually archived publicly on the Internet Archive.

**Why it failed.** TI monetized the Lisp advantage as a hardware line item inside a commodity semiconductor company, so when AI-lab and DARPA budgets collapsed after 1987 the Explorer had no non-AI market to retreat to and no internal constituency at TI willing to keep funding a workstation business it was losing anyway.

**What might have changed it.** Ship the microExplorer's price point — a Lisp processor as a card in a mass-market Macintosh — in 1985 rather than around 1988. Arriving before the crash and before Apple's own NuBus machines matured would have let Lisp ride an existing consumer channel instead of competing with one from a $70,000 standalone box.

**Evidence.** TI published its 553k-transistor Lisp processor in February 1987; the Explorer family used NuBus before Apple's Mac II; the Explorer's design was licensed from LMI in 1983 and LMI went bankrupt in 1987; SPIKE, the Hubble scheduling system, was developed on TI Explorers. Explorer-class machines shared the industry's sub-7,000-unit total through 1988.

Tags: business model failure, funding collapse, hardware tied to dying platform, strategic mismanagement, niche capture

Sources:
- https://en.wikipedia.org/wiki/Texas_Instruments_Explorer
- http://unlambda.com/lispm/
- https://en.wikipedia.org/wiki/Lisp_machine
- https://archive.org/details/ti-explorer
- https://positronia.com/the-rise-and-fall-of-lisp-machines

### Ubuntu Unity / Unity 8 convergence
*2010-2017 (community continues as Lomiri and the Ubuntu Unity remix) · Canonical Ltd. (Mark Shuttleworth)*

**What was brilliant.** A consistent desktop design from a Linux vendor: a global menu, the HUD (type-to-search app menus), a launcher and lenses/scopes. Unity 8 on Qt/QML and the Mir display server was the most serious attempt at one convergent shell that adapted between phone, tablet and desktop when docked, several years before Samsung DeX-style convergence.

**What happened.** Default in Ubuntu from 11.04 (April 2011). The Ubuntu Edge crowdfunding raised $12.8M of a $32M target in August 2013 and failed. In April 2017 Shuttleworth ended investment in Unity 8, Mir, the phone and convergence, and Ubuntu 18.04 LTS went back to GNOME. Reports put the Canonical layoffs at up to about 30% of roughly 700 staff. UBports kept it alive as Lomiri.

**Peak adoption.** About 20 million Ubuntu desktop users claimed by Canonical in May 2011, against a stated goal of 200 million by 2015 that was not reached

**Why it failed.** Canonical built a full in-house stack (Unity, Mir, the Unity 8 rewrite) that neither PC OEMs, phone carriers nor upstream Linux adopted. It had no hardware channel or revenue to pay for it, so when Canonical prepared for investors it cut the whole stack.

**What might have changed it.** Build convergence on the shared upstream stack (Wayland and GNOME Shell) instead of Mir and a separate Qt rewrite. That would have cut costs enough to survive, and a volume phone or PC OEM would then have been more likely to commit before the 2017 IPO-driven cuts.

**Evidence.** Shuttleworth, October 2017: 'I made some miscalculations around Unity. I really thought industry would rally to the idea of having a free platform that was independent,' and 'I couldn't make an argument for that to sit on Canonical's books any longer, if we were gonna go on a path to an IPO.' The Ubuntu Edge campaign fell $19M short. The Unity 8 rewrite to Qt/QML and Mir duplicated work being done upstream in Wayland and GNOME.

Tags: business model failure, no hardware channel, strategic mismanagement, perpetual rewrite, internal politics

Sources:
- https://www.omgubuntu.co.uk/2017/10/why-did-ubuntu-drop-unity-mark-shuttleworth-explains
- https://www.omgubuntu.co.uk/2017/04/ubuntu-18-04-ship-gnome-desktop-not-unity
- https://www.phoronix.com/news/Ubuntu-Dropping-Unity
- https://www.cio.com/article/234702/canonical-kills-unity-mir-and-ubuntu-phones.html
- https://www.engadget.com/2013-08-22-ubuntu-edge-indiegogo-campaign-fails.html

### WebTV / MSN TV
*1995–2013 · WebTV Networks (Steve Perlman, Bruce Leak, Phil Goldman); acquired by Microsoft 1997*

**What was brilliant.** A vertically integrated system engineered around one brutal constraint: NTSC television is a 60 Hz interlaced, overscanned 480i display with almost no chroma bandwidth, and web pages are drawn for progressive 72 dpi monitors. Perlman's team attacked both ends. On the client, a 112 MHz MIPS R4640 with 2 MB RAM, 2 MB ROM and 1 MB flash ran a full TCP/IP stack, an HTML renderer and a mail client, with a software filter (TVLens) correcting interlace flicker, chroma crosstalk/rainbowing and resolution. On the service side, a proxy farm re-laid-out, re-rendered and recompressed every page before it reached the box, explicitly filtering high-frequency components so single-pixel horizontal rules would not strobe at 30 Hz (patented, US6662218). That is server-side rendering and image transcoding for a thin client in 1996 — the same idea that reappeared as Opera Mini and Amazon Silk a decade later. WebTV also ran its own dial-up network so it could control round-trip latency and page assembly end to end.

**What happened.** Launched 18 September 1996 ($349 Sony, $329 Philips, $19.95/month). Microsoft acquired WebTV Networks 6 April 1997 for ~$425M in stock and cash (~$503M including vested shares). Rebranded MSN TV in July 2001. MSN TV 2 (2004) moved to a customised Windows CE and broadband. Perlman left in 1999; much of the team went on to build the Xbox. Microsoft shut the MSN TV service on 30 September 2013.

**Peak adoption.** 56,000 subscribers April 1997; ~150,000 autumn 1997; ~325,000 April 1998; ~800,000 May 1999; reported to peak near 1.1 million in the early 2000s, then declined continuously as dial-up died.

**Why it failed.** WebTV was built as a closed subscription appliance whose economics required a monthly fee for web access at exactly the moment the price of web access collapsed toward zero, and Microsoft ran it as a dial-up tier of MSN rather than opening it as a living-room application platform — so the box's fixed 2 MB ROM and 33.6 kbit/s pipe had to track a web that was growing faster than the hardware was ever refreshed.

**What might have changed it.** If Microsoft had turned WebTV into an open third-party application platform on broadband — an SDK, a store, a hardware refresh cadence — instead of an MSN subscription tier, it had the install base and the engineering team (the same people who then built the Xbox) to own the streaming living room a decade before Roku.

**Evidence.** Documented subscriber curve 56k (1997) → ~800k (1999) → shutdown (2013) with no growth after broadband arrived. Microsoft paid $425M in April 1997 and kept the service running 16 years without ever opening it to developers. The proxy transcoding architecture is described in Microsoft/WebTV patent US6662218B2 ('Method of transcoding documents in a network environment using a proxy server'), which specifically claims reducing high-frequency components to suppress interlace flicker. MSN TV 2's switch to Windows CE in 2004 was the last hardware generation; the service died 30 September 2013.

Tags: business model failure, strategic mismanagement, no app ecosystem, acquired and killed

Sources:
- https://en.wikipedia.org/wiki/MSN_TV
- https://patents.google.com/patent/US6662218B2/en
- https://allthingsd.com/20130706/microsoft-quietly-shuts-down-msn-tv-once-known-as-webtv/
- https://www.encyclopedia.com/education/economics-magazines/perlman-steve
- https://web.stanford.edu/class/ee380/9697fall/node5.html

## No channel onto the machines people buy (19)

### Atari TOS (GEMDOS + GEM VDI/AES)
*1985–1993 (TOS 1.0, 20 Nov 1985; TOS 4.04 final) · Atari Corporation (Jack Tramiel); GEM licensed from Digital Research, GEMDOS written at Atari after CP/M-68K was judged inadequate*

**What was brilliant.** A complete GUI operating system — GEMDOS filesystem, BIOS/XBIOS, GEM VDI device-independent graphics and GEM AES window/menu system — in 192 KB of ROM, booting to a usable desktop in seconds on an 8 MHz 68000 with a flat, unsegmented memory model. Because Atari held full independent 68000 development rights to GEM, TOS kept the overlapping resizable windows and live desktop metaphor that Apple's lawsuit forced Digital Research to strip out of PC GEM. OS-level MIDI In/Out with deterministic interrupt timing made the ST the default professional sequencer platform — Cubase and Notator/Logic were born on it — because ST MIDI jitter was measurably lower than PC or Mac interfaces of the era.

**What happened.** Atari discontinued the ST line in 1993 and pivoted to the Jaguar console; TOS 4.04 on the Falcon030 was the last official release. Atari Corp. reverse-merged into JTS Corporation on 30 July 1996, ending the company. EmuTOS (a free TOS-compatible ROM) and FreeMiNT keep the API alive in emulation.

**Peak adoption.** ~2.1 million ST units over the line's life is the best-documented figure; Atari marketing claimed over 3 million in Europe, and higher totals to 6M circulate without sourcing

**Why it failed.** Atari's cost-first hardware strategy stopped funding the OS — TOS had no multitasking, no memory protection and no journalled filesystem for eight years — so when Windows 3.0 (1990) gave commodity PCs a GUI, the ST's only remaining differentiator was MIDI latency, and the platform retreated permanently into the music studio.

**What might have changed it.** Shipping a multitasking TOS with the TT030 in 1989 rather than MultiTOS with the Falcon in 1993 — or committing to the PC-compatible market Atari repeatedly flirted with and abandoned — keeps the ST technically credible past Windows 3.0.

**Evidence.** TOS 1.0 shipped 20 November 1985; multitasking did not ship until MultiTOS in 1993, the year Atari left the computer business. Landon Dyer, who wrote the ST's BIOS boot code and floppy driver, records that CP/M-68K was rejected as not meeting the requirements of a modern OS and GEMDOS was written under extreme schedule pressure. The ST's commercial survival was longest in professional audio — a textbook niche capture driven by one OS-level property (MIDI timing).

Tags: no hardware channel, strategic mismanagement, technical shortfall, niche capture, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/Atari_TOS
- https://en.wikipedia.org/wiki/Atari_ST
- https://en.wikipedia.org/wiki/GEM_(desktop_environment)
- https://dfarq.homeip.net/atari-st-introduced-april-8-1985/

### Barrelfish
*Conceived 2006; development began October 2007; announced September 2009; final release 23 March 2020; project now inactive · ETH Zurich Systems Group (Timothy Roscoe, Andrew Baumann, Simon Peter) with Microsoft Research Cambridge (Paul Barham); later partly supported by HP Enterprise Labs, Huawei, Cisco, Oracle and VMware*

**What was brilliant.** The multikernel model: treat a single machine as a distributed system rather than as a shared-memory computer. Each core runs its own kernel instance with no shared kernel state whatsoever; all coordination between cores is explicit message passing; OS state is replicated per-core and kept consistent by agreement protocols instead of by locks riding on cache coherence. The payoff is that the OS keeps scaling on hardware where coherence does not — heterogeneous cores, deep NUMA, coherence islands, accelerators and smart NICs — and the 2009 SOSP paper showed cases where explicit message-passing updates beat shared-memory locking past a modest core count. Barrelfish also introduced the system knowledge base: hardware topology, cache hierarchies and interconnect costs expressed as logical facts and queried by a constraint solver, so the OS derives its own placement and routing policy for machines it has never seen rather than shipping hand-tuned constants.

**What happened.** Never intended as a product; Microsoft Research collaborated but did not productize it. Development continued at ETH with industrial funding through the 2010s, with the final release tagged release2020-03-23. barrelfish.org now states plainly: 'The Barrelfish project is no longer active.' The ideas were absorbed piecemeal rather than adopted wholesale — per-core data structures, RCU, NUMA-aware allocators, and the general acceptance in the Linux community that naive shared-memory kernel design does not scale. Roscoe's group moved on to Enzian and hardware/OS co-design.

**Why it failed.** Barrelfish bet that commodity hardware would stop being cache-coherent and that operating systems would therefore have to be rewritten as distributed systems; coherent shared memory kept scaling well enough, and Linux absorbed the specific remedies — per-core structures, RCU, NUMA allocators — without adopting the architecture, so the forcing function that would have required a rewrite never arrived.

**What might have changed it.** If Microsoft had adopted Barrelfish as the control-plane OS for a heterogeneous accelerator or SmartNIC platform — the role now filled by per-device runtimes and DPU operating systems — the multikernel would have had a hardware channel and a shipping product. Instead it stayed a paper artifact for thirteen years while the incumbent harvested its ideas one patch at a time.

**Evidence.** Motivation formed 2006 by Roscoe and Barham; implementation begun from scratch in October 2007 'so the researchers involved could explore the implications of multicore computing in detail without the need to worry about existing code in current computer operating systems'; announced September 2009 with a code release; final release 23 March 2020; project page now declares it inactive. Successive industrial sponsors — Microsoft Research, HPE Labs, Huawei, Cisco, Oracle, VMware — funded it for over a decade and none deployed it, which is the strongest available evidence that the blocker was the absence of a forcing hardware trend rather than the absence of money or interest.

Tags: incumbent lockin, no app ecosystem, funding collapse, no hardware channel

Sources:
- https://barrelfish.org/
- https://en.wikipedia.org/wiki/Barrelfish_(operating_system)
- https://www.microsoft.com/en-us/research/project/barrelfish/publications/
- https://www.microsoft.com/en-us/research/blog/barrelfish-exploring-multicore-os/
- https://barrelfish.org/documentation.html

### BeOS
*1995–2001 (Be Inc. founded 1990) · Be Inc. — Jean-Louis Gassée, Steve Sakoman, Dominic Giampaolo, Cyril Meurillon*

**What was brilliant.** Pervasive multithreading was architectural, not optional: every window ran on its own thread, the API was C++ shared libraries with message passing between threads, and SMP was designed in from the first line rather than retrofitted. BFS, written by Giampaolo and Meurillon in about ten months from September 1996, was a 64-bit journaling filesystem whose indexed extended attributes made the filesystem a live queryable database — the mail client and MP3 tagger were literally filesystem queries. The Media Kit was a node-graph for real-time audio/video with latency guarantees, so a 1998 dual-Pentium box could play several video streams while recording audio, which Windows and Mac OS could not do. Boot time was roughly 15 seconds.

**What happened.** BeBox hardware shipped October 1995 and was discontinued January 1997. Apple ran a bake-off and chose NeXT instead, announced 20 Dec 1996. The PowerPC Mac port lost its channel when Apple ended clone licensing in 1997. Ported to x86 for R3 (March 1998); R4 November 1998; R5 March 2000 with a free Personal Edition. IPO July 1999 on Nasdaq (BEOS). Pivoted to BeIA February 2000; assets sold to Palm for $11M, completed 13 Nov 2001. Be sued Microsoft in February 2002 and settled September 2003 for $23.25 million.

**Peak adoption.** Estimated 50,000–100,000 machines running BeOS in 1999; R5 Personal Edition reportedly over one million downloads. Be's net revenue was $2.7M in 1999 falling to $480,000 in 2000 (SEC 10-K).

**Why it failed.** Microsoft's OEM Windows license terms denied BeOS the only distribution channel that mattered for a desktop OS — factory preload — so no amount of technical superiority could put BeOS in front of a buyer who had not already sought it out.

**What might have changed it.** If the 1994 DOJ consent decree had reached boot-sequence and dual-boot restrictions rather than only per-processor pricing, the Hitachi, Compaq, Dell and Micron preloads Be had negotiated would have proceeded, and BeOS would have had a real retail channel in 1998–99 instead of a floppy-disk bootloader download.

**Evidence.** Scot Hacker's account: the confidential Windows OEM license specified a machine including Microsoft's OS 'must not also offer a non-Microsoft operating system as a boot option'; 'The hardware vendor does not get to choose which OSes to install on the machines they sell -- Microsoft does.' Be offered BeOS free to Dell, Compaq, Micron and Hitachi; only Hitachi shipped, and Microsoft sent two U.S. managers to Japan to express anger and 'remind' Hitachi of its license terms, after which the Flora Prius shipped with the BeOS partition hidden and users had to build boot floppies themselves. Gassée afterwards: 'I once preached peaceful coexistence with Windows. You may laugh at my expense -- I deserve it.' Microsoft paid $23.25M in September 2003 while admitting no wrongdoing. The popular story that 'Apple's choice of NeXT killed Be' is secondary: the Apple decision cost Be a buyer, but the OEM license terms cost it the market.

Tags: no hardware channel, incumbent lockin, licensing or legal, no app ecosystem, business model failure, acquired and killed

Sources:
- https://en.wikipedia.org/wiki/BeOS
- https://en.wikipedia.org/wiki/Be_Inc.
- https://birdhouse.org/beos/byte/30-bootloader/
- https://en.wikipedia.org/wiki/Be_File_System
- https://www.theregister.com/2002/02/20/be_inc_sues_microsoft/

### Cambridge CAP computer and operating system
*1970-late 1970s (project 1970-1977; operational from 1976) · University of Cambridge Computer Laboratory (Maurice Wilkes, Roger Needham, David Wheeler)*

**What was brilliant.** The first successful demonstration of capability-based protection enforced in both hardware and software. It had a microprogrammed 32-bit CPU with a 64-register capability unit and a hardware cache of evaluated capabilities, and an OS built entirely from protected procedures in a process tree under a 'Master Coordinator', so that system services needed no privileged supervisor mode. Its direct intellectual descendant is Cambridge's CHERI capability architecture, prototyped in silicon as Arm Morello.

**What happened.** A single experimental machine that served the Computer Laboratory. The design was documented in Wilkes and Needham's 1979 book and never commercialized. Its ideas came back decades later in CHERI/Morello (2010s-2020s).

**Peak adoption.** One machine built (research prototype); no commercial units.

**Why it failed.** CAP was built as a university research proof of concept with no commercial partner. In the late 1970s and 1980s the market went for cheap, unprotected microprocessors where hardware capabilities added cost with no buyer demand, so the design never reached a product.

**What might have changed it.** A UK manufacturer such as ICL could have adopted CAP-style capability hardware for a commercial secure-system line, the route Plessey took with the System 250.

**Evidence.** Wikipedia: 'The CAP project on memory protection ran from 1970 to 1977' and it became operational in 1976. Its features, a 'capability unit itself, which had 64 registers for holding evaluated capabilities' and a hardware capability cache, had no counterpart in commodity processors. CHERI's appearance in Cambridge's own lineage shows the ideas outlived the machine.

Tags: no hardware channel, business model failure, niche capture

Sources:
- https://en.wikipedia.org/wiki/CAP_computer
- https://www.cl.cam.ac.uk/events/50+5/assets/pdf/cap.pdf
- https://www.microsoft.com/en-us/research/publication/the-cambridge-cap-computer-and-its-operating-system/
- https://www.computerhistory.org/collections/catalog/102628040

### Danger hiptop OS (T-Mobile Sidekick)
*1999–2011 (Danger founded 9 Dec 1999; hiptop launched Oct 2002; Microsoft acquired Feb 2008; service shut down 2011) · Danger, Inc. — Andy Rubin, Joe Britt, Matt Hershenson (ex-Apple, WebTV, Philips)*

**What was brilliant.** The first genuinely cloud-native smartphone platform, shipping in 2002 — six years before the App Store. The handset was a thin client running a Java VM; all user state (contacts, calendar, mail, photos, IM history) lived on Danger's own operated service and synced continuously, so a lost or replaced device restored completely. The service also did server-side HTML transcoding and compression so that real web browsing worked over 2.5G GPRS, provided always-on push messaging, delivered over-the-air OS updates to the whole fleet, and ran a catalogue of signed, downloadable third-party applications billed through the carrier. Applications were suspended and resumed transparently so the user never saw a loading state. Hardware and software were co-designed around one swivel-hinge form factor with an exceptionally good physical keyboard.

**What happened.** Never escaped a single carrier's US youth-culture niche. Andy Rubin left in 2003 and founded Android, carrying the same architecture — Java app model, OTA updates, an app catalogue, cloud account sync — to what became a billion-device platform. Microsoft acquired Danger on 11 February 2008 for a reported ~$500 million and folded the team into 'Project Pink,' which shipped as the Microsoft Kin in May 2010 and was cancelled roughly 48 days later. On 2 October 2009 the Sidekick service failed and on 12 October Microsoft/Danger confirmed essentially all user data had been lost because there was no working backup — the most damaging consumer cloud-storage failure of the decade and an event that set back carrier confidence in server-side data models. The Danger service was shut down in 2011.

**Why it failed.** Danger's business model required each carrier to host, operate and revenue-share a Danger-run back-end service, which made every new carrier a bespoke multi-year integration rather than a product sale — so the platform could not scale past T-Mobile US, and Microsoft bought and dissolved the team before the model could be changed.

**What might have changed it.** If Danger had licensed the OS and a reference design to handset OEMs on terms that let carriers opt in or out of the back-end service, the same architecture would have scaled — which is exactly the change Andy Rubin made when he rebuilt it as Android two years later.

**Evidence.** The founding team (Rubin, Britt, Hershenson) and Rubin's 2003 departure to found Android are documented; the acquisition closed 11 Feb 2008 at a reported ~$500m; the Kin was pulled within about seven weeks of launch; the 12 Oct 2009 data loss occurred because Danger 'did not have an active backup' and recovery took over two months. The strongest evidence that the architecture was right and the distribution was wrong is that Android is, structurally, the hiptop model shipped through OEMs instead of through one carrier's service contract.

Tags: no hardware channel, business model failure, acquired and killed, incumbent lockin, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/Danger_(company)
- https://en.wikipedia.org/wiki/2009_Sidekick_data_loss
- https://en.wikipedia.org/wiki/T-Mobile_Sidekick
- https://appleinsider.com/articles/09/10/11/microsofts_danger_sidekick_data_loss_casts_dark_on_cloud_computing

### DR-DOS
*1988–2004 (3.31, 28 May 1988; 7.03, Jan 1999 last desktop; 8.0, 30 March 2004) · Digital Research, Inc.; then Novell (1991), Caldera (1996), Lineo, DeviceLogics*

**What was brilliant.** DR DOS 5.0 (May 1990) was the first DOS to relocate the kernel and device drivers into the High Memory Area and upper memory blocks (MemoryMAX), freeing an equivalent amount of the scarce first 640 KB — by far the most valuable thing a DOS could do in 1990 — and it beat MS-DOS 5.0 to market by over a year. It added a GEM-based graphical shell (ViewMAX), the patented BatteryMAX idle-detection power management for laptops, on-the-fly SuperStor disk compression, delete tracking and undelete in 6.0 (Sept 1991), and in Novell DOS 7 (Dec 1993) genuine preemptive multitasking of DOS applications in virtual machines plus DPMS. It also shipped password-level file security MS-DOS never had.

**What happened.** Novell acquired Digital Research in July 1991 and let the product drift; Caldera bought DR-DOS on 23 July 1996 and sued Microsoft the next day. Microsoft settled on 7 January 2000; the amount — $280 million — stayed sealed until November 2009. The codebase fragmented to Lineo and DeviceLogics for embedded use. DR-DOS 7.03 (January 1999) was the last real desktop release.

**Why it failed.** Microsoft used control of the adjacent Windows franchise to make the better DOS appear broken — beta Windows 3.1 carried encrypted, self-modifying code that produced a frightening spurious error on DR DOS — and then bound Windows 95 to MS-DOS, so DR-DOS's real technical lead could never be converted into OEM design wins.

**What might have changed it.** Filing suit in 1992, when Chappell and Schulman published the AARD analysis, rather than in 1996 after the market had already moved to Windows 95, could have produced an injunction while DOS still mattered. Alternatively, Novell shipping Novell DOS 7 with a credible Windows-compatible GUI in 1992 instead of a character-mode DOS in December 1993 gives DR-DOS a path through the GUI transition.

**Evidence.** Brad Silverberg on the purpose of the AARD message: 'What the [user] is supposed to do is feel uncomfortable, and when he has bugs, suspect that the problem is dr-dos and then go out to buy ms-dos.' Jim Allchin on Novell: 'If you're going to kill someone there isn't much reason to get all worked up about it and angry... We need to smile at Novell while we pull the trigger.' Geoff Chappell discovered the code on 17 April 1992; it shipped in the Windows 3.1 installer, WIN.COM and other binaries, XOR-encrypted and deliberately obfuscated. Caldera settled 7 January 2000 for $280M, unsealed November 2009. In discovery, Microsoft's German OEM account manager Stefanie Reichel testified she was pressured to delete 'questionable' emails.

Tags: incumbent lockin, licensing or legal, strategic mismanagement, acquired and killed, business model failure

Sources:
- https://en.wikipedia.org/wiki/DR-DOS
- https://en.wikipedia.org/wiki/AARD_code
- https://www.theregister.com/2000/02/01/unsealed_caldera_files_detail_ms/
- https://law.justia.com/cases/federal/district-courts/FSupp2/72/1295/2336233/
- https://www.geoffchappell.com/notes/windows/archive/aard/drdos/index.htm

### Google TV (original, 2010 platform)
*2010–2014 · Google, with Intel, Sony and Logitech*

**What was brilliant.** The first serious attempt to put a full, unrestricted desktop browser — Chrome with Flash 10.1 — on a television alongside live broadcast video. The box sat between the cable STB and the panel over HDMI passthrough and hardware-composited web content as picture-in-picture over live TV, while driving the set-top box by HDMI-CEC and IR blaster so a search result could physically tune the tuner. Its unified search indexed live listings, DVR contents and web video in one result set — the model every TV platform uses today, shipped in 2010. It was also the first production Android on x86: Android 2.1/3.x on Intel's CE4100 media SoC, which forced Android's first serious non-ARM port.

**What happened.** Announced 20 May 2010 at Google I/O; Logitech Revue ($299) and Sony Internet TV shipped October 2010. Within days of launch (21 October 2010) ABC, CBS, NBC and Hulu blocked Google TV by user agent. Logitech cut the Revue to $99 in July 2011, taking a $34M charge, and exited in November 2011. Google moved to ARM (Marvell Armada 1500) in 2012 and shipped Google TV 3.0 in January 2013 before replacing the entire platform with Android TV at I/O in June 2014.

**Peak adoption.** Approximately 1 million devices estimated in use by mid-2014, across all OEMs, before the SDK was withdrawn.

**Why it failed.** Google shipped a product whose central promise — the open web on your television — depended entirely on content the broadcasters controlled, and the broadcasters blocked it by user-agent string within a week of launch, leaving a $299 box whose remaining function was a slow browser bolted onto a cable box the user already owned.

**What might have changed it.** If Google had shipped Google TV as an application platform with the Android Market and a 10-foot SDK at launch in October 2010 — instead of a browser scraping broadcaster websites, with the SDK arriving only with the Honeycomb update a year later — it would have been Roku's direct competitor from 2010 rather than conceding four years and restarting as Android TV in 2014.

**Evidence.** ABC, CBS, NBC and Hulu blocked Google TV devices on 21 October 2010, days after launch; Dish was the only major distributor to co-operate. Logitech chairman Guerrino De Luca called the Revue 'a big mistake', said building inventory 'expecting everybody to line up for Christmas and buy them at $300' was the error, and that the programme cost Logitech 'well over $100 million in operating profit'; more Revues were returned by dealers than sold. The Revue's Atom CE4100 was taxed 'to the breaking point' in reviews. The Android Market only reached Google TV in the October 2011 Honeycomb update.

Tags: incumbent lockin, strategic mismanagement, no app ecosystem, technical shortfall

Sources:
- https://en.wikipedia.org/wiki/Google_TV_(operating_system)
- https://engadget.com/2010/10/21/television-networks-block-google-tv-from-accessing-web-based-con
- https://venturebeat.com/2011/11/11/no-new-logitech-revue-google-tv
- https://www.techradar.com/news/television/logitech-says-google-tv-revue-box-was-a-big-mistake-1040606
- https://www.theregister.com/on-prem/2011/07/29/google_tv_box_flop_costs_logitech_34m/

### JNode (Java New Operating System Design Effort)
*Registered 11 May 2003; last substantive release 0.2.8 on 3 February 2009; last project activity 11 February 2016 · Ewout Prangsma, with Levente Sántha and an open-source volunteer community (SourceForge, LGPL)*

**What was brilliant.** JNode reduced the native layer to a tiny assembler nanokernel and wrote everything above it in Java — the memory manager and garbage collector, the device drivers, the filesystems (ext2, FAT, NTFS, HFS+), the shell, the TCP/IP stack and a full AWT/Swing desktop — with an ahead-of-time compiler producing native code rather than interpreting. It ran the OpenJDK class library on itself, could compile Java on itself, and used JSR-121 isolates as its process abstraction: the same software-isolated-process idea Microsoft Research published in Singularity, implemented by a handful of volunteers at the same time.

**What happened.** 0.2.8 (3 February 2009) integrated OpenJDK 6 b13 and was the last real release; the isolate, Bjorne shell and HFS+ work was still in progress and never finished. Development slowed to nothing; the last SourceForge activity is 11 February 2016 and the project status is still listed as Alpha after more than a decade. A small 'JNode revisited' effort exists but is marginal.

**Why it failed.** JNode's proposition — Java all the way down — was invisible to users, who could already run every Java application on Linux or Windows with vastly better driver coverage, so the project had to fund a complete driver and class-library rewrite out of volunteer time in exchange for no user-visible benefit at all.

**What might have changed it.** Target one bounded appliance class — a hardened JVM appliance for servers, or a single well-supported board — instead of general-purpose PC hardware. A small team can finish a narrow target; it cannot finish 'all x86 machines,' which is why JNode stayed alpha for thirteen years.

**Evidence.** Project registered 2003-05-11; release 0.2.8 announced 3 February 2009 as the last major version, focused on 'general stability improvements and bug fixes' with isolates, Bjorne shell and HFS+ still in active development; last recorded project activity 2016-02-11; status 'Alpha.'

Tags: no hardware channel, no app ecosystem, compatibility gap, technical shortfall

Sources:
- https://sourceforge.net/projects/jnode/
- https://sourceforge.net/p/jnode/news/2009/02/jnode-028-released/
- https://jnode-revisited.github.io/

### KeyKOS (originally GNOSIS)
*1975-1991 (development began 1975 at Tymshare; production from January 1983 on an IBM 4341; Key Logic formed 1985; C rewrite begun 1988; complete 88K nanokernel October 1990; Key Logic ceased operations 1991) · Tymshare, Inc. -- Norman Hardy, Ann Hardy, Bill Frantz, Charlie Landau, Alan Bomberger, Peri Frantz, Jay Jonekait; after McDonnell Douglas bought Tymshare in 1984 the technology was spun out to Key Logic in 1985*

**What was brilliant.** A pure capability, orthogonally persistent nanokernel in roughly 20,000 lines of portable C plus under 2,000 lines of assembler, compiling to about 60KB and needing as little as 100KB of RAM to run. System-wide checkpoints every few minutes capture all state including process registers, so after a power failure the system simply resumes at the last checkpoint -- there is no filesystem check, no restart semantics, no partially-written-file corruption: 'The state at the last checkpoint is completely consistent and the system continues from that point.' Inter-domain message transfer cost 90 instructions on System/370 and under 500 cycles on the MC88000, several times cheaper than Mach or Chorus, which is what made it economical to decompose a system into many tiny least-privilege domains. KeyNIX, a 99%-compatible BSD 4.3 Unix in about 16,000 lines, was built by one developer in six months without reference to Unix source, running every Unix process as a separate KeyKOS domain. The confused-deputy problem was named and explained by Norm Hardy out of this work.

**What happened.** Key Logic ceased operations in 1991 and KeyKOS's last release was 1988 on S/370 with the 88K nanokernel finished in late 1990. The intellectual property became sufficiently encumbered that when Jonathan Shapiro started EROS in 1991 he built a clean-room reimplementation rather than licensing KeyKOS from a defunct company.

**Why it failed.** KeyKOS was a mainframe operating system owned by a tiny spin-out precisely during the years the mainframe market collapsed, so its only distribution channel disappeared before the microprocessor port could establish a new one.

**What might have changed it.** If the 1988-1990 rewrite had targeted the Intel 386 rather than the Motorola 88000, KeyKOS would have landed on the architecture that was about to become universal instead of one Motorola abandoned for PowerPC within a year of the port completing. The technology was ready; the host was not.

**Evidence.** The nanokernel paper is explicit about what the design bought: 'KeyKOS is the only commercially available operating system that meets these requirements' -- accounting accuracy, 24-hour uninterrupted service, and 'simultaneous, mutually suspicious time sharing customers with an unprecedented level of security.' On performance: 'On the Motorola 88x00 series, a typical message send takes less than 500 cycles ... task switch costs are very much lower than in traditional systems and several times lower than in competing microkernels such as MACH and Chorus.' The KeyNIX authors note the reaction of the mainstream: 'Reactions to the KeyNIX design from UNIX developers range from shocked to appalled at the profligate use of processes' -- a precise illustration of why a superior model with an unfamiliar cost structure gets rejected. Wikipedia and the EROS history confirm Key Logic's closure in 1991 and that this closure is what forced EROS to be a clean-room rebuild.

Tags: no hardware channel, funding collapse, hardware tied to dying platform, no app ecosystem, compatibility gap

Sources:
- https://css.csail.mit.edu/6.5660/2017/readings/keykos.pdf
- https://pdos.csail.mit.edu/6.828/2009/readings/keykos.pdf
- https://en.wikipedia.org/wiki/KeyKOS
- https://en.wikipedia.org/wiki/GNOSIS
- http://web.cs.wpi.edu/~cs557/f14/papers/confused_deputy-hardy.pdf

### Linux netbooks / Ubuntu Netbook Remix
*2007-2010 (UNR 2008-2010, folded into Unity) · OEM Linux preloads (Asus Eee PC with Xandros, Linpus, others) and Canonical (Ubuntu Netbook Remix)*

**What was brilliant.** The first time desktop Linux won a new mainstream PC category on OEM preload. Low-spec SSD netbooks with fast-boot, simplified launcher shells created the category. UNR's full-screen launcher and maximised-window design for 1024x600 screens directly led to Unity.

**What happened.** Linux went from about 90% of netbooks in early 2008 to roughly 4% by February 2009. OEMs switched to Windows XP, which Microsoft kept selling for low-cost PCs well after Vista launched. UNR was renamed Ubuntu Netbook Edition and then merged into the standard Unity desktop in Ubuntu 11.04 (2011). The netbook category itself later gave way to tablets.

**Peak adoption.** About 90% of netbooks in early 2008; Windows was on 10% of netbooks in H1 2008 (NPD, via The Register)

**Why it failed.** Microsoft extended Windows XP and priced it cheaply for low-cost PCs. That removed the price gap that was Linux's only structural advantage, and buyers who expected Windows software returned Linux units at higher rates. OEMs switched within about a year.

**What might have changed it.** A single, polished, Windows-familiar Linux shell (UNR-quality rather than each OEM's rushed Linpus or Xandros build) shipping on the first 2007-2008 netbooks, with Canonical-backed support, before XP's extension closed the price gap.

**Evidence.** NPD: Windows installation rate on netbooks went from 10% in H1 2008 to 96% in February 2009 (The Register, 6 April 2009). MSI's US sales director said in 2008 that Linux netbooks were returned at least four times as often as XP units, and Canonical confirmed Linux returns were higher than expected. The popular story that Linux simply failed on usability is contested: Asus said Eee PC return rates were similar for Linux and Windows. Commentators at the time put the switch down at least as much to Microsoft extending XP and discounting licences as to consumer demand.

Tags: incumbent lockin, compatibility gap, no app ecosystem, too late to market, strategic mismanagement

Sources:
- https://www.theregister.com/2009/04/06/windows_crushes_linux_on_netbooks
- https://linux.slashdot.org/story/08/10/05/123253/netbook-return-rates-much-higher-for-linux-than-windows
- https://www.laptopmag.com/articles/ubuntu-confirms-linux-netbook-returns-higher-than-anticpated
- https://www.osnews.com/story/20568/eeepc-return-rate-is-similar-for-windows-and-linux/
- https://ubuntu.com/blog/microsoft-fud-and-the-netbook-market

### LOCUS
*1980-1995 (UCLA research 1980-1983; Locus Computing Corp 1982-1995) · UCLA (Gerald Popek, Bruce Walker, Robert English, Charles Kline, Greg Thiel); Locus Computing Corporation*

**What was brilliant.** True single-system image across a network while staying upward-compatible with Unix. It had a network-transparent file system with automatic replicated storage and partition/merge recovery, transparent remote and migrating process execution, and nested transactions. By 1983 it ran 17 VAX-11/750s on Ethernet as one Unix. Locus's VPROC abstraction later underpinned OSF/1 AD, UnixWare NonStop Clusters and OpenSSI.

**What happened.** Locus Computing built it into IBM AIX PS/2 and AIX/370 as TCF (Transparent Computing Facility), which could migrate running processes between PS/2s or mainframes. IBM's mainstream AIX 3 for RS/6000 did not carry TCF forward, and AIX/370 ended up as an NFS server for RS/6000 workstations. Locus then did OSF/1 AD for the Intel Paragon and NonStop Clusters for SCO/Tandem. The company was acquired by Platinum Technology on 17 August 1995.

**Why it failed.** LOCUS never had its own platform. It reached users only as a contracted feature inside other vendors' Unixes (IBM AIX 1.x, Intel OSF/1 AD, SCO UnixWare), and when those vendors moved to simpler stateless networking such as NFS or to other products, the single-system-image layer was dropped with nothing to replace it.

**What might have changed it.** IBM could have kept TCF in AIX 3 on RS/6000, its high-volume Unix line. Transparent clustering would then have become a standard feature of a major commercial Unix in the early 1990s, instead of a niche HPC and cluster add-on.

**Evidence.** Wikipedia/Locus Computing: 'Locus was commissioned by IBM to produce a version of the AIX UNIX based operating system for the PS/2 and System/370 ranges, and the single-system image capabilities of LOCUS were incorporated under the name of AIX TCF.' AIX 1.x TCF allowed moving 'a running process between two PS/2s or two mainframes without stopping the process', yet AIX 3 marked 'a significant shift' away from it, and AIX/370 'was used as a NFS file server for AIX (RS/6000) workstations'. Later products (OSF/1 AD, NonStop Clusters) stayed in HPC and cluster niches.

Tags: no hardware channel, business model failure, technical shortfall, niche capture

Sources:
- https://en.wikipedia.org/wiki/LOCUS
- https://en.wikipedia.org/wiki/Locus_Computing_Corporation
- https://www.cs.princeton.edu/courses/archive/fall03/cs518/papers/locus.pdf
- https://www.osnews.com/story/29672/ibm-aix-for-ps2/
- https://gunkies.org/wiki/AIX

### Multics
*1965-2000 (design 1965; MIT service Oct 1969; Honeywell commercial product Jan 1973; development cancelled Jul 1985; last site shut down 30 Oct 2000) · MIT Project MAC, Bell Telephone Laboratories and General Electric; commercialised by Honeywell after it bought GE's computer division in 1970, later Honeywell Bull*

**What was brilliant.** A segmented single-level store in which every file is a memory-mapped segment addressed directly by the CPU, with no read/write calls; eight hardware protection rings instead of the two-state user/supervisor split; a per-segment access control list; dynamic linking resolved at first symbol reference; and hot-swap of CPUs, memory and disks for genuinely uninterrupted operation. Written in PL/I, one of the first operating systems built in a high-level language. It was the first system to earn a TCSEC Class B2 rating (1 Sept 1985) and held the only B2 for a general-purpose OS for years; the NCSC penetration team generated roughly 100 flaw hypotheses, explored 84 and confirmed 70 flaws, of which only five were critical -- an extraordinary result for a 20-year-old codebase.

**What happened.** Honeywell decided internally in November 1984 to stop development and formally cancelled Multics in July 1985, a month before the B2 rating was awarded. Bull closed the Cambridge Information Systems Laboratory in June 1986, the same month the B2 Final Evaluation Report was published. Customers kept running it for another 14 years; the last machine, at the Canadian Department of National Defence in Halifax, was shut down on 30 October 2000. Source released 2007; emulation revived 2017.

**Peak adoption.** About 77 sites worldwide at peak (multicians.org); roughly 25 installations by the end of the 1970s, with European growth in the 1980s -- Honeywell Bull sold 31 French sites alone. Each sale was multi-million dollar: a two-CPU 6180 comparable to MIT's cost about $7M in 1973. Multicians estimate Honeywell took several hundred million dollars of revenue from the line.

**Why it failed.** Multics was welded to a mainframe architecture sold by a succession of companies that could not compete in mainframes, so its fate was decided by its hardware vendor's market position and executive politics rather than by anything about the operating system.

**What might have changed it.** If Honeywell had funded a port off the GE-645/6180 architecture onto commodity hardware in the late 1970s -- the ring protection Multics depends on existed on the Intel 386 by 1985 -- Multics would have outlived its host hardware instead of dying with it.

**Evidence.** The popular story that Multics failed because it was bloated and Unix was the lean corrective is wrong. Multics shipped, ran in production for 31 years, and multicians.org records that at cancellation it was 'the only profitable product in the Office Marketing Systems Division.' The Multicians' own account is blunt: 'the cancellation of Multics had nothing to do with its technical abilities or its sales: it simply had no champion in the executive suite.' On the security rating, the B2 page notes Honeywell 'waited until evaluation had just been completed' before cancelling -- the highest security rating then available bought the product no market at all. Lipner's history confirms that by 1990 'only Multics and SCOMP had completed evaluations as general-purpose operating systems at class B2 or above.'

Tags: no hardware channel, internal politics, strategic mismanagement, niche capture, business model failure, hardware tied to dying platform

Sources:
- https://multicians.org/myths.html
- https://multicians.org/history.html
- https://multicians.org/b2.html
- https://www.stevelipner.org/links/resources/The%20Birth%20and%20Death%20of%20the%20Orange%20Book.pdf
- https://en.wikipedia.org/wiki/Multics

### NeXTSTEP / OPENSTEP after the hardware exit
*1993–1997 (software-only era; NeXTSTEP from 1989) · NeXT Software, Inc. (formerly NeXT Computer)*

**What was brilliant.** NeXTSTEP delivered in one box what nobody else assembled until the 2000s: Objective-C with a fully dynamic runtime, the Foundation and Application Kit object frameworks, Interface Builder editing a live object graph and archiving it to nib files, Display PostScript giving one imaging model for screen and print, a Mach plus BSD kernel with protected memory, and NetInfo and DriverKit for administration and drivers. The productivity gain was real and measured in orders of magnitude on real projects — Tim Berners-Lee wrote WorldWideWeb on it in months, and id wrote Doom and Quake on it. The frameworks were good enough that Apple bought the company for them and is still shipping them thirty-five years later.

**What happened.** Hardware manufacturing ended 9 February 1993 — 'Black Tuesday,' 330 of 500 employees cut, about 50,000 machines sold in total. NeXTSTEP 3.1 for Intel shipped May 1993, with SPARC and PA-RISC ports following. Sun invested $10M and the OpenStep API specification was published 19 October 1994. OPENSTEP 4.0 shipped July 1996, 4.2 January 1997; OPENSTEP Enterprise ran the frameworks on Windows NT 4.0. NeXT's actual revenue shifted to WebObjects. Apple announced the acquisition on 20 December 1996 for about $400M plus stock, closing February 1997.

**Peak adoption.** About 50,000 NeXT computers sold through February 1993. Intel/SPARC OPENSTEP install numbers undocumented; deployment was concentrated in Wall Street trading floors, telecoms, government and the US Navy.

**Why it failed.** Once NeXT stopped making machines it was selling a several-thousand-dollar OS-plus-frameworks onto commodity PCs whose owners already had an operating system bundled free with the hardware, so it could only close deals where bespoke in-house software justified the price — which is a consulting market, not a platform market, and consulting markets do not produce third-party applications.

**What might have changed it.** If Sun had actually shipped OpenStep as the application layer of Solaris after the 1993–94 agreement, NeXT gets a volume Unix installed base, a second independent implementation, and a shrink-wrap software market; instead Sun's next-generation platform bet moved to Java in 1995 and OpenStep on Solaris never shipped in volume.

**Evidence.** 9 February 1993: 330 of 500 staff cut, roughly 50,000 systems sold across the company's entire hardware life — a five-year hardware business smaller than a single quarter of Compaq's. Sun's $10M investment and the 19 October 1994 OpenStep specification were described as 'the first unadulterated piece of good news in the NeXT community in the last four years,' which is itself a statement about the state of the business. OPENSTEP Enterprise on Windows NT demonstrated the frameworks were portable but there was still no channel to sell them through. That Apple paid $400M for the technology, and that Foundation and AppKit remain the foundation of macOS and iOS, is the retrospective proof the system was excellent and that excellence was never the constraint.

Tags: no hardware channel, business model failure, no app ecosystem, niche capture, incumbent lockin

Sources:
- https://en.wikipedia.org/wiki/NeXTSTEP
- https://en.wikipedia.org/wiki/OpenStep
- https://www.cultofmac.com/apple-history/next-inc-quits-making-computers
- https://lowendmac.com/2014/full-circle-a-brief-history-of-next/
- https://en.wikipedia.org/wiki/NeXT_Computer

### OpenBSD as a desktop operating system
*1995-present (forked from NetBSD October 1995; OpenBSD 2.0 released October 1996; two releases per year sustained for thirty years) · Theo de Raadt and the OpenBSD project*

**What was brilliant.** OpenBSD originated a remarkable share of the exploit mitigations the rest of the industry later adopted: W^X (2003), address space randomisation, ProPolice/stack-smashing protection on by default, and privilege separation, introduced in OpenSSH 3.2 in 2002 and now a standard structural technique. Its pledge(2) and unveil(2) are the most developer-usable confinement primitives ever shipped -- an application author restricts their own program to a set of syscall classes and filesystem paths in two or three lines, with no policy file, no daemon and no administrator involvement, which is exactly what SELinux and AppArmor never achieved. It ships secure by default with nothing listening on the network, audits its entire source tree, and produced OpenSSH, LibreSSL and OpenBGPD, making it one of the most consequential codebases in infrastructure.

**What happened.** Not dead and not failing on its own terms -- but permanently excluded from the desktop. It remains a firewall, router, server, network appliance and developer workstation. Desktop use stays marginal because the project cannot staff GPU drivers, modern desktop stacks or browser ports, and because upstream desktop software is written against Linux-specific interfaces the project will not import. This is a deliberate trade the project would make again.

**Peak adoption.** W3Techs (16 September 2026): 'OpenBSD is used by less than 0.1% of all the websites whose operating system we know.' Desktop share is not separately measured by any credible source and is far below that. Treat any specific desktop percentage as undocumented.

**Why it failed.** Desktop viability is determined by GPU and peripheral drivers and by a modern browser, all of which are developed against Linux interfaces; a project of a few dozen volunteers cannot follow that treadmill, so OpenBSD's security work lands where hardware diversity is low -- network appliances and servers -- and not on laptops.

**What might have changed it.** If OpenBSD had accepted a Linux kernel-interface compatibility shim for graphics, as FreeBSD did with LinuxKPI for drm, desktop hardware support would track Linux instead of trailing it by years. The project rejects this because an unaudited compatibility layer inside the kernel contradicts its entire reason for existing -- which makes this a coherent choice rather than an oversight, and is why OpenBSD's exclusion from the desktop is structural rather than fixable.

**Evidence.** LWN's 'Crowding out OpenBSD' captures the mechanism in the developers' own words. Marc Espie: 'if you don't have tens of people, it becomes more and more of a losing battle,' and 'either you have the linux goodies, or you don't. And if you don't, you can forget anything modern.' Antoine Jacoutot, who maintains GNOME on OpenBSD, notes that essentially all upstream desktop work is done by Linux developers, leaving BSDs as 'just another user, able to make requests but with no ability to create the changes they would like to see.' The asymmetry is the point: OpenBSD invented mitigations that Linux, Windows and macOS all copied, and still cannot ship a laptop, because security innovation and driver headcount are independent resources.

Tags: no hardware channel, niche capture, no app ecosystem, compatibility gap, funding collapse

Sources:
- https://lwn.net/Articles/524606/
- https://en.wikipedia.org/wiki/OpenBSD
- https://w3techs.com/technologies/details/os-openbsd
- https://www.linux.com/news/theo-de-raadt-gives-it-all-openbsd/

### OS/2 (2.0 through Warp 4)
*1992–2006 (2.0 April 1992; support ended 31 Dec 2006) · IBM (after the 1990 split with Microsoft)*

**What was brilliant.** The first genuinely 32-bit, preemptively multitasked, memory-protected operating system for commodity PCs, and it did the hardest compatibility job anyone has attempted: multiple concurrent DOS virtual machines with virtual device drivers, plus a complete licensed Windows 3.1 personality (Win-OS/2) that ran Windows applications more reliably than Windows did, each in its own protected session. The Workplace Shell was a real object-oriented desktop built on SOM — a language-neutral object model with release-to-release binary compatibility, so a subclass compiled against one version kept working when the parent class changed, something COM never solved. Installable file systems, HPFS, SMP in Warp Server, and REXX as a first-class system scripting layer rounded it out.

**What happened.** OS/2 2.0 April 1992; Warp 3 October 1994; Warp 4 'Merlin' 25 September 1996, the last major release. IBM laid off the overwhelming majority of the OS/2 organisation after Windows 95, kept OS/2 alive purely for committed banking, insurance and retail customers, shipped Convenience Pack 2 in 2002, and ended support 31 December 2006. It survives as a licensed niche product (eComStation, then ArcaOS) for ATMs and industrial control.

**Peak adoption.** 1996: 2.5 million desktop licences at about $175 and 340,000 server licences at $350, roughly $550M revenue against $150M R&D (Forbes, 19 July 1997). Desktop share 3.6% at end-1993, briefly above 5% during the 1994–95 Warp push, 3.3% in 1996.

**Why it failed.** OS/2 lost because it could not be bought pre-installed — Microsoft's per-processor OEM licensing meant an OEM paid for Windows on every machine whether or not it shipped Windows, making a second OS a pure added cost — and IBM then destroyed its own application market by making Windows 3.1 compatibility so good that no ISV needed to write a native OS/2 version.

**What might have changed it.** If IBM had shipped OS/2 2.0 in 1992 with Win-OS/2 unbundled or priced separately, and preloaded OS/2 by default on every IBM-branded PC, native OS/2 applications would have had an economic reason to exist and IBM's own hardware volume would have supplied the installed base Microsoft was denying it through the channel.

**Evidence.** Forbes 1997: $550M revenue on $150M R&D, 3.3% desktop share. The OS/2 Museum's assessment: Warp 'offered excellent compatibility with DOS and Windows 3.1 applications' and this same feature 'discouraged vendors from developing native OS/2 applications, since their DOS/Windows versions already reached OS/2 users.' At the 1999 DOJ trial, IBM's Garry Norris testified that Microsoft tied IBM's Windows licence terms to IBM's promotion of OS/2 and that IBM did not receive its Windows 95 licence until minutes before the launch. IBM's own PC division declined to preload OS/2 consistently — the decisive failure was internal and channel-side, not technical. Per-processor licensing was ended by the 1994 DOJ consent decree, three years after the damage was done.

Tags: incumbent lockin, no app ecosystem, compatibility gap, strategic mismanagement, niche capture

Sources:
- https://en.wikipedia.org/wiki/OS/2
- https://www.forbes.com/1997/07/19/imbos.html
- https://www.os2museum.com/wp/os2-history/os2-warp/
- https://www.os2museum.com/wp/os2-history/os2-timeline/
- https://www.osnews.com/story/26780/what-ever-happened-to-os2/

### Qubes OS
*2010-present (announced April 2010; 1.0 released 3 September 2012; 4.0 in 2018; 4.2 in December 2023; 4.3.1 in June 2026) · Joanna Rutkowska and Rafał Wojtczuk, Invisible Things Lab; project lead since October 2018 Marek Marczykowski-Górecki*

**What was brilliant.** The only security-by-isolation desktop that a meaningful number of people actually use every day. Xen-based with a dom0 that has no network access at all; an unprivileged NetVM that owns the NIC and a separate firewall VM behind it; template VMs so dozens of qubes share one root filesystem and are patched once; disposable VMs destroyed after opening a single untrusted document; a GUI virtualization protocol that composites windows from different VMs onto one desktop with coloured borders indicating trust domain; and qrexec, an inter-VM RPC where every cross-domain action -- clipboard, file copy, USB device attach -- requires an explicit policy rule. The same group's Xen attack research and the Anti Evil Maid boot-integrity work hardened the substrate they depend on.

**What happened.** Alive, actively developed, widely respected, and permanently niche. Rutkowska stepped down as project lead in October 2018 to work on Golem, remaining an advisor. The project runs on grants (Open Technology Fund and similar) and donations rather than product revenue, and reaches hardware through a certification programme with boutique vendors -- nine certified models as of late 2024, from NovaCustom, Nitrokey, Star Labs, Insurgo and Purism, none of them a volume OEM.

**Peak adoption.** From the project's own counter (tools.qubes-os.org/counter/stats.json), August 2026: 56,947 unique clearnet IPv4 addresses contacting the update servers in the month, plus an estimated 14,881 Tor users -- roughly 70,000 machines. Growth is real but linear: about 8,600 clearnet addresses in January 2016 to about 57,000 a decade later. Certified hardware: nine models as of September 2024.

**Why it failed.** Qubes makes the user pay the isolation cost in hardware and in attention -- a working IOMMU and VT-d, large amounts of RAM, no GPU acceleration inside qubes, poor battery life, and a conscious cross-domain decision for every routine action -- so it can only be adopted by people whose threat model justifies that tax, which is a small and largely professional population.

**What might have changed it.** If a volume laptop OEM had shipped Qubes preinstalled and certified on a mainstream business line -- the path HP took with Bromium's microvisor -- the hardware compatibility problem that is the single most cited barrier would simply have disappeared for the buyers who most need the product. Qubes' security architecture is not the limiting factor; its distribution is.

**Evidence.** The counter data is the most honest adoption figure any system in this category publishes, and it shows the ceiling clearly: fourteen years of development, endorsements from Edward Snowden and Daniel J. Bernstein, an Access Innovation Prize finalist slot, and roughly 70,000 machines. The hardware dependency is documented in the project's own requirements: 64-bit Intel or AMD with virtualization extensions, IOMMU, 6GB RAM minimum with far more needed in practice, and an explicit note that AMD client platforms are not recommended because of inconsistent security support. The certified-hardware programme exists precisely because ordinary laptops cannot be relied on to work -- nine models certified in fourteen years is the measure of how narrow the compatible hardware base is.

Tags: no hardware channel, technical shortfall, niche capture, poor developer experience, business model failure

Sources:
- https://tools.qubes-os.org/counter/stats.json
- https://doc.qubes-os.org/en/latest/introduction/statistics.html
- https://www.qubes-os.org/news/2016/01/14/qubes-counter/
- https://en.wikipedia.org/wiki/Qubes_OS
- https://doc.qubes-os.org/en/latest/user/hardware/system-requirements.html

### Redox OS
*20 April 2015 – present; release 0.9.0 on 9 September 2024; active through 2026 · Jeremy Soller (principal engineer at System76) and community; MIT licensed*

**What was brilliant.** Redox is the most complete memory-safe general-purpose operating system ever built outside a corporation, and it is unusual in being safe all the way up, not just in the kernel: a Rust microkernel, plus a Rust userspace including RedoxFS (copy-on-write, ZFS-inspired, per-file checksums and encryption), relibc (a C standard library implemented in Rust, which is what makes porting ordinary POSIX software possible at all), the Orbital display server and Ion shell, with a URL-scheme-based namespace instead of a single file hierarchy. It targets x86, x86-64, AArch64 and RISC-V, has ported hundreds of Unix packages plus GTK 3 and XFCE, and its capability-based security work is funded by the EU's NGI Zero programme.

**What happened.** Not dead — this is the live test of whether the language advantage pays off — but eleven years in it remains pre-stable and has not reached ordinary users. Tanenbaum: it 'has real potential, but it is not there yet, but is worth watching.' The binding constraint is money and driver labour, not language: as of June 2026 the project reports operating costs above $3,000 per month against under $1,000 per month of revenue, subsidized from past donations, plus a €50,000 NGI Zero Commons grant and €11,500 for capability-based security. Testing still requires downloading special images or building from source.

**Why it failed.** Rust eliminates a class of bugs but not the two things that actually gate adoption of an operating system — drivers for hardware you do not control, and the applications people already run — and Redox has to fund both out of donations running at roughly a third of its costs, so the safety advantage never becomes a user-visible reason to switch.

**What might have changed it.** Have a hardware vendor commit to shipping Redox on one specific machine. System76 is the obvious candidate and Soller is its principal engineer; a single supported laptop collapses the driver surface from 'all PCs' to one board, and supplies both a distribution channel and a revenue base — the exact thing every managed-language OS in this list lacked.

**Evidence.** First published on GitHub 20 April 2015; 97 contributors as of September 2024; release 0.9.0 on 9 September 2024; pre-stable status. June 2026 project report: costs exceed '$3,000 per month' while revenue is 'less than $1,000 monthly,' offset by a €50,000 NGI Zero Commons grant plus €11,500 for capability-based security; GTK 3 ported but 'only supports demos currently.' Tanenbaum: 'has real potential, but it is not there yet, but is worth watching.'

Tags: no hardware channel, no app ecosystem, funding collapse, compatibility gap, incumbent lockin

Sources:
- https://en.wikipedia.org/wiki/Redox_(operating_system)
- https://www.redox-os.org/news/this-month-260630/
- https://www.redox-os.org/news/this-month-250630/
- https://changelog.com/podcast/280

### Sprite
*Autumn 1984 – 1994 (decision to wind down taken at the end of 1991) · John Ousterhout's group at UC Berkeley, with Fred Douglis, Michael Nelson, Brent Welch, Mendel Rosenblum, Mary Baker, Ken Shirriff and John Hartman*

**What was brilliant.** Three genuine firsts in one 200,000-line kernel. First, cooperative client-and-server main-memory file caching with hard consistency guarantees, which made Sprite's network file system faster than a local disk and, in Ousterhout's words, 'the fastest in the world until well into the 1990s'. Second, transparent process migration: a running process could be picked up mid-execution and moved to an idle workstation with all its kernel state following it, while keeping its original machine's identity for the file system and signals — pmake exploited this to get 'speedups of four or more on common system tasks such as recompilation'. Third, the log-structured file system (Rosenblum and Ousterhout, 1991), which wrote small files 'an order of magnitude faster than any other existing file system' by turning all writes into a sequential log. LFS is the direct ancestor of every flash translation layer and copy-on-write filesystem now in use.

**What happened.** 'At the end of 1991 we decided to bring the Sprite project to a gradual close.' Development stopped and student recruiting ceased; the system limped on a few machines into 1994. The ideas dispersed and won elsewhere: Rosenblum went on to co-found VMware, LFS reached NetApp's WAFL and flash controllers, and process migration reappeared and became ubiquitous as virtual-machine live migration.

**Peak adoption.** Peak user community 'around 80 in 1990 and 1991', with 25–35 simultaneous logins typical, of whom only about half a dozen were developers. Ran at 'about ten different sites' outside Berkeley. Over 200,000 lines of new code.

**Why it failed.** Sprite built an entire kernel from scratch, so every feature its own users saw in commercial UNIX had to be reimplemented by graduate students; the maintenance tax compounded faster than the research yield, and the group chose to stop rather than become an unpaid UNIX vendor.

**What might have changed it.** Ousterhout's own: 'Perhaps it would have been better if we had built Sprite as an extension to an existing operating system, rather than building a new operating system from scratch.' A Sprite shipped as a set of SunOS/BSD kernel modules — caching, migration, LFS — would have had a technology-transfer path into the systems its users already ran; as a whole OS it had none.

**Evidence.** Ousterhout's retrospective is explicit on every point. On the competitive squeeze: 'By 1990 there were several commercial versions of UNIX with massive support teams, such as System V, Solaris, and OSF. These systems were adding features at a rapid pace and our users wanted access to these features under Sprite.' On maintainability: 'Like most software, the Sprite kernel became harder and harder to maintain as it aged.' On the outcome: 'My biggest disappointment about the Sprite project is that we weren't able to transfer the Sprite technologies into mainstream usage.' And on the partial consolation: 'Some of the Sprite ideas are gradually finding their way into wider usage, such as process migration (popularized in the form of virtual machine migration) and LFS (used in commercial products such as NetApp and in control systems for flash memory).'

Tags: incumbent lockin, no app ecosystem, no hardware channel

Sources:
- https://web.stanford.edu/~ouster/cgi-bin/spriteRetrospective.php
- https://ftp.eecs.berkeley.edu/sprite/retrospective.html
- https://www2.eecs.berkeley.edu/Research/Projects/CS/sprite/sprite.html
- https://www2.eecs.berkeley.edu/Research/Projects/CS/sprite/sprite.papers.html

### Ubuntu Touch
*2013–2017 at Canonical (announced 2 Jan 2013; BQ Aquaris E4.5 Feb 2015; cancelled 5 April 2017); continued by the UBports Foundation to the present · Canonical Ltd (Mark Shuttleworth)*

**What was brilliant.** Ubuntu Touch is the only mobile OS that shipped true convergence rather than a separate desktop mode: plug the phone into a monitor and keyboard and the same running processes rendered a full desktop, because it was one Ubuntu userspace with one display server (Mir) and one shell (Unity 8) that reflowed by form factor — the same binaries, not a second session and not a remote view. It also solved application packaging ahead of the industry: 'click' packages were self-contained, installed without root and without a shared dependency tree, and were confined per-app by AppArmor with a declarative policy — essentially the model that Snap, Flatpak and Android's scoped permissions all converged on afterwards. 'Scopes' replaced the app grid with aggregated, queryable content surfaces, a genuinely different answer to what a phone home screen is for than anyone else attempted.

**What happened.** Canonical raised $12.8 million from more than 27,000 backers on Indiegogo in August 2013 for the Ubuntu Edge handset against a $32 million goal — at the time the largest crowdfunding campaign ever run — and refunded all of it. The OS reached retail through two OEMs, BQ (Aquaris E4.5, February 2015) and Meizu (MX4 Ubuntu Edition, June 2015), sold only through invite-only European flash sales that sold out within hours but in quantities Canonical never disclosed. On 5 April 2017 Shuttleworth ended the project, writing that he had believed convergence delivered as free software 'would be widely appreciated both in the free software community and in the technology industry, where there is substantial frustration with the existing, closed, alternatives available to manufacturers. I was wrong on both counts.' The UBports Foundation took over stewardship the same month and still ships OTA releases for the PinePhone, Librem 5, Fairphone and others.

**Why it failed.** Canonical attempted to enter the phone market between 2013 and 2015 with no carrier relationships, no volume OEM and a convergence pitch that required docks and desktop-class SoCs which did not yet exist at phone prices — so the one feature that differentiated the product could not actually be experienced by the people buying it.

**What might have changed it.** If Canonical had waited for USB-C DisplayPort alt-mode and 2018-class mobile SoCs — the hardware conditions Samsung DeX shipped into — convergence would have been demonstrable on hardware people already owned and on a dock costing $40 rather than a bespoke handset. The technology was right and the calendar was wrong by roughly four years.

**Evidence.** Shuttleworth's own postmortem is the primary source and is explicit about the misjudgement: 'I was wrong on both counts.' The Ubuntu Edge result quantifies the ceiling — a record-setting $12.8m from 27,000 backers was still only 40% of the target, and the phone was never built. Distribution was the binding constraint: two OEMs, invite-only flash sales, no carrier ever ranged the device. The technical claims are corroborated by convergence outliving the project — UBports continues to ship it, and click/AppArmor confinement anticipated Snap and Flatpak.

Tags: no hardware channel, no app ecosystem, funding collapse, too late to market, business model failure, strategic mismanagement

Sources:
- https://en.wikipedia.org/wiki/Ubuntu_Touch
- https://www.bit-tech.net/news/tech/software/canonical-abandons-unity/1/
- https://techcrunch.com/2013/08/22/edge-crowdfunding-fail/
- https://venturebeat.com/mobile/ubuntu-edge-crowdfunding-campaign-falls-short-by-19m-founder-keeps-dreaming
- https://en.wikipedia.org/wiki/BQ_Aquaris_E4.5

## Tied to a roadmap or funder it did not control (16)

### Apple SOS (Sophisticated Operating System)
*1980–1984 (Apple III announced 19 May 1980, shipped Nov 1980; withdrawn 24 April 1984, delisted Sept 1985) · Apple Computer — Apple III team under Wendell Sander*

**What was brilliant.** SOS split the system into three separately loaded pieces: SOS.KERNEL (the documented API), SOS.INTERP (the single interpreter/application environment), and SOS.DRIVER (a device-driver file generated by a configuration utility) — so adding a hard disk or printer meant regenerating a driver file, not patching the OS. On a 1980 8-bit personal computer it provided a hierarchical file system with named volumes and subdirectories, a clean character-device/block-device split (single-byte versus 512-byte-block drivers), a segment-based memory manager handling the 6502B's bank-switched 128–256 KB, and a coherent four-area API covering file, device, memory and utility calls. Apple II users would not get this file system until ProDOS in 1983 — and ProDOS adopted SOS's on-disk format verbatim, so SOS and ProDOS volumes remain interchangeable to this day.

**What happened.** The hardware killed it. Apple recalled the first 14,000 machines, formally reintroduced the Apple III on 9 November 1981, and never recovered the reputation; the line was withdrawn on 24 April 1984 and quietly removed from the price list in September 1985, with reported losses above $60 million. SOS died with the machine; its file system survives inside ProDOS and in tools like ADTPro.

**Peak adoption.** an estimated 65,000–75,000 Apple III units in total

**Why it failed.** A genuinely advanced operating system was welded to hardware that did not work — chips walked out of un-socketed board seats under thermal cycling in a fanless case the industrial design had mandated — so the platform's reputation was destroyed inside its first year, and no amount of OS quality could recover it.

**What might have changed it.** Shipping six to nine months later on a qualified board with proper cooling — Wendell Sander's own diagnosis of the failure — gives SOS a working machine, and Apple's business line, rather than the Apple II's ad-hoc DOS 3.3, becomes the base for Apple's 1980s software stack.

**Evidence.** Steve Wozniak in 1985 called SOS 'the finest operating system on any microcomputer ever,' notwithstanding the Apple III's hardware. Sander's own retrospective identifies the decisive cause as the machine being rushed to market six to nine months early. Apple recalled 14,000 units and relaunched in November 1981. The clearest proof of SOS's quality is that Apple reused its file system unchanged for the Apple II's ProDOS three years later.

Tags: hardware tied to dying platform, technical shortfall, strategic mismanagement, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/Apple_SOS
- https://en.wikipedia.org/wiki/Apple_III
- https://digibarn.com/collections/systems/appleIII/sandersinterview.html
- https://lowendmac.com/2015/apple-iii-chaos-apples-first-failure/
- https://prodos8.com/docs/techref/prodos-the-appleiii-and-sos/

### AtheOS / Syllable Desktop
*1994-2002 (AtheOS); 2002-2012 (Syllable Desktop, last release 0.6.7) · Kurt Skauen (AtheOS); Kristian Van Der Vliet and team (Syllable fork)*

**What was brilliant.** Mostly one person's work: a re-entrant, SMP-capable native kernel, the 64-bit journaled AtheOS File System (AFS) with BeOS-style attributes, and a native object-oriented C++ GUI API and app server. It booted in seconds on low-end hardware and was cleaner for driver modularity than contemporary Linux.

**What happened.** Skauen stopped updating AtheOS in 2002 after going silent for about nine months. Developers led by Van Der Vliet forked it as Syllable (0.4.0, July 2002). Syllable Desktop's last release was 0.6.7 on 12 April 2012. A Linux-based 'Syllable Server' side project also stalled, and the project has been effectively dormant since.

**Why it failed.** With no funding and a handful of volunteers, the project could not keep up with PC drivers, a modern web engine and application ports. The original author's refusal to open up governance cost it two years of momentum before the fork.

**What might have changed it.** Skauen accepting co-maintainers and a foundation around 2000-2001, when AtheOS was drawing BeOS refugees after Be Inc.'s collapse, instead of keeping it a 'pet project' until he abandoned it.

**Evidence.** OSnews (2004): 'Kurt effectively abandoned the project, leaving users and developers with no updates for months and silence on the mailing lists'. He 'preferred to keep it as his own pet project'. Syllable lead Vanders estimated 1.0 was '2 or 3 years away' if the team stayed the same size, and 1.0 never shipped. Wikipedia lists about five primary developers, with the last release in April 2012.

Tags: funding collapse, technical shortfall, no app ecosystem, internal politics, no hardware channel

Sources:
- https://www.osnews.com/story/7900/syllable-the-little-os-with-a-big-future/
- https://en.wikipedia.org/wiki/Syllable_Desktop
- https://archiveos.org/atheos/
- https://www.operating-system.org/betriebssystem/_english/bs-atheos.htm
- https://github.com/syllable-org/syllable

### CapROS (Capability-based Reliable Operating System)
*2005-c.2015 (forked from EROS 1.x when EROS development stopped; ports to IA-32 and ARM9; repositories now dormant) · Charles R. Landau, one of the original KeyKOS architects at Tymshare, continuing the EROS code base independently*

**What was brilliant.** The only KeyKOS descendant to carry the full single-level-store, orthogonally-persistent capability model onto modern embedded hardware -- ports exist for Intel IA-32 and ARM9 -- in an open-source, commercially licensable code base. That last point matters historically: the reason EROS had to be written from scratch was that KeyKOS's IP died with Key Logic in 1991. CapROS finally made a working member of that architectural family something an engineer could pick up and ship, preserving the space-bank resource accounting model in which every allocation is charged to an explicit, revocable authority rather than to an ambient global heap.

**What happened.** Never reached production. It became a single-maintainer project after 2005, and the SourceForge and GitHub repositories have gone quiet. No commercial product was ever built on it; no distribution or hardware partner ever adopted it.

**Why it failed.** A one-person continuation of a thirty-year-old architecture cannot simultaneously maintain a kernel, write drivers for current hardware, and build the application environment that would give anyone a reason to boot it -- the work is not hard, it is simply larger than one person.

**What might have changed it.** If the EROS community had consolidated in 2005 instead of splitting, one funded team might have reached a usable system. Instead a research community already numbering in the low tens divided into two competing forks of the same kernel -- Landau's CapROS continuation and Shapiro's Coyotos rewrite -- and both starved.

**Evidence.** The structural point is visible in the repository metadata itself: capros-os/capros is described as 'an experimental operating system based on object-capabilities, derived from EROS, KeyKOS, and Gnosis. Ports exist for the Intel IA-32 and ARM9 architectures' -- experimental after a decade of work, with no production claim. The split is documented by the projects' own pages: EROS development stopped in 2005 'in favor of a successor system, CapROS' while Shapiro simultaneously took the EROS Group toward Coyotos, so the same tiny community ran two successors to one dead kernel.

Tags: funding collapse, no app ecosystem, no hardware channel, poor developer experience, compatibility gap

Sources:
- https://www.capros.org/
- https://www.capros.org/overview.html
- https://github.com/capros-os/capros
- https://www.charlielandau.com/
- https://en.wikipedia.org/wiki/EROS_(microkernel)

### Corel Linux OS
*1999-2001 · Corel Corporation (CEO Michael Cowpland), Ottawa*

**What was brilliant.** The first Linux desktop from a major commercial software vendor built for Windows users. It was a Debian base with a graphical installer that took a few clicks, KDE 1.1.2 with a Corel file manager that browsed Windows SMB networks, and native WordPerfect plus early Linux ports of Quattro Pro and CorelDRAW. The distribution became the base of Xandros, which later shipped on the first Asus Eee PC.

**What happened.** Released in late 1999. Cowpland resigned in August 2000. On 2 October 2000 Microsoft bought $135M of non-voting preferred shares in Corel, and Corel then said it would pull back from Linux. The Linux division was sold to the startup Xandros in August 2001. Microsoft later sold its stake to Vector Capital for $12.9M.

**Why it failed.** Corel ran its Linux bet out of a company that was bleeding cash. The distribution earned only a few million dollars a year, so once Corel needed a rescue, and the rescuer was Microsoft, the Linux division was the obvious thing to cut.

**What might have changed it.** Spin Corel Linux out in 1999 as a separately funded company with its own capital, instead of leaving it inside a cash-strapped parent whose survival depended on Windows application revenue.

**Evidence.** Corel's own FY2000 10-K MD&A says Linux-related revenue fell 24% to $2.7M and that fewer resources would go to Corel LINUX OS as the company refocused on two main business lines. The popular story is that Microsoft's $135M bought Linux's death. Microsoft denied the investment was conditional and regulators looked into it, but the filings show the decision also made financial sense for Corel on its own terms. Linux was under 2% of revenue at a company whose CEO had just resigned.

Tags: funding collapse, business model failure, strategic mismanagement, no app ecosystem, incumbent lockin

Sources:
- https://www.sec.gov/Archives/edgar/data/0000890640/000089064001500011/fy00mda.htm
- https://news.microsoft.com/source/2000/10/02/corel-and-microsoft-announce-strategic-alliance/
- https://www.cbc.ca/news/business/microsoft-to-sell-stake-in-corel-for-fraction-of-cost-1.403880
- https://www.pinsentmasons.com/out-law/news/microsoft-investigated-over-investment-in-rival-corel
- https://www.cbc.ca/news/business/corel-offloads-linux-division-1.280661

### DG/UX
*1985–2001 (first release March 1985; 4.00 redesign 1988; 5.4 c. 1991; final 5.4 Release 4.20 MU07 in April 2001) · Data General Corporation (Westborough, Massachusetts)*

**What was brilliant.** The quietest and most advanced storage engineering in the commercial Unix field. DG/UX supported multiprocessor machines when most Unix variants did not, and around 1991 shipped a journaling filesystem, 2TB filesystem support, and online disk administration that let an administrator extend, relocate, mirror and even shrink a live filesystem without unmounting it — shrinking a mounted filesystem was a decade ahead of everyone — plus split-mirror online backup. Version 4.00 (1988) was a full redesign on SVR3 that added SMP to the Eclipse MV line; 5.4 added unified virtual memory management and moved to SVR4. Later releases added processor and memory affinity for the AViiON NUMA machines (AV 20000/25000), among the first commercial cache-coherent NUMA servers.

**What happened.** Data General staked AViiON on Motorola's 88000 RISC family; Motorola abandoned the 88000 for PowerPC, leaving DG to port DG/UX to Intel IA-32 without a competitive high-end CPU roadmap. EMC announced the acquisition of Data General on 9 August 1999 at about $19.58 per share — roughly $1.1B total — and completed it on 12 October 1999, overwhelmingly for the CLARiiON storage business rather than the servers. DG/UX's final release, 5.4 Release 4.20 (MU07), shipped in April 2001, and EMC wound the server business down.

**Why it failed.** DG/UX's advanced storage and NUMA engineering was locked to AViiON hardware built on Motorola's 88000, so when Motorola killed the 88000 the platform lost its processor roadmap — and EMC then bought Data General for its disk arrays, with no reason whatsoever to keep an operating system.

**What might have changed it.** Port DG/UX to Intel early and sell it as a standalone product rather than an AViiON accessory — DG did eventually ship IA-32 support, but only after the high-end story was gone — or spin the online storage-management technology out as a product in its own right, which is essentially what EMC bought the company for anyway. Note that the popular framing of 'Linux killed DG/UX' is wrong: the 88000's cancellation and the EMC acquisition did.

**Evidence.** First release March 1985 on SVR2 with 4.1BSD additions; SMP on Eclipse MV from version 4.00 (1988); journaling, 2TB filesystems, online extend/relocate/mirror/shrink and split-mirror backup circa 1991; processor and memory affinity for NUMA in later releases. EMC/Data General announced 9 Aug 1999 at ~$19.58/share and ~$1.1B total, completed 12 Oct 1999. Final DG/UX release April 2001.

Tags: hardware tied to dying platform, acquired and killed, no hardware channel, niche capture, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/DG/UX
- https://www.storagenewsletter.com/2022/07/21/history-1999-emc-acquired-data-general-for-1-1-billion/
- https://www.emc.com/about/news/press/us/1999/19991012-55.htm
- https://www.sec.gov/Archives/edgar/data/0000026999/000089882299000409/0000898822-99-000409.txt/seq-4

### Endless OS
*2012-present (for-profit Endless Mobile 2012-2020; nonprofit Endless OS Foundation since 1 April 2020) · Matt Dalio and Marcelo Sampaio, Endless Mobile Inc. (San Francisco / Rio de Janeiro)*

**What was brilliant.** An early consumer distribution built on an immutable, read-only root filesystem managed by OSTree, with applications delivered as Flatpaks. This is the architecture Fedora Silverblue and others later made mainstream. It was paired with large offline content libraries for users with little or no internet, and very cheap hardware (the $79 ARM Endless Mini, January 2016).

**What happened.** Kickstarter in April 2015 raised $176,538. Retail through Claro stores in Guatemala from November 2015. Preloads on Acer and ASUS no-OS laptop SKUs in Southeast Asia aimed at over a million devices. In 2020 it gave up on consumer commercialisation and converted into a nonprofit foundation focused on education and digital-access programmes.

**Why it failed.** Endless had OEM shelf space but no revenue model for an OS given away on no-OS laptop SKUs. A venture-funded startup could not pay to fight the consumer OS market head-on until the channel paid off.

**What might have changed it.** From the start, sell Endless as a funded public-sector or education platform (government and school procurement), which is where the foundation ended up, instead of spending eight years trying to be a VC-scaled consumer OS company.

**Evidence.** Endless engineer Robert McQueen, on launching the foundation (January 2021): the team framed itself as a Silicon Valley startup believing 'a successful commercial channel would be the most efficient way to scale the impact', but 'we've just learned through our experience that we don't have the funding to enter the computer and OS marketplace head-on.' User forums on Microsoft Q&A and Acer's community show buyers of Endless-preloaded Acer and ASUS laptops asking how to replace it with Windows. That is anecdotal, but consistent with the no-OS SKU pattern.

Tags: funding collapse, business model failure, no app ecosystem, compatibility gap, niche capture

Sources:
- https://ramcq.net/2021/01/22/launching-endless-os-foundation/
- https://en.wikipedia.org/wiki/Endless_OS_Foundation
- https://www.digitalnewsasia.com/digital-economy/endless-computers-launches-endless-os-pc
- https://www.phoronix.com/news/ASUS-Endless-OS-Linux-Laptops
- https://www.tomshardware.com/news/endless-mini-79-desktop-pc,30832.html

### GEOS (Berkeley Softworks, 8-bit)
*1986–early 1990s (C64 1986; C128 and Plus/4 1987; Apple II 1988; released as freeware 2003–04) · Berkeley Softworks — Brian P. Dougherty, who founded the company in 1983 with $100,000 of his own money after leaving Imagic*

**What was brilliant.** A full bitmapped, mouse-driven, WYSIWYG GUI with proportional fonts, pull-down menus, dialogs, a clipboard and a printer-driver architecture — on a 1 MHz 6510 with 64 KB of RAM and a 40-column display. To make it usable at all, Berkeley wrote diskTurbo, a fast loader that reprogrammed the 1541 drive's own onboard 6502 and cut load times several-fold, fixing Commodore's notorious serial-bus bottleneck in software. It was the first Commodore software to use a floppy disk as swap space — genuine virtual memory on a C64 — and it drove Apple LaserWriter and HP PCL printers through the geoCable interface, so a sub-$300 home computer could do real desktop publishing (geoPublish) with laser output.

**What happened.** Commodore bundled GEOS with the redesigned C64C for several years, which is how most of its users got it. The platform died with the 8-bit home computer: Berkeley Softworks renamed itself GeoWorks in 1990 and moved to PC/GEOS, abandoning the 8-bit line, and Commodore itself liquidated on 29 April 1994. The C64 and Apple II versions were released as freeware in 2003–04.

**Peak adoption.** ranked at its peak as the third most-shipped microcomputer operating system in the world by units, behind MS-DOS and Mac OS, and the second most widely used GUI after the Macintosh; a December 1987 Compute!'s Gazette reader survey found nearly half of respondents used GEOS. No audited unit figure exists

**Why it failed.** GEOS was extraordinary engineering bolted to an architecture with no future — the C64's 1 MHz and 64 KB ceiling could not be raised — so its fate was fixed by Commodore's decision never to build an 8-bit successor, and its own authors began leaving for the PC in 1986, the year it shipped.

**What might have changed it.** Moving the GEOS object model to the 68000 in 1986–87 — to the Amiga or Atari ST, both of which lacked a polished document-centric application suite and both of which their vendors were failing to supply — takes the codebase off the 8-bit dead end four years earlier than the eventual DOS port did, and into platforms that still had a future.

**Evidence.** GEOS's copy protection required booting from the original protected disk, so a failed boot disk cost the user every document ever created with it; retrospectives identify this as its deepest structural flaw — a business decision that made the platform fragile by design. Dougherty started PC/GEOS development in 1986, the same year the C64 version shipped: the company did not itself believe in the 8-bit future it was selling into.

Tags: hardware tied to dying platform, business model failure, no app ecosystem, technical shortfall

Sources:
- https://en.wikipedia.org/wiki/GEOS_(8-bit_operating_system)
- https://en.wikipedia.org/wiki/Berkeley_Softworks
- https://www.commodore.ca/commodore-history/geos-looking-back/
- https://www.c64-wiki.com/wiki/GEOS

### Helios (Transputer)
*1986-1998 (development autumn 1986; v1.0 summer 1988; final release 1.3.1 September 1992; Perihelion ceased trading 1998; later GPLv3 source release) · Perihelion Software (Tim King, Nick Garnett; King previously of TRIPOS and AmigaDOS at MetaComCo)*

**What was brilliant.** A Unix-like distributed microkernel OS for parallel machines. A network of Transputers appeared as one system with a distributed namespace, and services were reached through one messaging protocol. It included a POSIX compatibility library and automatic mapping of task-force process networks onto processor topologies. It was later ported to ARM (the Active Book tablet) and the TI TMS320C40 DSP.

**What happened.** Its best-known platform, the Atari Transputer Workstation (ATW-800, production from May 1989), sold almost nothing. SGS-Thomson bought Inmos in April 1989, and the T9000 transputer was delayed, missed its performance targets and was abandoned. Helios 1.4 (X11 and Motif support) was never completed. Perihelion ceased trading in 1998.

**Why it failed.** Helios bet everything on the Inmos Transputer as the future of computing. When the next-generation T9000 slipped and was abandoned and commodity x86/RISC processors outpaced transputers, Helios lost its hardware and remained a niche parallel-computing OS.

**What might have changed it.** If Perihelion had made commodity workstation processors (x86, SPARC, MIPS) the main target by about 1990, instead of treating the Transputer as primary, Helios could have become a distributed-cluster OS for networks of ordinary machines.

**Evidence.** Atari ATW: 'Sales were almost non-existent, and the product was canceled after only a few hundred units were made'; 'only 350 machines were produced'. T9000: it 'suffered significant delays, sufficient funding was not available and it did not meet performance goals', giving rise to the quip that 'the best host architecture for a T9000 was an overhead projector'. SGS-Thomson cancelled it and kept only the older cores as the ST20 microcontroller line.

Tags: hardware tied to dying platform, no hardware channel, no app ecosystem, niche capture

Sources:
- https://en.wikipedia.org/wiki/Helios_(operating_system)
- https://en.wikipedia.org/wiki/Atari_Transputer_Workstation
- https://en.wikipedia.org/wiki/Perihelion_Software
- https://en.wikipedia.org/wiki/Inmos
- https://www.rs-online.com/designspark/revisiting-the-inmos-transputer

### HP-UX
*1982–2025 (final release 2505.11iv3 on 22 May 2025; standard support ended 31 Dec 2025) · Hewlett-Packard, later Hewlett Packard Enterprise*

**What was brilliant.** The most conservatively and thoroughly engineered of the commercial Unixes, and the origin of several things now taken for granted. It was the first Unix to offer access control lists as an alternative to the standard rwx permission bits. HP's Logical Volume Manager, introduced in HP-UX 9.00 on the Series 800, is the direct design ancestor of Linux LVM — HP contributed the design. The Virtual Vault releases (10.24, 11.04) shipped compartmentalised mandatory access control with no all-powerful root in the 1990s, roughly the model SELinux and container security rediscovered a decade later. Serviceguard clustering, dynamic kernel tunables, loadable kernel modules, hard partitions (nPars) and virtual partitions (vPars) delivered mainframe-grade partitioning and uptime on midrange hardware.

**What happened.** HP-UX rode HP's architecture chain — FOCUS, then Motorola 68000, then PA-RISC, then Itanium from 11i v1.5 in 2001. HP dropped a planned x86-64 edition around 2012, binding HP-UX to Itanium alone. Oracle's 2011 announcement that it would stop developing for Itanium triggered HP v. Oracle. Intel ended Itanium shipments in 2021; HP 9000 PA-RISC systems went end-of-life 31 March 2021. The final HP-UX release shipped 22 May 2025 and standard support for 11i v3 ended 31 December 2025, with best-effort 'mature support' to 2028. Forty-three years, ended by the death of its processor rather than by a competing operating system.

**Why it failed.** HP-UX never shipped a commodity-hardware edition, so its total addressable market was exactly the volume of HP's own proprietary CPUs; when HP staked that volume on Itanium and Itanium failed, HP-UX inherited the failure with no escape route it had left itself.

**What might have changed it.** Ship HP-UX on x86-64. The port was planned and dropped around 2012, and arguably should have shipped around 2005 when Opteron made 64-bit commodity hardware real. Instead HP let Linux on its own ProLiant line cannibalise HP-UX while keeping HP-UX hostage to Itanium.

**Evidence.** HP-UX 11i v3 (B.11.31) released Feb 2007; last update 2505.11iv3 on 22 May 2025; support ended 31 Dec 2025 with mature support to 2028. HP 9000 PA-RISC EOL 31 March 2021; Intel ended Itanium shipments 2021. First Unix with ACLs; LVM debuted in HP-UX 9.00 and influenced Linux LVM; Virtual Vault compartments in 10.24/11.04. The popular framing that 'Linux killed HP-UX' is at best half the story: the proximate cause was Intel exiting Itanium and HP's own decision to abandon the x86-64 port.

Tags: hardware tied to dying platform, niche capture, strategic mismanagement, incumbent lockin, no hardware channel

Sources:
- https://en.wikipedia.org/wiki/HP-UX
- https://www.theregister.com/software/2026/01/05/the-last-supported-version-of-hp-ux-is-no-more/2347266
- https://www.hpe.com/global/softwarereleases/releases-media2/HPEredesign/latest/AR2505/AR2505_OEUR_Letter_English.pdf
- https://www.telecompaper.com/news/hp-ships-more-nt-workstations-in-1998--161466

### IRIX
*1988–2006 (IRIX 3.0 in 1988; 6.5 May 1998; 6.5.30 on 16 Aug 2006; support ended Dec 2013) · Silicon Graphics, Inc. (Mountain View, CA; founded 9 Nov 1981)*

**What was brilliant.** The best large-SMP Unix of its generation and the origin of infrastructure still in daily use. XFS, shipped in IRIX 5.3 in 1994, was extent-based, B+tree-indexed, journaled, with delayed allocation and terabyte-scale capability while competitors' fsck runs took hours — it is the default filesystem on Red Hat Enterprise Linux today. A single system image scaling from 1 to more than 1,024 processors on ccNUMA Origin hardware, at a time when four-way SMP was considered ambitious. Hard real-time facilities (REACT) with frame-scheduled processes, processor isolation and guaranteed-rate I/O, which is why IRIX ran film post-production and uncompressed video. And IRIS GL, SGI's in-OS graphics interface, became OpenGL — the industry standard graphics API emerged from this operating system.

**What happened.** SGI's revenue peaked at $3.7B under Ed McCracken (through 1997) with market capitalisation over $7B in 1995 and around 10,000 employees. In 1998 SGI announced it would end-of-life MIPS in favour of Intel's Itanium. Rick Belluzzo's Windows NT Visual Workstation 320/540 launched in 1999 with a proprietary Cobalt chipset, ARCS firmware in place of a BIOS and non-standard 3.3V PCI; it was discontinued in January 2002 and replaced by ordinary PCs. Itanium arrived late and slow while MIPS stagnated. SGI moved to Linux on Itanium (Altix), filed Chapter 11 on 8 May 2006 with a market capitalisation of $18M against $664M of debt and $332M of assets, filed again on 1 April 2009, and was sold to Rackable for $42.5M. IRIX's last release was August 2006; XFS was donated to Linux and outlived it.

**Why it failed.** IRIX was purchasable only bolted to SGI's own MIPS silicon, and SGI's 1998 decision to end-of-life MIPS in favour of an Itanium that did not yet exist removed the platform's future before any replacement was available — so the operating system died with the CPU roadmap, not from any deficiency in the OS.

**What might have changed it.** Ship IRIX, or at least its XFS / REACT / ccNUMA stack, on commodity x86 in 1997–98 instead of betting the company on Itanium and a proprietary Windows NT PC. The cultural failure is captured by a former staffer's line about the era: 'Everyone in management had read The Innovator's Dilemma, but no one knew how to execute on it.'

**Evidence.** IRIX 5.3 introduced XFS in 1994; IRIX scaled to >1,024 processors under a single system image. SGI peak revenue $3.7B (1997), market cap >$7B (1995) falling to $120M by Nov 2005 and $18M at the Chapter 11 filing. Cray acquired Feb 1996 for $740M, its business systems division resold within three months for 'significantly less than $100M', the brand spun out 31 March 2000 for $35M plus a million shares. Visual Workstation launched Aug 1999 and discontinued Jan 2002. Chapter 11 on 8 May 2006 and again 1 April 2009; Rackable acquisition $42.5M.

Tags: hardware tied to dying platform, strategic mismanagement, no hardware channel, too late to market, niche capture

Sources:
- https://en.wikipedia.org/wiki/IRIX
- https://en.wikipedia.org/wiki/Silicon_Graphics
- https://www.theregister.com/2006/05/09/sgi_chapter11_analysis/
- https://en.wikipedia.org/wiki/SGI_Visual_Workstation
- https://www.telecompaper.com/news/hp-ships-more-nt-workstations-in-1998--161466

### ITS (Incompatible Timesharing System)
*1967-1990 (MIT); hobbyist revival on emulators and museum KS10 · MIT Artificial Intelligence Lab hackers (Greenblatt, Knight, Holloway, Eastlake et al.)*

**What was brilliant.** PCLSRing: system calls that look atomic to user processes and can be interrupted safely, so no process can ever see another mid-syscall. This idea is the ancestor of the Unix 'PC-loser-ing' debate in Gabriel's 'Worse is Better'. It also had device-independent terminal output, job devices (user-mode programs acting as devices, a precursor of FUSE and Plan 9 file servers), and network-transparent file access on early ARPANET. Emacs, MACLISP, Scheme, CLU, Macsyma and Zork were born on ITS.

**What happened.** Ran on the AI, ML, DM and MC PDP-6/PDP-10 machines at MIT. The machines retired through the 1980s: MC stopped in 1988 and all MIT ITS machines were permanently shut down by 1990. Today it survives in the PDP-10/its GitHub reconstruction and on museum hardware.

**Why it failed.** ITS was hand-written in MIDAS assembly for MIT's modified PDP-6/PDP-10s, with custom paging hardware and microcode. When DEC cancelled the 36-bit line in 1983, the system had nowhere to go and died with its hardware.

**What might have changed it.** If ITS had been written in a portable high-level language, or picked up by a vendor such as DEC in the way TENEX was, its concepts could have shipped commercially instead of spreading only through the people and programs it produced.

**Evidence.** Computer History Wiki: later versions ran on KA10s 'modified with MIT-designed and built paging hardware', and the KL10 and KS10 needed 'custom microcode that emulated the operation of the MIT paging box'. 'By 1990 all MIT machines were shut down permanently.' It deliberately had no file protection ('any user could read or write any file'), a design that suited its community and ruled out commercial use.

Tags: hardware tied to dying platform, niche capture, no hardware channel, compatibility gap

Sources:
- https://gunkies.org/wiki/Incompatible_Timesharing_System
- https://en.wikipedia.org/wiki/Incompatible_Timesharing_System
- https://github.com/PDP-10/its
- http://fare.tunes.org/tmp/emergent/pclsr.htm

### Moblin
*2007–2010 (moblin.org launched July 2007; transferred to the Linux Foundation April 2009; Moblin 2.1 late 2009; merged into MeeGo February 2010) · Intel Open Source Technology Center*

**What was brilliant.** Moblin is the project that made Linux boot and resume fast enough for a consumer appliance, and most of that work is still in production everywhere. Intel's OSTC attacked the boot path directly — readahead profiling, asynchronous init, kernel and udev changes — and demonstrated cold boot in the single-digit seconds on Atom netbook hardware at a time when mainstream distributions took a minute. Its components outlived the OS by a wide margin: ConnMan, the connection manager, is still the network manager in Sailfish, Tizen, Automotive Grade Linux and countless embedded systems; Clutter, the GPU-accelerated retained scene-graph toolkit that came with Intel's 2008 acquisition of OpenedHand, went on to underpin GNOME Shell; and the Poky/OpenEmbedded work from the same acquisition became the Yocto Project, now the standard way the embedded Linux industry builds device images. The MyZone shell was also a serious attempt at an activity-centric rather than app-grid home screen.

**What happened.** Reached a handful of Atom netbooks (Acer Aspire One, MSI U135) as a preinstall option and one announced smartphone, the LG GW990 shown at CES 2010, which was cancelled and never shipped. Intel handed governance to the Linux Foundation in April 2009 and then, at MWC in February 2010, merged Moblin with Nokia's Maemo into MeeGo — before Moblin had shipped a single flagship device of its own. MeeGo in turn was abandoned by Intel in September 2011 in favour of Tizen. The device category Moblin was built for, the Atom-based netbook and Mobile Internet Device, was itself destroyed by the iPad in April 2010 and by cheap Android tablets immediately after.

**Why it failed.** Moblin existed to sell Intel Atom silicon into phones and MIDs, and Atom lost the mobile performance-per-watt argument to ARM decisively between 2008 and 2011; once the hardware thesis failed there was no independent commercial reason for any OEM to ship the operating system, however good it was.

**What might have changed it.** If Intel had shipped and marketed the LG GW990 in 2010 as a real product rather than folding Moblin into a brand-new joint project with Nokia, it would at least have had a device to iterate on. But the binding constraint was silicon: without a competitive ARM-class power envelope, no software decision saves Moblin — which is why its best engineering survived as components inside other people's ARM products.

**Evidence.** The component lineage is the clearest evidence of technical quality: ConnMan, Clutter and the Poky build system (now the Yocto Project) all came out of Moblin/OpenedHand and are still industry infrastructure. The commercial record is the opposite: transferred to the Linux Foundation April 2009, merged into MeeGo February 2010 with no flagship device ever shipped, LG GW990 cancelled, MeeGo dead by September 2011.

Tags: hardware tied to dying platform, no hardware channel, strategic mismanagement, no app ecosystem, niche capture, perpetual rewrite

Sources:
- https://en.wikipedia.org/wiki/Moblin
- https://www.engadget.com/2010-02-15-meego-nokia-and-intel-merge-maemo-and-moblin.html
- https://www.osnews.com/story/22875/nokia-intel-merge-moblin-maemo-into-meego/
- https://archiveos.org/moblin/

### Nemesis
*c.1992–1999; built under the ESPRIT Pegasus and Pegasus II projects (Pegasus II = ESPRIT LTR 21917); final release Nemesis II, 26 April 1999 · University of Cambridge Computer Laboratory — Ian Leslie, Derek McAuley, Timothy Roscoe, Paul Barham, Richard Black, Steven Hand, David Evers, Robin Fairbairns, Eoin Hyden — with the University of Twente, University of Glasgow, SICS and APM Ltd*

**What was brilliant.** Nemesis is the only operating system that treated resource accounting as the primary design constraint and restructured everything else around it. Its diagnosis was QoS crosstalk: in a conventional kernel, or in any shared server, work performed on behalf of one application is charged to the wrong principal, so a background compile can starve a video decoder no matter how clever the scheduler is. The fix was structural rather than algorithmic — make the kernel tiny and push essentially all operating-system work into shared libraries executing inside the application's own domain and on its own CPU allocation, so every cycle is charged to whoever caused it. That is the vertically structured OS. It ran as a single address space system with per-domain protection, so cross-domain data movement required no copying — critical for multimedia streams. And it introduced self-paging: each application manages its own physical memory allocation and services its own faults, so no process can inflict paging cost on another. The result was genuine, measured QoS isolation for continuous media on commodity hardware in the mid-1990s, something Linux and Windows still do poorly in 2026.

**What happened.** The ESPRIT funding ended, Nemesis II shipped on 26 April 1999, and the project stopped. Its people and its central argument went straight into the system that did win: substantially the same Cambridge group — Barham, Hand, Fraser, Pratt, with Roscoe — built Xen, whose design goal (strict, accountable resource isolation between mutually untrusting tenants on one machine) is Nemesis' goal restated for the server room, and whose thin-hypervisor-plus-fat-guest structure is Nemesis' vertical structure with a different boundary. Xen became the foundation of Amazon EC2. Roscoe went to ETH Zurich and started Barrelfish.

**Why it failed.** Nemesis solved per-application resource accounting for continuous media, a problem that mattered acutely only for the 1990s 'multimedia workstation' — a product category that never materialised; by the time the same problem reappeared in a form people would pay for, as cloud multi-tenancy, the commercially viable answer had to run existing operating systems unmodified, so it had to be a hypervisor, and the same team went and built one.

**What might have changed it.** If the Pegasus consortium had shipped Nemesis' vertical structure as a resource-isolation layer beneath existing operating systems in 1997, rather than as a standalone OS with no applications, it would have been Xen five years early — and Cambridge, not Amazon, would have set the terms of the first decade of cloud computing.

**Evidence.** Pegasus II ran as ESPRIT LTR 21917 with Cambridge, Twente, Glasgow, SICS and APM Ltd as partners; the project's stated philosophy was that 'resource management to provide application QoS guarantees is required' and that 'generic multimedia platforms, rather than single multimedia applications' were the future. The design principle is recorded in the project's own documentation: 'the majority of code could execute in the application process itself... an extremely small lightweight kernel, and performs most operating system functions in shared libraries which execute in the user's process.' Self-paging was published at OSDI '98 (Hand, 'Self-Paging in the Nemesis Operating System'). Final release 26 April 1999. The clinching evidence for the causal claim is personnel: the Nemesis authors are, in large part, the authors of the Xen SOSP 2003 paper, which makes the same accountability argument for a different boundary and succeeded commercially.

Tags: funding collapse, no app ecosystem, no hardware channel, niche capture

Sources:
- https://www.cl.cam.ac.uk/research/srg/netos/projects/archive/pegasus/
- https://www.cl.cam.ac.uk/research/srg/netos/projects/archive/pegasus/pegasus1.html
- https://www.cs.columbia.edu/~nieh/teaching/e6118_s02/papers/2_5_hand.html
- https://dl.acm.org/doi/10.1145/191525.191537
- https://en.wikipedia.org/wiki/Nemesis_(operating_system)

### Sailfish OS
*2012–present (Jolla founded 2011, incorporated 29 March 2011; OS announced July 2012; Jolla Phone Nov 2013; original Jolla Oy bankrupt May 2024; business continues as Jollyboys with a new Jolla Phone announced Dec 2025) · Jolla Oy — the ex-Nokia MeeGo/Harmattan engineering team (Sami Pienimäki, Jussi Hurmola, Marc Dillon, Stefano Mosconi, Antti Saarnio) on the Mer fork of the MeeGo core*

**What was brilliant.** Sailfish took the Harmattan gesture model further than anyone has since: there is no home button and no back button, and the entire OS is driven by edge swipes plus 'pulley menus' — a menu revealed by dragging the top of a list downward and selected by releasing, so every command lives inside one continuous thumb gesture without ever lifting the finger. The 'ambience' system re-themes the whole OS from the palette of the chosen wallpaper. Architecturally it is a real, modern Linux distribution: RPM packages, systemd, Wayland with the Qt/QML Lipstick compositor, a full shell and package manager. Most importantly it has a legally clean Android application compatibility layer — originally Myriad's Alien Dalvik, since 2019 'AppSupport' running AOSP inside LXC containers — so a Sailfish device runs Android apps without being Android and without being bound by Google's Compatibility Definition or Mobile Application Distribution Agreement. That is the only shipping European mobile OS with a credible digital-sovereignty story, which is why it kept winning government work.

**What happened.** The Jolla Phone (Nov 2013) and the crowdfunded Jolla Tablet both under-delivered badly: only 540 tablets were ever manufactured out of a campaign backed by more than 21,000 people for over $2.5 million, and the remainder were refunded through 2016. Jolla laid off about half its staff in November 2015 when investor financing was delayed, then split hardware from software and pivoted to licensing. The licensing business became overwhelmingly dependent on Russia: Sailfish was certified by the Russian government in November 2016 as its first approved Android alternative, Rostelecom invested in March 2018 and ultimately took 75% of the Aurora OS derivative. After February 2022 Jolla had to unwind that relationship entirely, which took the revenue with it. The Pirkanmaa District Court approved a corporate restructuring in November 2023 transferring the business to a management-owned entity; the original Jolla filed for bankruptcy in May 2024. Other deployments: the Intex Aqua Fish (India, 2016), Sony Xperia X/10 Open Devices ports, the Planet Gemini PDA, and 30+ community ports.

**Why it failed.** Sailfish is a good OS built by a company with no route to volume hardware, so it had to sell sovereignty instead of product — and a licensing business whose one substantial paying customer was the Russian state was destroyed the moment that customer became commercially and legally unusable in February 2022.

**What might have changed it.** If Jolla had converted the Sony Open Devices relationship into an officially retailed Sony variant rather than an enthusiast firmware download — an OEM channel rather than a flashing guide — it would have had a hardware route to volume that did not depend on state licensing revenue, and the 2022 decoupling would have been survivable.

**Evidence.** 540 tablets manufactured against 21,000+ backers and $2.5m raised is the clearest single measure of the hardware problem. The financial sequence is documented: half the staff laid off November 2015, Russian government certification November 2016, Rostelecom investment March 2018 and 75% of Aurora, business discontinued in Russia February 2022, court restructuring November 2023, bankruptcy May 2024. The technical quality is evidenced by survival itself — the same codebase is still shipping in 2026 with a new device, more than a decade after its parent platform was cancelled by Nokia.

Tags: funding collapse, no hardware channel, business model failure, licensing or legal, no app ecosystem, niche capture

Sources:
- https://en.wikipedia.org/wiki/Sailfish_OS
- https://en.wikipedia.org/wiki/Jolla
- https://techcrunch.com/2016/02/01/jolla-confirms-the-sailfish-tablet-is-dead
- https://techcrunch.com/2016/11/29/jollas-sailfish-os-now-certified-as-russias-first-android-alternative/
- https://techcrunch.com/2022/03/01/jolla-cut-ties-russia/

### SPIN
*1994–1997 (initial release 1994; final release 1.0 in November 1996) · Brian Bershad's group at the University of Washington — Bershad, Stefan Savage, Przemysław Pardyak, Emin Gün Sirer, Marc Fiuczynski, David Becker, Craig Chambers and Susan Eggers; ARPA and NSF funded, with a DEC equipment grant*

**What was brilliant.** SPIN produced the hardest numbers anyone has for the claim that language-enforced protection beats hardware-enforced protection. Applications dynamically link Modula-3 extensions into the kernel's own address space; the compiler's safe subset plus logical protection domains built from typed pointers guarantees an extension cannot forge a reference, touch memory it was not handed, or execute a privileged instruction, and an event/handler dispatch system lets extensions replace scheduling, paging or network paths for themselves only. Measured on 133 MHz DEC Alpha AXP workstations against DEC OSF/1 V2.1 and Mach 3.0: a protected in-kernel call costs 0.13 µs where a cross-address-space call costs 845 µs on OSF/1 and 104 µs on Mach; system calls 4 µs versus 5 and 7; kernel thread fork-join 22 µs versus 198 and 101; page-fault handling 29 µs versus 329 and 415; the Appel1 VM benchmark 39 µs versus 382 and 819. A SPIN web server saturated the network while consuming half the processor of the OSF/1 server. The whole kernel was 65,652 lines of Modula-3.

**What happened.** Final release 1.0 in November 1996; the project wound down in the late 1990s. Its implementation language died with its corporate sponsor — DEC SRC, source of the Modula-3 compiler SPIN depended on, was dismantled after the Compaq acquisition — removing any industrial path. The idea outlived the system: safe, verified, dynamically loaded in-kernel extensions are now mainstream as eBPF, reached by a bytecode verifier rather than a type-safe source language.

**Why it failed.** SPIN's safety guarantee was inherited from one specific compiler for one specific language on one specific vendor's hardware, all three of which — Modula-3, DEC SRC and the Alpha — ceased to exist within a few years, so the system had no substrate to survive on regardless of how well it measured.

**What might have changed it.** Reimplement the extension model on a language with an industrial future — or, as eBPF later did, on an independent bytecode verifier that does not require the whole kernel to be written in the safe language. SPIN proved the performance case in 1995; the idea then had to wait roughly twenty years to reach Linux because its 1995 implementation was welded to a dying toolchain.

**Evidence.** Bershad et al., SOSP '95, Table 2: protected in-kernel call 0.13 µs (SPIN) versus cross-address-space call 845 µs (DEC OSF/1) and 104 µs (Mach); Table 3 fork-join 22/198/101 µs; Table 4 page fault 29/329/415 µs and Appel1 39/382/819 µs; Table 1 kernel size 65,652 lines and 810,550 bytes of text using 'the DEC SRC Modula-3 compiler, release 3.5'; 'SPIN and its extensions are written in Modula-3 and run on DEC Alpha workstations'; the SPIN HTTP server and DEC OSF/1 both 'saturate the network, but SPIN consumes only half as much of the processor.' The project's own retrospective verdict: 'Modula-3 was a win; not as painful as anticipated.'

Tags: funding collapse, hardware tied to dying platform, incumbent lockin, no app ecosystem, niche capture

Sources:
- https://cseweb.ucsd.edu/~savage/papers/Sosp95.pdf
- https://en.wikipedia.org/wiki/SPIN_(operating_system)
- https://cs.uwaterloo.ca/~brecht/servers/readings/Summaries/Seltzer-OS/readings/bershad-1995.html
- https://www-spin.cs.washington.edu/papers/index.html
- https://dl.acm.org/doi/10.1145/202453.202472

### webOS (Palm / HP)
*2009–2011 as a phone OS (announced CES 8 Jan 2009; Pre shipped 6 June 2009; HP cancelled all webOS hardware 18 Aug 2011); open-sourced 2012; LG TV platform from 25 Feb 2013 to today · Palm, Inc. (Jon Rubinstein, Matias Duarte, Mitch Allen), then HP, then LG Electronics*

**What was brilliant.** webOS made the web stack the system stack rather than a wrapper around one. Applications were HTML/CSS/JavaScript running against Palm's Mojo (later Enyo) framework on a WebKit/Linux base, and LunaSysMgr — the window manager itself — was written in JavaScript. webOS 2.0 (October 2010) shipped Node.js as the OS service layer, making server-side JavaScript the supported way to write background services: the first shipping mobile OS to do that, and roughly five years ahead of anyone else. Three of its UI inventions became universal: 'cards,' where running apps are a horizontally scrolled deck you flick upward to kill (later adopted in iOS, Android and Windows Phone); a genuinely non-modal notification bar that never interrupted the foreground app; and Synergy, which merged Google, Exchange and Facebook contacts and message threads into single unified records on-device. 'Just Type' — start typing from anywhere to search or act — predates both Android's and iOS's universal search behaviours.

**What happened.** Launched US-exclusive on Sprint, on a TI OMAP3430 that could not drive a JavaScript-composited UI smoothly, with the SDK delayed for months after announcement and no paid apps until August 2009, two months after launch — by which time developers had moved to Android. HP acquired Palm for $1.2 billion (announced April 2010, closed July 2010), announced in February 2011 that webOS would ship on every HP PC, launched the TouchPad on 1 July 2011, and cancelled all webOS hardware on 18 August 2011 — 49 days later — as part of Léo Apotheker's pivot away from consumer devices. The $99 fire-sale TouchPad promptly became one of the best-selling tablets in the US. HP released Open webOS in 2012, sold the platform to LG on 25 February 2013 and the patents to Qualcomm in January 2014. It is now by far the largest-volume survivor in this list: LG shipped 22.6 million TVs in 2024 (16.1% of the global TV market), all running webOS, and LG's webOS platform business passed KRW 1 trillion in revenue in 2024.

**Peak adoption.** Palm shipped roughly 370,000 Pre units into the channel across May–June 2009 and was producing about 15,000/day at peak; sell-through was never disclosed. As a TV OS today, webOS ships on LG's ~22.6 million annual TV units.

**Why it failed.** webOS was a software design two hardware generations ahead of the silicon Palm could afford — a JavaScript-composited UI on a 2008-class SoC, which made a beautiful OS feel sluggish — and the only company that could have fixed that, HP, killed the hardware 49 days after shipping its flagship for reasons that had nothing to do with the operating system.

**What might have changed it.** If HP had given webOS the eighteen months and the Snapdragon-class hardware it had paid $1.2bn to fund — or, earlier, if Palm had launched the Pre on Verizon or AT&T instead of Sprint alone — webOS had the design, the notification model and the developer story to be the third ecosystem. The fire-sale demand spike is direct evidence that the product was desirable at the right price on the right hardware.

**Evidence.** 49 days from TouchPad launch (1 July 2011) to cancellation (18 Aug 2011) is the decisive fact. The $99 fire sale producing a demand spike large enough to briefly make the TouchPad a top-selling US tablet shows the failure was price and hardware, not the OS. PCWorld's postmortem documents the concrete developer failures: no SDK for months after announcement, no paid apps until August 2009, and Sprint exclusivity held so long that by the time the Pre reached Verizon, Sprint had switched its marketing to the HTC EVO. webOS 2.0's Node.js service layer and the migration of the card metaphor into iOS, Android and Windows Phone establish the technical claim.

Tags: hardware tied to dying platform, technical shortfall, strategic mismanagement, no hardware channel, poor developer experience, niche capture

Sources:
- https://en.wikipedia.org/wiki/WebOS
- https://www.pcworld.com/article/482038/webos_what_went_wrong.html
- https://thenextweb.com/insider/2011/08/18/hp-announces-it-will-discontinue-touchpad-stop-webos-device-development/
- https://en.wikipedia.org/wiki/HP_TouchPad
- https://www.osnews.com/story/21754/palm-sold-370000-pre-phones-in-may-june/

## No inherited software (14)

### Amoeba
*1983 (V1.0 prototype) – 1996 (final release 5.3, 30 July 1996) · Andrew S. Tanenbaum, Vrije Universiteit Amsterdam and CWI, with Frans Kaashoek, Sape Mullender, Robbert van Renesse, Henri Bal, Leendert van Doorn and Kees Verstoep*

**What was brilliant.** The processor-pool model: there is no 'your machine'. All CPUs in the building are a fungible pool, and running a command allocates whatever processors are free, with the OS load-balancing transparently — so a parallel build used the whole department. Underneath sat sparse capabilities: unforgeable object references carrying a cryptographic check field, so that every object in the system (file, process, directory, device, service) had one network-wide, self-authenticating name that could be passed around without any central authority or ACL lookup. FLIP, its purpose-built network protocol, kept addressing correct across process migration, and the Bullet file server stored whole files contiguously and immutably, which removed fragmentation and made whole-file reads close to raw-disk speed. It was a coherent, complete answer to 'what if the network were the computer' — five years before that became a slogan.

**What happened.** Development at VU ceased after version 5.3 in July 1996. Commercial distribution rights had been granted to ACE Associated Computer Experts in 1991, but no product came of it. The system's most durable legacy is accidental: Guido van Rossum built Python at CWI as a scripting language for the Amoeba project, because writing Amoeba administration tools in C or in Bourne shell was intolerable. Python outlived the operating system it was built for by thirty years and counting. The research group's people — Kaashoek to MIT, van Renesse to Cornell, Bal and Tanenbaum at VU — went on to shape distributed systems research broadly; Tanenbaum's own next system, MINIX 3, went a different route entirely.

**Why it failed.** The processor-pool model presupposed that CPU cycles were scarce and centrally poolable; between 1985 and 1995 the personal workstation and then the commodity PC made per-user compute abundant, so the single most valuable thing Amoeba sold — access to someone else's idle cycles — stopped being scarce before the system was finished.

**What might have changed it.** If VU or ACE had repositioned Amoeba around 1993 as a cluster and batch-scheduling fabric layered on commodity UNIX rather than as a replacement operating system, the processor-pool scheduler and capability naming would have had a real market — the one that Condor, Beowulf, and eventually Borg/Mesos/Kubernetes took. The scheduling insight was right; bundling it with a whole OS made it unadoptable.

**Evidence.** Final release 5.3, 30 July 1996; development at VU ceased thereafter. Commercial rights granted to ACE in 1991 produced no shipped product in five years. 48-bit port numbers as network-wide thread addresses and capability-based object naming are documented in Tanenbaum and Sharp's overview and in the 1990 CACM 'Experiences with the Amoeba Distributed Operating System'. Python's origin as the Amoeba project's scripting language is the project's own historical record.

Tags: no app ecosystem, no hardware channel, business model failure, incumbent lockin, compatibility gap

Sources:
- https://en.wikipedia.org/wiki/Amoeba_(operating_system)
- https://www.cs.vu.nl/pub/amoeba/Intro.pdf
- https://www.cs.vu.nl/~ast/Publications/Papers/compcom-1991.pdf
- https://dl.acm.org/doi/10.1145/96267.96281
- https://www.scs.stanford.edu/nyu/03sp/sched/amoeba.pdf

### Apollo Domain/OS (Aegis)
*1981–1992 (AEGIS SR1 on 27 March 1981; Domain/OS SR10.4.1.2 in March 1992; HP support ended 1 Jan 2001) · Apollo Computer — William Poduska and colleagues, Chelmsford, Massachusetts, founded 1980*

**What was brilliant.** The most architecturally ambitious workstation OS ever shipped in volume. Domain/OS took Multics' single-level store and dynamic linking and applied them across a network: objects were mapped into a machine-wide address space and demand-paged over the wire from other nodes' disks, so a remote file was not 'accessed' but mapped — there was no local/remote distinction in the filesystem at all, unlike NFS. A typed, object-oriented filesystem with user-extensible type managers. Diskless boot as a first-class mode; the network was structural, not optional — even a standalone Apollo could not be configured without a network card. Apollo Token Ring ran at 12 Mbit/s over RG-6U with 100+ nodes per network while Ethernet was 10 Mbit/s and shared. The Display Manager gave every window an unbounded, never-truncated, directly editable transcript with one uniform editing language across all windows — still better than any terminal emulator today. DSEE, Apollo's configuration-management system, is the direct ancestor of Rational ClearCase, and Apollo's NCS RPC became the basis of OSF DCE and fed into both CORBA and MS-RPC. Most of it was written in Pascal.

**What happened.** Apollo was the largest network workstation manufacturer from 1980 to 1987, holding at the end of 1986 the largest worldwide share of the engineering workstation market at roughly twice the share of second-place Sun, with quarterly sales passing $100M for the first time in late 1986. By the end of 1987 it had fallen to third, behind DEC and Sun. Domain/OS SR10 (1988) belatedly let customers install AEGIS, System V and BSD environments side by side. HP acquired Apollo in 1989 for $476M; Apollo product sales fell from $550M in 1989 to $360M in 1990, and HP wound the line down over 1990–97, replacing it with PA-RISC and HP-UX. HP support for Domain/OS ended 1 January 2001.

**Peak adoption.** largest worldwide share of the engineering workstation market at end-1986, roughly twice second-place Sun's share; first $100M quarter in late 1986; Apollo product revenue of $550M in 1989 falling to $360M in 1990.

**Why it failed.** Apollo's brilliance was inseparable from its proprietary stack, so when the CAD and EDA ISVs that defined its own market standardised on Unix, Ethernet and NFS between 1985 and 1988, Domain/OS could only offer Unix as an emulation layer (Domain/IX) rather than as the system itself — and Apollo lost the application base before it could ship the SR10 three-environment fix in 1988.

**What might have changed it.** Ship SR10's native BSD and System V environments in 1985 rather than 1988, or publish the Domain protocols the way Sun published NFS in 1984. Sun's decisive competitive act was giving away a protocol; Apollo kept ATR and Domain closed, and 'open systems' became the purchasing criterion against it.

**Evidence.** Market leadership 1980–87, 2x Sun's share at end-1986, third place by end-1987 behind DEC and Sun and ahead of HP and IBM; sales $550M → $360M from 1989 to 1990; HP paid $476M in 1989 and shut the line down over 1990–97; Domain/OS support ended 1 Jan 2001. The structural clue to the strategy trap: 'even a standalone Apollo machine cannot be configured without a network card,' i.e. the proprietary network was the architecture, not an add-on.

Tags: compatibility gap, strategic mismanagement, acquired and killed, no app ecosystem, hardware tied to dying platform

Sources:
- https://en.wikipedia.org/wiki/Apollo_Computer
- https://en.wikipedia.org/wiki/Domain/OS
- https://en.wikipedia.org/wiki/Apollo/Domain
- https://hackaday.com/2024/08/04/apollo-computer-the-forgotten-workstations/

### Bada
*2009–2013 (announced December 2009; Wave S8500 shipped 24 May 2010; Tizen merger announced June 2012; development ceased 25 Feb 2013) · Samsung Electronics*

**What was brilliant.** Bada's genuinely unusual architectural idea was a kernel-agnostic platform layer: the same C++ application framework and service layer ran unchanged on either Mentor Graphics' Nucleus RTOS or a Linux kernel, in a four-layer stack (kernel, device, service, framework). That was the correct engineering answer to the real problem of 2010 — Android needed too much RAM and flash to reach sub-$150 handsets — because it let one application platform scale from RTOS-class feature-phone silicon up to mid-range smartphone silicon while keeping app compatibility across the whole range. Bada also shipped a real first-party service layer that Samsung operated itself (push notification, in-app purchase, mapping, social), a complete C++ SDK with an on-device simulator, and in Bada 2.0 (Dec 2011) HTML5 support and multitasking. And it sold: 4.5 million units in a single quarter.

**What happened.** Bada was Samsung's hedge against dependence on Google, and commercially it worked better than most people remember — Canalys counted 3.5 million units in Q1 2011 and 4.5 million in Q2 2011, and Gartner put its peak at 3.0% of global smartphone sales in Q3 2012 (5.2 million units), at points ahead of Windows Phone, with 8.8% share in France in early 2011. But it never acquired a third-party app ecosystem: developers had no reason to learn a proprietary C++ framework for a platform present only in Western Europe and Korea, and Samsung put its own marketing behind Galaxy/Android. Samsung announced in June 2012 that Bada would be folded into Tizen, ended development on 25 February 2013, and closed bug reporting in April 2014. Share fell to 0.4% by Q2 2013.

**Peak adoption.** 4.5 million units shipped in Q2 2011 (Canalys, up 355% year on year); peak 3.0% of global smartphone sales and 5.2 million units in Q3 2012 (Gartner); 8.8% share in France, 7.8% in Germany in the twelve weeks to April 2011.

**Why it failed.** Bada was a negotiating position rather than a product bet — Samsung built it to retain leverage over Google — so once Galaxy Android succeeded beyond expectation the company had no incentive to fund the app catalogue that would have made Bada a genuine alternative, and the platform was starved by its own owner while still growing.

**What might have changed it.** If Samsung had put Bada rather than Android on its entire low-end and emerging-market line from 2011 and paid developers to populate that catalogue, it would have held real leverage in the 2012–2014 Google negotiations and owned the segment where volume growth actually was. Samsung eventually tried exactly this with Tizen and the Z1 in 2015 — five years too late, when the sub-$100 Android app catalogue was already complete.

**Evidence.** The shipment figures show Bada was not failing on demand: 3.5m (Q1 2011) → 4.5m (Q2 2011) → 5.2m and 3.0% share (Q3 2012), with double-digit regional share in France. The failure is visible in the decision, not the numbers: Samsung announced the Tizen merger in June 2012, one quarter before Bada's peak, and Samsung's own marketing spend went to Galaxy throughout. The kernel-configurable architecture is documented in Samsung's own platform description (Nucleus RTOS or Linux under one C++ framework).

Tags: no app ecosystem, strategic mismanagement, internal politics, business model failure, poor developer experience, too late to market

Sources:
- https://en.wikipedia.org/wiki/Bada_(operating_system)
- https://www.pocketgamer.com/news/samsung-bada-shipments-up-355-to-4-5-million-units-in-q2-2011/
- https://www.linux-magazine.com/Online/News/Bada-New-Mobile-Platform-from-Samsung
- https://www.techradar.com/news/software/operating-systems/samsung-won-t-launch-any-more-bada-or-tizen-phones-this-year-1093314
- https://commsbusiness.mabdev.co.uk/content/news/bada-gaining-traction-across-europe

### EROS (Extremely Reliable Operating System)
*1991-2005 (begun 1991 at the University of Pennsylvania; moved to Johns Hopkins with Shapiro in 2000; development halted 2005 in favour of Coyotos) · Jonathan S. Shapiro with Jonathan M. Smith and David J. Farber, University of Pennsylvania, then Johns Hopkins University; funded by DARPA, AFRL and NSF*

**What was brilliant.** EROS destroyed the received wisdom that capability systems are inherently slow -- a belief the paper notes was 'largely justified by experience' from the Intel iAPX 432. Measured against Linux on the same hardware in the SOSP'99 paper: pipe latency 5.66 microseconds versus Linux's 8.34; pipe bandwidth 281 MB/s versus 260; process creation 0.664ms versus 1.92ms; context switch 1.19 microseconds versus 1.26. The starkest result was heap growth, where EROS took 3.67 microseconds per page against Linux's 687. It delivered all this while providing transparent orthogonal persistence -- consistent system-wide snapshots taken without any application involvement. And its confinement mechanism received a machine-checked formal proof (Shapiro and Weber, 2000): a real security property proved about a real running system, nine years before seL4.

**What happened.** Development stopped in 2005. The trigger was technical honesty, not neglect: in 2003 the group identified serious security problems inherent to synchronous-IPC kernel designs, affecting EROS and the entire L4 family alike, and concluded the architecture needed replacing rather than patching. Effort moved to Coyotos; the EROS code base passed to Charles Landau as CapROS. EROS itself was never deployed outside research.

**Why it failed.** EROS proved capabilities could be fast but never built a path for existing software to run on it, so the only way to use the system was to rewrite your application in an unfamiliar programming model -- and outside the research group nobody had a reason to pay that cost.

**What might have changed it.** If EROS had shipped a credible POSIX personality -- which its own ancestor KeyKOS had demonstrated was a six-month job for one developer with KeyNIX -- rather than restarting the kernel as Coyotos in 2004, it could have accumulated real users during the 2000s wave of Linux security hardening, when there was active appetite for a better isolation story.

**Evidence.** The SOSP'99 benchmark table is the load-bearing evidence that the capability performance objection was empirically false, and the paper's own framing is careful: the results 'suggest that capabilities are a reasonable substrate for a high-performance, high-security system.' The failure is not technical. It is that fourteen years of DARPA and AFRL funding produced a system with no applications, and that the group's response to the 2003 synchronous-IPC discovery was to start over rather than to ship. The EROS lineage -- GNOSIS to KeyKOS to EROS to CapROS and Coyotos -- is a 35-year sequence of restarts in which no generation ever reached a shippable product, and each restart discarded the accumulated userland of the previous one.

Tags: no app ecosystem, compatibility gap, perpetual rewrite, funding collapse, poor developer experience

Sources:
- https://flint.cs.yale.edu/cs428/doc/eros.pdf
- https://dl.acm.org/doi/10.1145/319344.319163
- https://en.wikipedia.org/wiki/EROS_(microkernel)
- https://www.cs.jhu.edu/seminars/2000/spring/may24/jonathan-s-shapiro
- http://thomas.enix.org/pub/rmll2005/rmll2005-shapiro1.pdf

### Exokernel (Aegis, XOK/ExOS, Cheetah)
*1994–2000; Aegis/ExOS at SOSP '95 (December 1995), XOK/ExOS and Cheetah at SOSP '97, Engler's thesis 1998 · MIT Parallel and Distributed Operating Systems group — Dawson R. Engler, M. Frans Kaashoek, James O'Toole Jr., with Gregory Ganger, Héctor Briceño, Russell Hunt, David Mazières and Thomas Pinckney*

**What was brilliant.** The cleanest implemented statement of the separation of protection from management. The exokernel securely multiplexes raw hardware and does nothing else: it exports disk blocks, physical memory pages, TLB entries and packet filters directly to applications, protected by secure bindings, and every abstraction — file system, page table, IPC, process structure — becomes library code linked into the application and therefore replaceable per application. The measured results are what make it serious rather than merely elegant. Aegis' protected control transfer and exception dispatch ran an order of magnitude faster than Ultrix's equivalents. XOK/ExOS ran unmodified UNIX applications at performance comparable to mature BSD systems, so there was no general-case penalty. And the specialised case won enormously: Cheetah, a web server that used a custom on-disk layout and merged the file system with TCP so it could pre-compute checksums and transmit file data straight out of the disk cache, achieved roughly a factor of eight improvement over the best UNIX servers of the day.

**What happened.** Never commercialised. Exokernels remain a research effort and have not been used in any major commercial operating system. The model was absorbed rather than adopted: the library-OS idea reappeared as unikernels (MirageOS, OSv, IncludeOS), as Microsoft's Drawbridge library OS and the Windows Subsystem for Linux picoprocess, and as Xen's paravirtualized split of protection from management; 'BPF for storage' and similar work still identify explicitly as exokernel-inspired. The people scattered into work of enormous influence elsewhere — Engler into static bug-finding and Coverity, Mazières into SFS and Stellar, Ganger to CMU's Parallel Data Lab, Kaashoek remaining at MIT.

**Why it failed.** The exokernel's benefit accrues only to applications willing to implement their own operating system, and almost no application team will ever pay that cost — so the model delivers its gains exclusively where one application owns the whole machine, which is precisely the virtualization and unikernel niche it was later absorbed into under other names.

**What might have changed it.** If MIT had shipped XOK as a hosted library-OS layer — the Xen or Drawbridge framing, running beneath or beside a conventional system rather than replacing it — it would have had a hardware channel and a real adoption path in 1998, instead of waiting a decade for Xen to rediscover the architecture from the other direction and take the cloud with it.

**Evidence.** SOSP '95: 'Exokernel: an operating system architecture for application-level resource management'; the Aegis prototype showed protected control transfer and exception dispatch an order of magnitude faster than Ultrix. SOSP '97 ('Application Performance and Flexibility on Exokernel Systems'): 'Common unmodified UNIX applications performed comparably on Xok/ExOS and BSD UNIXes, while customized applications benefited substantially from control over their resources (e.g., a factor of eight improvement for a Web server).' That pairing — no loss on the general case, 8x on the specialised one — is the strongest quantitative result any of these research systems produced, and it still did not produce a product, which is the point. As of 2024 exokernels 'have not been used in any major commercial operating systems.'

Tags: no app ecosystem, poor developer experience, incumbent lockin, no hardware channel

Sources:
- https://dl.acm.org/doi/10.1145/224057.224076
- https://www.cs.columbia.edu/~nieh/teaching/e6118_s00/papers/kaashoek_exo-sosp97.pdf
- https://courses.cs.washington.edu/courses/cse551/17wi/readings/exokernel-sosp97.pdf
- https://users.ece.cmu.edu/~ganger/papers/exo-sosp97/exo-sosp97.pdf
- https://citeseerx.ist.psu.edu/document?repid=rep1&type=pdf&doi=5f11b6bd3f7dcb892b226ec734730081d5716c55

### Genode / Sculpt OS
*Bastei architecture work at TU Dresden from the mid-2000s; first Genode release 2008; 'Sculpt for Early Adopters' announced in release 18.02 (February 2018); still shipping quarterly, release 26.05 on 29 May 2026 · Norman Feske and Christian Helmuth, TU Dresden → Genode Labs GmbH, Dresden, Germany*

**What was brilliant.** Genode is the only project that has made a capability microkernel usable as a whole general-purpose desktop, and it does it by making the operating system recursive. Every component runs in its own sandbox, is created by a parent out of the parent's own budget, and inherits only the rights the parent explicitly hands over — so there is no ambient authority anywhere in the system and no global root account to compromise. That same structure repeats at every level, which makes trust relationships explicit and auditable from the boot loader down to an individual window. It is also kernel-agnostic in a way nothing else is: the identical component tree runs on seL4, NOVA, Fiasco.OC, OKL4 v2.1, L4Ka::Pistachio, on Linux as ordinary processes, or on Genode's own base-hw kernel — which makes it the only practical route to actually doing work on a formally verified kernel. Sculpt surfaces all of this to the user as a live, editable component graph: you can see the system's structure and rewire it while it runs, hand a USB device to one VM and the network to another, and run Linux or Windows guests as just more sandboxed components.

**What happened.** Alive, funded, self-hosting, and with no measurable user base. Eighteen years of quarterly releases; Sculpt is the Genode Labs team's own daily-driver OS; it runs on commodity x86-64 PCs, the PinePhone and the MNT Reform. Genode Labs remains a small consultancy funding development through commercial licences (AGPL-3.0 or proprietary) and contract engineering for embedded and high-assurance customers, not through Sculpt users. It is the clearest modern instance of a technically complete microkernel desktop with no distribution channel: no OEM preloads it, no app store targets it, and every application must be ported or run inside a VM.

**Why it failed.** Sculpt is a complete operating system with no applications and no hardware channel, so its capability architecture — which is its entire value — is invisible to anyone who would first have to abandon their existing software to experience it; the security property cannot be demonstrated to a user who cannot get their work done on the machine.

**What might have changed it.** If Genode had shipped Sculpt preinstalled on one specific, fully supported device with a curated application set, it would have the user base its eighteen years of engineering deserve. That is exactly the strategy that worked for Qubes OS through Purism and Nitrokey and for GrapheneOS through the Pixel line — a single guaranteed-working hardware target plus a known-good software set. Sculpt instead ships as an ISO for hardware that may or may not have drivers, which is the same distribution posture that has failed every microkernel desktop before it.

**Evidence.** Genode's own framing: 'a tool kit for building highly secure special-purpose operating systems', combining 'the construction principles of L4 with Unix philosophy', in which 'each program is executed in a dedicated sandbox and gets granted only those access rights and resources that are required to fulfill its specific purpose.' Sculpt's first release, in Genode 18.02, named its audience precisely: 'Sculpt for Early Adopters (EA)... Its target audience are enthusiasts who are already familiar with Genode and are eager to use a Genode-based operating system on their machines' — an explicit acknowledgement, at launch, that the addressable market was existing Genode developers. Eight years later the description is unchanged in substance: 'used as day-to-day OS by the Genode developers.' Supported kernels span NOVA, seL4, Fiasco.OC, OKL4 v2.1, Pistachio, Linux and base-hw, across x86 32/64-bit, ARM 32/64-bit and RISC-V 64.

Tags: no app ecosystem, no hardware channel, niche capture, business model failure, poor developer experience

Sources:
- https://genode.org/about/index
- https://en.wikipedia.org/wiki/Genode
- https://genode.org/download/sculpt
- https://genode.org/documentation/release-notes/18.02
- https://genode.org/news/sculpt-os-release-26.04

### Lindows / Linspire
*2001-2008 (revived as a brand by PC/OpenSystems from 2018) · Michael Robertson (MP3.com founder), Lindows.com Inc., San Diego*

**What was brilliant.** Click-N-Run (CNR), a one-click, GUI-driven software store with an account-based catalogue that grew past 2,400 titles by 2006. It predated consumer app stores by about six years. Lindows was also the first Linux preloaded on mass-retail PCs: Walmart.com sold $199.86 Microtel VIA C3 desktops with LindowsOS from June 2002. The original plan was to run Windows apps through Wine behind a Windows-like shell.

**What happened.** Microsoft sued over the trademark in December 2001. It lost two injunction requests in the US, then in July 2004 paid about $20M ($15M up front, $5M on handover of domains) to settle, and the company renamed itself Linspire. Robertson stepped down as CEO on 15 June 2005 and Kevin Carmony replaced him. In July 2008 Xandros bought the assets, mainly for CNR, and discontinued Linspire. PC/OpenSystems bought the name in 2018 and sells a small revival.

**Why it failed.** Lindows sold itself on running Windows software and never delivered it at consumer quality. It then tried to charge subscription fees for installing free software. Buyers of $199 PCs got neither Windows compatibility nor a reason to pay, so the retail channel it had won never grew.

**What might have changed it.** Drop the Windows-compatibility promise early. Position CNR as a free, OEM-bundled app store funded by commercial app revenue share, and keep the Walmart and Microtel style preload deals going at scale until the store earned enough.

**Evidence.** The popular story says Microsoft's lawsuit killed Lindows. The record says otherwise: Lindows won the key US pretrial rulings, and Microsoft paid Lindows about $20M to settle (Microsoft press release, 19 July 2004). The company kept running for four more years. The real problems were the product and its pricing. The Wine-based Windows compatibility behind the Lindows name was dropped as a goal in favour of Linux apps, and CNR originally needed a paid membership ($49.95, later $20 basic / $50 gold a year) until it went free in 2006. Xandros bought the company in 2008 mainly for CNR, not for the OS.

Tags: compatibility gap, business model failure, strategic mismanagement, no app ecosystem, licensing or legal, acquired and killed

Sources:
- https://www.computerworld.com/article/1464816/microsoft-to-pay-20m-to-end-lindows-trademark-battle.html
- https://news.microsoft.com/source/2004/07/19/microsoft-and-lindows-settle-trademark-case/
- https://en.wikipedia.org/wiki/Linspire
- https://en.wikipedia.org/wiki/Microsoft_Corp._v._Lindows.com,_Inc.
- https://www.theregister.com/2002/09/02/walmart_com_flogs_199_linux/

### Oberon / Native Oberon / A2 (Bluebottle)
*1985–1989 design and first implementation; V1 1987, V2 and System 3 1991, V4 1992, Native Oberon mid-1990s, AOS 2002 → Bluebottle 2005 → A2 2008, Project Oberon 2013 edition · Niklaus Wirth and Jürg Gutknecht, ETH Zürich (originally on the in-house Ceres NS32032 workstation)*

**What was brilliant.** A complete garbage-collected, type-safe, modular operating system with compiler, GUI, editor, network stack and browser, designed and written by two people in about two years, and small enough that the full distribution fit on a single 1.44 MB floppy while the running system needed on the order of 200 KB. It had whole-system garbage collection and type-safe dynamic module loading in 1987, a decade before Java. The text user interface erased the CLI/GUI boundary: any visible text of the form Module.Command was executable by middle-click, so documents, logs and error messages were all live command surfaces. Later work produced slim binaries (Franz and Kistler) — compressed portable syntax trees code-generated at load time, portable across x86, 68K and PowerPC, which is what Java bytecode and .NET IL reinvented less elegantly — and Active Oberon/A2, which put active objects and SMP into the language itself. Project Oberon 2013 reimplements the whole stack including its own RISC processor on an FPGA; the entire system compiles itself in under ten seconds.

**What happened.** Adopted as ETH's own workstation and teaching environment and given serious trade-press attention (BYTE 1991, 1993 and 1995; c't 1994), but never commercialized. V4 was effectively orphaned around 2000; Native Oberon and A2 continue with a small group of maintainers; popularity declined sharply after 2000. Wirth's 1995 IEEE Computer article 'A Plea for Lean Software' turned Oberon into the standing empirical argument against software bloat — which is now its main cultural role.

**Why it failed.** Oberon's leanness was purchased precisely by refusing compatibility — no existing binaries, no C FFI worth the name, no POSIX — so adopting it meant rewriting your entire software world, a price only its own authors' department was ever willing to pay.

**What might have changed it.** Ship Oberon as a hosted, sandboxed application runtime with a real C foreign-function interface instead of as a whole operating system. That is functionally what Java did in 1995 with worse GC, no slim binaries and no type-safe dynamic linking story to speak of; Oberon had the better technology and chose the harder distribution problem.

**Evidence.** Wirth, 'A Plea for Lean Software' (IEEE Computer 28(2), Feb 1995): the Oberon system needed roughly 200 KB including editor and compiler, against workstation software then jumping 'from several to many megabytes' per release; his stated lesson, 'The belief that complex systems require armies of designers and programmers is wrong.' The full Native Oberon distribution — compiler, utilities, web browser, TCP/IP and GUI — fits on one 3.5-inch floppy. Project Oberon 2013 on a Spartan-3 FPGA: 'whole system compiles in less than 10 seconds.' Slim binaries produced portable object code across Intel x86, Motorola 68K and PowerPC.

Tags: no app ecosystem, compatibility gap, no hardware channel, business model failure, niche capture

Sources:
- https://en.wikipedia.org/wiki/Oberon_(operating_system)
- https://people.inf.ethz.ch/wirth/ProjectOberon1992.pdf
- https://dl.acm.org/doi/10.1109/2.348001
- https://en.wikipedia.org/wiki/Wirth's_law
- https://en.wikipedia.org/wiki/Ceres_(workstation)

### PC/GEOS (GeoWorks Ensemble)
*1990–2009 (PC/GEOS 1.0, Nov 1990; Ensemble 2.0, 1993; NewDeal Office 1996–2000; Breadbox Ensemble 4.1.3, Aug 2009) · Berkeley Softworks / GeoWorks Corporation — Brian P. Dougherty*

**What was brilliant.** An object-oriented GUI operating environment written largely in hand-tuned 8086 assembly with a custom object system — GOC (GEOS Object C), the Espire assembler, and an interpreted bytecode called IZL — which is how it delivered preemptive multitasking, scalable outline fonts, dynamic linking and a complete application suite on an 8088 or 286 with 512 KB of RAM, on machines that could not run Windows 3.0 at all. The object model gave it per-object memory-managed resources and a genuine separation between document model and interface, so the same application could re-render its UI for a desktop, a handheld or a phone screen. That is why it, and not Windows, ended up inside the Nokia 9000 Communicator.

**What happened.** Reviews were strong but third-party application developers never arrived, and GeoWorks stopped marketing PC/GEOS for the desktop in late 1993 after Windows 3.1. It survived as licensed OEM technology: America Online's graphical MS-DOS client ran on it through the 1990s; NEC and Sony bundled it as CD Manager in 1992; it shipped in the Tandy/Casio Zoomer (1993), the HP OmniGo 100/120 (1995), and as PEN/GEOS 3.0 in the Nokia 9000 Communicator, which began shipping in Europe on 15 August 1996 after Nokia invested $7.5 million in GeoWorks in February 1995. GeoWorks IPO'd in 1994 (1.5 million shares), licensed Ensemble to NewDeal in 1996, sold its UK operation to Teleca and filed for bankruptcy protection in the US in 2003; Breadbox carried the code to 2009.

**Why it failed.** PC/GEOS was explicitly aimed at the machines Windows could not run, and by 1992–93 those machines were being replaced rather than upgraded — so an operating system targeted at obsolete hardware inherited that hardware's obsolescence, while its hand-tuned assembly object framework was so unlike anything mainstream developers knew that no native application base ever formed.

**What might have changed it.** Exposing the object framework through a mainstream C++ toolchain in 1991 instead of requiring GOC and 8086 assembly — or shipping a Windows-binary compatibility layer — gives the platform third-party applications. Alternatively, pivoting wholesale to handhelds in 1992 rather than 1994 puts GEOS into the PDA market ahead of Palm OS, which is the market the technology was actually best suited to.

**Evidence.** GeoWorks pitched the '16 million older-model PCs that were unable to run Microsoft Windows 2.x' — an explicitly shrinking addressable market, stated as the strategy. A former GeoWorks CEO claimed Microsoft threatened to withdraw MS-DOS supply from hardware makers that bundled GeoWorks. The niche capture is documented: Nokia's $7.5M investment in February 1995 and the Nokia 9000's European launch on 15 August 1996, with GEOS 3.0 as the Communicator's OS — the first mass-market smartphone ran the OS that lost the desktop.

Tags: no app ecosystem, incumbent lockin, poor developer experience, too late to market, niche capture, business model failure

Sources:
- https://en.wikipedia.org/wiki/GEOS_(16-bit_operating_system)
- https://en.wikipedia.org/wiki/Berkeley_Softworks
- https://en.wikipedia.org/wiki/Nokia_9000_Communicator
- https://mgroeber.de/nathan/news96/nok20815.html
- https://tidbits.com/1990/10/15/geoworks-ensemble/

### Phantom OS
*Design dating to the late 1980s by its author's account; public implementation from around 2010; last release 17 October 2019; Genode-based fork started 2020 · Dmitry Zavalishin, Digital Zone (Russia), with Innopolis University*

**What was brilliant.** Phantom carries orthogonal persistence to its logical conclusion: there is no filesystem and no save operation. The managed object heap is the persistent store, snapshotted atomically to disk, so a power failure leaves every running program mid-statement and it resumes after reboot as though nothing happened — applications 'continue their work after the next OS boot up as if no shutdown ever happened.' Because all code is bytecode running in a managed heap with no pointer arithmetic, there are no raw pointers to invalidate across a snapshot, which is the precise technical reason earlier persistent-store designs (Multics, KeyKOS/EROS, Grasshopper) could never make it general. It is the most complete implementation of an idea the field has been circling since the 1960s.

**What happened.** Still an alpha. The last release is 17 October 2019, IA-32 only; ARM, MIPS and x86-64 ports were begun and not finished. A Genode-based fork started in 2020, restarting much of the work on a different substrate. No commercial deployment and no documented production use.

**Why it failed.** Orthogonal persistence only pays off when the entire software world lives inside the persistent heap, so Phantom must reimplement every application a user wants before its single advantage becomes visible — an unbounded amount of work for a small team, which is why it has been alpha for over a decade.

**What might have changed it.** Ship the persistent-object VM as a runtime library on Linux, so existing applications could opt into crash-consistent persistence incrementally, rather than requiring a new operating system before anyone can see the benefit. The Genode fork is a partial, late admission of this; doing it first would have let the one genuinely novel idea be adopted on its own merits.

**Evidence.** Persistent virtual memory is 'implemented so that an abrupt computer failure or loss of power leaves the system in a coherent state, with applications continuing their work after the next OS boot up as if no shutdown ever happened.' Principle: 'Everything is an object,' against Unix's 'Everything is a file.' Latest release 17 October 2019, alpha, IA-32 only, with ARM/MIPS/x86-64 ports incomplete; Genode-based fork begun 2020.

Tags: no app ecosystem, compatibility gap, perpetual rewrite, no hardware channel, technical shortfall

Sources:
- https://en.wikipedia.org/wiki/Phantom_OS
- https://github.com/dzavalishin/phantomuserland/wiki/PhantomArchitecture
- http://phantomos.org/
- https://phantomdox.readthedocs.io/en/latest/

### Rhapsody
*1996–1999 · Apple Computer, on NeXT's OPENSTEP for Mach*

**What was brilliant.** Unlike everything else on this list, Rhapsody was a working modern OS in 1997, not a plan for one: a Mach 2.5/BSD kernel with protected memory and preemptive multitasking, the OpenStep object frameworks (Foundation and the Application Kit) that are still the basis of every Apple platform today, Display PostScript for a unified screen-and-print imaging model, and a Blue Box running unmodified Mac OS 8 at near-native speed. It ran on both PowerPC and IA-32 simultaneously, and Yellow Box also ran on Windows NT 4.0, which would have made Cocoa a genuinely cross-platform application runtime.

**What happened.** Announced by Amelio at Macworld, 7 January 1997. Developer Release 1 shipped 13 October 1997 to about 10,000 developers (PowerPC first, Intel weeks later); DR2 on 14 May 1998. At WWDC in May 1998 Apple announced Carbon and abandoned the plan to make OpenStep the sole forward API. Rhapsody shipped instead as Mac OS X Server 1.0 on 16 March 1999, ending with 1.2 on 14 January 2000. Yellow Box for Windows was cancelled. The kernel, frameworks and Blue Box all resurfaced in Mac OS X 10.0 in March 2001, three years late.

**Why it failed.** Rhapsody-as-planned died because it required the entire Mac software industry to rewrite from scratch in an API none of them knew, and the developers Apple most needed simply refused — Apple had no leverage to compel a rewrite from companies whose Windows business was larger than their Mac business, so it had to invent Carbon and delay the transition by three years.

**What might have changed it.** If Apple had announced Carbon — the Mac Toolbox API reimplemented natively on the new kernel — at WWDC 1997 alongside Rhapsody instead of waiting until WWDC 1998, the consumer Mac OS transition happens in 1998–99 rather than 2001, and Apple ships a modern OS while the Mac installed base is still eroding rather than after it has bottomed.

**Evidence.** Mac developers named Blue Box 'the penalty box' because their codebases would be frozen inside an emulator that would never be updated. Microsoft and Adobe 'balked outright, and refused to consider porting to OpenStep.' By WWDC 1998, not a single major third-party developer had committed to rewriting for Rhapsody — after fifteen months of evangelism and a developer release in 10,000 hands. Apple's response was to build Carbon, which is Apple conceding the developers' argument in full. The technology was never the problem: the identical kernel and frameworks shipped successfully in 2001 once a compatibility API existed.

Tags: no app ecosystem, compatibility gap, strategic mismanagement, poor developer experience, niche capture

Sources:
- https://en.wikipedia.org/wiki/Rhapsody_(operating_system)
- https://tidbits.com/1998/05/18/mac-os-x-rhapsody-a-mac-developer-could-love/
- https://apple.fandom.com/wiki/Worldwide_Developers_Conference_1998
- https://betawiki.net/wiki/Mac_OS_X_Server_1.x
- https://apple.fandom.com/wiki/Carbon

### Singularity
*9 July 2003 – 7 February 2015; RDK 1.1 released 4 March 2008, RDK 2.0 (final) 14 November 2008 · Microsoft Research (Galen C. Hunt and James R. Larus)*

**What was brilliant.** Singularity replaced the memory management unit with the type system and got away with it. Software-Isolated Processes run in a single address space, in ring 0, with no hardware protection domain at all; isolation is established by language safety rules and enforced by static verification of the intermediate code at install time, so a process switch costs roughly a function call rather than a TLB flush. Processes communicate only over bidirectional channels whose protocols are declared as explicit state machines in Sing#, and conformance is checked statically — so a message-protocol violation is a compile error rather than a runtime crash. No memory is shared between SIPs, so each has its own heap and its own garbage collector and a failure cannot corrupt a neighbour. Programs are sealed and described by a manifest, which makes the composed system statically analysable: Singularity could prove properties about a whole running configuration that no conventional OS can even express.

**What happened.** Two Research Development Kit releases under a shared-source licence in 2008, and then nothing further. The codebase and its design were carried forward into Midori, Microsoft's attempt to turn the research into a product; Microsoft shipped neither. The project page was formally closed on 7 February 2015. Its concepts resurfaced indirectly — in .NET's Span<T> and ref structs, nullable reference types and async/await, in the broader safe-systems-language argument that Rust went on to win, and in the component-model thinking behind WebAssembly.

**Why it failed.** Singularity's isolation guarantee holds only if all code is type-safe and statically verifiable, which excludes by construction the entire installed base of native Windows software — so the only way to ship it was to ask the world to rewrite everything, and Microsoft's whole strategic position was the opposite bet.

**What might have changed it.** If Singularity had been positioned as a verified-code sandbox inside Windows rather than as a replacement for it — the role later taken by Hyper-V, then WSL, and eventually WebAssembly — it would have had a compatibility path and an incremental adoption story. As a whole-OS replacement written in a new language it had neither.

**Evidence.** Developed at Microsoft Research from 9 July 2003 to 7 February 2015. Kernel, device drivers and applications all ran in ring 0 in a shared address space with no hardware memory protection, relying on type safety and static analysis. Sing# (an extension of Spec#, itself C#-derived) carried the channel contracts; the Bartok compiler translated CIL to x86 at install time. Only two releases ever shipped, RDK 1.1 (4 March 2008) and RDK 2.0 (14 November 2008), both under a shared-source licence, both x86/x86-64 with a command-line interface only — no GUI, no driver breadth, no application story. The design then moved wholesale into Midori, which was also cancelled.

Tags: compatibility gap, no app ecosystem, internal politics, strategic mismanagement, incumbent lockin, niche capture

Sources:
- https://www.microsoft.com/en-us/research/project/singularity/
- https://en.wikipedia.org/wiki/Singularity_(operating_system)
- https://www.microsoft.com/en-us/research/wp-content/uploads/2016/02/osr2007_rethinkingsoftwarestack.pdf
- https://www.microsoft.com/en-us/research/wp-content/uploads/2005/10/tr-2005-135.pdf
- https://cs.uwaterloo.ca/~brecht/courses/702/Possible-Readings/oses/singularity-deconstructing-process-isolation-mem-system-perf-2006.pdf

### Tizen (mobile)
*2011–2021 as a phone platform (announced Sept 2011 as MeeGo's successor; first release 30 April 2012; Samsung Z1 Jan 2015; Z4 in May 2017 the last; Tizen Store closed 31 Dec 2021) · Linux Foundation with Intel and Samsung, merging the MeeGo and LiMo efforts under the Tizen Association*

**What was brilliant.** Tizen's distinctive engineering bet was EFL — the Enlightenment Foundation Libraries — as the system toolkit. EFL's Evas canvas does its own retained scene-graph rendering with both software and GL backends and aggressive damage-region tracking, which makes it unusually good on weak hardware; Tizen ran a genuinely fluid UI in 512MB of RAM with a memory and storage footprint Android could not match at the time. The Samsung Z1 shipped in January 2015 at ₹5,700 and was measurably faster in use than equivalently priced Android One devices. Tizen also offered both an HTML5/web app model and a native C/C++ API over a real Linux base with a well-documented SDK, and was architected as a genuine multi-profile OS — mobile, wearable, TV and in-vehicle infotainment — from a single source tree, which is why the TV and watch profiles survived when the phone profile did not.

**What happened.** The phone plan collapsed repeatedly before it started: NTT DoCoMo cancelled its planned Tizen handset in January 2014, Orange cancelled the same year, and the Samsung Z was pulled from its Russian launch in July 2014. Samsung finally shipped the Z1 in India in January 2015; it sold over a million units in six months across India, Sri Lanka and Bangladesh and reportedly took 23.4% of its segment in India in Q1 2015. But Samsung shipped only four Tizen phones ever — Z1, Z2, Z3, Z4 — stopped after the Z4 in May 2017, lost WhatsApp, Facebook, Instagram and Messenger support in 2020, and closed the Tizen Store on 31 December 2021. Tizen survives dominantly everywhere except phones: it is the world's most-installed smart TV operating system (about 12.9% of all smart TVs, roughly 120 million sets, in 2024 per CTVMA, rising toward 21% on 2025 figures), and it ran Samsung's Gear and Galaxy Watch line until Samsung merged it into Google's Wear OS in 2021.

**Peak adoption.** Over 1 million Samsung Z1 units sold in six months across India, Sri Lanka and Bangladesh (Jan–May 2015), with 500,000+ in India alone and a reported 23.4% segment share in Q1 2015; four phone models total. In its surviving niche: the #1 smart TV OS worldwide, roughly 120 million television sets in 2024.

**Why it failed.** Samsung never put Tizen on a phone it actually wanted to sell — the platform launched only in the lowest-margin segment of one geography, so no developer had a commercial reason to port, and a phone OS whose own creator will not ship it on a flagship cannot generate an app catalogue at any price.

**What might have changed it.** If Samsung had shipped one Galaxy S-class flagship on Tizen in 2014–2015 with the full Samsung marketing budget behind it, the volume would have forced app ports the way Samsung's Android volume forced them. Instead Tizen functioned as leverage in Samsung's 2014 cross-licensing negotiation with Google, and once that was settled Samsung stood down.

**Evidence.** The cancellations are the tell: DoCoMo pulled out in January 2014, Orange in 2014, and the Samsung Z was withdrawn from Russia in July 2014 — three carrier-side abandonments before a single Tizen phone reached retail. Only four models shipped in seven years. The Z1's million units in six months demonstrate the product was competent in market; the absence of a fifth model demonstrates the decision was internal. The profile architecture's soundness is confirmed by Tizen's continued #1 position in smart TVs at roughly 120 million sets.

Tags: no app ecosystem, strategic mismanagement, internal politics, too late to market, no hardware channel, niche capture

Sources:
- https://en.wikipedia.org/wiki/Tizen
- https://www.notebookcheck.net/Samsung-Z1-sales-exceed-one-million-units.145670.0.html
- https://www.gsmarena.com/samsung_z1_sales_cross_500000_mark_in_india-news-12639.php
- https://www.sammobile.com/2018/12/16/samsung-tizen-smartphones-possibly-discontinued
- https://m.gsmarena.com/samsung_shuts_down_the_tizen_app_store-news-52598.php

### Windows RT
*2012–2015 (support to 2023) · Microsoft (Windows Division under Steven Sinofsky)*

**What was brilliant.** A complete port of the Windows NT kernel, HAL, driver model and the full Win32 subsystem to 32-bit ARMv7 — done well enough that the desktop Office 2013 applications (Word, Excel, PowerPoint, OneNote, later Outlook) ran natively as recompiled Win32 desktop binaries at full fidelity on an NVIDIA Tegra 3. Rebuilding NT's entire API surface for a new ISA with a completely different power and memory model is genuinely hard systems work, and it is the direct ancestor of everything Windows on ARM is today: ARM64 Windows, the x86/x64 emulation layer, and Copilot+ PCs all descend from this port. Windows RT was also the first mainstream consumer OS shipped with mandatory, non-defeatable UEFI Secure Boot and kernel code integrity — technically the strongest security posture Microsoft had ever shipped to consumers.

**What happened.** Released 26 October 2012 with Surface RT. Only five third-party models ever shipped: Asus VivoTab RT (Oct 2012), Dell XPS 10 (Dec 2012), Lenovo IdeaPad Yoga 11 (Dec 2012), Samsung Ativ Tab (Dec 2012, UK only) and Nokia Lumia 2520 (Oct 2013). HP declined before launch in June 2012; Samsung cancelled its US release in January 2013; Asus quit in August 2013. Microsoft took a $900M inventory write-down in July 2013 after cutting Surface RT from $499 to $349. Sinofsky left Microsoft on 13 November 2012. Surface 2 and Lumia 2520 production ended by February 2015. Store purchases were disabled and extended support ended 10 January 2023.

**Peak adoption.** ~260,000 Surface RT units shipped in Q1 2013 (IDC), against ~750,000 Surface Pro. Total Windows RT devices across all OEMs were never separately disclosed; the $900M write-down is the best available proxy for the scale of unsold inventory.

**Why it failed.** Microsoft shipped an operating system that presented the full Windows desktop and carried the Windows name while forbidding every Windows application except its own, so buyers experienced it as a broken Windows rather than as a new platform — and because Microsoft reserved the sole exception (Office) for itself, no third-party developer had a path to the desktop and therefore no reason to port.

**What might have changed it.** If Microsoft had opened signed ARM desktop Win32 to third parties on the same terms it granted Office — one policy change, no engineering work, since the jailbreak proved recompiled ARM desktop apps ran fine — Windows RT would have had a real software catalogue on day one, and ARM Windows would have arrived a decade before it actually did.

**Evidence.** The January 2013 jailbreak demonstrated that third-party ARM desktop applications executed correctly and were blocked only by a code-signing level check, not by any technical limitation — the restriction was policy. OEM statements confirm the positioning failure: HP said customers 'felt Intel-based tablets were more appropriate for business use'; Samsung cited 'unclear positioning' and 'modest demand'; Asus called the platform 'not very promising'. Five OEM models total over the platform's life; $900M write-down in July 2013; Ballmer publicly described sales as 'modest'.

Tags: compatibility gap, strategic mismanagement, no app ecosystem, incumbent lockin

Sources:
- https://en.wikipedia.org/wiki/Windows_RT
- https://www.cbc.ca/news/business/microsoft-reveals-900m-write-down-on-surface-rt-tablet-1.1303871
- https://news.slashdot.org/story/13/01/07/1548223/windows-rt-jailbroken-to-run-third-party-desktop-apps
- https://www.theregister.com/2012/10/26/surface_review/
- https://www.theregister.com/2016/07/15/windows_fix_closes_rt_unlock_loophole/

## Out of time: perpetual rewrite or too late (13)

### BlackBerry 10 (QNX Neutrino)
*2011–2022 (QNX acquired by RIM April 2010; PlayBook OS April 2011; BB10 launched 30 Jan 2013; new APIs/SDKs halted 26 Oct 2015; services terminated 4 Jan 2022) · BlackBerry / Research In Motion, atop QNX Software Systems' Neutrino microkernel (Dan Dodge and Gordon Bell, 1980)*

**What was brilliant.** BB10 put a genuine, commercially hardened microkernel under a consumer smartphone. QNX Neutrino runs the filesystem, the networking stack and every device driver as ordinary user-space processes communicating by synchronous message passing, so a crashed graphics driver or media codec cannot take down the kernel and can be restarted in place. The kernel is small enough to be formally analysed and is certified to IEC 61508 SIL 3 and ISO 26262 ASIL D — it is the OS used in nuclear plant control rooms and surgical robots, and RIM shipped it in a phone you could buy at a carrier store. On top of it: BlackBerry Balance, which used the microkernel's process-isolation primitives to run two fully separated personas with separate filesystems and separate encryption on one device, years before Android Work Profiles or iOS managed apps; Cascades, a Qt-based hardware-accelerated declarative UI toolkit; a full POSIX userspace plus an Android runtime (API levels 10–18) so Android APKs could be repackaged; the Hub, a single unified inbox for every account reachable by an edge gesture from inside any running app; and a predictive keyboard that placed next-word candidates above the keys you would have to type, still one of the best soft keyboards ever shipped.

**What happened.** RIM shipped almost 1 million Z10 units in the three weeks of Q4 FY2013 the device was on sale, then took a $934 million inventory write-down in September 2013 on unsold Z10s, with a $965 million quarterly loss. BB10 shipments were 2.7 million in Q1 FY2014, against Android's roughly 200 million per quarter; BlackBerry's overall smartphone share was already down to 1.7% by Q3 2013 (IDC). Thorsten Heins was replaced by John Chen in November 2013; Chen pivoted the company to enterprise software and security, shipped Android-based BlackBerry hardware (the Priv, 2015) and halted new BB10 APIs and SDKs on 26 October 2015. BB10 services were switched off on 4 January 2022. QNX itself is the enormous exception: BlackBerry reported QNX embedded in 255 million vehicles as of October 2024, up from 235 million in June 2023 — a larger install base than BlackBerry ever achieved in phones — with a QNX royalty backlog of roughly $640 million.

**Peak adoption.** ~1 million Z10 units shipped in the first three weeks (Q4 FY2013); 2.7 million BB10 devices in Q1 FY2014; 70,000 third-party apps at launch, 120,000 by mid-2013; BlackBerry's overall share 1.7% in Q3 2013. In its surviving niche: QNX in 255 million vehicles as of October 2024.

**Why it failed.** BB10 shipped in January 2013, nearly three years after RIM bought QNX and five and a half years after the iPhone, by which time BlackBerry's own enterprise customers had already gone BYOD to iPhone — so the platform's entire historic differentiator, managed corporate mail, had been commoditised by Exchange ActiveSync on iOS and Android while BB10 was still being built, and technical excellence had nothing left to defend.

**What might have changed it.** If RIM had shipped QNX on phones in 2011 instead of putting it first on the PlayBook tablet — which launched in April 2011 without a native email client, the single most self-defeating launch decision in the platform's history — BB10 would have arrived while BlackBerry still held roughly 13% of the smartphone market and an intact enterprise lock-in, rather than 1.7%.

**Evidence.** The $934m Z10 inventory write-down in September 2013, alongside a $965m quarterly loss, is the clean documentary evidence that hardware was built and not sold. 2.7 million BB10 units in a quarter against ~200 million Android units in the same quarter fixes the scale. The technical claim is independently corroborated by QNX's survival: the same microkernel that failed in phones is now in 255 million cars, up 20 million year on year and 80 million since 2020 — the OS was never the problem.

Tags: too late to market, no app ecosystem, niche capture, strategic mismanagement, incumbent lockin, business model failure

Sources:
- https://en.wikipedia.org/wiki/BlackBerry_10
- https://www.itpro.com/mobile/20680/blackberry-hit-934m-inventory-charge-over-unsold-z10-smartphones
- https://www.engadget.com/2013-06-28-blackberry-sold-a-mere-2-7-million-bb-10-handsets-last-quarter.html
- https://www.engadget.com/2013-03-28-blackberry-2013-q4.html
- https://www.automotiveworld.com/news-releases/blackberry-qnx-embedded-technology-powers-255-million-vehicles-on-the-road-today/

### Cairo
*1991–1996 · Microsoft — Jim Allchin*

**What was brilliant.** Cairo's architecture was correct and the industry spent the following fifteen years reimplementing it piecemeal: a replicated, X.500-style distributed directory covering an entire enterprise namespace; the Object File System, a content-indexed, queryable store where typed properties were first-class and 'find' was a query rather than a filename scan; distributed objects with location transparency; and a shell that presented the whole network as one namespace. Allchin's team was solving the federated-namespace and desktop-search problems roughly a decade before the rest of the industry admitted they were problems.

**What happened.** Announced by Allchin at the 1991 Professional Developers Conference; demonstrated at the 1993 Cairo/Chicago PDC with builds handed to attendees. Microsoft repeatedly redefined it, sometimes calling it a product and sometimes 'a collection of technologies.' The Object File System was abandoned in April 1996 in favour of incremental NTFS enhancements plus the Exchange Server directory engine, with the beta pushed to 1997 and product to a possible 1998. The surviving components shipped separately: features into Windows NT 4.0 (24 August 1996), the directory as Active Directory in Windows 2000, content indexing into IIS and later Windows Search. The object store was revived as WinFS and cancelled again in June 2006.

**Peak adoption.** None — Cairo never shipped as a product. Beta builds were distributed to 1993 PDC attendees only.

**Why it failed.** Cairo was cancelled by Microsoft itself, not lost to a competitor: once Windows 95 and NT proved the installed base could be held without it, the object file system's cost — a forklift data migration for every customer — exceeded any revenue Microsoft could attach to it while it already owned the market.

**What might have changed it.** If OFS had been scoped from the start as an index-and-property layer over NTFS rather than a replacement file system — which is exactly what eventually shipped, a decade late, as Indexing Service and then Windows Search — Cairo delivers most of its user-visible value in 1995 or 1996 instead of never.

**Evidence.** Mark Wood, Microsoft's lead NT product marketing manager, April 1996: 'OFS was stripped of its objects-only meaning long ago and the term OFS actually described a feature set rather than a specific product.' Wood's stated rationale was that a new file system would have been 'extremely costly for users' — a forklift upgrade — so Microsoft chose gradual NTFS upgrades instead. The same idea was attempted and abandoned a second time as WinFS in June 2006, which is strong evidence that the failure was the scope of the data-migration problem rather than any lack of will or engineering capacity. The common description of Cairo as 'vaporware' is imprecise: Microsoft said publicly it was a technology programme, and most of it shipped — only the object store never did, twice.

Tags: perpetual rewrite, technical shortfall, strategic mismanagement, too late to market

Sources:
- https://en.wikipedia.org/wiki/Cairo_(operating_system)
- https://www.techmonitor.ai/technology/microsoft_dumps_cairos_object_file_system
- https://www.betaarchive.com/wiki/index.php/Microsoft_Cairo
- https://betawiki.net/wiki/Microsoft_Cairo

### Copland (Mac OS 8)
*1994–1996 · Apple Computer (NuKernel by Bill Bruffey; announced by David Nagel)*

**What was brilliant.** Architecturally, Copland was the right answer and Apple knew it: NuKernel, a from-scratch microkernel informed by Mach 3.0 but with substantial soft-real-time scheduling additions for multimedia; protected memory and preemptive multitasking for new applications; a cooperative compatibility box keeping every existing Mac application running unmodified; a hardware abstraction layer so Apple could stop hand-tuning the OS per model; the themable Appearance Manager; and a relational metadata store in the Finder. The individual pieces were real and several shipped — the multithreaded Finder and Appearance Manager in Mac OS 8 (July 1997), HFS+ in 8.1 (January 1998), a preemption-capable nanokernel in 8.6.

**What happened.** Development began March 1994, announced by David Nagel in May 1994. Parts were demonstrated at WWDC May 1995 with a beta promised by year end and shipment in early 1996. Ship date slipped to mid-1996, then late 1996, then end of 1997. Amelio's WWDC keynote in May 1996 was almost entirely Copland with almost nothing running. 'Developer Release 0' reached selected partners in August 1996 and crashed so often it was unusable for development. Ellen Hancock stopped the project in August 1996; it was formally superseded when Apple announced the NeXT acquisition on 20 December 1996.

**Peak adoption.** None — no public release. DR0 (August 1996) went only to selected partners.

**Why it failed.** Copland failed as a management artifact rather than as a design: features were added by acclamation with no architect empowered to refuse them and no integration discipline across teams, so the components never converged into a system that could boot reliably, and Apple ran out of time before it ran out of ideas.

**What might have changed it.** If Apple had frozen Copland's feature set in 1995 to NuKernel plus protected memory plus the existing Mac OS running in a compatibility box, and shipped that as Mac OS 8 in 1996, it is the same product Apple eventually shipped as Mac OS X in 2001 — five years earlier, with no acquisition required.

**Evidence.** Amelio's own description: Copland was 'just a collection of separate pieces, each being worked on by a different team ... that were expected to magically come together somehow.' At WWDC in May 1996 Amelio returned to the stage at the end of the show to add microkernel multithreading to the feature list because attendees had complained — scope being set live, on stage, two years into the project. Hancock, arriving mid-1996, found no real plan and a system so unstable it could not be developed on, and concluded it would never ship. The four successive slips (end-1995, mid-1996, late-1996, end-1997) with no corresponding narrowing of scope are the signature of the failure.

Tags: perpetual rewrite, strategic mismanagement, internal politics, technical shortfall, too late to market

Sources:
- https://en.wikipedia.org/wiki/Copland_(operating_system)
- https://www.cultofmac.com/apple-history/mac-os-copland
- https://apple.fandom.com/wiki/Copland
- https://en.wikipedia.org/wiki/Nukernel
- https://en.wikipedia.org/wiki/Ellen_Hancock

### Coyotos
*2004-2009 (announced January 2005; development halted when Shapiro joined Microsoft in April 2009; never resumed) · Jonathan S. Shapiro and The EROS Group, LLC, working from Johns Hopkins University*

**What was brilliant.** Coyotos was designed to fix the specific flaw that EROS and the entire L4 family shared -- blocking synchronous IPC semantics that let mutually suspicious processes deny service to one another -- and to become the first formally verified general-purpose OS kernel. Its most ambitious move was BitC: rather than verify C against an ad hoc model of C's semantics, Shapiro set out to build a systems programming language with a formally specified semantics, so that the kernel source itself would be a mathematical object that a prover could reason about directly. Verifying a kernel written in a language designed for verification, with full low-level control, remains a goal the field has still not routinely achieved twenty years later.

**What happened.** In April 2009 Shapiro announced on the project mailing list that he had been hired by Microsoft (to work on the Midori project) and could not continue Coyotos or BitC. He promised a final BitC release but warned it might not be possible. He left Microsoft in March 2010 and did not restart either project. The 'first formally verified OS kernel' title went to seL4, whose functional correctness proof was completed on 29 July 2009 -- the same year Coyotos stopped.

**Why it failed.** Coyotos attempted to invent a new systems programming language, a new kernel and a verification methodology simultaneously on effectively one person's funding, and lost the race to a team that verified a conventional C kernel with an existing theorem prover.

**What might have changed it.** If Shapiro had verified a C kernel against Isabelle/HOL -- precisely the route Gerwin Klein's group took, at a documented cost of about 20 person-years for 8,700 lines -- instead of first building BitC and BitCC, verification was plausibly reachable inside the project's funded life. The language was the thing that made the kernel unreachable, not the other way round.

**Evidence.** Shapiro's own retrospective on BitC concedes the strategic problem: the language work did not, in the end, serve the systems work, and he notes the Microsoft work 'didn't seem to require or benefit from BitC.' The timing is the sharpest evidence -- Coyotos was abandoned in April 2009 and seL4's proof completed in July 2009, so the specific prize Coyotos was chasing was taken by a competitor within three months of Coyotos stopping, using the unglamorous method Coyotos had rejected. The EROS Group's own status note read that 'Active work on Coyotos stopped several months ago, and is unlikely to resume.'

Tags: perpetual rewrite, funding collapse, too late to market, poor developer experience, no app ecosystem

Sources:
- https://www.osnews.com/story/21262/jonathan-shapiro-of-coyotos-bitc-joins-microsoft/
- http://danluu.com/bitc-retrospective/
- http://lambda-the-ultimate.org/node/3264
- http://www.gnu.org/software/hurd/microkernel/coyotos.html
- http://www.cap-lore.com/CapTheory/KK/Shap/eros-comparison.html

### GNU Hurd
*1986 (first design attempt on TRIX) / 1990 (development begins in earnest) – present; still no 1.0 after 36 years · Thomas (Michael) Bushnell, BSG, for Richard Stallman and the Free Software Foundation, on top of CMU Mach 3.0 (now GNU Mach)*

**What was brilliant.** The translator architecture is genuinely original and still has no mainstream equivalent. Any unprivileged user can attach a 'translator' — an ordinary program — to a node in the filesystem namespace, and thereby extend the system: mount a filesystem image, an FTP site, a tar archive, or a network transport, with no root privilege and no kernel change. Combined with replaceable per-process servers for auth, proc and exec, and per-user subenvironments, this dissolved the root/non-root dichotomy into capability handing: a user can run their own /servers, their own filesystem, even their own process server, without endangering anyone else. It is the most thorough attempt ever made to give ordinary users the extensibility that Unix reserves for the kernel.

**What happened.** Mach licensing uncertainty delayed the start by three years (1987→1990). Linux 0.01 appeared in September 1991 and took the entire GNU userland with it. The Hurd then re-chose its kernel three more times: an L4 port attempted 2004–2006 (stalled), Coyotos evaluated 2005–2006 (judged unsuitable), Viengoos written 2008–2009 (paused for lack of resources), then back to GNU Mach. First formal release 0.2 in 2013; four releases 2015–2016; then stagnation. Debian GNU/Hurd 2025 (August 2025) finally completed 64-bit support and uses NetBSD userland disk drivers via a Rump layer, building around 72% of the Debian archive — and is still explicitly not recommended for production.

**Why it failed.** The Hurd was rewritten from its foundations four times across three decades, so it never became usable during the single window (1990–1993) in which the GNU project had no kernel and the free-software world would have adopted whatever the FSF shipped; Linux filled that window in a few months of one student's spare time and the window never reopened.

**What might have changed it.** Bushnell's own, stated in retrospect: adapt the 4.4BSD-Lite kernel in 1987 instead of waiting three years for Mach's licence and then building a multi-server system on it. 'It is now perfectly obvious to me that this would have succeeded splendidly and the world would be a very different place today.'

**Evidence.** Stallman's own admission: 'I take full responsibility for the technical decision to develop the GNU kernel based on Mach, a decision which seems to have been responsible for the slowness of the development.' On Bushnell's method: he 'several times redesigned and rewrote large parts of the code based on what he had learned, rather than trying to make the Hurd run as soon as possible... it was good design practice, but it wasn't the right practice for our goal: to get something working ASAP.' The 2025 Debian port builds ~72% of the archive, 35 years after development began; the driver problem was only relieved in 2023–25 by importing NetBSD drivers through Rump rather than by writing Hurd drivers. Note the popular story that 'the Hurd failed because microkernels are slow' is not well supported — the Hurd's three abandoned kernel migrations and its driver gap are far better documented causes than any measured performance shortfall.

Tags: perpetual rewrite, too late to market, no hardware channel, technical shortfall

Sources:
- https://en.wikipedia.org/wiki/GNU_Hurd
- https://lists.debian.org/debian-hurd/2025/08/msg00038.html
- https://lwn.net/Articles/1033414/
- https://www.phoronix.com/news/Debian-GNU-Hurd-2025
- https://www.gnu.org/software/hurd/news.html

### Haiku
*2001-present (still pre-R1) · Michael Phipps (OpenBeOS), later Haiku, Inc. (nonprofit, 2003)*

**What was brilliant.** A clean-room reimplementation of BeOS, not a fork, that is binary-compatible with BeOS R5 applications on 32-bit builds and API-compatible on 64-bit. It has a hybrid kernel derived from ex-Be engineer Travis Geiselbrecht's NewOS, BFS extended attributes with live queries, and a pervasively multithreaded Interface Kit. Its package system mounts .hpkg packages read-only through packagefs instead of extracting files, so installs and rollbacks are atomic.

**What happened.** Renamed from OpenBeOS in 2004 to avoid Palm trademark issues. Alpha 1 in September 2009, package management went live in 2013, Beta 1 in 2018, Beta 5 on 13 September 2024. There is still no final R1 after about 25 years, and it remains an enthusiast OS.

**Why it failed.** Rebuilding a year-2000 OS exactly, with volunteers, took 17 years to reach beta. By then the BeOS application base it was compatible with was gone, and modern hardware (GPU acceleration, wide driver support) and apps were beyond its resources.

**What might have changed it.** Dropping strict BeOS R5 binary compatibility early and adopting a Linux or BSD driver layer and existing toolkits in the 2000s, trading purity for reaching usable-on-modern-hardware status a decade sooner.

**Evidence.** Timeline: founded August 2001, first alpha only in September 2009 'after seven years of development', beta 1 in 2018, beta 5 in 2024 (Haiku release notes and Wikipedia). The popular description of Haiku as a 'BeOS fork' is wrong: it is a clean reimplementation with its own kernel lineage (NewOS). Reviewers still call it fast but 'an interesting diversion' rather than a daily platform.

Tags: too late to market, no app ecosystem, technical shortfall, no hardware channel, niche capture

Sources:
- https://en.wikipedia.org/wiki/Haiku_(operating_system)
- https://www.haiku-os.org/get-haiku/r1beta5/release-notes/
- https://news.slashdot.org/story/24/09/14/158240/haiku-originally-openbeos-releases-long-awaited-r1beta5
- https://hackaday.com/2024/10/30/haiku-oss-beta-5-release-brings-us-into-a-new-beos-era/
- https://www.theregister.com/2023/01/11/haiku_beta_4/

### MiNT / MultiTOS
*1990–1993 (Atari); FreeMiNT 1993–2013 · Eric R. Smith, an independent developer later hired by Atari Corporation*

**What was brilliant.** Smith set out to port GNU tools to the Atari ST, decided it was easier to bolt a Unix personality onto TOS than to port Unix, and produced a preemptive multitasking kernel with Unix process semantics (fork, signals, pipes, ptrace), a MINIX-derived filesystem with long filenames, and partial memory protection on 68030 machines with an MMU — all on top of a single-tasking OS. The decisive engineering feat is that it preserved binary compatibility with the existing single-tasking GEM application base and ran those unmodified applications preemptively alongside new multitasking ones. Original recursive acronym: 'MiNT is Not TOS'.

**What happened.** Atari hired Smith and shipped the kernel plus a multitasking AES as MultiTOS with the Falcon030 in 1993 — the acronym was officially re-read as 'MiNT is Now TOS'. Atari abandoned the computer market the same year. The community continued it as FreeMiNT under an open-source licence, moving through AtariForge, SourceForge and GitHub; last release 1.18.0, March 2013.

**Why it failed.** The multitasking kernel reached Atari customers in 1993, the same year Atari exited the computer business, so the OS's only distribution channel shut before any application base could form around it — a technically successful kernel with no platform left to run on.

**What might have changed it.** Had Atari adopted MiNT in 1990–91, when Smith first released it publicly, and shipped it in ROM on the TT030 and STE, MultiTOS gets two to three years of platform life and a plausible claim as the cheap Unix-like desktop of the early 1990s — before Linux consolidated that role in 1993–94.

**Evidence.** Smith released MiNT publicly in May 1990; Atari did not ship MultiTOS until the Falcon030 in 1993. The adoption is itself notable: Atari took an outsider's hobby-originated free-software kernel and made it the official OS — a rare vendor capitulation to external engineering — but did it at the moment the company was leaving the market.

Tags: too late to market, hardware tied to dying platform, no hardware channel, niche capture, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/MiNT
- https://en.wikipedia.org/wiki/Atari_TOS

### Palm OS (Pilot OS / Garnet OS), and Palm OS 6 Cobalt
*1996–2009 (Pilot 1000 in 1996; Garnet OS 5.4.9 final release Oct 2007; Cobalt delivered to licensees Jan 2004 and never shipped) · Palm Computing (Jeff Hawkins, Ed Colligan, Donna Dubinsky); spun out as PalmSource 2003; acquired by ACCESS 2005*

**What was brilliant.** The most aggressively latency-optimized handheld OS ever shipped for consumers. Hawkins' design rules — every common task in three taps or fewer, no boot sequence, instant-on — were enforced architecturally: there was no file system, only a record-oriented database held directly in RAM, and applications executed in place from RAM, so app launch was effectively zero-cost. The entire OS plus bundled apps fit in 512KB of ROM on a 16MHz Motorola 68328 with 128KB of RAM. Graffiti deliberately traded naturalness for accuracy — a constrained unistroke alphabet that in practice beat Newton's far more ambitious cursive recognizer. Palm OS 5 (2002) moved to ARM while preserving binary compatibility for every 68k app through the PACE emulator.

**What happened.** ~85% of the PDA market by 2000. Palm split into PalmSource (OS) and palmOne (hardware) in 2003, severing the OS from its forcing customer. Palm OS 6 'Cobalt' — a ground-up rewrite with protected memory and preemptive multitasking, built on technology from Be Inc. whose assets Palm had bought in 2001 — was delivered to licensees in January 2004 and no licensee ever shipped a device with it. Palm OS's share of PDAs fell from 41.8% (Q2 2004) to 18.8% (Q2 2005) while Windows Mobile rose from 36.6% to 45.7%. ACCESS acquired PalmSource in September 2005 and pivoted to the ACCESS Linux Platform, which also never reached volume. Palm itself shipped a Windows Mobile Treo (700w, 2006) and then built webOS. Final Garnet release October 2007; the Centro was the last significant Palm OS device.

**Peak adoption.** ~85% of the worldwide PDA market in 2000, with over 6 million Palm devices sold by March 2000; 41.8% of all PDAs shipped in Q2 2004, falling to 18.8% by Q2 2005.

**Why it failed.** The RAM-as-storage, no-memory-protection, single-application architecture that made Palm OS instantaneous on 1996 hardware could not be extended to wireless, multimedia and background tasks; the replacement broke compatibility badly enough that not one licensee shipped it, leaving Palm selling a 1996 OS into a 2005 market.

**What might have changed it.** If Palm had not split the OS company from the hardware company in 2003, Cobalt would have had a committed launch device and a shipping deadline rather than being a spec-complete OS with no customer. The structural error was governance, not engineering.

**Evidence.** Cobalt's licensee adoption was literally zero — ACCESS representatives presenting the successor platform in 2008 mentioned Cobalt only 'to say that it never shipped in any devices.' A revealing licensing detail: Palm OS ran on the KADAK AMX 68000 kernel, which did support multitasking, but the license terms 'specifically state that Palm may not expose the API for creating/manipulating tasks' — so Palm shipped a single-tasking OS on a multitasking kernel for contractual reasons. The popular story that the iPhone killed Palm OS is wrong by two years: by 2005 Palm was already shipping Windows Mobile Treos because its own next-generation OS had no takers.

Tags: perpetual rewrite, technical shortfall, strategic mismanagement, licensing or legal, compatibility gap, business model failure

Sources:
- https://en.wikipedia.org/wiki/Palm_OS
- https://www.phonescoop.com/articles/article.php?a=1595
- https://www.thestreet.com/investing/stocks/palmone-loses-market-grip-10222063
- https://lowendmac.com/2016/a-history-of-palm-part-5-the-end-and-the-post-mortem/
- https://www.theregister.com/on-prem/2004/09/27/palmsource-reboots-cobalt-but-no-phones-until-2005/388364

### Pink
*1988–1992 · Apple Computer — Advanced Technology Group / Object Based Systems (Erich Ringewald, later Ed Birss)*

**What was brilliant.** In 1988 Pink specified what the industry did not get until 2001: a fully object-oriented operating system with preemptive multitasking, memory protection, deep internationalization, and a componentized document model, built on a microkernel called Opus that exported C++ object interfaces directly, with file systems, device drivers, databases and networking living outside the kernel as objects rather than inside a monolith. It deliberately carried no System 6 compatibility burden so the object model could be uniform all the way down — the one design decision that later killed it, and the reason the design was as clean as it was.

**What happened.** Conceived at Apple's March 1988 Sonoma Mission Inn offsite, where OS ideas were sorted onto blue (incremental), pink (next-generation) and red (further out) index cards. Ringewald left by October 1988 with 'grave doubts' about feasibility. The team went from five to about 25 in two months, past 100 by April 1989, and about 150 by 1990 — drawing engineers out of the rest of Apple. IBM saw a demo in April 1991, the AIM alliance formed October 1991, and on 2 March 1992 Pink left Apple entirely, becoming Taligent Inc. Never shipped in any form.

**Peak adoption.** None — never released. Peak internal headcount about 150 engineers (1990).

**Why it failed.** Pink had no ship date and no migration path for existing Mac software, so Apple could never allow it to displace the revenue-bearing Blue line; the only way to keep funding it was to hand it to a partner, which removed it from Apple's control before it was finished and guaranteed Apple would have to start a third OS project.

**What might have changed it.** If Apple had required Pink from day one to ship a Mac-compatibility environment — the concession NeXT eventually needed and that Carbon later proved was the whole ballgame — Pink would have had a migration story, would not have needed IBM's money, and Apple never starts Copland.

**Evidence.** The blue/pink index-card origin is the founding document of Apple's decade-long OS problem: it institutionalised a split between the team that had to ship and the team that got to design. Ringewald, the project's own founding lead, left within seven months doubting it could be built. By June 1990 Bill Bruffey had started NuKernel for a separate Mac OS modernisation effort — Apple was funding two competing next-generation kernels at once, two years before either had shipped anything. Spinning Pink out in March 1992 was not a market defeat; it was Apple choosing not to finish its own system.

Tags: perpetual rewrite, internal politics, strategic mismanagement, compatibility gap, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/Taligent
- https://en.wikipedia.org/wiki/Copland_(operating_system)
- https://apple.fandom.com/wiki/Taligent
- https://tedium.co/2019/02/28/ibm-workplace-os-taligent-history/

### ReactOS
*1998-present (still alpha) · Jason Filby and contributors, from the ashes of FreeWin95; later Aleksey Bragin and ReactOS Deutschland e.V.*

**What was brilliant.** A clean-room, open-source Windows NT-architecture kernel meant to run unmodified Windows drivers and applications, not just a Win32 layer on another kernel. It shares userland DLLs with Wine and has grown to about 14.9 million lines and 88,000 commits from 301 contributors.

**What happened.** 0.1.0 on 1 February 2003; 0.3.0 in August 2006. In January 2006 developer Hartmut Birr alleged code derived from disassembled Windows; contributions were frozen and a source audit began. 0.4.0 on 16 February 2016; 0.4.15 in March 2025. It is still alpha, the x86_64 port lacks WoW64, and UEFI, SMP and modern GPU work sits in out-of-tree branches.

**Why it failed.** Clean-room binary compatibility with Windows was so slow to build that ReactOS was always a decade behind a target that kept moving. The 2006 leaked-code audit froze development just as the 0.3 era had momentum.

**What might have changed it.** Reaching a stable, XP/2003-compatible beta before Windows XP's April 2014 end of support, when there was a large base of users and institutions wanting a free XP-compatible replacement.

**Evidence.** The project's own 30-year retrospective says early work was slow because contributors had to 'first build a very basic NT-like kernel before they could develop drivers for it', and that the 2006 audit 'considerably slowed development momentum throughout the 0.3.x era'. Birr's allegation of 17 January 2006 led to repository access being disabled on 27 January 2006. The gap from 0.3.0 (2006) to 0.4.0 (2016) was ten years.

Tags: perpetual rewrite, compatibility gap, technical shortfall, licensing or legal, too late to market

Sources:
- https://reactos.org/blogs/30yrs-of-ros/
- https://en.wikipedia.org/wiki/ReactOS
- https://handwiki.org/wiki/Software:ReactOS
- https://windowsforum.com/windows-news.4/reactos-at-30-open-source-windows-compatibility-drivers-and-future.398744/

### Windows CE / Windows Embedded Compact
*1996–2023 · Microsoft (codename 'Pegasus')*

**What was brilliant.** A genuinely small, genuinely hard-real-time operating system with a Win32-compatible API — a combination nobody else achieved. Deterministic, bounded interrupt latency; 256 fixed thread priority levels with priority inheritance to bound priority inversion; a kernel that fit in about one megabyte; and Platform Builder, a componentised build system where an OEM selected only the modules it needed and received kernel source to adapt — a level of source access highly unusual for Microsoft. It ran on MIPS, SuperH, PowerPC, ARM and x86, and it let embedded engineers use Visual Studio, Win32 and later the .NET Compact Framework on devices with a few megabytes of RAM. It powered the Handheld PC and Pocket PC, Windows Mobile, the Sega Dreamcast's optional runtime library, AutoPC and Ford Sync, the Zune HD, and Windows Phone 7.

**What happened.** CE 1.0 released 16 November 1996 for Handheld PCs. Pocket PC 2000–2003, then Windows Mobile, which peaked at 42% of the US smartphone market in 2007 and fell to 27% in 2008, 7.9% of worldwide smartphone sales by Q3 2009 and ~5% by August 2010. Windows Phone 7 (2010) was the last consumer CE-based release; Windows Phone 8 (2012) replaced the CE kernel with the NT kernel and orphaned every WP7 device. Windows Embedded Compact 7 (2011) and Compact 2013 (13 June 2013) were the final versions. Mainstream support ended 9 October 2018; extended support ended 10 October 2023; OEM licences remain purchasable into 2028. It survives in a very large installed base of industrial handhelds, HMIs, POS terminals and medical equipment — the textbook niche capture.

**Peak adoption.** Windows Mobile (CE-based) held 42% of the US smartphone market in 2007 and 37% in 2006; Gartner recorded 3.1 million CE-based units in Q1 2007 alone. Total lifetime CE device shipments across embedded and consumer were never disclosed but run into the hundreds of millions.

**Why it failed.** Microsoft repeatedly rebuilt the consumer layer above the CE kernel — Handheld PC, Palm-size PC, Pocket PC, Smartphone, Windows Mobile, Windows Phone 7 — and each rebuild broke its predecessor's applications, so the platform never accumulated a compounding third-party catalogue; when the iPhone arrived Microsoft answered by discarding CE entirely in Windows Phone 8, proving to developers that no Microsoft mobile target was durable.

**What might have changed it.** If Windows Phone 7 had shipped on the NT kernel in 2010 — or if Windows Phone 8 had kept CE and preserved binary compatibility — Microsoft would not have thrown away its own application base twice in three years, and the platform that held 42% of the US smartphone market in 2007 would have had something to defend with.

**Evidence.** The share collapse is documented and fast: 42% US (2007) → 27% (2008) → 7.9% worldwide (Q3 2009) → ~5% (Aug 2010) → <0.1%. No Windows Phone 7 device could be upgraded to Windows Phone 8 when it shipped in October 2012, because the kernel changed. CE's real-time credentials are documented by Microsoft itself: 256 priority levels with priority inheritance and deterministic interrupt latency from version 3.0 onward, kernel operable in ~1 MB. Compact 2013 was the last release (June 2013); extended support ended 10 October 2023, 26 years after launch, with the surviving install base entirely industrial.

Tags: perpetual rewrite, strategic mismanagement, no app ecosystem, niche capture, compatibility gap

Sources:
- https://en.wikipedia.org/wiki/Windows_CE
- https://en.wikipedia.org/wiki/Windows_Mobile
- https://techshelps.github.io/MSDN/BACKGRND/html/msdn_rtdraft6.htm
- https://learn.microsoft.com/en-gb/lifecycle/end-of-support/end-of-support-2023
- https://www.cs.cornell.edu/courses/cs614/1999sp/papers/wince.html

### Windows Phone (7 / 8 / 8.1 / 10 Mobile)
*2010–2019 (WP7 21 Oct 2010; WP8 29 Oct 2012; WP8.1 Aug 2014; Windows 10 Mobile 17 Mar 2016; end of support 14 Jan 2020) · Microsoft*

**What was brilliant.** Windows Phone 7 was the first mobile OS with an original, coherent visual language rather than a skeuomorphic one. Metro — typographic, motion-driven, content-first, derived from transit signage and the Zune HD — was so evidently right that both Apple and Google moved to flat, typographic design within three years. The engineering achievements were real and specific: a strictly enforced hardware chassis specification (fixed capacitive buttons, minimum RAM, CPU, GPU and camera) that let Microsoft guarantee performance across OEMs in a way Android's licensees could not; a compositor that ran animations on a thread separate from application code, so the UI held 60fps even when an app blocked — Android did not get equivalent behaviour until Project Butter in 2012; Live Tiles, a genuinely novel middle ground between an icon and a widget; a fully managed app model (Silverlight and XNA, later .NET) with an enforced sandbox and no filesystem access from day one; and in Windows Phone 8.1 memory tuning good enough to run the whole OS well on 512MB devices like the Lumia 520, for a while the most-used Windows Phone in the world.

**What happened.** WP7 launched more than three years after the iPhone and two after Android, without copy/paste or third-party multitasking. Microsoft then broke its own developers twice: Windows Phone 8 (Oct 2012) moved from the Windows CE kernel to the NT kernel and no Windows Phone 7 device could be upgraded — Microsoft offered the cosmetic 7.8 update on 31 January 2013 instead — and Windows 10 Mobile then stranded most Windows Phone 8 devices again. Microsoft acquired Nokia's devices business for just over $7 billion (announced 2 Sept 2013, closed April 2014), wrote off $7.6 billion in July 2015, cut 7,800 phone jobs and exited. Peak share was 3.6% globally in Q3 2013 (Gartner), with Nokia supplying 93.2% of all Windows Phones that quarter. Joe Belfiore confirmed in October 2017 that Microsoft would build no further phones or features; end of support was 14 January 2020.

**Peak adoption.** 3.6% of global smartphone sales in Q3 2013 (Gartner); 3.4% for full-year 2013 (IDC). Total Lumia shipments across the entire life of the line: 111.3 million units, Q4 2011 to Q3 2016, best quarter 10.5 million in Q4 2014. Lumia was 95% of all Windows Phone sales.

**Why it failed.** Windows Phone arrived three years late into a market where app availability had already become the purchase criterion, and Microsoft then destroyed the developer trust it needed to close that gap by making both of its major OS releases non-upgradeable — so every developer's platform investment was written off twice in four years, which made the app gap self-reinforcing.

**What might have changed it.** If Windows Phone 8 had shipped as an upgrade for existing Windows Phone 7 hardware — either by carrying CE forward one more release or by delaying the NT transition — the platform would have kept its early adopters and its developers through 2012–2014, the one period when its share was actually compounding (+156% year-on-year shipments in Q3 2013).

**Evidence.** The kernel break is the mechanism: WP7 ran Windows CE 6.0, WP8 ran the NT kernel, 'the NT kernel won't run on hardware that was designed for the CE kernel,' and Microsoft's answer to owners was the cosmetic 7.8 update in January 2013. The financial record: $7bn+ for Nokia's devices business in 2013–14, a $7.6bn writedown fifteen months later. The common claim that Microsoft was simply 'too late' understates a real 2013 trajectory — 3.6% and growing 156% year on year, and #1 or #2 share in Italy, Poland and parts of Latin America — that was then ended by buying Nokia and thereby converting every remaining OEM (HTC, Samsung, Huawei) into a competitor.

Tags: too late to market, no app ecosystem, compatibility gap, strategic mismanagement, incumbent lockin, business model failure

Sources:
- https://en.wikipedia.org/wiki/Windows_Phone
- https://en.wikipedia.org/wiki/Microsoft_Lumia
- https://en.wikipedia.org/wiki/Windows_Phone_7
- https://thenextweb.com/news/idc-android-hit-81-0-smartphone-share-q3-2013-ios-fell-12-9-windows-phone-took-3-6-blackberry-1-7
- https://www.windowscentral.com/idc-windows-phone-experiences-156-jump-handsets-shipped-q3-2013

### Workplace OS / IBM Microkernel (with OS/2 Warp Connect, PowerPC Edition)
*1991–1996 · IBM (Boca Raton and Austin), on an OSF Research Institute fork of CMU Mach 3.0*

**What was brilliant.** The most complete multi-personality microkernel system ever actually built and shipped. A hardened Mach 3.0 fork (branded 'IBM Microkernel') hosted OS/2, DOS, Windows 3.1 and a Unix personality (WPIX) as concurrent user-space servers over a single kernel, with a personality-neutral device driver framework and a shared shell, portable across PowerPC, x86 and ARM. Taligent's TalOS and OS/400 were designed in as further personalities. It booted, it ran real OS/2 applications on PowerPC hardware, and it produced a gold master — which is more than any other multi-personality microkernel of the era managed.

**What happened.** Research began 1991; first public demo at Comdex late 1992. Gerstner said at Comdex 1993 that the microkernel would not replace AIX, removing its largest internal customer. The AIX personality was abandoned in January 1995 over endianness incompatibility; IBM Rochester rejected the OS/400 personality. Gold master 15 December 1995; OS/2 Warp Connect (PowerPC Edition) 1.0 was made available 5 January 1996 by special order to selected customers at $215 — with no networking, despite the 'Connect' name. IBM cancelled the whole program in March 1996.

**Why it failed.** Decomposing several mature, mutually incompatible operating systems into cooperating user-space servers cost more performance and produced more complexity than any single personality was worth, so every IBM division that was supposed to adopt it refused — the platform was killed by its own intended customers before any external market ever evaluated it.

**What might have changed it.** If IBM had scoped it as an OS/2-only PowerPC kernel shipping in 1993 — one personality, one architecture, native applications recompiled — it would have had a product on the market while PowerPC still had momentum and while OS/2's installed base was at its peak, instead of a gold master in December 1995 four months after Windows 95.

**Evidence.** Fleisch and Co's 1997 post-mortem calls it 'one of the most significant operating systems software investments of all time' and 'one of the largest operating system failures in modern times,' and names the Second System Effect as a principal lesson. IBM architect Freeman L. Rawson III, May 1997: 'There is no good way to factor multiple existing systems into a set of functional servers without making them excessively large and complex.' The $2 billion figure equals roughly 0.6% of IBM's revenue for the period. IBM's own stated cancellation reasons were inadequate performance, weak PowerPC market acceptance, the poor PowerPC 620 launch, cost overruns, and the absence of the AIX, Windows and OS/400 personalities — every one of those an internal decision or an internal shortfall.

Tags: perpetual rewrite, technical shortfall, internal politics, strategic mismanagement, hardware tied to dying platform, no app ecosystem

Sources:
- https://en.wikipedia.org/wiki/Workplace_OS
- https://tedium.co/2019/02/28/ibm-workplace-os-taligent-history/
- https://www.osnews.com/story/27436/workplace-microkernel-and-os-a-case-study/
- https://www.os2museum.com/wp/os2-history/os2-warp-powerpc-edition/

## Licensing or legal friction in the window (7)

### BSD/386 and BSD/OS
*1991–2004 (BSDi founded 1991; BSD/386 1.0 March 1993; renamed BSD/OS at 2.0 in Jan 1995; final 5.1 Oct 2003; support ended end-2004) · Berkeley Software Design, Inc. — founded by members of Berkeley's Computer Systems Research Group including Mike Karels, Keith Bostic, Rob Kolstad, Donn Seeley, Bill Jolitz and Trent Hein*

**What was brilliant.** The mature, commercially supported BSD, built by the people who wrote the Berkeley TCP/IP stack, on the hardware everyone actually had. For most of the 1990s BSD/OS was what the commercial Internet ran on: UUNET and a large share of early ISPs, backbone routers, news servers and mail exchangers chose it because its networking stack sustained load nothing else could. It was the first commercially supported BSD on commodity PC hardware, and it shipped complete source for $995 — against AT&T System V source licences costing orders of magnitude more, which is precisely what made it dangerous.

**What happened.** USL sued BSDi in April 1992 over that $995 source offering, later adding the Regents of the University of California, seeking an injunction over Unix copyrights and trade secrets. Judge Dickinson R. Debevoise denied the injunction and threw out all but two of the complaints in December 1992; the University counter-sued in state court in early 1993 over USL's failure to attribute BSD code in System V. The case settled in January 1994 with three files removed from the 18,000 in Networking Release 2 and USL copyright notices added to about 70 more, all still freely redistributable. 4.4BSD-Lite followed in June 1994. But the legal cloud sat over the entire BSD lineage for the twenty-two months in which a free Unix for the PC was up for grabs. Wind River acquired BSD/OS in April 2001, stopped selling it at the end of 2003 and ended support in 2004.

**Why it failed.** AT&T's lawsuit put the entire BSD lineage under legal doubt for exactly the period (April 1992 – January 1994) when a free Unix for commodity PCs was being decided, and Linux — which carried no such cloud — took that constituency permanently; the substantive merit of the claim proved to be three files out of eighteen thousand.

**What might have changed it.** USL never files, or files in 1990 and resolves before 386BSD ships. Linus Torvalds has said that if 386BSD had been available when he started, he would probably never have written Linux. A legally clean BSD in 1992 plausibly becomes the free Unix, and the permissive BSD licence rather than the GPL becomes the default of open-source infrastructure — a different software industry.

**Evidence.** Suit filed April 1992; Debevoise denied the injunction and dismissed all but two complaints in December 1992; UC counter-sued in early 1993; settlement January 1994 removing three files from 18,000 and annotating about 70; 4.4BSD-Lite released June 1994 with USL agreeing not to sue anyone building on it. BSD/386 source licences were $995, undercutting AT&T System V source licences — the commercial provocation. BSD/OS launched March 1993, sold to Wind River April 2001, sales ended end-2003, support ended 2004.

Tags: licensing or legal, business model failure, funding collapse, acquired and killed, no hardware channel

Sources:
- https://www.oreilly.com/openbook/opensources/book/kirkmck.html
- https://en.wikipedia.org/wiki/BSD/OS
- https://en.wikipedia.org/wiki/UNIX_System_Laboratories,_Inc._v._Berkeley_Software_Design,_Inc.
- https://klarasystems.com/articles/history-of-freebsd-part-2-bsdi-and-usl-lawsuits/

### eComStation / ArcaOS (licensed OS/2 continuations)
*2001-2011 (eComStation, last GA 2.1); 2017-present (ArcaOS) · Serenity Systems and Mensys BV (eCS); Arca Noae LLC (ArcaOS), both under licence from IBM*

**What was brilliant.** They kept OS/2's Workplace Shell (a SOM-based object-oriented desktop) and its fast, stable 32-bit kernel usable on new hardware without source-level ownership. They added a modern installer, ACPI, JFS boot, USB/NVMe drivers and, in ArcaOS 5.1.x, installation on UEFI systems and GPT disks.

**What happened.** IBM ended OS/2 support on 31 December 2006. eComStation stalled after Mensys ran into financial trouble in 2012; ownership moved to XEU.com/PayGlobal and 2.1 (2011) was the last GA. Arca Noae released ArcaOS 5.0 in 2017 and 5.1.2 on 8 March 2026, selling personal licences at $139, mostly to legacy OS/2 installations.

**Peak adoption.** About 30,000-40,000 eComStation licences sold by 2014

**Why it failed.** The vendors licensed IBM's OS/2 as binaries they could extend but not own or rearchitect. The platform therefore stayed a 32-bit x86 system with no modern app base, and could only serve customers already locked into OS/2 rather than win new ones.

**What might have changed it.** IBM open-sourcing OS/2, or transferring full source rights, when it ended support in 2006. A community and vendor could then have made it 64-bit and ported a modern browser and toolchain.

**Evidence.** IBM licensed the codebase to Serenity Systems in 2001 and later to Arca Noae, and Arca Noae is now the only OS/2 licensee. Coverage of ArcaOS 5.1.2 notes it remains a 32-bit x86 platform with 'few modern amenities' despite UEFI/GPT work. eCS peaked at only 30-40k licences in total by 2014, consistent with a legacy-maintenance market.

Tags: licensing or legal, niche capture, incumbent lockin, no app ecosystem, technical shortfall, hardware tied to dying platform

Sources:
- https://en.wikipedia.org/wiki/EComStation
- https://www.arcanoae.com/faqwd/what-is-ecs-or-ecomstation/
- https://www.howtogeek.com/19-years-after-ibm-windows-killer-died-getting-another-update-as-arcaos/
- https://www.techspot.com/news/111647-os2-isnt-dead-arcaos-update-brings-1980s-era.html

### GEM (Graphics Environment Manager) for DOS
*1985–1988 retail (GEM 1.0, 28 Feb 1985; GEM/3 3.11, 3 Nov 1988); GPL'd April 1999 · Digital Research, Inc. — Lee Lorenzen, Darrell Miller and team, building on DRI's GSX graphics layer (1982)*

**What was brilliant.** Device independence from the ground up. GEM VDI descended from GSX, which was deliberately structured like CP/M itself — GDOS/GIOS mirroring BDOS/BIOS — so applications drew to an abstract device and drivers handled CGA screens, plotters and PostScript typesetters alike. That abstraction is why Ventura Publisher, the dominant PC desktop-publishing package of the late 1980s, was a GEM application and could drive professional typesetters from an 8086. GEM 1.0 shipped 28 February 1985, nine months before Windows 1.0, ran usably on an 8088 with CGA, and gave DOS overlapping resizable windows and a live desktop two years before Windows had them.

**What happened.** Apple sued DRI over look-and-feel; DRI settled rather than litigate. GEM Desktop 2.0 (24 March 1986) removed overlapping windows, replaced the desktop with two permanently open fixed file-manager panes, narrowed the scrollbars, altered the trash icon and stripped the animations. GEM/3 (Nov 1988) was the last retail release. Novell acquired DRI in July 1991; Caldera released the GEM source under GPL-2.0 in April 1999, spawning FreeGEM and OpenGEM.

**Why it failed.** Apple's look-and-feel suit forced Digital Research to remove the exact features that made GEM competitive — overlapping windows and a live desktop — thirteen months after launch, converting a real technical lead over Windows 1.0 into a visibly inferior product, while Microsoft, which signed a licence with Apple in November 1985 instead of being sued, was left free to keep iterating.

**What might have changed it.** Had DRI negotiated a licence from Apple on Microsoft's November 1985 terms rather than settling by mutilating the product, GEM/2 keeps overlapping windows and the desktop; combined with Amstrad's installed base in Europe, that is a live fight against Windows 2.x.

**Evidence.** GEM 1.0 shipped 28 Feb 1985; GEM 2.0 shipped 24 March 1986 carrying the settlement-mandated changes, and the New York Times reported DRI modifying GEM on 1 October 1985. Atari, holding independent 68000 rights, kept the original unmutilated design in TOS — a natural experiment showing exactly what PC GEM would have looked like without the settlement. Lee Lorenzen, a principal GEM architect, left DRI to co-found Ventura Software shortly after 1.0, draining the team.

Tags: licensing or legal, strategic mismanagement, no app ecosystem, incumbent lockin, compatibility gap

Sources:
- https://en.wikipedia.org/wiki/GEM_(desktop_environment)
- https://www.osnews.com/story/26322/apple-vs-dri-the-iotheri-look-and-feel-lawsuit/
- https://ctrl-alt-rees.com/2023-05-13-how-apple-ruined-gem-and-nearly-windows-too.html

### L4 (Liedtke lineage: L3, original L4, Fiasco, Hazelnut, L4Ka::Pistachio)
*L3 commercially deployed from the late 1980s; L4 name from the V2 ABI in 1995; Liedtke died 2001; the lineage continues in descendants · Jochen Liedtke, at GMD, then IBM T.J. Watson, then the University of Karlsruhe; carried forward by TU Dresden (Fiasco), Karlsruhe (Hazelnut, Pistachio) and UNSW/NICTA*

**What was brilliant.** Liedtke falsified the received wisdom that microkernels are inherently slow, and did it by measurement rather than argument. By designing the kernel around the hardware instead of around an abstraction — strictly synchronous IPC, message registers mapped onto physical CPU registers for genuine zero-copy, a kernel small enough to stay resident in cache, hand-written assembly on the critical path — he took one-way IPC from Mach's ~115 µs to 5.00 µs on a 50 MHz i486 in 1993, then 0.75 µs (121 cycles) on a 160 MHz Pentium in 1997, and Pistachio hit 36 cycles on Itanium 2 in 2005. He also invented hierarchical external pagers, moving all physical memory management to user level while keeping it fast. Härtig et al. then showed L4Linux running within 5% of native Linux on AIM benchmarks where Mach-derived MkLinux was far behind. 'On µ-Kernel Construction' (SOSP '95) won the ACM SIGOPS Hall of Fame Award in 2015.

**What happened.** The ideas won completely; L4 as a platform never did. L3 was 'commercially deployed in a few thousand installations (mainly schools and legal practices)'. Fiasco was 'the first L4 kernel with significant commercial use (estimated shipments up to 100,000)'. Liedtke died in 2001, before the V4 ABI he designed was implemented. The family then fragmented into mutually incompatible descendants — Fiasco.OC/L4Re, Pistachio, OKL4, NOVA, PikeOS, seL4 — each with its own ABI and its own user-level servers. No L4 ever became a user-facing operating system; every one of them is a substrate somebody else has to build an OS on.

**Peak adoption.** L3: 'a few thousand installations'. Fiasco: estimated shipments up to 100,000. Descendants (OKL4, seL4, PikeOS) counted separately.

**Why it failed.** L4 was a kernel, not an operating system, and Liedtke's minimality principle deliberately excluded everything a user-facing system needs; because the community never converged on one set of user-level servers, each group had to build an entire OS on top, and each built a different and incompatible one, so no shared application platform ever accumulated.

**What might have changed it.** If the IP regime GMD and IBM imposed on Version X had not driven TU Dresden to write a clean-room kernel from scratch in 1998, the L4 world might have had one ABI and one personality stack during the late-1990s window in which L4Linux was within 5% of native Linux — the only moment at which a microkernel-hosted Linux was a credible product.

**Evidence.** The Dresden fork's name is itself the evidence: 'GMD and IBM imposed an IP regime which proved too restrictive for other researchers, prompting Dresden to implement a new x86 version from scratch, called Fiasco in reference to their experience in trying to deal with IP issues' (Elphinstone and Heiser, 'From L3 to seL4', SOSP 2013). Published IPC cost table: Original/i486 1993, 250 cycles, 5.00 µs; Original/Pentium 1997, 121 cycles, 0.75 µs; Pistachio/Itanium 2 2005, 36 cycles, 0.02 µs; seL4/Haswell 2013, 301 cycles, 0.09 µs. L3 'a few thousand installations'; Fiasco 'estimated shipments up to 100,000'. The authors' own conclusion: 'It is rare that a research operating system has both a significant developer community, significant commercial deployment, as well as a long period of evolution' — an unusually candid framing of a kernel that succeeded everywhere except as a product.

Tags: licensing or legal, no app ecosystem, internal politics, niche capture

Sources:
- https://www.cs.yale.edu/flint/cs428/doc/L3toseL4.pdf
- https://en.wikipedia.org/wiki/L4_microkernel_family
- https://dl.acm.org/doi/10.1145/224056.224075
- https://www.cs.princeton.edu/courses/archive/fall07/cos518/papers/ukernel.pdf
- https://lists.gnu.org/archive/html/bug-hurd/2001-06/msg00082.html

### NeWS (Network extensible Window System)
*1985–1993 (codenamed SunDew; initial release Oct 1986; merged into OpenWindows as XNeWS 1989–91; abandoned after COSE/CDE in 1993) · Sun Microsystems — James Gosling and David S. H. Rosenthal*

**What was brilliant.** The window system that got the architecture right and lost anyway. A clean-room PostScript interpreter inside the display server, extended with cooperative lightweight threads, object-oriented class extensions to the PostScript language, a canvas tree, and a synchronous event model with interests and monitors that guaranteed ordering — a class of correctness X11 still does not offer. Because application code was downloaded into the server, all interactive feedback — menus, rubber-banding, dragging, widget response — ran locally at the display with no network round trip, which is precisely the latency problem X11 has never solved. One resolution-independent imaging model shared by screen and printer. Arbitrary-shaped and translucent windows in 1986. The OPEN LOOK NeWS Toolkit was written entirely in PostScript; Arthur van Hoff's HyperLook and Don Hopkins's HyperLook SimCity with pie menus demonstrated the model. Gosling carried the core idea — a portable bytecode interpreter into which code is downloaded — forward into Java.

**What happened.** MIT distributed X11 free; NeWS required commercial licences from Sun, with Adobe and Xerox PARC also holding rights. Sun capitulated architecturally and built XNeWS/OpenWindows (1989–91), running an X11 server and a NeWS interpreter side by side in one process. Rosenthal's own verdict on that compromise: it was 'a ghastly kludge' that 'seriously degraded the NeWS interpreter performance and was not considered a very good X11 server either.' When OPEN LOOK lost to Motif and the March 1993 COSE alliance settled the industry on CDE/Motif, Sun stopped supporting NeWS.

**Why it failed.** Sun refused to open-source NeWS against a free X11 from MIT, protecting licence revenue and Adobe's PostScript business — and a display protocol that competitors cannot adopt for free cannot become a standard, regardless of how much better it is.

**What might have changed it.** Open-source NeWS on MIT-like terms in 1987. Rosenthal states plainly that he and Gosling understood NeWS 'could not displace X11 as the Unix standard window system without being equally open source' and that management overruled them. Secondarily: do not require every application programmer to absorb a stack-oriented, object-oriented and multi-threaded PostScript simultaneously — Rosenthal's own second admitted error.

**Evidence.** Rosenthal's 2024 first-person retrospective gives three causes: a one-size-fits-all imaging model that pixel-starved early hardware and later GPUs both rejected; the language barrier ('the majority of application programmers were intimidated' — and NeWS's stack orientation, object orientation and multi-threading were 'none of these optional'); and the licensing failure. X11 was free from MIT from 15 Sept 1987; NeWS products required licences from Sun, Adobe and Xerox PARC.

Tags: licensing or legal, poor developer experience, no app ecosystem, strategic mismanagement

Sources:
- https://blog.dshr.org/2024/07/x-window-system-at-40.html
- https://en.wikipedia.org/wiki/NeWS
- https://www.theregister.com/software/2024/07/10/lead-developer-of-original-x-rival-news-looks-at-why-it-died/1190574
- https://www.usenix.org/legacy/publications/library/proceedings/usenix2000/invitedtalks/gettys_html/text16.htm

### Plan 9 from Bell Labs
*c. 1985–2015 at Bell Labs (1st edition to universities 1992; 4th edition April 2002; MIT relicensing 2021) · Computing Science Research Center, Bell Labs — Rob Pike, Ken Thompson, Dave Presotto, Phil Winterbottom, Dennis Ritchie, Howard Trickey, Sean Dorward*

**What was brilliant.** Took 'everything is a file' to its conclusion and made it distributed. One protocol — 9P — for every resource, local or remote, so a remote machine's network stack, graphics, or process table mounts into your tree and is used with read and write. Per-process mutable namespaces with bind and union directories, which is the direct ancestor of Linux mount namespaces and therefore of containers. /proc as the only process interface, with no ptrace. rfork for fine-grained selection of what a child shares (the model Linux copied as clone(2)). The CPU-server / file-server / terminal split as first-class architecture rather than NFS bolted onto standalone workstations. UTF-8, designed by Thompson and Pike in September 1992 and now the encoding of essentially the entire web. Venti content-addressed archival storage and the Fossil snapshotting file system.

**What happened.** Distributed to universities only in 1992. In 1995 AT&T sold commercial source licences for $350 through publisher Harcourt Brace, aiming at embedded systems rather than the general computer market. Lucent dropped commercial support in the late 1990s; a third edition came out under an open-source licence in 2000, the fourth edition in April 2002 under the Lucent Public License 1.02. Bell Labs development ceased around 2015; the copyright passed to the Plan 9 Foundation in 2021 and was relicensed MIT. It never became anyone's primary system; it survives as 9front, and its ideas survive in UTF-8, /proc, Linux namespaces, WSL's use of 9P, and Go.

**Why it failed.** Bell Labs kept Plan 9 under restrictive licences through exactly the years (1992–1999) when a free Unix-like kernel could still capture the research and hobbyist constituency, so Linux and the BSDs absorbed that constituency instead, and Plan 9's deliberate incompatibility with the POSIX/X11 software stock then left it no path to applications.

**What might have changed it.** Release Plan 9 under a BSD-style licence in 1992 instead of university-only distribution, and instead of $350 commercial licences in 1995. Linux 0.99 was contemporaneous and technically far behind; the binding constraint was the licence, not the engineering.

**Evidence.** Pike's own 2000 talk 'Systems Software Research is Irrelevant' estimates that 90–95% of the work in Plan 9 went to honouring externally imposed standards, leaving 'little slop left for novelty.' Pike later reported abandoning Plan 9 as his daily system at Google because it was 'too inconvenient to live on a machine without a C++ compiler, without good NFS and SSH support, and especially without a web browser.' Eric Raymond's verdict: Plan 9 'failed simply because it fell short of being a compelling enough improvement on Unix to displace its ancestor.' Licence chronology: universities 1992 → $350 Harcourt Brace licences 1995 → open source 2000 → LPL 2002 → MIT 2021 — the free release arrived eight years after Linux 0.01.

Tags: licensing or legal, no app ecosystem, compatibility gap, strategic mismanagement, niche capture, incumbent lockin

Sources:
- https://en.wikipedia.org/wiki/Plan_9_from_Bell_Labs
- http://doc.cat-v.org/bell_labs/utah2000/utah2000.html
- http://www.herpolhode.com/rob/utah2000.pdf
- https://interviews.slashdot.org/story/04/10/18/1153211/rob-pike-responds
- https://opensource.org/license/LPL-1.0

### VAX VMM Security Kernel (VAX SVS)
*1981-1990 (VAX-11/730 prototype early 1980s; external field test late 1980s; formally cancelled by DEC on 1 March 1990; retrospective published IEEE TSE Nov 1991) · Digital Equipment Corporation -- Paul A. Karger, Mary Ellen Zurko, Douglas W. Bonin, Andrew H. Mason, Clifford E. Kahn*

**What was brilliant.** An A1-targeted virtual machine monitor security kernel that ran unmodified VMS and ULTRIX-32 as guests on standard VAX hardware, needing only microcode engineering change orders for virtualization -- high assurance without rewriting a single application. Formally specified in Ina Jo with a formal top-level specification, covert channel analysis, and hardware and software trusted distribution procedures. The team explicitly rejected the conventional kernel-plus-emulator design after calculating that 'the software development costs of a VMS emulator would be comparable to the cost of development of the VMS operating system itself' and would then have to track VMS forever. It passed a successful external customer field test.

**What happened.** Cancelled by DEC on 1 March 1990 while the kernel was essentially complete and in field test, before A1 evaluation concluded. Karger et al. published the full retrospective in IEEE Transactions on Software Engineering in November 1991 -- one of the most candid engineering post-mortems in the security literature. Lipner records it as the terminal event for the whole category: 'The last of the high-security evaluations by a major vendor (Digital Equipment) was cancelled in 1990 because of an insufficient market.'

**Why it failed.** US State Department export controls on B3 and A1 operating systems made the largest identified segment of demand -- allied NATO governments -- effectively unsellable, so the addressable market could not repay the cost of A1 development.

**What might have changed it.** If export controls on B3/A1 operating systems had been relaxed, as the National Research Council's 1990 report Computers at Risk explicitly recommended, the NATO-ally demand surfaced by the field test would have been reachable and the product would plausibly have shipped. Secondarily, if management had not deferred Ethernet support to shorten the schedule, the field test's single biggest criticism would have vanished.

**Evidence.** The retrospective's Section X, Cancellation, is unusually direct: 'a significant fraction of the customer demand came from foreign countries who are allied with the United States ... The current U.S. State Department export controls on operating systems at the B3 and A1 levels are extremely onerous and would likely have interfered with many potential sales, even to close NATO allies.' It adds that the controls fail on their own terms because the technology 'is primarily based on strict application of well-known software engineering practices, such as layering and information hiding,' and that Computers at Risk 'suggests that the export controls on B3 and A1 systems are in fact discouraging U.S. industry from developing systems which employ such technology.' On the self-inflicted wound: 'management chose not to implement Ethernet support in the initial versions ... The primary criticisms during the external field test came from the lack of Ethernet support.' And the closing lesson: 'A high-security time-sharing system is no longer sufficient for the marketplace of the 1990's.'

Tags: licensing or legal, business model failure, strategic mismanagement, technical shortfall, no app ecosystem

Sources:
- https://lukemuehlhauser.com/wp-content/uploads/Karger-et-al-A-retrospective-on-the-VAX-VMM-security-kernel.pdf
- https://ieeexplore.ieee.org/document/106971/
- https://www.cs.utexas.edu/~witchel/380L/papers/karger90oakland-vmm.pdf
- https://www.stevelipner.org/links/resources/The%20Birth%20and%20Death%20of%20the%20Orange%20Book.pdf

## Technical shortfall or developer cost (9)

### Firefox OS (Boot to Gecko)
*2011–2016 (announced as Boot to Gecko July 2011; first commercial devices 2013; phone development halted 8 Dec 2015; all development ended Sept 2016) · Mozilla (Andreas Gal, Chris Jones, Michael Vines)*

**What was brilliant.** Firefox OS is the only mobile OS in which the entire user interface was a web page. Gaia — the lock screen, dialer, SMS app, camera, settings and homescreen — was HTML, CSS and JavaScript running in Gecko, which ran on Gonk: a Linux kernel plus Android's HAL and drivers, so it could reuse the existing silicon support of the entire Android supply chain. There was no native application layer at all. To make that possible Mozilla had to invent and then standardise the missing device APIs, and that is its durable legacy: the Battery Status API, Vibration API, Screen Orientation API, Ambient Light Events, Device Orientation, Page Visibility, Web Notifications and getUserMedia all reached W3C recommendation or broad browser adoption through this work, and the Web Telephony, Web SMS and Web Bluetooth proposals came out of the same effort. Every web developer on earth could write an app on day one with no SDK, and apps were distributed as a URL plus a manifest, with no store gatekeeper required.

**What happened.** Aimed at the sub-$100 emerging-market segment in Latin America, India and Eastern Europe; by 16 December 2014, fourteen operators in 28 countries offered Firefox OS phones (ZTE Open, Alcatel One Touch Fire, Geeksphone Peak). The hardware the strategy required — 256MB of RAM on single-core ARMv6 — could not render a JavaScript-composited UI smoothly, and by 2014 Android One and sub-$50 Android handsets had erased the price gap the whole plan depended on. Mozilla announced on 8 December 2015 that it would stop developing and selling Firefox OS smartphones, and in September 2016 ended all development, removing B2G code from mozilla-central. The codebase did not die: TCL's KaiOS Technologies forked it into KaiOS, which passed 100 million devices by May 2019 largely on Reliance Jio's JioPhone, overtook iOS for second place in Indian OS share in 2018, and drew a $22 million investment from Google in June 2018. Panasonic shipped the same lineage on TVs as 'My Home Screen' until 2024.

**Peak adoption.** Fourteen operators across 28 countries as of 16 December 2014; Mozilla never published unit sales and they are believed to be in the low millions. Its direct fork KaiOS passed 100 million devices by May 2019.

**Why it failed.** Mozilla bet that the price floor of an Android smartphone would stay above roughly $100 and built a JavaScript-rendered OS to occupy the gap below it; Android's own descent below $50 in 2014 removed the gap, and the $35 hardware Mozilla had committed to was too slow for the architecture Mozilla had committed to — so it lost on both price and performance at the same moment.

**What might have changed it.** If Mozilla had aimed Gecko-on-Gonk at the feature-phone replacement market with physical keypads from the start — precisely the target KaiOS later hit — it would have been competing against Nokia Series 30/40, not against Android, and the modest hardware would have been adequate rather than inadequate for the job.

**Evidence.** The standards record is the proof of technical quality: the Battery Status, Vibration, Screen Orientation, Ambient Light and Device Orientation APIs all exist in browsers today because of this project. The fatal figures are commercial: fourteen operators in 28 countries but no disclosed volume, then cancellation on 8 December 2015 with Mozilla citing 'intense competition in the smartphone market.' The usual explanation — 'web apps can't compete with native' — is contradicted by KaiOS, which shipped the same Gecko-on-Gonk architecture to over 100 million devices by targeting a different segment. The architecture was sound; the segment was wrong.

Tags: technical shortfall, hardware tied to dying platform, no app ecosystem, business model failure, too late to market, niche capture

Sources:
- https://en.wikipedia.org/wiki/Firefox_OS
- https://techcrunch.com/2015/12/08/mozilla-will-stop-developing-and-selling-firefox-os-smartphones/
- https://en.wikipedia.org/wiki/KaiOS
- https://www.gsmarena.com/mozillas_firefox_os_is_officially_dead_at_least_for_smartphones-news-15407.php
- https://www.engadget.com/2013-07-01-firefox-os-hands-on-alcatel-onetouch-fire-and-zte-open-video.html

### House (Haskell User's Operating System and Environment), with hOp
*hOp from 2003; House from 2004; ICFP 2005 paper; version 0.8 in 2006; GHC 6.8.2 port October 2008; dormant thereafter · Thomas Hallgren, Mark P. Jones, Rebekah Leslie and Andrew Tolmach, Programatica project, OGI School of Science and Engineering / Portland State University; building on hOp by Sébastien Carlier and Jeremy Bobbio*

**What was brilliant.** House's contribution is the H monad: a principled, typed interface to raw hardware — physical memory, page tables, interrupt handlers, I/O ports — designed so the type system tracks exactly what a piece of code is permitted to touch. That made it possible to write device drivers (VGA text, PS/2 keyboard and mouse, NE2000 and Intel PRO/100 network cards), a protocol stack (Ethernet, IPv4, ARP, DHCP, ICMP, UDP, TFTP, TCP), the Gadgets window system ported to Concurrent Haskell, and a loader that runs a.out binaries in user mode — all in pure Haskell on bare x86 — and then to reason formally about the virtual-memory subsystem, which was the actual research point.

**What happened.** Remained a research artifact. Last release 0.8 (2006), a GHC 6.8.2 port in October 2008, then dormant; the group's follow-on work moved toward an L4-compatible microkernel design and into the Habit/HASP language work at Portland State. The method — write the kernel in Haskell first and then refine it — is what reached production, in seL4's Haskell prototype (2009), not House itself.

**Why it failed.** House was a feasibility proof on a research grant, and the obstacle it honestly exposed — that a lazy, garbage-collected runtime with a large C-implemented RTS cannot give the timing and memory guarantees a kernel needs — is precisely why the same line of research only reached deployed systems by using Haskell as a specification language for a C kernel rather than as the kernel itself.

**What might have changed it.** Aim the H-monad work at generating or verifying C from the Haskell model from the outset. seL4 did exactly this in 2009 — Haskell prototype, C implementation, machine-checked refinement — and shipped into billions of devices; House had the hardware-interface theory first and ran it on the metal instead.

**Evidence.** Hallgren, Jones, Leslie and Tolmach, 'A Principled Approach to Operating System Construction in Haskell,' ICFP 2005: a monadic interface to low-level hardware features as the basis for OS construction in Haskell. House includes device drivers, a full window system, an IPv4/TCP stack and a user-mode program loader, all in Haskell; built on hOp, a GHC variant for standalone environments. Last release 0.8 (2006); GHC 6.8.2 port October 2008; follow-on work toward an L4-compatible microkernel.

Tags: technical shortfall, funding collapse, no app ecosystem, niche capture, no hardware channel

Sources:
- https://web.cecs.pdx.edu/~apt/icfp05.pdf
- https://programatica.cs.pdx.edu/House/
- https://dl.acm.org/doi/10.1145/1086365.1086380
- https://hasp.cs.pdx.edu/history.html
- http://lambda-the-ultimate.org/node/299

### JavaOS
*1996–1999 · Sun Microsystems — JavaSoft (Jim Mitchell, Peter Madany), then SunSoft, then jointly with IBM*

**What was brilliant.** The only serious attempt to run a managed-language runtime directly on bare metal as the operating system. A small microkernel booted the JVM, and then the device drivers, the AWT graphics stack and the TCP/IP networking were themselves written in Java and executed by the VM — type safety and mobile code all the way down to the hardware, with no host OS underneath. It ran on SPARC, x86, PowerPC and ARM from the same bytecode, and the JavaStation booted its entire environment from flash and the network with no local disk, which made an administration story nobody else could match: there was no local state to corrupt, patch or steal.

**What happened.** Codenamed Kona, announced 29 May 1996. Transferred from JavaSoft to SunSoft in early 1997; rebuilt with IBM from late 1997 as JavaOS for Business, aimed at replacing IBM 3270 green-screen and Unix X terminals. Licensed to more than 25 manufacturers including Oracle, Acer, Xerox, Toshiba and Nokia. Final release 23 August 1999, when Sun and IBM jointly announced discontinuation. By 2003 Sun's own documentation described JavaOS as 'legacy technology' and directed developers to Java ME. Sun's thin-client business went instead to Sun Ray, which used a hardware display protocol and no Java at all.

**Peak adoption.** Licensed to 25+ manufacturers; JavaStation unit sales undocumented and small. Superseded by Sun Ray.

**Why it failed.** JavaOS staked the entire system on JVM performance at the one moment in Java's history when there were no production JIT compilers, so the machine that was supposed to be cheaper and better than a PC was visibly slower than one — and the cost argument for thin clients evaporated anyway when PC prices fell below $1,000 in 1997.

**What might have changed it.** If JavaOS had shipped on HotSpot-class JIT technology — Sun acquired Animorphic in 1997 and HotSpot shipped in 1999 — rather than an interpreter, the performance objection disappears and the network-computer pitch survives the sub-$1,000 PC long enough to reach the enterprise terminal-replacement market IBM was aiming at.

**Evidence.** Drivers, graphics and networking were all Java bytecode executed by an interpreting VM at launch in 1996. The consistently reported chief complaint about the JavaStation was slow application loading and general sluggishness. Ownership moved from JavaSoft to SunSoft in early 1997 and was then re-architected with IBM in late 1997 — three owners and two architectures in under two years, which is an internal-politics signature, not a market one. Sun and IBM discontinued it jointly on 23 August 1999, and Sun's own thin-client successor abandoned the Java-on-the-metal premise entirely.

Tags: technical shortfall, strategic mismanagement, internal politics, no app ecosystem, too late to market, business model failure, incumbent lockin, hardware tied to dying platform

Sources:
- https://en.wikipedia.org/wiki/JavaOS
- https://en.wikipedia.org/wiki/JavaStation
- https://hackaday.com/2018/04/10/the-forgotten-workstation-sun-javastation/
- https://en.wikipedia.org/wiki/Network_Computer
- https://www.techmonitor.ai/technology/javaos_for_business_bites_the_dust

### Jini (later Apache River)
*1994–2007 at Sun; Apache River 2007–2022 · Sun Microsystems — conceived by Bill Joy at the Aspen Smallworks lab (1994); architects Ken Arnold and Jim Waldo*

**What was brilliant.** The best-designed answer anyone has produced to building distributed systems that survive partial failure. Three ideas, all still correct: (1) leases — every resource grant is time-bounded and must be renewed, so a crashed or partitioned participant's state is reclaimed automatically and the federation self-heals rather than leaking; (2) mobile-code proxies — a service registers a serialised Java proxy object with the lookup service and the client downloads its driver at runtime, so there is no pre-installed driver, no agreed wire protocol, and the protocol becomes an implementation detail hidden inside the proxy; (3) multicast discovery plus distributed transactions plus JavaSpaces, a Linda-style tuple space for coordination. Jini was the engineering embodiment of Waldo and Arnold's 1994 'A Note on Distributed Computing': it refused to pretend the network was reliable or that remote calls were local.

**What happened.** Announced July 1998, with partner announcements November 1998, under the Sun Community Source License. Sun simplified the SCSL and eliminated the Jini licensing fee in October 2000 after adoption complaints. Donated to the Apache Software Foundation in 2007 as 'River'; final release 3.0.0 on 5 October 2016; retired to the Apache Attic in early 2022 for lack of activity. Its ideas survive indirectly — leases in DHCP-like renewal patterns, service discovery in mDNS/Zeroconf and Consul, tuple spaces in GigaSpaces.

**Peak adoption.** More than 95,000 developers signed the SCSL and downloaded the Jini Starter Kit (Sun's Jim Hurley). No consumer appliance ever shipped with Jini as its primary networking layer; real deployments were enterprise Java systems, not the connected living room Sun pitched.

**Why it failed.** Jini required a full JVM with RMI and dynamic class loading on every participating node, which in 1999 cost more in silicon and memory than the cheap consumer appliances it was designed to federate — the floor price of joining the federation was higher than the devices themselves.

**What might have changed it.** If Sun had specified a language-neutral wire protocol that preserved the leasing, discovery and transaction semantics — ceding only the mobile-proxy elegance — Jini's architecture could have shipped in a $30 appliance. Instead Sun bound the design to Java object mobility, its most beautiful and least portable property, and the cheap-and-ugly alternatives (UPnP, Zeroconf) took the market.

**Evidence.** Jini requires every participating device to run a JVM, excluding non-Java devices entirely — a hard constraint acknowledged in the contemporaneous literature. Sun shipped it under the SCSL, not an open-source licence, and had to scrap the Jini licensing fee in October 2000 to unblock adoption. Jini developer Frank Sommers later argued the failure was positioning rather than technology — Sun marketed a 'Jini-enabled living room' that never materialised because internet-enabled devices arrived late and cheap, not early and Java-capable. Apache retired River in early 2022 after six years without a release.

Tags: technical shortfall, licensing or legal, poor developer experience, no hardware channel

Sources:
- https://en.wikipedia.org/wiki/Jini
- https://www.infoworld.com/article/2164754/java-s-secret-weapon.html
- https://en.wikipedia.org/wiki/Sun_Community_Source_License
- https://river.apache.org/release-doc/current/api/net/jini/space/JavaSpace.html
- https://www.scss.tcd.ie/stephen.barrett/lectures/cs7051/pdfs/BillDay_jini.pdf

### Mach
*1985–1994 (CMU project); Mach 3.0 microkernel from 1989; code still shipping inside Apple XNU · Richard Rashid and Avie Tevanian, Carnegie Mellon University, under DARPA funding; later maintained by the Open Software Foundation (OSF MK)*

**What was brilliant.** Mach unified virtual memory and IPC into one mechanism, which nobody had done. Memory objects were backed by user-space 'external pagers', so distributed shared memory, network paging and novel filesystems could be built entirely outside the kernel. Copy-on-write message passing made large transfers cheap in principle. It separated 'task' (address space + resources) from 'thread' (schedulable entity) — a split now universal in every OS on earth. Its VM subsystem was good enough that it was lifted wholesale into 4.4BSD, FreeBSD, NetBSD and (via NeXTSTEP) macOS, and ports gave a capability-flavoured, location-transparent naming layer for kernel objects.

**What happened.** CMU wound the project down in 1994 with Mach 3.0/Mach 4. The Open Software Foundation built OSF/1 on it (→ Digital UNIX → Tru64 UNIX, killed by HP after the Compaq merger). NeXTSTEP (1989) deliberately refused the microkernel structure and co-located a 4.3BSD kernel in Mach's address space for speed; Apple's XNU still does this, incorporating OSFMK 7.3 alongside 4.4BSD-Lite2 in one kernel. MkLinux (1996), the real Mach 3.0 + Linux-server microkernel product, was abandoned. No pure Mach 3.0 multi-server system ever shipped as a mainstream product.

**Peak adoption.** As a microkernel platform, effectively zero. As a kernel component absorbed into a hybrid, Mach-derived code ships in every macOS/iOS/iPadOS device — but only because the microkernel structure was removed.

**Why it failed.** Mach 3.0's performance deficit came from its own design — a large, cache-unfriendly kernel with heavyweight port-rights checking and message validation — so every commercial adopter had to undo the microkernel structure to ship, and the architecture Mach existed to prove was the one thing none of its users kept.

**What might have changed it.** If Mach 3.0's IPC had been co-designed with the hardware — synchronous, register-based, kernel small enough to stay in cache, as Liedtke demonstrated with L3/L4 only five years later — MkLinux and OSF/1 would have shipped genuinely microkernel-structured systems at competitive speed, and the 1990s 'microkernels are slow' consensus would never have formed.

**Evidence.** Chen and Bershad (SOSP '93, 'The Impact of Operating System Structure on Memory System Performance') measured Mach 3.0 + CMU UNIX server against DEC Ultrix across thirteen industry workloads and found the microkernel system executed substantially more non-idle instructions per equivalent workload and had a higher memory-cycles-per-instruction cost, with system code showing poorer locality on Mach than on Ultrix. Reported measurements put Mach 3.0 single-server UNIX at roughly 50% slower than native UNIX on mid-1990s hardware; on a 486DX-50, Mach IPC cost ~114 µs against ~21 µs for BSD syscalls, with about 80% of the excess attributed to port-rights checking and message validation rather than message passing itself. Härtig et al. (SOSP '97) then showed L4Linux within 5% of native Linux on AIM benchmarks while Mach-derived MkLinux lagged badly. Liedtke's L4 achieved IPC 'a factor 10–20 faster than other contemporary microkernels', i.e. Mach.

Tags: technical shortfall, niche capture, incumbent lockin

Sources:
- https://en.wikipedia.org/wiki/Mach_(kernel)
- https://people.eecs.berkeley.edu/~prabal/resources/osprelim/CB93.pdf
- https://dl.acm.org/doi/10.1145/173668.168629
- https://www.cs.yale.edu/flint/cs428/doc/L3toseL4.pdf
- https://www.cs.princeton.edu/courses/archive/fall07/cos518/papers/ukernel.pdf

### SELinux / Flask and the SELinux-hardened distributions
*1992-present (Trusted Mach and DTOS at TIS/Secure Computing; Flask on the Fluke kernel at Utah 1997-1999; NSA public release 22 Dec 2000; merged into Linux 2.6.0-test3 on 8 Aug 2003; strict policy abandoned as a default in Fedora Core 3, 2004) · NSA Information Assurance Research Office (Peter Loscocco, Stephen Smalley) with Secure Computing Corporation and the University of Utah Flux Research Group*

**What was brilliant.** Flask solved the problem every earlier mandatory access control system got wrong. It cleanly separated the policy decision point (a security server) from the enforcement points (object managers) with an access vector cache between them, and -- the genuinely novel part -- supported revocation of permissions already granted, so a policy change takes effect on open handles rather than only on new accesses. That separation made one engine able to express Bell-LaPadula, Biba, RBAC, type enforcement and domain transition as configuration rather than as kernel code. Type enforcement with automatic domain transitions on execve gave Linux a genuinely general confinement primitive, and the LSM framework that Torvalds demanded as the price of admission became the hook layer for AppArmor, Smack, TOMOYO and Landlock too.

**What happened.** The kernel technology won; the security model lost. Torvalds refused to merge SELinux directly and required a modular interface, producing LSM. SELinux was disabled by default in Fedora Core 2 (2004) after strict policy proved unusable; by Fedora Core 3 Red Hat had retreated to a targeted policy confining roughly ten network-facing domains and running everything else in an unconfined_t domain -- that is, the shipping default deliberately does not confine the user's own applications, which is the exact inversion of the design intent. The SELinux-hardened niche distributions (Adamantix, Hardened Gentoo, the NSA reference distribution) all died. SELinux survived by disappearing into infrastructure: RHEL's targeted policy, and SEAndroid, present since Android 4.3 and fully enforcing from Android 5.0.

**Peak adoption.** Billions of devices via SEAndroid, plus the RHEL/Fedora/CentOS installed base -- but this is niche capture of the enforcement engine, not adoption of the security model. The MLS and strict policies the architecture was built for remain confined to national-security deployments; Lipner in 2015 places SELinux's meaningful use, alongside Trusted Solaris, in 'some specialized applications in the national security community.'

**Why it failed.** Writing and maintaining a complete policy costs more than the security it buys for anyone who is not operating a classified network, so the only version of SELinux that could ship enabled by default was the one that confines almost nothing -- the model was adopted while its purpose was abandoned.

**What might have changed it.** If SELinux had shipped a policy-authoring model that derived confinement from observed program behaviour and package metadata rather than from hand-written type-enforcement rules, strict confinement could have been the default instead of the mode administrators famously turn off. AppArmor's simpler path-based profiles gestured at this and won adoption in Ubuntu and SUSE on usability grounds despite being the weaker mechanism.

**Evidence.** Dan Walsh's own SELinux Symposium 2005 talk, 'SELinux Targeted vs Strict policy History and Strategy,' is the primary account of the retreat: after experience with strict policy the focus shifted to targeting a handful of domains 'while continuing to leave userspace to run in an unconfined nature, and targeted policy was born.' Red Hat's own documentation states plainly that under targeted policy 'processes that are not targeted run in an unconfined domain.' The persistent field complaint -- that implementing SELinux policy is difficult, slow and complex enough that organisations with the capability often simply disable it -- is why the popular claim that 'SELinux is everywhere so it succeeded' is misleading: what is everywhere is Flask's enforcement plumbing, configured to enforce a policy far weaker than the one the architecture was designed to carry.

Tags: poor developer experience, niche capture, compatibility gap, incumbent lockin, technical shortfall

Sources:
- http://selinuxsymposium.org/2005/presentations/session4/4-1-walsh.pdf
- https://www.nsa.gov/portals/75/images/resources/everyone/digital-media-center/publications/research-papers/flexible-support-for-security-policies-into-linux-jun2001-report.pdf
- https://www-old.cs.utah.edu/flux/fluke/html/flask.html
- https://en.wikipedia.org/wiki/Security-Enhanced_Linux
- https://docs.redhat.com/en/documentation/red_hat_enterprise_linux/7/html/selinux_users_and_administrators_guide/chap-security-enhanced_linux-targeted_policy

### SkyOS
*1996-2009 · Robert Szeleney (largely a single developer), Austria*

**What was brilliant.** A from-scratch x86 desktop OS with an original kernel with SMP, a compositing GUI (SkyGI, double-buffered with transparency by 2006) and SkyFS, a fork of OpenBFS with indexed, multi-keyword live queries into file contents. It also had system-wide mouse gestures and a POSIX layer good enough to port Firefox-class applications and Mono.

**What happened.** Free through 4.x. From SkyOS 5 in 2003, access to betas cost $30. Last beta build 6947 in August 2008. Development halted in January 2009. In 2013 the last build was released free as a proprietary live CD and the site went down. The source was never released.

**Why it failed.** SkyOS was closed source, paid and built by one developer, so no outside community could write the device drivers it needed. The workload of keeping up with PC hardware made the project unsustainable.

**What might have changed it.** Open-source it (or at least its driver model) around 2003 instead of moving to paid closed betas, letting the BeOS and alt-OS hobbyist community add drivers, as Haiku's contributors did.

**Evidence.** Wikipedia's summary of the shutdown: as 'the OS was mainly the work of one man, Robert Szeleney, there was increasing difficulty to add new device drivers.' The $30 paid-beta model, introduced in 2003, limited testers to paying users. The code stayed proprietary even after the project died, so no successor could continue it.

Tags: technical shortfall, strategic mismanagement, no app ecosystem, no hardware channel, business model failure

Sources:
- https://en.wikipedia.org/wiki/SkyOS
- https://betawiki.net/wiki/SkyOS
- https://discuss.haiku-os.org/t/other-last-skyos-5-0-beta-released-for-free/3461
- https://www.osnews.com/topic/skyos/

### Subgraph OS
*2014-2017 (Subgraph founded 2010 in Montreal, pivoted to privacy work 2013; OS announced 2014; first public alpha 2016; final alpha 22-23 September 2017) · Subgraph, Montreal -- David Mirza Ahmad and Bruce Leidl; funded in part by the Open Technology Fund*

**What was brilliant.** The most credible attempt anyone made at a hardened, genuinely usable, Debian-based privacy desktop for non-experts. It combined a grsecurity/PaX-patched kernel with Oz, an application sandbox built on Linux namespaces and seccomp-bpf that gave each sandboxed application its own X server via Xpra -- so a compromised application could neither see other applications' windows nor capture their keystrokes, closing the hole that makes ordinary Linux desktop sandboxing theatre. It added a per-application outbound firewall that prompted by process rather than by port, mandatory full-disk encryption, and forced all traffic through Tor via a Metaproxy layer. Edward Snowden publicly endorsed it. The stated goal -- 'a usable desktop that is resistant to an adversary armed with reliable exploits' -- was the right goal.

**What happened.** On 11 April 2017 Micah Lee, working with Joanna Rutkowska after a Tor meeting in Amsterdam, published a complete break of the security model. A file named sgos_handbook.pdf.desktop executed arbitrary code when opened in Nautilus, because Nautilus itself ran unsandboxed and only 22 applications had Oz profiles at all. The demonstrated payload read SSH keys and saved Wi-Fi networks, took webcam snapshots, and exfiltrated the MAC address and Tor exit node. The project shipped one more alpha in September 2017, then stopped: no further security patches, blog silent since September 2017, GitHub quiet since 2020. It never left alpha.

**Why it failed.** Subgraph's security depended on sandboxing applications individually, but it inherited a GNOME desktop with hundreds of applications and had profiles for 22, so a single unsandboxed component -- the file manager -- voided the entire model, and the public demonstration of that destroyed the project's credibility before it ever reached a stable release.

**What might have changed it.** If Oz had been default-deny -- nothing executes outside a sandbox unless explicitly profiled -- the Nautilus break would have been structurally impossible. The fatal decision was to run an otherwise ordinary GNOME desktop and maintain an allowlist of sandboxed applications on top of it, which inverts the burden of proof: every unprofiled binary is a hole, and there is no bound on how many there are.

**Evidence.** Lee's write-up states the architectural lesson precisely: 'Qubes provides security by compartmentalization, while Subgraph OS provides OS hardening and app sandboxes. As this exploit hopefully demonstrates, these are not the same thing.' And on the mechanism: 'If an attacker can trick a user into running an unsandboxed script in either Nautilus (what my attack does) or in the terminal, it's game over.' This is the cleanest documented case in the whole category of a security OS killed by a demonstration rather than by market forces -- the break was published in April 2017 and the project was effectively over by the end of that year. It is also the strongest available empirical argument that application sandboxing on a conventional desktop cannot substitute for VM-level compartmentalization.

Tags: technical shortfall, funding collapse, no app ecosystem, business model failure, perpetual rewrite

Sources:
- https://micahflee.com/2017/04/breaking-the-security-model-of-subgraph-os/
- https://groups.google.com/g/qubes-users/c/XRiCDXa_2AE
- https://en.wikipedia.org/wiki/Subgraph_(operating_system)
- https://subgraph.com/blog/index.en.html
- https://www.linux-magazine.com/Online/Features/Subgraph

### Symbian OS (EPOC32 → Symbian^3)
*1997–2014 (EPOC32 at Psion 1997; Symbian Ltd formed June 1998; last Nokia device 2012; Nokia stopped accepting Symbian apps 1 Jan 2014) · Psion Software / Symbian Ltd (Nokia, Ericsson, Motorola, Psion), later Nokia and the Symbian Foundation*

**What was brilliant.** The EKA2 nanokernel (designed by Dennis May) is the specific achievement: a hard-real-time nanokernel with bounded, fully preemptible kernel latency that could run a GSM/3G protocol stack AND a rich application OS on a single ARM core, removing the need for a separate baseband processor — a real bill-of-materials saving when ARM cores were expensive. Around it: memory-protected multitasking in a few megabytes of RAM; a capability-based process permission model (Symbian Signed, from v9, 2005) that predates iOS and Android sandboxing; active objects, a single-threaded event-dispatch framework that avoided per-task thread overhead; descriptors, length-checked string/buffer types that structurally prevented buffer overruns; and demand paging added in 9.3. It delivered multi-day battery life and instant responsiveness on hardware two orders of magnitude weaker than a 2010 smartphone.

**What happened.** Peaked around 73% of smartphone OS share in 2006. Symbian Foundation founded 24 June 2008; the codebase was released under EPL on 4 February 2010 — the largest open-sourcing of a codebase in history at the time. Stephen Elop's 'burning platform' memo leaked 8 Feb 2011; Nokia announced the Microsoft/Windows Phone partnership 11 Feb 2011. The Foundation reverted to a licensing-only shell in Nov 2010, shut its sites and code repositories on 17 Dec 2010, and the code went closed again; LG and Motorola had already left, Samsung and Sony Ericsson departed for Android in 2011. Maintenance was outsourced to Accenture. The Nokia 808 PureView (2012) was the last Nokia Symbian phone; Nokia stopped accepting new Symbian software on 1 Jan 2014. The Symbian Foundation became legally insolvent on 15 April 2022.

**Peak adoption.** ~73% of the smartphone OS market in 2006; an estimated 385 million devices shipped through Q2 2010; still 37.6% of smart mobile device sales in 2010 (Gartner); under 1% by 2013.

**Why it failed.** Symbian's programming model was tuned for 1998 memory budgets and imposed a months-long learning curve on any new developer, so when app availability became the purchase criterion around 2008 Nokia could not convert a 400-million-unit installed base into an app ecosystem faster than Apple and Google could build whole platforms from zero — the kernel won on merit and the SDK lost the developers.

**What might have changed it.** Nokia acquired Trolltech in January 2008 and then ran S60 C++/Avkon and Qt as parallel application frameworks until 2010. If Qt had been made the single mandatory, fully supported Symbian app framework in 2008, Symbian would have had a modern, learnable SDK three years earlier, while it still had 50%+ share and the largest installed base in the industry.

**Evidence.** Elop's memo states the diagnosis precisely: 'Our competitors aren't taking our market share with devices; they are taking our market share with an entire ecosystem,' and 'Symbian is proving to be an increasingly difficult environment in which to develop.' The memo also cites Apple going from 25% to 61% of the $300+ segment between 2008 and 2010. The Finnish press retrospective reports the Nokia board, led by Jorma Ollila, gave Elop 'stinging feedback' over the leak because it condemned roughly 150 million Symbian units Nokia still planned to sell. Symbian Foundation timeline: founded 24 June 2008, EPL release 4 Feb 2010, sites shut 17 Dec 2010, insolvent 15 April 2022. The popular story that 'Nokia ignored the iPhone' is wrong: Nokia ran three overlapping replacement efforts (S60 touch/Symbian^3, Qt/Symbian^4, Maemo/MeeGo) and shipped none of them cleanly.

Tags: poor developer experience, perpetual rewrite, strategic mismanagement, internal politics, no app ecosystem, too late to market

Sources:
- https://en.wikipedia.org/wiki/Symbian
- https://en.wikipedia.org/wiki/Symbian_Foundation
- https://en.wikipedia.org/wiki/EKA2
- https://www.engadget.com/2011-02-08-nokia-ceo-stephen-elop-rallies-troops-in-brutally-honest-burnin.html
- https://allaboutsymbian.com/flow/item/18700_A_retrospective_on_THAT_memo_f.php
