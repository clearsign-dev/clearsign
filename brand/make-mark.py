"""The ClearSign mark, constructed rather than drawn.

A C as an aperture — the frame you look through — with an S inside it as a
signature stroke. The C has flat-cut terminals because it is an instrument; the
S has rounded ones because it is a pen. Both are built from arcs on one grid, so
the mark stays exact at any size instead of softening as it scales.
"""
import math, pathlib, sys

S = 1024                  # canvas
CX = CY = S / 2
R_C = 292                 # radius of the aperture's centreline
W_C = 132                 # its stroke
GAP = 54                  # half-angle of the opening, in degrees
R_S = 118                 # radius of each half of the S
W_S = 104                 # the pen stroke
S_SHIFT = 26              # how far the S sits into the opening

def point(cx, cy, r, deg):
    a = math.radians(deg)
    return cx + r * math.cos(a), cy - r * math.sin(a)

def aperture():
    x1, y1 = point(CX, CY, R_C, GAP)
    x2, y2 = point(CX, CY, R_C, -GAP)
    # counterclockwise, the long way round, leaving the opening on the right
    return f"M {x1:.1f} {y1:.1f} A {R_C} {R_C} 0 1 0 {x2:.1f} {y2:.1f}"

def signature():
    """An S is two circles that touch at the waist, each swept the opposite way.

    The top bowl runs counterclockwise from the upper right, over the top and
    down to the waist; the bottom bowl carries on clockwise, under and out to
    the lower left. Terminals stop short of closing, so it reads as a stroke
    someone made rather than a shape that was filled.
    """
    cx = CX + S_SHIFT
    top_c = (cx, CY - R_S)
    bot_c = (cx, CY + R_S)
    start = point(*top_c, R_S, 34)     # upper right terminal
    waist = point(*top_c, R_S, 270)    # where the two circles meet
    end = point(*bot_c, R_S, 214)      # lower left terminal
    return (f"M {start[0]:.1f} {start[1]:.1f} "
            f"A {R_S} {R_S} 0 1 0 {waist[0]:.1f} {waist[1]:.1f} "
            f"A {R_S} {R_S} 0 1 1 {end[0]:.1f} {end[1]:.1f}")

# The S is drawn twice: once fat in the background colour to cut a gap out of
# the aperture behind it, then properly on top. Without that the two forms merge
# into one blob at small sizes, which is where an icon spends most of its life.
KNOCKOUT = 30

MARK = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {S} {S}" role="img" aria-label="ClearSign">
  <path d="{aperture()}" fill="none" stroke="var(--mark-c, #2d6a8a)" stroke-width="{W_C}" stroke-linecap="butt"/>
  <path d="{signature()}" fill="none" stroke="var(--mark-gap, #0d1217)" stroke-width="{W_S + KNOCKOUT}" stroke-linecap="round" stroke-linejoin="round"/>
  <path d="{signature()}" fill="none" stroke="var(--mark-s, #e8ecef)" stroke-width="{W_S}" stroke-linecap="round" stroke-linejoin="round"/>
</svg>'''

ICON = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {S} {S}">
  <defs>
    <linearGradient id="g" x1="0" y1="0" x2="0" y2="1">
      <stop offset="0" stop-color="#1b232b"/>
      <stop offset="1" stop-color="#0d1217"/>
    </linearGradient>
  </defs>
  <rect width="{S}" height="{S}" rx="228" fill="url(#g)"/>
  <g transform="translate({S/2} {S/2}) scale(0.78) translate({-S/2} {-S/2})">
    <path d="{aperture()}" fill="none" stroke="#4aa3c9" stroke-width="{W_C}" stroke-linecap="butt"/>
    <path d="{signature()}" fill="none" stroke="#141a20" stroke-width="{W_S + KNOCKOUT}" stroke-linecap="round" stroke-linejoin="round"/>
    <path d="{signature()}" fill="none" stroke="#f2f5f7" stroke-width="{W_S}" stroke-linecap="round" stroke-linejoin="round"/>
  </g>
</svg>'''

# "ClearSign" set at 300 with -8 tracking measures about 1269 units wide in IBM
# Plex Sans, so it runs from x=900 to roughly x=2169. The box is cut wider than
# that on purpose: a viewer without Plex installed falls back to a face that may
# set wider, and a clipped wordmark is worse than a little trailing air.
WORDMARK_W = 2260

WORDMARK = f'''<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 {WORDMARK_W} {S}" role="img" aria-label="ClearSign">
  <path d="{aperture()}" fill="none" stroke="var(--mark-c, #2d6a8a)" stroke-width="{W_C}" stroke-linecap="butt"/>
  <path d="{signature()}" fill="none" stroke="var(--mark-gap, #0d1217)" stroke-width="{W_S + KNOCKOUT}" stroke-linecap="round" stroke-linejoin="round"/>
  <path d="{signature()}" fill="none" stroke="var(--mark-s, #e8ecef)" stroke-width="{W_S}" stroke-linecap="round" stroke-linejoin="round"/>
  <text x="900" y="{S/2}" dominant-baseline="central" font-family="IBM Plex Sans, ui-sans-serif, system-ui, sans-serif"
        font-size="300" font-weight="600" letter-spacing="-8" fill="var(--mark-word, #12161b)">ClearSign</text>
</svg>'''

pathlib.Path("clearsign-wordmark.svg").write_text(WORDMARK + "\n")
pathlib.Path("clearsign-mark.svg").write_text(MARK + "\n")
pathlib.Path("clearsign-icon.svg").write_text(ICON + "\n")
print("mark and icon written")
