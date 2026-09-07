# Lookup tail: window search, column sums, overflow gathers

Date: 2026-09-07. Local working tree, not a full-chain indexer benchmark.

## Decisions

- Keep a hybrid window-start search: monotonic linear advancement for nearby requests, binary partitioning for large gaps. The crossover compares the request gap with the binary-search comparison bound; there is no tuned magic threshold. Pure binary and galloping variants remain benchmark-only. Same-process sparse spread: linear 176.959 µs versus hybrid 2.750 µs; dense 18.167 µs for both.
- Keep selective column sums via the existing per-column sorted read operations, with a reusable scratch buffer and a batched callback preserving the read-only columnar publication gate across columns. Transformed sources forward this operation. Dense requests retain cursor/chunk summation. No new persistent cache or multi-source lazy vector. Cached spread: cursor 26.709 µs versus selective 0.209 µs; transformed spread: 60.125 versus 0.292 µs. Uncached Pco remains decompression-bound (279.917 versus 275.125 µs).
- Keep native compact sorted gathers in both overflow writer and reader, decoding only selected values through one sidecar reader. Exactly consecutive requests use the existing compact range path. This preserves holes, duplicates, staged sidecar state, and publication locking. Mixed reader spread: cursor 162.084 µs versus selective 0.125 µs; dense 15.833 versus 6.292 µs.

## Method and limitations

Fixture: 262,144 entries; one request, clustered 32, spread 32, dense 4,096, and duplicate-heavy 4,096 requests. Timestamp source is resident, monotonic with duplicates; window is 14 days. Sums cover two Pco columns, resident cached columns, and transformed cached columns. Overflow fixtures cover inline, 1/64 overflow, and all-overflow values.

The ignored benchmark asserts every measured result against independently computed expected values after timing. One warm-up plus eleven samples, median reported. Values below a microsecond are near timer granularity: ratios there should not be interpreted as precise capacity estimates. Allocations are included; storage is warm, not cold I/O.

Normal workspace test profile for lookup timings: opt-level 2, thin LTO, native target CPU, debug assertions. No benchmark profile override. Initial before/after wall-clock timings varied substantially with machine scheduling/load: even the unchanged standalone linear window algorithm slowed from 94.500 to 176.959 µs. Therefore the paired same-process cursor/linear comparisons are the decision evidence, not the chronological baseline alone. Cursor references reproduce the former default sorted implementation on these hole-free fixtures. They are not correctness references for mutable holes.

Dense sums/windows are effectively unchanged in the paired run; clustered cached sums show a small absolute regression (0.833 to 1.000 µs) and duplicate-heavy cached sums 12.459 to 12.917 µs. No universal speedup claim.

Reproduce:

```sh
cargo test --locked -p bitview_compute --test lookup_tail_bench -- --ignored --nocapture --test-threads=1
```

## Timings

All values in µs. Historical baseline was captured before this wave. Paired reference and final are from the same final executable/run.

| Case | Historical baseline | Paired cursor/linear | Final |
| --- | ---: | ---: | ---: |
| window/production/one | 0.083 | 0.125 | 0.125 |
| window/production/clustered | 0.166 | 0.250 | 0.291 |
| window/production/spread | 95.333 | 176.959 | 2.750 |
| window/production/dense | 8.375 | 18.167 | 18.167 |
| window/production/duplicates | 5.167 | 11.000 | 11.042 |
| sum/pco/one | 6.333 | 9.625 | 8.834 |
| sum/pco/clustered | 6.417 | 11.041 | 9.875 |
| sum/pco/spread | 202.917 | 279.917 | 275.125 |
| sum/pco/dense | 30.667 | 47.709 | 48.000 |
| sum/pco/duplicates | 11.250 | 21.500 | 22.084 |
| sum/cached/one | 0.541 | 0.709 | 0.125 |
| sum/cached/clustered | 0.583 | 0.833 | 1.000 |
| sum/cached/spread | 19.083 | 26.709 | 0.209 |
| sum/cached/dense | 6.500 | 14.625 | 14.625 |
| sum/cached/duplicates | 4.583 | 12.459 | 12.917 |
| sum/transformed/one | 1.500 | 1.958 | 0.125 |
| sum/transformed/clustered | 1.500 | 2.042 | 2.042 |
| sum/transformed/spread | 46.250 | 60.125 | 0.292 |
| sum/transformed/dense | 10.125 | 19.583 | 19.500 |
| sum/transformed/duplicates | 5.667 | 13.666 | 13.834 |
| overflow_writer/inline/one | 2.917 | 3.709 | 0.083 |
| overflow_reader/inline/one | 4.167 | 5.458 | 0.083 |
| overflow_writer/inline/clustered | 2.875 | 3.916 | 0.167 |
| overflow_reader/inline/clustered | 4.208 | 5.542 | 0.167 |
| overflow_writer/inline/spread | 89.042 | 116.708 | 0.166 |
| overflow_reader/inline/spread | 132.417 | 159.750 | 0.166 |
| overflow_writer/inline/dense | 7.792 | 15.625 | 5.334 |
| overflow_reader/inline/dense | 9.875 | 16.416 | 6.708 |
| overflow_writer/inline/duplicates | 7.333 | 15.083 | 6.416 |
| overflow_reader/inline/duplicates | 9.417 | 16.250 | 6.083 |
| overflow_writer/mixed/one | 3.167 | 3.917 | 0.083 |
| overflow_reader/mixed/one | 4.500 | 6.166 | 0.083 |
| overflow_writer/mixed/clustered | 3.208 | 3.958 | 0.167 |
| overflow_reader/mixed/clustered | 4.542 | 6.459 | 0.167 |
| overflow_writer/mixed/spread | 97.125 | 122.917 | 0.166 |
| overflow_reader/mixed/spread | 136.833 | 162.084 | 0.125 |
| overflow_writer/mixed/dense | 7.583 | 14.834 | 4.875 |
| overflow_reader/mixed/dense | 10.125 | 15.833 | 6.292 |
| overflow_writer/mixed/duplicates | 7.500 | 14.791 | 6.083 |
| overflow_reader/mixed/duplicates | 9.375 | 15.792 | 5.791 |
| overflow_writer/overflow/one | 6.208 | 7.167 | 0.083 |
| overflow_reader/overflow/one | 4.000 | 4.583 | 0.084 |
| overflow_writer/overflow/clustered | 6.250 | 7.209 | 0.208 |
| overflow_reader/overflow/clustered | 4.083 | 4.708 | 0.167 |
| overflow_writer/overflow/spread | 199.166 | 229.167 | 0.167 |
| overflow_reader/overflow/spread | 129.916 | 146.500 | 0.167 |
| overflow_writer/overflow/dense | 10.875 | 17.709 | 10.708 |
| overflow_reader/overflow/dense | 9.000 | 14.292 | 7.250 |
| overflow_writer/overflow/duplicates | 10.667 | 16.916 | 11.042 |
| overflow_reader/overflow/duplicates | 8.791 | 14.166 | 6.459 |

## Correctness and integration

Passed:

- Full `cargo test --locked -p vecdb --all-features -p bitview_compute`, including doctests (test debug info disabled and test LTO disabled for this correctness build).
- New window regression cases compare the old linear semantics across duplicate timestamps, sparse/dense/out-of-range requests, zero/extreme durations, appends, and same-length timestamp rewrites.
- Column sums cover duplicates, output append, cached/denied/rebuilt cache paths, transformed sources, and concurrent publication with Bytes/Pco.
- Overflow cases cover physical holes, duplicate indices, out-of-range clipping, inline/overflow replacement, freed sidecar reuse, unwritten values, truncation, publication, and reopen.
- Both server `tests::series_ranges` HTTP fixture tests passed, covering daily publication coherence and date/timestamp range behavior.
- Existing populated-server ignored query benchmark passed; outputs below. It uses a small populated local fixture and real query/formatting code, not full-chain production data. These timings compare that benchmark's own existing variants, not this wave's before/after. Most timed work excludes HTTP transport. They demonstrate exercised integration, not production throughput gains.
- `git diff --check`.

Server fixtures require temporary loopback listeners; the sandbox denied binding, and the built test executables passed when run with that permission. No production server was restarted or data rewritten.

## Raw chronological baseline

```text
running 1 test
test benchmark_lookup_tail ... window/production/one: median=83ns min=41ns max=167ns
window/linear/one: median=42ns min=41ns max=84ns
window/binary/one: median=42ns min=41ns max=84ns
window/galloping/one: median=42ns min=41ns max=84ns
window/production/clustered: median=166ns min=125ns max=208ns
window/linear/clustered: median=125ns min=84ns max=167ns
window/binary/clustered: median=667ns min=666ns max=709ns
window/galloping/clustered: median=250ns min=250ns max=292ns
window/production/spread: median=95.333µs min=93.792µs max=118.541µs
window/linear/spread: median=94.5µs min=93.541µs max=94.959µs
window/binary/spread: median=1.458µs min=1.458µs max=1.542µs
window/galloping/spread: median=1.833µs min=1.833µs max=1.875µs
window/production/dense: median=8.375µs min=8.333µs max=8.625µs
window/linear/dense: median=8.334µs min=8.292µs max=8.458µs
window/binary/dense: median=72.834µs min=69.125µs max=73.042µs
window/galloping/dense: median=12.542µs min=12.458µs max=13.125µs
window/production/duplicates: median=5.167µs min=5.125µs max=5.5µs
window/linear/duplicates: median=5.333µs min=5.25µs max=6.292µs
window/binary/duplicates: median=18.709µs min=18.625µs max=18.958µs
window/galloping/duplicates: median=6.167µs min=6.125µs max=6.291µs
sum/pco/one: median=6.333µs min=6.25µs max=7.083µs
sum/pco/clustered: median=6.417µs min=6.334µs max=6.5µs
sum/pco/spread: median=202.917µs min=201.125µs max=204.75µs
sum/pco/dense: median=30.667µs min=30.458µs max=31.291µs
sum/pco/duplicates: median=11.25µs min=10.958µs max=11.375µs
sum/cached/one: median=541ns min=500ns max=625ns
sum/cached/clustered: median=583ns min=541ns max=625ns
sum/cached/spread: median=19.083µs min=18.625µs max=20.25µs
sum/cached/dense: median=6.5µs min=6.417µs max=6.625µs
sum/cached/duplicates: median=4.583µs min=4.541µs max=4.625µs
sum/transformed/one: median=1.5µs min=1.458µs max=2.542µs
sum/transformed/clustered: median=1.5µs min=1.458µs max=1.583µs
sum/transformed/spread: median=46.25µs min=46.083µs max=47.208µs
sum/transformed/dense: median=10.125µs min=10.041µs max=10.417µs
sum/transformed/duplicates: median=5.667µs min=5.583µs max=5.917µs
overflow_writer/inline/one: median=2.917µs min=2.833µs max=10.166µs
overflow_reader/inline/one: median=4.167µs min=4.125µs max=4.375µs
overflow_writer/inline/clustered: median=2.875µs min=2.833µs max=3.125µs
overflow_reader/inline/clustered: median=4.208µs min=4.167µs max=4.375µs
overflow_writer/inline/spread: median=89.042µs min=88.708µs max=89.875µs
overflow_reader/inline/spread: median=132.417µs min=130.75µs max=156.75µs
overflow_writer/inline/dense: median=7.792µs min=7.5µs max=10.625µs
overflow_reader/inline/dense: median=9.875µs min=9.667µs max=11.167µs
overflow_writer/inline/duplicates: median=7.333µs min=7.25µs max=9.875µs
overflow_reader/inline/duplicates: median=9.417µs min=9.292µs max=9.708µs
overflow_writer/mixed/one: median=3.167µs min=3.125µs max=3.375µs
overflow_reader/mixed/one: median=4.5µs min=4.458µs max=4.792µs
overflow_writer/mixed/clustered: median=3.208µs min=3.166µs max=3.375µs
overflow_reader/mixed/clustered: median=4.542µs min=4.5µs max=4.667µs
overflow_writer/mixed/spread: median=97.125µs min=96.625µs max=101.917µs
overflow_reader/mixed/spread: median=136.833µs min=135.791µs max=143.875µs
overflow_writer/mixed/dense: median=7.583µs min=7.416µs max=13.125µs
overflow_reader/mixed/dense: median=10.125µs min=9.458µs max=28.25µs
overflow_writer/mixed/duplicates: median=7.5µs min=7.333µs max=9.834µs
overflow_reader/mixed/duplicates: median=9.375µs min=9.292µs max=9.541µs
overflow_writer/overflow/one: median=6.208µs min=6.166µs max=6.542µs
overflow_reader/overflow/one: median=4µs min=3.958µs max=4.042µs
overflow_writer/overflow/clustered: median=6.25µs min=6.208µs max=6.458µs
overflow_reader/overflow/clustered: median=4.083µs min=4.041µs max=4.167µs
overflow_writer/overflow/spread: median=199.166µs min=198.166µs max=200.333µs
overflow_reader/overflow/spread: median=129.916µs min=124.125µs max=135.375µs
overflow_writer/overflow/dense: median=10.875µs min=10.708µs max=14.958µs
overflow_reader/overflow/dense: median=9µs min=8.917µs max=12.166µs
overflow_writer/overflow/duplicates: median=10.667µs min=10.208µs max=14.209µs
overflow_reader/overflow/duplicates: median=8.791µs min=8.75µs max=11.541µs
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Raw final before adding paired references

Retained to show timing variability rather than silently selecting a run.

```text
running 1 test
test benchmark_lookup_tail ... window/production/one: median=167ns min=166ns max=375ns
window/linear/one: median=125ns min=83ns max=250ns
window/binary/one: median=125ns min=83ns max=208ns
window/galloping/one: median=125ns min=125ns max=167ns
window/production/clustered: median=333ns min=291ns max=666ns
window/linear/clustered: median=292ns min=291ns max=500ns
window/binary/clustered: median=1.5µs min=1.417µs max=1.583µs
window/galloping/clustered: median=583ns min=500ns max=625ns
window/production/spread: median=3.25µs min=3.167µs max=3.625µs
window/linear/spread: median=208.125µs min=177µs max=217.375µs
window/binary/spread: median=2.75µs min=2.709µs max=3.417µs
window/galloping/spread: median=3.5µs min=3.417µs max=4.292µs
window/production/dense: median=18.209µs min=18.166µs max=20.958µs
window/linear/dense: median=18.25µs min=18.125µs max=20.625µs
window/binary/dense: median=137.917µs min=137.75µs max=140.417µs
window/galloping/dense: median=26.833µs min=26.709µs max=33µs
window/production/duplicates: median=12.708µs min=12.666µs max=12.875µs
window/linear/duplicates: median=13.334µs min=13.25µs max=15.541µs
window/binary/duplicates: median=36.875µs min=36.791µs max=42µs
window/galloping/duplicates: median=13.708µs min=13.625µs max=15.834µs
sum/pco/one: median=8.791µs min=8.666µs max=9µs
sum/pco/clustered: median=9.916µs min=9.792µs max=11.584µs
sum/pco/spread: median=275.541µs min=275.125µs max=279.958µs
sum/pco/dense: median=53.458µs min=53.291µs max=55.958µs
sum/pco/duplicates: median=24.292µs min=24.042µs max=26.792µs
sum/cached/one: median=125ns min=125ns max=333ns
sum/cached/clustered: median=958ns min=916ns max=1.042µs
sum/cached/spread: median=292ns min=250ns max=459ns
sum/cached/dense: median=17.583µs min=17.375µs max=21.875µs
sum/cached/duplicates: median=15.042µs min=14.792µs max=17.375µs
sum/transformed/one: median=166ns min=125ns max=250ns
sum/transformed/clustered: median=2.458µs min=2.416µs max=2.791µs
sum/transformed/spread: median=459ns min=375ns max=542ns
sum/transformed/dense: median=23.417µs min=22.875µs max=26.333µs
sum/transformed/duplicates: median=14.792µs min=14.666µs max=16.917µs
overflow_writer/inline/one: median=84ns min=42ns max=333ns
overflow_reader/inline/one: median=84ns min=83ns max=458ns
overflow_writer/inline/clustered: median=250ns min=208ns max=375ns
overflow_reader/inline/clustered: median=208ns min=208ns max=458ns
overflow_writer/inline/spread: median=291ns min=208ns max=375ns
overflow_reader/inline/spread: median=166ns min=125ns max=209ns
overflow_writer/inline/dense: median=15.625µs min=15µs max=18.209µs
overflow_reader/inline/dense: median=7.708µs min=7.625µs max=8µs
overflow_writer/inline/duplicates: median=16.667µs min=16.541µs max=19.167µs
overflow_reader/inline/duplicates: median=7.375µs min=7.209µs max=9.333µs
overflow_writer/mixed/one: median=83ns min=83ns max=250ns
overflow_reader/mixed/one: median=83ns min=83ns max=208ns
overflow_writer/mixed/clustered: median=209ns min=209ns max=333ns
overflow_reader/mixed/clustered: median=167ns min=125ns max=375ns
overflow_writer/mixed/spread: median=250ns min=250ns max=292ns
overflow_reader/mixed/spread: median=166ns min=125ns max=167ns
overflow_writer/mixed/dense: median=15.791µs min=15.166µs max=19µs
overflow_reader/mixed/dense: median=9.75µs min=8.959µs max=11.125µs
overflow_writer/mixed/duplicates: median=16.959µs min=16.542µs max=19.458µs
overflow_reader/mixed/duplicates: median=7.875µs min=7.291µs max=9.459µs
overflow_writer/overflow/one: median=83ns min=83ns max=791ns
overflow_reader/overflow/one: median=83ns min=83ns max=167ns
overflow_writer/overflow/clustered: median=208ns min=208ns max=1.458µs
overflow_reader/overflow/clustered: median=167ns min=125ns max=292ns
overflow_writer/overflow/spread: median=250ns min=208ns max=292ns
overflow_reader/overflow/spread: median=167ns min=125ns max=209ns
overflow_writer/overflow/dense: median=14.959µs min=14.583µs max=18.625µs
overflow_reader/overflow/dense: median=8.75µs min=7.875µs max=10.791µs
overflow_writer/overflow/duplicates: median=17.958µs min=16.833µs max=23.75µs
overflow_reader/overflow/duplicates: median=8.25µs min=8.125µs max=10.041µs
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.04s
```

## Raw paired final run

```text
running 1 test
test benchmark_lookup_tail ... window/production/one: median=125ns min=83ns max=375ns
window/linear/one: median=125ns min=83ns max=250ns
window/binary/one: median=125ns min=84ns max=167ns
window/galloping/one: median=84ns min=83ns max=125ns
window/production/clustered: median=291ns min=208ns max=375ns
window/linear/clustered: median=250ns min=208ns max=333ns
window/binary/clustered: median=1.25µs min=1.25µs max=1.333µs
window/galloping/clustered: median=500ns min=459ns max=583ns
window/production/spread: median=2.75µs min=2.708µs max=2.875µs
window/linear/spread: median=176.959µs min=175.958µs max=194.125µs
window/binary/spread: median=2.75µs min=2.708µs max=2.75µs
window/galloping/spread: median=3.5µs min=3.416µs max=4.25µs
window/production/dense: median=18.167µs min=18.125µs max=21.875µs
window/linear/dense: median=18.167µs min=18.084µs max=20.709µs
window/binary/dense: median=138.042µs min=138µs max=140.5µs
window/galloping/dense: median=32.209µs min=32.042µs max=34.833µs
window/production/duplicates: median=11.042µs min=11µs max=11.125µs
window/linear/duplicates: median=11µs min=10.958µs max=13.084µs
window/binary/duplicates: median=31.25µs min=31.167µs max=33.834µs
window/galloping/duplicates: median=11.417µs min=11.209µs max=13.25µs
sum/pco/one: median=8.834µs min=8.792µs max=9.208µs
sum_cursor/pco/one: median=9.625µs min=9.541µs max=11.833µs
sum/pco/clustered: median=9.875µs min=9.667µs max=15.25µs
sum_cursor/pco/clustered: median=11.041µs min=9.833µs max=12.209µs
sum/pco/spread: median=275.125µs min=270.125µs max=279.75µs
sum_cursor/pco/spread: median=279.917µs min=273.291µs max=311.458µs
sum/pco/dense: median=48µs min=47.458µs max=56.875µs
sum_cursor/pco/dense: median=47.709µs min=47.25µs max=53.916µs
sum/pco/duplicates: median=22.084µs min=21.375µs max=25.459µs
sum_cursor/pco/duplicates: median=21.5µs min=21.416µs max=25.166µs
sum/cached/one: median=125ns min=125ns max=208ns
sum_cursor/cached/one: median=709ns min=666ns max=917ns
sum/cached/clustered: median=1µs min=834ns max=1.25µs
sum_cursor/cached/clustered: median=833ns min=792ns max=834ns
sum/cached/spread: median=209ns min=208ns max=2.167µs
sum_cursor/cached/spread: median=26.709µs min=24.542µs max=55.542µs
sum/cached/dense: median=14.625µs min=14.541µs max=18.417µs
sum_cursor/cached/dense: median=14.625µs min=14.25µs max=17.833µs
sum/cached/duplicates: median=12.917µs min=12.667µs max=13.25µs
sum_cursor/cached/duplicates: median=12.459µs min=12.125µs max=13.875µs
sum/transformed/one: median=125ns min=84ns max=209ns
sum_cursor/transformed/one: median=1.958µs min=1.875µs max=2.208µs
sum/transformed/clustered: median=2.042µs min=2µs max=2.166µs
sum_cursor/transformed/clustered: median=2.042µs min=1.958µs max=2.083µs
sum/transformed/spread: median=292ns min=291ns max=500ns
sum_cursor/transformed/spread: median=60.125µs min=59.708µs max=65.375µs
sum/transformed/dense: median=19.5µs min=19.208µs max=23.458µs
sum_cursor/transformed/dense: median=19.583µs min=19.25µs max=24.083µs
sum/transformed/duplicates: median=13.834µs min=13.542µs max=16.542µs
sum_cursor/transformed/duplicates: median=13.666µs min=13.583µs max=16.5µs
overflow_writer/inline/one: median=83ns min=42ns max=333ns
overflow_reader/inline/one: median=83ns min=83ns max=167ns
overflow_writer_cursor/inline/one: median=3.709µs min=3.666µs max=4.583µs
overflow_reader_cursor/inline/one: median=5.458µs min=5.333µs max=6.459µs
overflow_writer/inline/clustered: median=167ns min=166ns max=333ns
overflow_reader/inline/clustered: median=167ns min=125ns max=1.625µs
overflow_writer_cursor/inline/clustered: median=3.916µs min=3.75µs max=4.75µs
overflow_reader_cursor/inline/clustered: median=5.542µs min=5.5µs max=6.833µs
overflow_writer/inline/spread: median=166ns min=125ns max=458ns
overflow_reader/inline/spread: median=166ns min=125ns max=209ns
overflow_writer_cursor/inline/spread: median=116.708µs min=116.416µs max=130.167µs
overflow_reader_cursor/inline/spread: median=159.75µs min=159.584µs max=160.125µs
overflow_writer/inline/dense: median=5.334µs min=5.166µs max=6.625µs
overflow_reader/inline/dense: median=6.708µs min=6.541µs max=8.167µs
overflow_writer_cursor/inline/dense: median=15.625µs min=15.083µs max=18.208µs
overflow_reader_cursor/inline/dense: median=16.416µs min=16.292µs max=18.833µs
overflow_writer/inline/duplicates: median=6.416µs min=6.334µs max=8.167µs
overflow_reader/inline/duplicates: median=6.083µs min=5.958µs max=7.541µs
overflow_writer_cursor/inline/duplicates: median=15.083µs min=14.917µs max=16.959µs
overflow_reader_cursor/inline/duplicates: median=16.25µs min=16.208µs max=18.833µs
overflow_writer/mixed/one: median=83ns min=41ns max=166ns
overflow_reader/mixed/one: median=83ns min=83ns max=125ns
overflow_writer_cursor/mixed/one: median=3.917µs min=3.834µs max=4.792µs
overflow_reader_cursor/mixed/one: median=6.166µs min=5.375µs max=6.834µs
overflow_writer/mixed/clustered: median=167ns min=166ns max=209ns
overflow_reader/mixed/clustered: median=167ns min=166ns max=250ns
overflow_writer_cursor/mixed/clustered: median=3.958µs min=3.916µs max=4.083µs
overflow_reader_cursor/mixed/clustered: median=6.459µs min=5.5µs max=6.875µs
overflow_writer/mixed/spread: median=166ns min=125ns max=208ns
overflow_reader/mixed/spread: median=125ns min=125ns max=250ns
overflow_writer_cursor/mixed/spread: median=122.917µs min=115.5µs max=131.375µs
overflow_reader_cursor/mixed/spread: median=162.084µs min=161.833µs max=163.208µs
overflow_writer/mixed/dense: median=4.875µs min=4.791µs max=6.584µs
overflow_reader/mixed/dense: median=6.292µs min=6.208µs max=7.75µs
overflow_writer_cursor/mixed/dense: median=14.834µs min=14.625µs max=18.792µs
overflow_reader_cursor/mixed/dense: median=15.833µs min=15.75µs max=18.042µs
overflow_writer/mixed/duplicates: median=6.083µs min=5.958µs max=7.625µs
overflow_reader/mixed/duplicates: median=5.791µs min=5.667µs max=7.625µs
overflow_writer_cursor/mixed/duplicates: median=14.791µs min=14.542µs max=18.25µs
overflow_reader_cursor/mixed/duplicates: median=15.792µs min=15.625µs max=18.083µs
overflow_writer/overflow/one: median=83ns min=41ns max=125ns
overflow_reader/overflow/one: median=84ns min=42ns max=167ns
overflow_writer_cursor/overflow/one: median=7.167µs min=7.125µs max=8.916µs
overflow_reader_cursor/overflow/one: median=4.583µs min=4.541µs max=5.75µs
overflow_writer/overflow/clustered: median=208ns min=166ns max=292ns
overflow_reader/overflow/clustered: median=167ns min=125ns max=209ns
overflow_writer_cursor/overflow/clustered: median=7.209µs min=7.208µs max=7.292µs
overflow_reader_cursor/overflow/clustered: median=4.708µs min=4.666µs max=5.792µs
overflow_writer/overflow/spread: median=167ns min=166ns max=250ns
overflow_reader/overflow/spread: median=167ns min=125ns max=167ns
overflow_writer_cursor/overflow/spread: median=229.167µs min=228.917µs max=234.541µs
overflow_reader_cursor/overflow/spread: median=146.5µs min=145.417µs max=146.833µs
overflow_writer/overflow/dense: median=10.708µs min=10µs max=12.375µs
overflow_reader/overflow/dense: median=7.25µs min=7.125µs max=8.834µs
overflow_writer_cursor/overflow/dense: median=17.709µs min=17.666µs max=19.834µs
overflow_reader_cursor/overflow/dense: median=14.292µs min=13.958µs max=17.625µs
overflow_writer/overflow/duplicates: median=11.042µs min=10.333µs max=13.75µs
overflow_reader/overflow/duplicates: median=6.459µs min=6.333µs max=7.625µs
overflow_writer_cursor/overflow/duplicates: median=16.916µs min=16.708µs max=20.666µs
overflow_reader_cursor/overflow/duplicates: median=14.166µs min=13.875µs max=17.458µs
ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.06s
```

## Populated local query benchmark

Correctness build profile (opt-level 2, no LTO), not the lookup timing profile.

```text
CSV columns=1 rows=0: previous 2.239µs, early-empty 2.188µs
CSV columns=1 rows=1: previous 2.29µs, early-empty 2.247µs
CSV columns=2 rows=0: previous 2.473µs, early-empty 2.392µs
CSV columns=2 rows=1: previous 2.653µs, early-empty 2.646µs
CSV columns=32 rows=0: previous 8.267µs, early-empty 7.748µs
CSV columns=32 rows=1: previous 10.002µs, early-empty 9.973µs
health local query: unbounded 5.741µs, admitted 5.868µs
length=false conditional=false: unbounded 8.607µs, admitted 6.989µs
length=false conditional=true: unbounded 8.734µs, admitted 7.245µs
length=true conditional=false: unbounded 8.27µs, admitted 6.732µs
exact length validator: 6.805µs
length=true conditional=true: unbounded 8.517µs, admitted 7.215µs
exact length validator: 7.062µs
test tests::populated_server::benchmark_populated_series_scalars ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 98 filtered out; finished in 12.17s
```

