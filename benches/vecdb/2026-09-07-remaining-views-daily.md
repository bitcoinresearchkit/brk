# Remaining lazy views and DailyView — 2026-09-07

## Method

Local synthetic benchmarks, not production endpoint or indexer throughput.
Benchmarks use the workspace's optimized test profile: opt-level 2, thin LTO,
native CPU flags, and debug assertions. Before measurements were captured before
changing these views. The final measurements below were run sequentially from
already-built binaries, after our compilation jobs finished.

There was substantial timing noise during concurrent builds. The shared target
directory also lost files during a full test build; final verification used
`/private/tmp/bitview-view-verification.E4UAhn`. No shared build directory was
cleaned by this task. Do not treat these figures as stable production baselines.

Delta/remaining-view fixtures contain 262,144 values; short reads select the last
64. Rolling windows are 131,072. Nested cases use a real multiply-by-two
transformation, not identity forwarding. Each reports the median of nine timed
rounds after warmup, rotating case order and verifying every output.

DailyView repeats each daily value 144 times into 262,144 outputs, including
unavailable final days. Its short range contains 1,024 outputs. Both cached and
disk-backed sources/mappings are measured. DailyView uses eleven timed rounds
after warmup. A copy of the old cursor algorithm is retained in the benchmark
for same-process before/after comparisons. LastDay tests use selective weekly
mappings with missing trailing periods.

## DailyView decision

Keep the single mapping read followed by a simple slice lookup loop. It reuses
the same mapping and source snapshots for direct reads, folds, and bounded output
chunks. Only the daily source span needed by the requested range is loaded.

Same-process comparison, median microseconds:

| RepeatDay input | Range | Old cursor | Selected implementation |
|---|---|---:|---:|
| Pco | 1,024 outputs | 7.625 | 7.666 |
| Pco | 262,144 outputs | 849.792 | 515.500 |
| Cached | 1,024 outputs | 0.834 | 0.708 |
| Cached | 262,144 outputs | 423.333 | 149.708 |

Full reads are approximately **1.65× faster on Pco** and **2.83× faster cached**.
The short disk-backed read is effectively unchanged.

Other measured alternatives: two endpoint reads plus collected mappings,
two endpoint reads plus borrowed mapping chunks, and run-length filling.
Run filling slightly improves very repetitive mappings but badly regresses
low-repeat mappings. On the 32,768-output cached density fixture:

| Outputs per day | Simple lookup (µs) | Run filling (µs) |
|---:|---:|---:|
| 1 | 23.125 | 918.375 |
| 6 | 20.125 | 156.208 |
| 144 | 20.125 | 14.875 |

Do not add a density heuristic or special fast path for that modest 144-repeat
gain. The simpler lookup algorithm generalizes.

Keep LastDay's deduplicated selective-read algorithm. The alternative that reads
direct indices without deduplicated slots was slower: full cached 1.625 vs 1.500 µs,
full Pco 6.125 vs 5.875 µs. LastDay now supports bounded output chunks and avoids
an existing underflow for empty periods/source (`then_some(next_first - 1)`
eagerly evaluated the subtraction; the conditional is now lazy).

## Remaining views

Median microseconds; these are separate before/after runs, unlike the paired
DailyView comparison.

| View/input | Range/path | Before | After |
|---|---|---:|---:|
| Delta/Pco | short direct | 201.000 | 5.917 |
| Delta/Pco | short nested | 201.583 | 6.041 |
| Delta/Pco | full direct | 1042.750 | 612.792 |
| Delta/Pco | full nested | 13042.875 | 724.833 |
| Delta/cached | short direct | 23.625 | 0.292 |
| Delta/cached | short nested | 18.250 | 0.375 |
| Delta/cached | full direct | 698.708 | 298.417 |
| Delta/cached | full nested | 1680.375 | 409.250 |
| Rolling ratio | short direct | 0.958 | 0.541 |
| Rolling ratio | short nested | 1.125 | 0.500 |
| Rolling ratio | full direct | 875.834 | 531.959 |
| Rolling ratio | full nested | 1393.542 | 641.292 |
| Block-count ratio | short direct | 0.417 | 0.292 |
| Block-count ratio | short nested | 0.375 | 0.583 |
| Block-count ratio | full direct | 310.333 | 213.333 |
| Block-count ratio | full nested | 432.291 | 358.000 |
| Rolling block-count ratio | short direct | 1.000 | 0.875 |
| Rolling block-count ratio | short nested | 0.916 | 1.042 |
| Rolling block-count ratio | full direct | 666.917 | 726.625 |
| Rolling block-count ratio | full nested | 937.500 | 888.959 |
| Lazy column | short direct | 0.208 | 0.208 |
| Lazy column | short nested | 0.208 | 0.208 |
| Lazy column | full direct | 251.916 | 127.750 |
| Lazy column | full nested | 326.833 | 213.750 |
| Lazy column | full rows | 883.041 | 512.500 |

The block-count results are mixed, not an across-the-board speedup: small nested
reads have extra batching overhead and the rolling direct measurement is about
9% slower than the earlier run. Its direct checkpoint-sum algorithm is retained.
An attempted iterator-based denominator rewrite regressed more substantially
and was rejected; CachedBlockCountReader remains unchanged. Chunk visitors now
use the retained fold algorithm and reusable output scratch.

Delta reads skip the unused gap between disjoint historical/current ranges and
merge overlapping ranges. General rolling ratios share a static transform
iterator across bulk reads/folds. LazyColumnarVec selects monomorphized row and
column loops at construction instead of dispatching the transform per element.

No persistent derived-output or full cumulative block-count cache was added.
Fallible consumers retain scalar transform/error ordering. Source reads finish
before ratio/delta consumers run, avoiding publication-lock reentry.

## Verification

Passed:

- Full `vecdb` and `bitview_compute` test suites, including doctests, using the
  isolated target and `CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_TEST_LTO=false`.
  Those profile overrides were for correctness tests only, not reported benchmarks.
- Standalone default-feature `vecdb` chunk/columnar tests, plus Pco-enabled tests.
- `cargo check --locked -p bitview_default -p bitview_server`.
- Scoped rustfmt checks and `git diff --check`.

Coverage includes direct/nested/chunk/fold/append/sorted paths, empty/reversed
ranges, metadata shorter than sources, cached and disk-backed inputs, missing
and duplicate daily mappings, selective LastDay source reads, scalar early
failure, delta read counts for disjoint/overlapping windows, cache invalidation,
and non-reentrant source reads.

## Reproduce

```sh
cargo test --locked -p vecdb --features pco --test lazy_delta_bench -- --ignored --nocapture --test-threads=1
cargo test --locked -p bitview_compute --test remaining_views_bench -- --ignored --nocapture --test-threads=1
cargo test --locked -p bitview_compute --test daily_view_bench -- --ignored --nocapture --test-threads=1
```

Use an isolated `--target-dir` if another process is cleaning/rebuilding the
workspace. Run the resulting binaries after compilation has stopped for less
noisy timing.

## Final raw output

### daily

```text
running 2 tests
test benchmark_daily_view ... repeat/cached=false/from=261120/view: median=7.666µs min=7.541µs max=8.917µs
repeat/cached=false/from=261120/collected: median=21.041µs min=20.75µs max=22.334µs
repeat/cached=false/from=261120/chunks: median=20.958µs min=20.708µs max=23.5µs
repeat/cached=false/from=261120/runs: median=20.5µs min=20.375µs max=21.125µs
repeat/cached=false/from=261120/one_read: median=7.583µs min=7.458µs max=8µs
repeat/cached=false/from=261120/one_read_runs: median=7.25µs min=7.125µs max=8.292µs
repeat/cached=false/from=261120/baseline_cursor: median=7.625µs min=7.5µs max=7.833µs
repeat/cached=false/from=0/view: median=515.5µs min=514.75µs max=518.334µs
repeat/cached=false/from=0/collected: median=531µs min=528.958µs max=531.625µs
repeat/cached=false/from=0/chunks: median=588.125µs min=585.625µs max=596.042µs
repeat/cached=false/from=0/runs: median=526.541µs min=523.833µs max=529.417µs
repeat/cached=false/from=0/one_read: median=517.167µs min=515.25µs max=517.958µs
repeat/cached=false/from=0/one_read_runs: median=507.375µs min=505.5µs max=510.333µs
repeat/cached=false/from=0/baseline_cursor: median=849.792µs min=848.5µs max=859.5µs
last/cached=false/from=240/view: median=542ns min=458ns max=1.333µs
last/cached=false/from=240/direct_indices: median=625ns min=542ns max=2.416µs
last/cached=false/from=0/view: median=5.875µs min=5.708µs max=8.125µs
last/cached=false/from=0/direct_indices: median=6.125µs min=5.875µs max=7.209µs
repeat/cached=true/from=261120/view: median=708ns min=708ns max=792ns
repeat/cached=true/from=261120/collected: median=750ns min=708ns max=833ns
repeat/cached=true/from=261120/chunks: median=667ns min=625ns max=709ns
repeat/cached=true/from=261120/runs: median=292ns min=291ns max=292ns
repeat/cached=true/from=261120/one_read: median=708ns min=666ns max=750ns
repeat/cached=true/from=261120/one_read_runs: median=333ns min=291ns max=334ns
repeat/cached=true/from=261120/baseline_cursor: median=834ns min=791ns max=917ns
repeat/cached=true/from=0/view: median=149.708µs min=149.083µs max=150.208µs
repeat/cached=true/from=0/collected: median=151µs min=150.375µs max=151.792µs
repeat/cached=true/from=0/chunks: median=143.458µs min=143.208µs max=143.833µs
repeat/cached=true/from=0/runs: median=132.958µs min=132.209µs max=134.041µs
repeat/cached=true/from=0/one_read: median=150.375µs min=149.958µs max=155.375µs
repeat/cached=true/from=0/one_read_runs: median=139.833µs min=138.75µs max=141.458µs
repeat/cached=true/from=0/baseline_cursor: median=423.333µs min=422.958µs max=429.708µs
last/cached=true/from=240/view: median=209ns min=208ns max=375ns
last/cached=true/from=240/direct_indices: median=334ns min=291ns max=1.167µs
last/cached=true/from=0/view: median=1.5µs min=1.417µs max=2.042µs
last/cached=true/from=0/direct_indices: median=1.625µs min=1.541µs max=2.167µs
ok
test benchmark_repeat_density ... density/repeat=1/one_read: median=23.125µs
density/repeat=1/one_read_runs: median=918.375µs
density/repeat=6/one_read: median=20.125µs
density/repeat=6/one_read_runs: median=156.208µs
density/repeat=144/one_read: median=20.125µs
density/repeat=144/one_read_runs: median=14.875µs
ok

test result: ok. 2 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.12s
```

### remaining

```text
running 1 test
test benchmark_remaining_views ... rolling_ratio/short_direct: median=541ns min=416ns max=2.167µs
rolling_ratio/short_nested: median=500ns min=333ns max=750ns
rolling_ratio/full_direct: median=531.959µs min=527.833µs max=555.333µs
rolling_ratio/full_nested: median=641.292µs min=639.041µs max=749.25µs
block_ratio/short_direct: median=292ns min=250ns max=500ns
block_ratio/short_nested: median=583ns min=542ns max=1.208µs
block_ratio/full_direct: median=213.333µs min=212.416µs max=230.167µs
block_ratio/full_nested: median=358µs min=356.791µs max=364.75µs
rolling_block_ratio/short_direct: median=875ns min=667ns max=1µs
rolling_block_ratio/short_nested: median=1.042µs min=875ns max=1.5µs
rolling_block_ratio/full_direct: median=726.625µs min=722.917µs max=756.125µs
rolling_block_ratio/full_nested: median=888.959µs min=883.542µs max=913µs
lazy_column/short_direct: median=208ns min=125ns max=292ns
lazy_column/short_nested: median=208ns min=166ns max=292ns
lazy_column/full_direct: median=127.75µs min=127.541µs max=128.041µs
lazy_column/full_nested: median=213.75µs min=213.625µs max=213.875µs
lazy_column/rows: median=512.5µs min=429.833µs max=547.916µs
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```

### delta

```text
running 1 test
test benchmark_delta_ranges ... pco/short_direct: median=5.917µs min=5.833µs max=8.458µs
pco/short_nested: median=6.041µs min=5.333µs max=8.833µs
pco/full_direct: median=612.792µs min=612.208µs max=617.209µs
pco/full_nested: median=724.833µs min=723.583µs max=738.5µs
cached/short_direct: median=292ns min=250ns max=416ns
cached/short_nested: median=375ns min=291ns max=583ns
cached/full_direct: median=298.417µs min=295.75µs max=303.042µs
cached/full_nested: median=409.25µs min=407.584µs max=416.291µs
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```
