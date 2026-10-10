# Patches to lightweight-charts 5.2.1 (Bitview)

Applied by hand to `dist/lightweight-charts.standalone.production.mjs`, the only file Studio loads (the dev page imports
it, the build inlines it). A fresh download loses them: apply each again at its anchor (minified names are 5.2.1's),
then open `tests.html` (served from the repo) and check it says 0 failing and every step the same.

The plots draw exactly what the library does, but for the log marks (1) and the rounded bars (7, only where a radius is
set); the axes' tags are rounded (6). `tests.html` runs the patched file and 5.2.1 as published side by side through every
series type, every price scale mode (inverted too), pans, jumps, zooms, the crosshair, restyles, live updates, new data,
resizes, margins, moves between scales, a held price range and conflation, and compares their plots pixel for pixel, the
axes left out (664 steps).

## 1. Log scales: marks on round values

The library steps a log scale's marks down from the top of the range, each step a share of the value it's at: the marks
land on values like 6.31 (written "6") and shift as the chart pans. Its log marks are now round values, as many as fit
`tickMarkDensity` apart: every decade (every few, where they don't fit); in each decade 3, or 2 and 5, or more of 1 to 9;
then each stretch between two marks parted evenly (by 5, else 2) while all of its parts fit (a stretch only partly in
view, when at least two marks' height of it shows; zoomed into one, down to where marks show). None finer than the
price's least move (the format's `minMove`), no two written alike by the format. Ranges it can't mark (zero or below,
not finite, 60 decades or more, no room between marks: a density of 0), or where no round value fits, keep the
library's marks. Edge marks (`ensureEdgeTickMarksVisible`) over these: each mark closer to one than a mark's height goes
(the library drops one at each end by a linear span's measure).

- In `class pi` (the price scale's tick mark builder), `Zl(t,i,n,s,e,r)` (its marks between two logicals: in log mode,
  prices) starts with `if(a.ho()&&this.bvLogMarks(t,n,s,e,r))return;`.
- `bvLogMarks(t,n,s,e,r)` added to `class pi`, after `Gl()`; `Zl` keeps whether it marked (`bvUsed`), and `io()` (the
  edge marks) drops ours near them when it did.

`tests.html` runs it over 20,000 random ranges (10^-12 to 10^22, a millionth of a decade to 30, 30 to 2,000 px) and
hand-picked edges, each written three ways (unformatted; two decimals, a least move of 0.01, the library's default;
three significant figures, Studio's): it ends (well under a millisecond), its marks are in the range and its margins, a
mark's height apart, round at their step, none under the least move, none written alike; with no room between marks,
it leaves them to the library at once.

## 2. A point's style from its row, not a search for it

Building a series' points (each data change) and restyling them (each `applyOptions`: a color, a spotlight) asked the
bar colorer for every point's style by its index, a binary search each, with a copy of the row found. The row is now
handed over where it's known to be that point's: the series' own rows (not conflated ones) at the point's place, with
the point's index.

- `class Es` (a series' pane view): `bvOwn()` (the colorer, handing over the row being built from), used by the three
  `zM()` (line-like, bar-like, custom: `t.Wt=i` before each point is built).
- `Es.BM()` (the restyle): the row at the point's place, when its index is the point's.

## 3. Restyles in place

`BM()` made every point anew (`{...point, ...style}`): a restyle allocated as many objects as points, and new points
made the coordinates (patch 4) start over. It now assigns each point's style onto it (`Object.assign`): the same values,
the same points. Nothing else in the library holds or compares the points array (`kM` is only replaced, by `zM()`, never
changed in place; the renderers are handed it on each update).

## 4. Coordinates kept while their mapping holds

Every update mapped every point in view to x (index to pixel) and y (price to pixel), though a crosshair's move or a
restyle changes neither mapping, and a pan whose price range holds changes only x. Each is now kept, per series, with what it depends
on (`bvMap()`: the time scale's base index, right offset, width and bar spacing; the price scale's range, margins,
height, inversion, mode, log formula, and the base value of percent and indexed scales) and the stretch of points
mapped: the same mapping and the same points, only points newly in view are mapped; otherwise all of them, as before. A
view that jumps past the stretch starts a new one (a gap would hold stale coordinates).

- `Es.bvRun(slot, key, map)`; line-like and bar-like `OM()` call it for x (`Ic`) and y (`Jo`, `t_`). New points
  (`bvOwn()`, as `zM()` starts) let go of what was kept for the old ones.
- `bvMap()` on the time scale (before `Ic`) and on the price scale (before `Jo`).

## 5. Points built without spreads

`Rb(t,i,n)` (a point from a row: its fields, then its style) spread two fresh objects into a third for each point. It
now assigns the style onto the fields' object: the same keys in the same order, one object fewer per point.

## 6. Axis tags rounded at both ends

A price axis's tags (a series' last value, a price line's, the crosshair's) were rounded on their outer side only, square
where they meet the plot; the time axis's (the crosshair's date) at its bottom only. Each is now rounded at every corner,
by the same radius (2 px).

- `class L` (a price axis tag), `st()`: one `A(...)` call with `[l.ft,l.ft,l.ft,l.ft]` for either side (it chose
  `[l.ft,0,0,l.ft]` on a left scale, `[0,l.ft,l.ft,0]` on a right one).
- `class W` (the time axis's tag), `st()`: its path (square top, rounded bottom) is now `B(t,r,h,l-r,c-h,[d,d,d,d])`,
  the library's own rounded box.

`tests.html` leaves the axes out of its pictures (it compares the plots).

## 7. Candles' and histogram bars' corners rounded (an option)

`cornerRadius` on a candlestick or histogram series: a radius in CSS pixels, or a function of the bar spacing giving
one (TradingView's rounded candles plugin takes the same). Unset, 0, negative or not a number, the bars are drawn as the
library draws them. Set, one rule for both: a bar is rounded when it's 3 device pixels wide or more and a device pixel
or more from the next (touching, rounded bars would overlap), by at most half its width, where that comes to half a
device pixel or more; others keep the library's rectangles. A candle's body and its border are rounded (the border a
ring: the box less its inside, the inside's radius the border's less, a plain box when that's none); a histogram bar's
far end (from its base). The rounded bars of each color are one path, filled once a frame (they never overlap: the
order isn't seen).

- `bvRadius(option, spacing)`, `bvCorner(radius, ratio, width, gap)`, `bvPath(map, color)`, `bvFill(context, map,
  rule)` added (module level, before `B`).
- Candles: the pane view's `NM()` hands `cornerRadius` over (`bvR`); `class tr`'s `Vm()` (borders) and `wS()` (bodies).
- Histogram: its pane view's `NM()` the same; `class sr`'s `et()`, which also sets the fill color only when it changes
  (it set it for every bar: the same pixels, less work).

`tests.html`: with no radius (0, negative, NaN, a function giving 0) the pictures are 5.2.1's, step by step; with one
(Studio's rule, a third of the spacing, 2, 1000), at spacings from 0.5 to 60, rounding only takes from the bars (no
pixel painted where they left the background), changes no more than their corners hold, changes nothing where bars
touch (a device pixel apart or less), and changes something from 6 device pixels apart. Run at a true 2× too (Chromium with `--force-device-scale-factor=2`: an emulated ratio
gives 1× canvases, and Mullvad's fingerprinting protection scrambles a canvas read back).

Measured (Chromium at a true 2×, the library's time over a 2.5 s pan, medians of 6, before → rounded with Studio's
rule): candles 60 → 56 ms at a spacing of 6, 50 → 45.5 at 12; a two-color histogram 44.5 → 51 at 6 (+15%), 41 → 41 at
12. Not rounded (spacing 3), as before or less.

## Measured (Chromium, CPU time in script; scratch benchmarks, before → after)

- 40 layers: a crosshair sweep −45% (Max) and −56% (1Y); a pan −18%, a wheel zoom −19% (Max).
- A load of 96,000 points × 3 series (a fine interval's history, at its most): 108 → 88 ms.
- A restyle (a spotlight's): 14.5 → 5 ms.
- Mullvad (Firefox), 40 layers at Max: the longest frame of a pan 167–200 → 83–100 ms.
