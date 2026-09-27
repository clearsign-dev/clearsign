# The ClearSign mark

Constructed, not drawn: `make-mark.py` generates every file here from one set of
numbers, so the mark is exact at any size rather than softening as it scales.

```sh
python3 brand/make-mark.py
```

| File | For |
|---|---|
| `clearsign-mark.svg` | The mark alone. Takes its three colours from CSS variables, so it belongs in a light or dark page rather than being a picture pasted onto one |
| `clearsign-icon.svg` | The mark on its rounded field — the application icon |
| `clearsign-wordmark.svg` | Mark and name together, for a page header or a site |

## What it is

A **C** as an aperture — the thing you look through — with an **S** inside it as
a signature stroke. The C has flat-cut terminals because it is an instrument; the
S has rounded ones because it is a pen. Both are arcs on the same grid.

The S is drawn twice: once fat in the background colour to cut a gap out of the
aperture behind it, then properly on top. Without that the two forms merge into
one shape at small sizes, which is where an icon spends most of its life. It was
checked at 256, 128, 64 and 32 pixels before it was accepted.

## Colours

| | |
|---|---|
| Aperture | `#2d6a8a` on light, `#4aa3c9` on dark and on the icon |
| Signature | The page's own ink — near-black on light, near-white on dark |
| Gap | Whatever is behind the mark |

## Regenerating the application icons

```sh
python3 brand/make-mark.py
# render clearsign-icon.svg to a 1024×1024 PNG, then:
cd desktop && npm run icons
```
