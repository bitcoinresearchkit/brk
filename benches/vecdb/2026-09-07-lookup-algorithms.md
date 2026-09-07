# Lookup algorithm experiments — 2026-09-07

## Decisions

1. **Keep selective sorted reads for LazyAggVec and DailyView.** Resolve only requested output buckets/days; deduplicate requested source indices and scatter through recorded slots. Consecutive requests reuse the existing range path. Sparse aggregation shares the same bucket-boundary helper across scalar, range, and sorted reads.
2. **Keep two-stream SparseRead planning.** Merge sorted current/history requests and record result slots in the merge. Lookback, window, rolling ratio, and cached-block-count rolling ratio no longer binary-search for every output. Nonmonotonic custom history retains the general sort/deduplicate fallback.
3. **Keep direct compressed-page gathering.** Copy requested values from borrowed decoded pages without copying the whole page through Cursor. Adjacent requested pages share a reader; gaps split runs so buffered I/O does not prefetch unused intervening pages. Existing mmap/I/O selection remains in force. No new cache or unsafe block.
4. **Reject dedicated single-fold aggregate chunks.** Implemented and tested, then removed from production. It amortizes mapping acquisition, but its timing advantage was small/inconsistent and Sparse's request/slot/value temporaries grow with the entire range instead of the existing bounded chunks. The benchmark retains a single-fold experimental transport and the old bounded transport for comparison.

## Method

Local synthetic fixtures, not indexer throughput or HTTP latency. The baseline was recorded immediately before this wave of production changes, on top of the preceding view/chunk work—not against Git HEAD.

- Standard workspace optimized test profile (opt-level 2, thin LTO, native CPU, debug assertions); same isolated target directory for both benchmark builds.
- 262,144 cumulative source values; RepeatDay maps blocks to day i/144; LastDay has 4,096 outputs at seven-day boundaries; sparse aggregation has 16,384 outputs at 16-element boundaries.
- cached=false uses compressed Pco-backed inputs; cached=true uses pinned resident CachedVec snapshots.
- One warmup followed by 11 timed reads; median, minimum, maximum reported. Output allocation is timed; independent expected-output assertions run after timing.
- one: one near-tail index; clustered: 32 consecutive near-tail indices; spread: 32 indices spanning the whole output; dense: final 4,096 outputs.
- Full aggregate direct/nested tests emit all 16,384 outputs; the nested transform doubles each present value.
- Compressed tests are warm OS-cache reads, **not cold disk benchmarks**. Sub-microsecond results approach timer granularity; do not interpret enormous ratios at that scale literally. Baseline and final are separate runs, so modest differences may be noise.
- The initial production chunk candidate measured 438.750 vs 456.083 microseconds for compressed nested output, and 76.333 vs 79.000 microseconds resident, relative to a same-process legacy-loop emulation. This was insufficient to justify its memory tradeoff.

## Validation

Targeted tests cover selective source evaluation, duplicate indices, empty buckets, missing/duplicate days, published source bounds, out-of-range requests, append-to-buffer behavior, compressed/raw page boundaries, unflushed pushes, truncation/rewrite/reopen, and forced buffered-I/O and mmap decoders. SparseRead separately covers missing and nonmonotonic history.

Reproduction:

```sh
cargo test --locked -p bitview_compute --test lookup_algorithms_bench -- --ignored --nocapture --test-threads=1
CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_TEST_LTO=false cargo test --locked -p vecdb --all-features -p bitview_compute
cargo check --locked -p bitview_default -p bitview_server
```

## Before / after medians

All durations below are microseconds. Speedup = before / after; below 1.00 is slower.

| Workload | Before | After | Speedup |
|---|---:|---:|---:|
| repeat/cached=false/one | 13.916 | 9.917 | 1.40x |
| repeat/cached=false/clustered | 13.958 | 10.083 | 1.38x |
| repeat/cached=false/spread | 436.666 | 222.833 | 1.96x |
| repeat/cached=false/dense | 20.917 | 14.209 | 1.47x |
| last/cached=false/one | 101.541 | 6.458 | 15.72x |
| last/cached=false/clustered | 104.875 | 6.667 | 15.73x |
| last/cached=false/spread | 98.458 | 44.417 | 2.22x |
| last/cached=false/dense | 105.083 | 61.750 | 1.70x |
| agg/cached=false/one | 191.708 | 2.666 | 71.91x |
| agg/cached=false/clustered | 182.625 | 2.875 | 63.52x |
| agg/cached=false/spread | 728.833 | 83.667 | 8.71x |
| agg/cached=false/dense | 184.292 | 107.000 | 1.72x |
| agg/cached=false/full_direct | 704.250 | 440.791 | 1.60x |
| agg/cached=false/full_nested | 717.666 | 442.959 | 1.62x |
| source/cached=false/one | 2.500 | 2.667 | 0.94x |
| source/cached=false/clustered | 3.250 | 2.708 | 1.20x |
| source/cached=false/spread | 80.167 | 83.208 | 0.96x |
| source/cached=false/dense | 14.042 | 10.917 | 1.29x |
| lookback/cached=false/one | 5.042 | 5.167 | 0.98x |
| lookback/cached=false/clustered | 6.208 | 5.792 | 1.07x |
| lookback/cached=false/spread | 158.000 | 159.416 | 0.99x |
| lookback/cached=false/dense | 183.667 | 39.333 | 4.67x |
| window/cached=false/one | 5.000 | 5.250 | 0.95x |
| window/cached=false/clustered | 6.041 | 5.875 | 1.03x |
| window/cached=false/spread | 149.791 | 159.334 | 0.94x |
| window/cached=false/dense | 172.083 | 43.083 | 3.99x |
| ratio/cached=false/one | 5.167 | 5.209 | 0.99x |
| ratio/cached=false/clustered | 6.541 | 5.875 | 1.11x |
| ratio/cached=false/spread | 164.750 | 160.917 | 1.02x |
| ratio/cached=false/dense | 185.959 | 44.792 | 4.15x |
| rolling_block/cached=false/one | 5.125 | 5.458 | 0.94x |
| rolling_block/cached=false/clustered | 12.583 | 12.500 | 1.01x |
| rolling_block/cached=false/spread | 166.250 | 165.292 | 1.01x |
| rolling_block/cached=false/dense | 695.416 | 543.917 | 1.28x |
| repeat/cached=true/one | 2.583 | 0.083 | 31.12x |
| repeat/cached=true/clustered | 3.250 | 0.125 | 26.00x |
| repeat/cached=true/spread | 79.541 | 8.333 | 9.55x |
| repeat/cached=true/dense | 9.083 | 4.500 | 2.02x |
| last/cached=true/one | 21.208 | 0.084 | 252.48x |
| last/cached=true/clustered | 21.167 | 0.333 | 63.56x |
| last/cached=true/spread | 21.208 | 0.625 | 33.93x |
| last/cached=true/dense | 27.875 | 23.917 | 1.17x |
| agg/cached=true/one | 15.375 | 0.042 | 366.07x |
| agg/cached=true/clustered | 15.250 | 0.250 | 61.00x |
| agg/cached=true/spread | 65.292 | 0.250 | 261.17x |
| agg/cached=true/dense | 23.500 | 19.708 | 1.19x |
| agg/cached=true/full_direct | 70.625 | 73.375 | 0.96x |
| agg/cached=true/full_nested | 75.875 | 75.250 | 1.01x |
| source/cached=true/one | 0.042 | 0.042 | 1.00x |
| source/cached=true/clustered | 0.083 | 0.083 | 1.00x |
| source/cached=true/spread | 0.083 | 0.042 | 1.98x |
| source/cached=true/dense | 3.584 | 4.333 | 0.83x |
| lookback/cached=true/one | 0.042 | 0.042 | 1.00x |
| lookback/cached=true/clustered | 0.917 | 0.417 | 2.20x |
| lookback/cached=true/spread | 1.167 | 0.375 | 3.11x |
| lookback/cached=true/dense | 162.875 | 29.500 | 5.52x |
| window/cached=true/one | 0.042 | 0.042 | 1.00x |
| window/cached=true/clustered | 1.000 | 0.542 | 1.85x |
| window/cached=true/spread | 1.000 | 0.458 | 2.18x |
| window/cached=true/dense | 167.458 | 36.709 | 4.56x |
| ratio/cached=true/one | 0.042 | 0.042 | 1.00x |
| ratio/cached=true/clustered | 0.958 | 0.583 | 1.64x |
| ratio/cached=true/spread | 1.000 | 0.500 | 2.00x |
| ratio/cached=true/dense | 169.292 | 34.917 | 4.85x |
| rolling_block/cached=true/one | 0.166 | 0.208 | 0.80x |
| rolling_block/cached=true/clustered | 6.875 | 7.125 | 0.96x |
| rolling_block/cached=true/spread | 4.500 | 4.417 | 1.02x |
| rolling_block/cached=true/dense | 695.709 | 537.042 | 1.30x |

Large gains are concentrated in selective derived lookups and dense current/history batches. Sparse compressed source reads remain decoder-dominated and roughly flat (spread: 80.167 to 83.208); resident full aggregate direct reads also remain roughly flat (70.625 to 73.375). Resident source dense reads increased from 3.584 to 4.333 despite their unchanged cached implementation. These small-duration results are not a claim of universal improvement.

The final single-fold experiment versus the bounded legacy emulation was 424.542 vs 444.958 microseconds compressed and 73.375 vs 76.083 resident. The experimental implementation remains benchmark-only.

Validation completed successfully: full vecdb/bitview_compute tests with all features (including Pco/LZ4/Zstd), default Bitview/server integration compilation, and git diff whitespace checks. An existing unused ColumnId import warning remains in bounded_cohorts.rs. Ignored unrelated benchmarks were not run by the full suite; this wave's ignored benchmark was explicitly run and validated.

## Raw baseline

```text
running 1 test
test benchmark_lookup_algorithms ... repeat/cached=false/one: median=13.916µs min=13.792µs max=14.583µs
repeat/cached=false/clustered: median=13.958µs min=13.792µs max=14.125µs
repeat/cached=false/spread: median=436.666µs min=416.25µs max=438.542µs
repeat/cached=false/dense: median=20.917µs min=20.666µs max=21.958µs
last/cached=false/one: median=101.541µs min=100.459µs max=105.292µs
last/cached=false/clustered: median=104.875µs min=96.958µs max=137.834µs
last/cached=false/spread: median=98.458µs min=97.291µs max=103.041µs
last/cached=false/dense: median=105.083µs min=103.833µs max=109.541µs
agg/cached=false/one: median=191.708µs min=190.5µs max=193.209µs
agg/cached=false/clustered: median=182.625µs min=181.5µs max=193.292µs
agg/cached=false/spread: median=728.833µs min=719.167µs max=760.667µs
agg/cached=false/dense: median=184.292µs min=182.666µs max=190.5µs
agg/cached=false/full_direct: median=704.25µs min=674.292µs max=750.167µs
agg/cached=false/full_nested: median=717.666µs min=704.75µs max=757.541µs
source/cached=false/one: median=2.5µs min=2.5µs max=2.917µs
source/cached=false/clustered: median=3.25µs min=2.542µs max=3.334µs
source/cached=false/spread: median=80.167µs min=78.542µs max=87.334µs
source/cached=false/dense: median=14.042µs min=13.958µs max=14.75µs
lookback/cached=false/one: median=5.042µs min=4.958µs max=5.458µs
lookback/cached=false/clustered: median=6.208µs min=6.042µs max=9.167µs
lookback/cached=false/spread: median=158µs min=154.542µs max=175.834µs
lookback/cached=false/dense: median=183.667µs min=175.791µs max=245.625µs
window/cached=false/one: median=5µs min=4.916µs max=5.209µs
window/cached=false/clustered: median=6.041µs min=5.958µs max=7.958µs
window/cached=false/spread: median=149.791µs min=146.125µs max=165.833µs
window/cached=false/dense: median=172.083µs min=170.708µs max=192.792µs
ratio/cached=false/one: median=5.167µs min=4.75µs max=6.417µs
ratio/cached=false/clustered: median=6.541µs min=6.375µs max=7.875µs
ratio/cached=false/spread: median=164.75µs min=149.5µs max=177.709µs
ratio/cached=false/dense: median=185.959µs min=174.084µs max=205.208µs
rolling_block/cached=false/one: median=5.125µs min=5.042µs max=7µs
rolling_block/cached=false/clustered: median=12.583µs min=12.458µs max=16.5µs
rolling_block/cached=false/spread: median=166.25µs min=159.75µs max=178.375µs
rolling_block/cached=false/dense: median=695.416µs min=678.125µs max=788.042µs
repeat/cached=true/one: median=2.583µs min=2.5µs max=2.667µs
repeat/cached=true/clustered: median=3.25µs min=2.541µs max=3.541µs
repeat/cached=true/spread: median=79.541µs min=77.167µs max=91.666µs
repeat/cached=true/dense: median=9.083µs min=8.833µs max=10.792µs
last/cached=true/one: median=21.208µs min=18.708µs max=21.625µs
last/cached=true/clustered: median=21.167µs min=19.084µs max=26.125µs
last/cached=true/spread: median=21.208µs min=18.833µs max=25.75µs
last/cached=true/dense: median=27.875µs min=25.084µs max=31.417µs
agg/cached=true/one: median=15.375µs min=14.958µs max=16.125µs
agg/cached=true/clustered: median=15.25µs min=15.042µs max=17.25µs
agg/cached=true/spread: median=65.292µs min=62.875µs max=73.833µs
agg/cached=true/dense: median=23.5µs min=21.291µs max=25.709µs
agg/cached=true/full_direct: median=70.625µs min=60.75µs max=76.25µs
agg/cached=true/full_nested: median=75.875µs min=73µs max=88.125µs
source/cached=true/one: median=42ns min=0ns max=42ns
source/cached=true/clustered: median=83ns min=41ns max=84ns
source/cached=true/spread: median=83ns min=41ns max=125ns
source/cached=true/dense: median=3.584µs min=3.417µs max=4.709µs
lookback/cached=true/one: median=42ns min=41ns max=84ns
lookback/cached=true/clustered: median=917ns min=875ns max=1.208µs
lookback/cached=true/spread: median=1.167µs min=1.125µs max=1.333µs
lookback/cached=true/dense: median=162.875µs min=159.375µs max=189.875µs
window/cached=true/one: median=42ns min=41ns max=83ns
window/cached=true/clustered: median=1µs min=958ns max=1.084µs
window/cached=true/spread: median=1µs min=917ns max=1.167µs
window/cached=true/dense: median=167.458µs min=154.125µs max=192.167µs
ratio/cached=true/one: median=42ns min=41ns max=84ns
ratio/cached=true/clustered: median=958ns min=916ns max=1.042µs
ratio/cached=true/spread: median=1µs min=917ns max=1.167µs
ratio/cached=true/dense: median=169.292µs min=153µs max=188.875µs
rolling_block/cached=true/one: median=166ns min=125ns max=250ns
rolling_block/cached=true/clustered: median=6.875µs min=6.833µs max=7.458µs
rolling_block/cached=true/spread: median=4.5µs min=4.458µs max=4.917µs
rolling_block/cached=true/dense: median=695.709µs min=649.833µs max=772.416µs
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.11s
```

## Raw final

```text
running 1 test
test benchmark_lookup_algorithms ... repeat/cached=false/one: median=9.917µs min=9.708µs max=10.625µs
repeat/cached=false/clustered: median=10.083µs min=9.959µs max=10.25µs
repeat/cached=false/spread: median=222.833µs min=221.584µs max=244.292µs
repeat/cached=false/dense: median=14.209µs min=14.083µs max=14.584µs
last/cached=false/one: median=6.458µs min=6.292µs max=6.667µs
last/cached=false/clustered: median=6.667µs min=6.625µs max=8.5µs
last/cached=false/spread: median=44.417µs min=43.875µs max=44.75µs
last/cached=false/dense: median=61.75µs min=60.916µs max=65.208µs
agg/cached=false/one: median=2.666µs min=2.625µs max=2.708µs
agg/cached=false/clustered: median=2.875µs min=2.833µs max=3.208µs
agg/cached=false/spread: median=83.667µs min=82.75µs max=84.5µs
agg/cached=false/dense: median=107µs min=105.541µs max=109.25µs
agg/cached=false/full_direct: median=440.791µs min=436.791µs max=447.333µs
agg/cached=false/full_nested: median=442.959µs min=441.459µs max=449.792µs
agg/cached=false/experimental_single_fold_nested: median=424.542µs min=422.042µs max=428.583µs
agg/cached=false/legacy_nested: median=444.958µs min=443.417µs max=450.042µs
source/cached=false/one: median=2.667µs min=2.583µs max=3.459µs
source/cached=false/clustered: median=2.708µs min=2.666µs max=3.041µs
source/cached=false/spread: median=83.208µs min=82.083µs max=83.958µs
source/cached=false/dense: median=10.917µs min=10.791µs max=11.167µs
lookback/cached=false/one: median=5.167µs min=5.125µs max=5.291µs
lookback/cached=false/clustered: median=5.792µs min=5.625µs max=8.75µs
lookback/cached=false/spread: median=159.416µs min=157.917µs max=162.375µs
lookback/cached=false/dense: median=39.333µs min=38.834µs max=39.75µs
window/cached=false/one: median=5.25µs min=5.167µs max=5.417µs
window/cached=false/clustered: median=5.875µs min=5.75µs max=8.458µs
window/cached=false/spread: median=159.334µs min=157.875µs max=162.208µs
window/cached=false/dense: median=43.083µs min=42.667µs max=43.417µs
ratio/cached=false/one: median=5.209µs min=5.166µs max=5.458µs
ratio/cached=false/clustered: median=5.875µs min=5.75µs max=6.125µs
ratio/cached=false/spread: median=160.917µs min=160.375µs max=163µs
ratio/cached=false/dense: median=44.792µs min=44.209µs max=45.084µs
rolling_block/cached=false/one: median=5.458µs min=5.416µs max=7.834µs
rolling_block/cached=false/clustered: median=12.5µs min=12.333µs max=14.583µs
rolling_block/cached=false/spread: median=165.292µs min=163.625µs max=166.458µs
rolling_block/cached=false/dense: median=543.917µs min=534.209µs max=545.708µs
repeat/cached=true/one: median=83ns min=42ns max=125ns
repeat/cached=true/clustered: median=125ns min=125ns max=250ns
repeat/cached=true/spread: median=8.333µs min=6.708µs max=10.875µs
repeat/cached=true/dense: median=4.5µs min=4.417µs max=5.833µs
last/cached=true/one: median=84ns min=41ns max=166ns
last/cached=true/clustered: median=333ns min=291ns max=625ns
last/cached=true/spread: median=625ns min=583ns max=792ns
last/cached=true/dense: median=23.917µs min=21.833µs max=39.916µs
agg/cached=true/one: median=42ns min=41ns max=84ns
agg/cached=true/clustered: median=250ns min=250ns max=458ns
agg/cached=true/spread: median=250ns min=208ns max=334ns
agg/cached=true/dense: median=19.708µs min=17.416µs max=20.291µs
agg/cached=true/full_direct: median=73.375µs min=72.083µs max=74.75µs
agg/cached=true/full_nested: median=75.25µs min=73µs max=85µs
agg/cached=true/experimental_single_fold_nested: median=73.375µs min=64.292µs max=86.917µs
agg/cached=true/legacy_nested: median=76.083µs min=73.458µs max=84.125µs
source/cached=true/one: median=42ns min=0ns max=42ns
source/cached=true/clustered: median=83ns min=42ns max=84ns
source/cached=true/spread: median=42ns min=41ns max=84ns
source/cached=true/dense: median=4.333µs min=3.541µs max=4.375µs
lookback/cached=true/one: median=42ns min=41ns max=83ns
lookback/cached=true/clustered: median=417ns min=375ns max=625ns
lookback/cached=true/spread: median=375ns min=375ns max=542ns
lookback/cached=true/dense: median=29.5µs min=28.5µs max=33.958µs
window/cached=true/one: median=42ns min=41ns max=83ns
window/cached=true/clustered: median=542ns min=500ns max=833ns
window/cached=true/spread: median=458ns min=416ns max=541ns
window/cached=true/dense: median=36.709µs min=32.917µs max=44.667µs
ratio/cached=true/one: median=42ns min=41ns max=83ns
ratio/cached=true/clustered: median=583ns min=541ns max=750ns
ratio/cached=true/spread: median=500ns min=458ns max=583ns
ratio/cached=true/dense: median=34.917µs min=34.334µs max=39.084µs
rolling_block/cached=true/one: median=208ns min=166ns max=333ns
rolling_block/cached=true/clustered: median=7.125µs min=7.083µs max=7.375µs
rolling_block/cached=true/spread: median=4.417µs min=4.375µs max=4.541µs
rolling_block/cached=true/dense: median=537.042µs min=532.667µs max=541.458µs
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.07s
```
