# Where this design came from

The brief was to take the structure and feel of `daoism.systems` and put
ClearSign's own identity and words inside it. This records how that was done,
because the first thing the work ran into was that the reference no longer
exists.

## The site is gone

`daoism.systems` resolves — its DNS still points at Vercel, `76.76.21.21` — but
nothing answers on it:

```
$ curl -sS --max-time 20 https://www.daoism.systems/
curl: (28) Connection timed out

$ nc -z -G 8 76.76.21.21 443
443 closed/filtered
```

Checked on 27 September 2026 from two networks, including one unrelated to this
machine, which returned `ECONNREFUSED 76.76.21.21:443`. Apex and `www`, ports 80
and 443, all refuse. This is not a block on one address; the site is down.

## What it was reconstructed from

The Internet Archive holds captures back to February 2022. Two mattered:

| Capture | What it kept |
|---|---|
| `20260613065801` | The current HTML shell — the markup, the preloader, the `canvas` element, and the hashed filenames of the build it was serving |
| `20230513083430` | The **stylesheets and bundles**, which the 2026 crawl did not keep |

The two are three years apart, so the second is only useful if the site barely
changed between them. It did barely change: four of the chunk filenames in the
2026 shell are byte-identical to the 2023 ones —
`framework-3583eef75b58b7b2.js`, `webpack-142e59777b5c82a1.js`,
`4bdf9057-d61cb3b0436e1764.js` and `fb7d5399-7c9bf26a58b46800.js`. Those names
are content hashes. Same hash, same bytes, same build for three years.

So the design system below was read out of the 2023 stylesheet
(`_next/static/css/8eb4802be5bfb777.css`, 111 KB of CSS modules) and the two
bundles, and cross-checked against the 2026 shell for structure.

## What was measured, not guessed

The stylesheet is CSS modules, so every rule is prefixed with the component file
it came from. That gives the section inventory directly: `hero`, `about`,
`services`, `work`, `projects`, `references`, `team`, `build`, `blog`,
`contact`, plus `menu`, `slider`, `toggle`, `logo`, `button`, `social`, `title`,
and a `desktop`/`mobile` pair of nearly everything.

| Property | Measured value |
|---|---|
| Root font size | `0.9vw` desktop, `100%` below 992px, `80%` below 378px |
| Breakpoints | 992px and 378px, and no others |
| Background | `#040404`, text `#fff`, secondary `#cacaca`, hairlines `#848484` / `#353535` |
| Accent | `#ffdf37` |
| Type | `Russo One` for display, `Outfit` for body |
| Section model | `position: absolute`, full size, `opacity 0 → 1`, `scale(.95) → scale(1)`, `transition: opacity .4s ease-out, transform .8s ease-out, visibility .4s ease-out` |
| Easings | `cubic-bezier(.43,.195,.02,1)`, `cubic-bezier(.77,0,.175,1)`, `cubic-bezier(.165,.84,.44,1)` |
| Letter reveal | `blink` keyframe `0/20/40/60/80/100% → 0/1/1/0/1/1` opacity, `.3s`, per-letter delays `.04s` to `.28s` across seven positions, each with a different easing |
| Rings | `rotate` 30s, 40s, 45s, 45s reversed, and 145s, all linear infinite |
| Halos | 3s pulse, `animation-delay: 1s` |
| Transition vocabulary | `all .3s`, `.4s`, `.7s` and `1s ease-out`, with `.2s` and `.5s` stagger delays |
| Background | Three.js `WebGLRenderer` and `ShaderMaterial` over simplex/perlin noise |
| Sliders | Swiper |

## What was kept, and what was replaced

Kept: the stage-not-scroll section model and its exact transition; the fluid
`vw` root; the per-letter reveal with its three easings; the tick-mark section
controls, vertical on desktop and horizontal on mobile; the fullscreen menu with
a field behind it and outlined ghost copies of each item on hover; the hamburger
that folds into a cross; the plus-in-a-circle accordion that rotates 45°; the
underlined tab row; the haloed icon grid; the slow rings; the preloader; the
floating-label form; the breakpoints and the transition timings.

Replaced, because they are not ours: the name, every word of copy, the logo, the
palette, the typefaces, and the illustrations. No asset was copied from the
reference. The two colour decisions worth naming are that the near-black ground
was kept because it is structural rather than distinctive, and the accent moved
from their yellow to ClearSign's `#4aa3c9`.

Two things were deliberately not reproduced. The reference shipped a separate
component for nearly every section on mobile; this ships one set of content that
scrolls inside its section instead, which is one copy of the words to maintain
rather than two. And the field is about a hundred lines of raw WebGL rather than
Three.js plus Swiper — a page about not trusting what you cannot see has no
business shipping 1.2 MB of library to draw a moving gradient.
