# Lookup followups — 2026-09-07

## Outcome

All four candidates were implemented, benchmarked, and retained.

1. **Block-count batches:** snapshot counts/checkpoints once; advance the preceding sum when it is cheaper than rescanning from the nearest checkpoint. Large gaps restart at a checkpoint; duplicates do not rescan. The rolling-ratio caller now batches both endpoint sums rather than acquiring a snapshot and reconstructing sums separately for every output.
2. **Shared delta lookup planning:** moved the existing SparseRead implementation from bitview_compute into vecdb and reused it in LazyDeltaVec. One merge/slot implementation serves both crates. Monotonic related indices use linear merging; custom nonmonotonic metadata keeps the sort/deduplicate fallback.
3. **Neighbour gathers:** previous-delta and next-index count views batch sparse neighbour requests through that shared planner; cumulative next-index requests forward shifted indices directly. Consecutive previous-delta/count requests reuse existing range reads. Dense duplicate-heavy requests retain the cursor loop. Mutable sources retain their cursor semantics; a missing requested value also prevents using the paired gather result.
4. **Column projections:** added sorted-column forwarding through stored, cached, and transformed columnar sources. A projection now reaches the selected underlying column's native gather. Dense duplicate-heavy transformed requests evaluate the requested span once and gather from it.

No new persistent cache, data layout, or multi-source lazy vector. The shared planner replaces duplicated planning code rather than introducing a parallel implementation.

## Method and limits

The baseline was run before this wave's production edits, with the same benchmark source and standard workspace test profile: opt-level 2, thin LTO, native CPU, debug assertions. Final measurements ran from built binaries after this task's compilation/tests completed. Other work on the machine was not stopped.

- 262,144 source values; window width 2,017 inclusive elements.
- Previous-delta values increase by 3; next-index boundaries are three elements apart.
- Two scalar columns (3*i and 5*i), plus a doubling column transform.
- Variable block counts i%7 test checkpoint reconstruction independently.
- cached=false uses Pco-backed source inputs; cached=true uses resident snapshots. **The block_count reader is resident in both passes**; the label describes the surrounding fixture mode, not uncached block-count storage.
- one: one near-tail index; clustered: 32 consecutive near-tail indices; spread: 32 positions across the whole source; dense: 4,096 consecutive tail indices; duplicates: 4,096 requests covering 1,024 consecutive positions, four requests per position.
- One warmup and 11 timed reads; median/min/max; output allocation timed, independent output assertions outside timing.
- Warm OS-cache reads, not cold disk, indexer throughput, or HTTP latency. Very small timings approach timer resolution.
- These are separate before/after runs, not a controlled same-process A/B comparison. Several compressed sparse cases were slower in the final run (for example delta spread 162.209 to 190.292 us), despite earlier post-change samples being roughly flat. The data does not establish whether those differences are code cost or machine variability; there is no universal-speedup claim.

## Results

All times below are microseconds. Speedup = before / after; values below 1.00 are slower.

| Workload | Before | After | Speedup |
|---|---:|---:|---:|
| delta/cached=false/one | 4.125 | 4.833 | 0.85x |
| delta/cached=false/clustered | 6.500 | 6.875 | 0.95x |
| delta/cached=false/spread | 162.209 | 190.292 | 0.85x |
| delta/cached=false/dense | 140.791 | 47.625 | 2.96x |
| delta/cached=false/duplicates | 120.666 | 37.833 | 3.19x |
| previous/cached=false/one | 3.208 | 2.917 | 1.10x |
| previous/cached=false/clustered | 2.916 | 3.000 | 0.97x |
| previous/cached=false/spread | 82.167 | 93.041 | 0.88x |
| previous/cached=false/dense | 28.166 | 17.708 | 1.59x |
| previous/cached=false/duplicates | 14.166 | 14.042 | 1.01x |
| next_cumulative/cached=false/one | 3.500 | 3.125 | 1.12x |
| next_cumulative/cached=false/clustered | 3.458 | 3.209 | 1.08x |
| next_cumulative/cached=false/spread | 84.167 | 93.875 | 0.90x |
| next_cumulative/cached=false/dense | 13.125 | 11.667 | 1.12x |
| next_cumulative/cached=false/duplicates | 8.959 | 8.625 | 1.04x |
| next_count/cached=false/one | 3.458 | 3.125 | 1.11x |
| next_count/cached=false/clustered | 3.000 | 3.125 | 0.96x |
| next_count/cached=false/spread | 86.333 | 99.209 | 0.87x |
| next_count/cached=false/dense | 20.583 | 9.584 | 2.15x |
| next_count/cached=false/duplicates | 12.000 | 10.334 | 1.16x |
| column/cached=false/one | 3.542 | 2.958 | 1.20x |
| column/cached=false/clustered | 2.958 | 2.834 | 1.04x |
| column/cached=false/spread | 86.667 | 87.417 | 0.99x |
| column/cached=false/dense | 15.417 | 11.459 | 1.35x |
| column/cached=false/duplicates | 8.709 | 7.041 | 1.24x |
| column_transformed/cached=false/one | 3.916 | 3.625 | 1.08x |
| column_transformed/cached=false/clustered | 3.375 | 3.000 | 1.13x |
| column_transformed/cached=false/spread | 102.792 | 90.334 | 1.14x |
| column_transformed/cached=false/dense | 17.541 | 15.292 | 1.15x |
| column_transformed/cached=false/duplicates | 7.417 | 6.291 | 1.18x |
| block_count/cached=false/one | 0.166 | 0.166 | 1.00x |
| block_count/cached=false/clustered | 3.042 | 0.250 | 12.17x |
| block_count/cached=false/spread | 1.833 | 1.917 | 0.96x |
| block_count/cached=false/dense | 237.084 | 9.958 | 23.81x |
| block_count/cached=false/duplicates | 541.083 | 9.375 | 57.72x |
| delta/cached=true/one | 0.167 | 0.084 | 1.99x |
| delta/cached=true/clustered | 0.792 | 0.417 | 1.90x |
| delta/cached=true/spread | 0.959 | 0.417 | 2.30x |
| delta/cached=true/dense | 134.209 | 35.667 | 3.76x |
| delta/cached=true/duplicates | 115.292 | 28.916 | 3.99x |
| previous/cached=true/one | 0.167 | 0.209 | 0.80x |
| previous/cached=true/clustered | 0.292 | 0.083 | 3.52x |
| previous/cached=true/spread | 6.791 | 0.334 | 20.33x |
| previous/cached=true/dense | 15.791 | 2.542 | 6.21x |
| previous/cached=true/duplicates | 8.792 | 7.750 | 1.13x |
| next_cumulative/cached=true/one | 0.250 | 0.208 | 1.20x |
| next_cumulative/cached=true/clustered | 0.250 | 0.291 | 0.86x |
| next_cumulative/cached=true/spread | 7.292 | 0.166 | 43.93x |
| next_cumulative/cached=true/dense | 6.416 | 5.292 | 1.21x |
| next_cumulative/cached=true/duplicates | 5.125 | 5.167 | 0.99x |
| next_count/cached=true/one | 0.208 | 0.250 | 0.83x |
| next_count/cached=true/clustered | 0.292 | 0.084 | 3.48x |
| next_count/cached=true/spread | 8.125 | 0.375 | 21.67x |
| next_count/cached=true/dense | 15.250 | 4.750 | 3.21x |
| next_count/cached=true/duplicates | 8.625 | 7.125 | 1.21x |
| column/cached=true/one | 0.208 | 0.042 | 4.95x |
| column/cached=true/clustered | 0.250 | 0.042 | 5.95x |
| column/cached=true/spread | 7.250 | 0.042 | 172.62x |
| column/cached=true/dense | 4.875 | 3.458 | 1.41x |
| column/cached=true/duplicates | 4.042 | 3.417 | 1.18x |
| column_transformed/cached=true/one | 0.625 | 0.042 | 14.88x |
| column_transformed/cached=true/clustered | 0.667 | 0.125 | 5.34x |
| column_transformed/cached=true/spread | 20.834 | 0.125 | 166.67x |
| column_transformed/cached=true/dense | 8.167 | 8.542 | 0.96x |
| column_transformed/cached=true/duplicates | 5.500 | 3.334 | 1.65x |
| block_count/cached=true/one | 0.167 | 0.167 | 1.00x |
| block_count/cached=true/clustered | 3.708 | 0.250 | 14.83x |
| block_count/cached=true/spread | 2.250 | 1.834 | 1.23x |
| block_count/cached=true/dense | 237.083 | 9.625 | 24.63x |
| block_count/cached=true/duplicates | 237.917 | 9.000 | 26.44x |

## Rolling-ratio integration benchmark

This is the existing lookup_algorithms_bench fixture, with all-one counts and 4,096 dense tail requests. The before values come from the final preceding-wave run recorded in 2026-09-07-lookup-algorithms.md; the after values were measured after this wave. All output assertions passed.

| Numerator source | Before | After | Speedup |
|---|---:|---:|---:|
| cached=false | 543.917 | 53.375 | 10.19x |
| cached=true | 537.042 | 40.250 | 13.34x |

## Verification

Passed:

- Full vecdb and bitview_compute tests with all features, including doc tests.
- Default Bitview/server integration compilation.
- Targeted delta operator tests: sum, average, change, rate; duplicate requests, shortened metadata, nonmonotonic starts, truncation and rewrite.
- Checkpoint tests: boundaries, large gaps, duplicates, nonmonotonic and invalid rolling starts, same-length rewrite.
- Column tests: stored/transformed/cached reads, duplicate-heavy requests, cache denial, invalidation, rewrite, append-to-output and bounds.
- Next-index terminal-bound and rewrite checks; mutable previous-delta cursor-alignment regression.
- Both benchmark fixtures' output assertions and git diff whitespace checks.

An existing unused ColumnId import warning remains in bounded_cohorts.rs. Unrelated ignored benchmarks were not run as part of the full suite.

Reproduction (the recorded runs used an isolated target directory):

```sh
cargo test --locked -p bitview_compute --test lookup_followups_bench --test lookup_algorithms_bench -- --ignored --nocapture --test-threads=1
CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_TEST_LTO=false cargo test --locked -p vecdb --all-features -p bitview_compute
cargo check --locked -p bitview_default -p bitview_server
```

## Raw baseline

```text
Finished `test` profile [optimized + debuginfo] target(s) in 6.69s
     Running tests/lookup_followups_bench.rs (/private/tmp/bitview-view-verification.E4UAhn/debug/deps/lookup_followups_bench-fd66369f5304c632)

running 1 test
test benchmark_lookup_followups ... delta/cached=false/one: median=4.125µs min=4.083µs max=4.5µs
delta/cached=false/clustered: median=6.5µs min=6µs max=9.083µs
delta/cached=false/spread: median=162.209µs min=160.959µs max=171.417µs
delta/cached=false/dense: median=140.791µs min=139.542µs max=159.916µs
delta/cached=false/duplicates: median=120.666µs min=119.875µs max=130.375µs
previous/cached=false/one: median=3.208µs min=2.625µs max=3.542µs
previous/cached=false/clustered: median=2.916µs min=2.791µs max=3.584µs
previous/cached=false/spread: median=82.167µs min=81.667µs max=83.834µs
previous/cached=false/dense: median=28.166µs min=27.958µs max=35.458µs
previous/cached=false/duplicates: median=14.166µs min=11.792µs max=15.541µs
next_cumulative/cached=false/one: median=3.5µs min=2.875µs max=6.334µs
next_cumulative/cached=false/clustered: median=3.458µs min=2.875µs max=3.666µs
next_cumulative/cached=false/spread: median=84.167µs min=83.25µs max=99.583µs
next_cumulative/cached=false/dense: median=13.125µs min=11.542µs max=14.25µs
next_cumulative/cached=false/duplicates: median=8.959µs min=7.75µs max=10.083µs
next_count/cached=false/one: median=3.458µs min=3.041µs max=3.792µs
next_count/cached=false/clustered: median=3µs min=2.917µs max=3.167µs
next_count/cached=false/spread: median=86.333µs min=85.916µs max=95.042µs
next_count/cached=false/dense: median=20.583µs min=20.375µs max=25.917µs
next_count/cached=false/duplicates: median=12µs min=9.625µs max=12.375µs
column/cached=false/one: median=3.542µs min=3.083µs max=4.083µs
column/cached=false/clustered: median=2.958µs min=2.791µs max=3.875µs
column/cached=false/spread: median=86.667µs min=85.875µs max=90.958µs
column/cached=false/dense: median=15.417µs min=15.167µs max=20.041µs
column/cached=false/duplicates: median=8.709µs min=6.792µs max=11.25µs
column_transformed/cached=false/one: median=3.916µs min=3.333µs max=4.292µs
column_transformed/cached=false/clustered: median=3.375µs min=3.292µs max=4.416µs
column_transformed/cached=false/spread: median=102.792µs min=99.875µs max=112.375µs
column_transformed/cached=false/dense: median=17.541µs min=17.167µs max=22.208µs
column_transformed/cached=false/duplicates: median=7.417µs min=7.375µs max=9.625µs
block_count/cached=false/one: median=166ns min=125ns max=333ns
block_count/cached=false/clustered: median=3.042µs min=3.041µs max=3.083µs
block_count/cached=false/spread: median=1.833µs min=1.792µs max=1.958µs
block_count/cached=false/dense: median=237.084µs min=237.041µs max=241.792µs
block_count/cached=false/duplicates: median=541.083µs min=510.042µs max=541.416µs
delta/cached=true/one: median=167ns min=125ns max=291ns
delta/cached=true/clustered: median=792ns min=791ns max=1.291µs
delta/cached=true/spread: median=959ns min=833ns max=1.042µs
delta/cached=true/dense: median=134.209µs min=129.042µs max=145.542µs
delta/cached=true/duplicates: median=115.292µs min=113.708µs max=120.041µs
previous/cached=true/one: median=167ns min=125ns max=292ns
previous/cached=true/clustered: median=292ns min=291ns max=375ns
previous/cached=true/spread: median=6.791µs min=6.083µs max=7.917µs
previous/cached=true/dense: median=15.791µs min=15.458µs max=19.458µs
previous/cached=true/duplicates: median=8.792µs min=7.583µs max=9.291µs
next_cumulative/cached=true/one: median=250ns min=208ns max=417ns
next_cumulative/cached=true/clustered: median=250ns min=208ns max=375ns
next_cumulative/cached=true/spread: median=7.292µs min=6.75µs max=8.875µs
next_cumulative/cached=true/dense: median=6.416µs min=5.25µs max=6.666µs
next_cumulative/cached=true/duplicates: median=5.125µs min=5µs max=6.5µs
next_count/cached=true/one: median=208ns min=166ns max=250ns
next_count/cached=true/clustered: median=292ns min=291ns max=334ns
next_count/cached=true/spread: median=8.125µs min=7.417µs max=8.417µs
next_count/cached=true/dense: median=15.25µs min=15µs max=17.042µs
next_count/cached=true/duplicates: median=8.625µs min=7.333µs max=8.917µs
column/cached=true/one: median=208ns min=166ns max=250ns
column/cached=true/clustered: median=250ns min=208ns max=333ns
column/cached=true/spread: median=7.25µs min=5.583µs max=7.875µs
column/cached=true/dense: median=4.875µs min=4.459µs max=6.208µs
column/cached=true/duplicates: median=4.042µs min=4µs max=5.458µs
column_transformed/cached=true/one: median=625ns min=625ns max=875ns
column_transformed/cached=true/clustered: median=667ns min=666ns max=750ns
column_transformed/cached=true/spread: median=20.834µs min=18.875µs max=24.5µs
column_transformed/cached=true/dense: median=8.167µs min=6.75µs max=9.875µs
column_transformed/cached=true/duplicates: median=5.5µs min=4.709µs max=7.292µs
block_count/cached=true/one: median=167ns min=125ns max=459ns
block_count/cached=true/clustered: median=3.708µs min=3.042µs max=3.916µs
block_count/cached=true/spread: median=2.25µs min=1.792µs max=2.417µs
block_count/cached=true/dense: median=237.083µs min=237µs max=244.875µs
block_count/cached=true/duplicates: median=237.917µs min=237.833µs max=240.959µs
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.05s
```

## Raw final

```text
running 1 test
test benchmark_lookup_followups ... delta/cached=false/one: median=4.833µs min=4.75µs max=5.125µs
delta/cached=false/clustered: median=6.875µs min=6.75µs max=8.833µs
delta/cached=false/spread: median=190.292µs min=188.25µs max=200.375µs
delta/cached=false/dense: median=47.625µs min=46.833µs max=55µs
delta/cached=false/duplicates: median=37.833µs min=36.417µs max=38.25µs
previous/cached=false/one: median=2.917µs min=2.916µs max=3.125µs
previous/cached=false/clustered: median=3µs min=2.916µs max=4.083µs
previous/cached=false/spread: median=93.041µs min=92.625µs max=115.292µs
previous/cached=false/dense: median=17.708µs min=17.375µs max=19.167µs
previous/cached=false/duplicates: median=14.042µs min=13.708µs max=31.083µs
next_cumulative/cached=false/one: median=3.125µs min=3.042µs max=3.625µs
next_cumulative/cached=false/clustered: median=3.209µs min=3.167µs max=3.5µs
next_cumulative/cached=false/spread: median=93.875µs min=93.25µs max=103.167µs
next_cumulative/cached=false/dense: median=11.667µs min=11.459µs max=12.042µs
next_cumulative/cached=false/duplicates: median=8.625µs min=8.541µs max=9.292µs
next_count/cached=false/one: median=3.125µs min=3.041µs max=3.292µs
next_count/cached=false/clustered: median=3.125µs min=3.083µs max=3.375µs
next_count/cached=false/spread: median=99.209µs min=94µs max=109.958µs
next_count/cached=false/dense: median=9.584µs min=9.25µs max=14.542µs
next_count/cached=false/duplicates: median=10.334µs min=10.25µs max=22.958µs
column/cached=false/one: median=2.958µs min=2.833µs max=4.834µs
column/cached=false/clustered: median=2.834µs min=2.791µs max=2.875µs
column/cached=false/spread: median=87.417µs min=87.042µs max=96.667µs
column/cached=false/dense: median=11.459µs min=11.375µs max=11.542µs
column/cached=false/duplicates: median=7.041µs min=6.917µs max=7.291µs
column_transformed/cached=false/one: median=3.625µs min=2.917µs max=11.959µs
column_transformed/cached=false/clustered: median=3µs min=2.958µs max=3.208µs
column_transformed/cached=false/spread: median=90.334µs min=89.417µs max=101.083µs
column_transformed/cached=false/dense: median=15.292µs min=15.083µs max=15.875µs
column_transformed/cached=false/duplicates: median=6.291µs min=6.166µs max=9.625µs
block_count/cached=false/one: median=166ns min=125ns max=250ns
block_count/cached=false/clustered: median=250ns min=250ns max=250ns
block_count/cached=false/spread: median=1.917µs min=1.916µs max=1.959µs
block_count/cached=false/dense: median=9.958µs min=9.916µs max=10.125µs
block_count/cached=false/duplicates: median=9.375µs min=9.333µs max=9.541µs
delta/cached=true/one: median=84ns min=83ns max=167ns
delta/cached=true/clustered: median=417ns min=375ns max=625ns
delta/cached=true/spread: median=417ns min=416ns max=625ns
delta/cached=true/dense: median=35.667µs min=34.834µs max=36.042µs
delta/cached=true/duplicates: median=28.916µs min=27µs max=45.166µs
previous/cached=true/one: median=209ns min=167ns max=250ns
previous/cached=true/clustered: median=83ns min=41ns max=166ns
previous/cached=true/spread: median=334ns min=333ns max=541ns
previous/cached=true/dense: median=2.542µs min=2.5µs max=2.833µs
previous/cached=true/duplicates: median=7.75µs min=7.708µs max=7.917µs
next_cumulative/cached=true/one: median=208ns min=208ns max=333ns
next_cumulative/cached=true/clustered: median=291ns min=250ns max=292ns
next_cumulative/cached=true/spread: median=166ns min=125ns max=250ns
next_cumulative/cached=true/dense: median=5.292µs min=5.25µs max=5.625µs
next_cumulative/cached=true/duplicates: median=5.167µs min=5.083µs max=17.792µs
next_count/cached=true/one: median=250ns min=208ns max=333ns
next_count/cached=true/clustered: median=84ns min=83ns max=125ns
next_count/cached=true/spread: median=375ns min=334ns max=416ns
next_count/cached=true/dense: median=4.75µs min=4.625µs max=6.292µs
next_count/cached=true/duplicates: median=7.125µs min=7.041µs max=7.25µs
column/cached=true/one: median=42ns min=0ns max=42ns
column/cached=true/clustered: median=42ns min=41ns max=84ns
column/cached=true/spread: median=42ns min=41ns max=84ns
column/cached=true/dense: median=3.458µs min=3.416µs max=5.25µs
column/cached=true/duplicates: median=3.417µs min=3.375µs max=3.708µs
column_transformed/cached=true/one: median=42ns min=41ns max=42ns
column_transformed/cached=true/clustered: median=125ns min=84ns max=167ns
column_transformed/cached=true/spread: median=125ns min=83ns max=125ns
column_transformed/cached=true/dense: median=8.542µs min=7.083µs max=16.792µs
column_transformed/cached=true/duplicates: median=3.334µs min=3.25µs max=3.583µs
block_count/cached=true/one: median=167ns min=125ns max=208ns
block_count/cached=true/clustered: median=250ns min=208ns max=250ns
block_count/cached=true/spread: median=1.834µs min=1.833µs max=1.875µs
block_count/cached=true/dense: median=9.625µs min=9.541µs max=9.75µs
block_count/cached=true/duplicates: median=9µs min=8.917µs max=9.125µs
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.03s
```

## Raw final rolling-ratio fixture

```text
running 1 test
test benchmark_lookup_algorithms ... repeat/cached=false/one: median=15.875µs min=15.709µs max=17.292µs
repeat/cached=false/clustered: median=16µs min=15.75µs max=18.5µs
repeat/cached=false/spread: median=422.167µs min=366.666µs max=432.042µs
repeat/cached=false/dense: median=27.5µs min=26.625µs max=31.75µs
last/cached=false/one: median=12.041µs min=11.75µs max=14.584µs
last/cached=false/clustered: median=12.25µs min=12.167µs max=13.542µs
last/cached=false/spread: median=71.5µs min=71.333µs max=79.375µs
last/cached=false/dense: median=102.917µs min=100.084µs max=149.709µs
agg/cached=false/one: median=4.375µs min=4.25µs max=5.459µs
agg/cached=false/clustered: median=4.75µs min=4.625µs max=6.583µs
agg/cached=false/spread: median=138.25µs min=137.083µs max=152.167µs
agg/cached=false/dense: median=178.417µs min=177.375µs max=185.041µs
agg/cached=false/full_direct: median=634.792µs min=579.709µs max=738.375µs
agg/cached=false/full_nested: median=591.042µs min=550.166µs max=619.084µs
agg/cached=false/experimental_single_fold_nested: median=516µs min=509.583µs max=548.5µs
agg/cached=false/legacy_nested: median=513.667µs min=493.792µs max=524.292µs
source/cached=false/one: median=3.041µs min=3µs max=3.209µs
source/cached=false/clustered: median=3.083µs min=3µs max=3.542µs
source/cached=false/spread: median=94.083µs min=93.417µs max=98.5µs
source/cached=false/dense: median=12.541µs min=12.292µs max=14.667µs
lookback/cached=false/one: median=6.084µs min=5.958µs max=7.417µs
lookback/cached=false/clustered: median=7.334µs min=6.459µs max=9.333µs
lookback/cached=false/spread: median=184.292µs min=176.083µs max=188.584µs
lookback/cached=false/dense: median=42.875µs min=42.333µs max=46.458µs
window/cached=false/one: median=5.708µs min=5.625µs max=5.916µs
window/cached=false/clustered: median=6.416µs min=6.167µs max=9.084µs
window/cached=false/spread: median=177.917µs min=175.959µs max=181.708µs
window/cached=false/dense: median=46.667µs min=44.541µs max=48.333µs
ratio/cached=false/one: median=5.5µs min=5.375µs max=5.625µs
ratio/cached=false/clustered: median=6.291µs min=5.958µs max=7.458µs
ratio/cached=false/spread: median=170µs min=168.916µs max=175.292µs
ratio/cached=false/dense: median=46.541µs min=46.083µs max=47.375µs
rolling_block/cached=false/one: median=5.625µs min=5.625µs max=6.875µs
rolling_block/cached=false/clustered: median=6.666µs min=6.25µs max=7.75µs
rolling_block/cached=false/spread: median=175.417µs min=172.542µs max=176.958µs
rolling_block/cached=false/dense: median=53.375µs min=52.5µs max=55.625µs
repeat/cached=true/one: median=42ns min=41ns max=167ns
repeat/cached=true/clustered: median=125ns min=125ns max=208ns
repeat/cached=true/spread: median=7.083µs min=6.25µs max=8.375µs
repeat/cached=true/dense: median=4.583µs min=4.458µs max=5.708µs
last/cached=true/one: median=83ns min=41ns max=166ns
last/cached=true/clustered: median=250ns min=250ns max=541ns
last/cached=true/spread: median=541ns min=458ns max=625ns
last/cached=true/dense: median=24.084µs min=22.791µs max=29.709µs
agg/cached=true/one: median=42ns min=0ns max=42ns
agg/cached=true/clustered: median=208ns min=208ns max=333ns
agg/cached=true/spread: median=250ns min=209ns max=333ns
agg/cached=true/dense: median=20.417µs min=17.833µs max=22.917µs
agg/cached=true/full_direct: median=90.375µs min=87.25µs max=93.417µs
agg/cached=true/full_nested: median=95.542µs min=91.583µs max=96.75µs
agg/cached=true/experimental_single_fold_nested: median=72.209µs min=70µs max=78.958µs
agg/cached=true/legacy_nested: median=95.542µs min=89.958µs max=96.166µs
source/cached=true/one: median=42ns min=0ns max=42ns
source/cached=true/clustered: median=42ns min=41ns max=417ns
source/cached=true/spread: median=42ns min=41ns max=84ns
source/cached=true/dense: median=4.167µs min=3.375µs max=6.667µs
lookback/cached=true/one: median=42ns min=0ns max=84ns
lookback/cached=true/clustered: median=417ns min=375ns max=625ns
lookback/cached=true/spread: median=417ns min=375ns max=542ns
lookback/cached=true/dense: median=34.167µs min=29.083µs max=43.167µs
window/cached=true/one: median=42ns min=41ns max=125ns
window/cached=true/clustered: median=458ns min=416ns max=791ns
window/cached=true/spread: median=458ns min=458ns max=542ns
window/cached=true/dense: median=34.625µs min=32.458µs max=38.334µs
ratio/cached=true/one: median=42ns min=41ns max=84ns
ratio/cached=true/clustered: median=542ns min=458ns max=667ns
ratio/cached=true/spread: median=625ns min=583ns max=708ns
ratio/cached=true/dense: median=37.5µs min=33.834µs max=43.667µs
rolling_block/cached=true/one: median=250ns min=208ns max=375ns
rolling_block/cached=true/clustered: median=875ns min=709ns max=1.292µs
rolling_block/cached=true/spread: median=5.208µs min=5.041µs max=5.333µs
rolling_block/cached=true/dense: median=40.25µs min=39.25µs max=51.292µs
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.08s
```

