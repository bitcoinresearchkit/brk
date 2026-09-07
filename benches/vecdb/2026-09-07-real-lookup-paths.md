# Real lookup paths: outspends, mutable batches, interval hints

Date: 2026-09-07. Implemented in the local working tree; no server restart or production-data write by this task.

## Kept changes

1. **Outspend position lookups:** retain cursors for already sorted requests; sort shuffled spending-input requests, gather each compressed source in sorted order, and restore the original output positions. Repeated transaction indices share decoded pages through the native sorted reader rather than requiring a separate deduplication map. Published input/transaction/height bounds, checked vin conversion, plugin guards, mempool merging, and representation validation remain in place.
2. **Mutable sorted reads:** merge dense requests with ordered holes and pending updates, reusing one underlying reader. Read-only batches filter holes before native sorted gathering. Sparse requests retain independent tree lookups, avoiding a walk across large gaps. No persistent cache.
3. **RangeMapCursor:** an immutable, request-local last-interval hint. TxHeights exposes a stable read guard so outspend batches acquire the mapping lock once. The shared borrow prevents invalidation by append/truncate while a cursor exists. Ordinary get_shared remains unchanged.

## Paired lookup timings

Warm local fixtures, results validated independently after each timed operation. These are component comparisons, not HTTP speedups.

| Case | Reference | New |
| --- | ---: | ---: |
| Outspend positions: one | 10.292 µs | 10.292 µs |
| Outspend positions: two | 8.708 µs | 8.708 µs |
| Outspend positions: clustered 32 | 8.917 µs | 9.000 µs |
| Outspend positions: shuffled 32 | 445.500 µs | 422.083 µs |
| Outspend positions: shuffled 4,096 | 23.262 ms | 0.565 ms |
| Outspend positions: ordered 4,096 | 28.250 µs | 36.125 µs |
| Mutable writer: dense 4,096 | 172.750 µs | 13.167 µs |
| Mutable reader: dense 4,096 | 89.375 µs | 12.417 µs |
| Height lookup: 4,096 in one block interval | 77.708 µs | 4.917 µs |
| Height lookup: 4,096 random intervals | 172.167 µs | 191.667 µs |

Tradeoffs are intentional and bounded: ordered outspend position batches retain the cursor algorithm but still incur request-order detection and checked-result handling; the measured phase is about 8 µs slower at 4,096 entries. One/two-output position reads were flat. The interval hint is slower for random misses, so it is used in the grouped outspend path rather than replacing ordinary binary search globally. Sub-microsecond measurements are timer-sensitive.

Outspend fixture: 262,144 inputs, three inputs per transaction; paired cursor reference includes the same transaction bound and vin checks. The request list is supplied to both variants (building it from the spent-index vector is outside timing), as are txid reads, status construction, JSON, and HTTP. Median of twelve retained rounds, alternating variant order. Test build: opt-level 2, no LTO/debug info.

Mutation fixture: 262,144 values, holes every 17, updates every 23; dense and duplicate-heavy requests plus single/spread controls. Reader reference uses a fixed hole set and excludes its original outer read lock, making it a conservative reference. Median of eleven retained rounds. RangeMap fixture: one million boundaries; median of twenty retained alternating rounds. Both use normal workspace optimized test profile (opt-level 2, thin LTO, debug assertions). Different builds are not compared against one another.

## Monthly chart investigation

The original server on port 3110 stopped before the first attempted profile. It subsequently returned under PID 25979, and a bounded live stack sample succeeded. The request loop stopped on HTTP 504 after 17 successful requests. The capture contains too few query-stack samples to assign reliable CPU percentages or explain the timeout.

Nevertheless, the sampled live monthly request proves this concrete path is exercised:

```text
month1 LazyAggVec
  -> budgeted CachedVec
  -> LazyDeltaVec::read_sorted_into_at
  -> SparseRead
  -> transformed/cached source
  -> LazyColumnSumVec<AgeRangeId>::read_sorted_into_at
  -> ReadOnlyColumnarVec::for_each_column_sorted_at
  -> compressed sorted gather
  -> Pco page decompression
```

Raw capture: /private/tmp/bitview-chart-profile-6j7rrri1/sample.txt (query stack at lines 526–547). The running executable was not switched by this task. This is evidence of a real caller, not a controlled before/after comparison.

A reproducible local diagnostic uses the same LazyAggVec / budgeted rolling cache / LazyDeltaVec / Pco-source composition, one million synthetic cumulative values, 144 blocks/day and fixed 30-day months. It excludes query resolution, JSON, HTTP, real timestamp irregularity, and the live source's multi-column work. Its optional cumulative cache allows explicit residency comparison without changing production policy.

| Fixture cache state | 365 daily points | 120 monthly points | 4,096 block points |
| --- | ---: | ---: | ---: |
| Ordinary partial reads, snapshots nonresident | 178.625 µs | 621.667 µs | 38.375 µs |
| Explicitly resident cumulative source | 5.375 µs | 2.042 µs | 9.500 µs |
| Explicitly resident rolling source | 1.750 µs | 0.750 µs | 0.792 µs |

Monthly endpoints cover 122 source chunks versus 56 for the daily endpoints, despite fewer output rows. The fixture checks snapshot residency after the repeated reads: ordinary partial requests do not force a full retained snapshot. This supports page access/decompression and cache residency as the next profiling focus; it does not justify eagerly materializing every source or changing the global cache budget.

## Verification

- Full vecdb all-feature and brk_types suites, including doctests: passed.
- Normal bitview_query unit suite: 56 passed, 7 ignored.
- Targeted paired outspend benchmark: passed.
- Bytes and ZeroCopy mutation regression tests: physical holes, duplicates, pending updates, pushed values, publication, truncation and reopen passed.
- RangeMap cursor regression tests: empty maps, duplicate boundaries, backwards requests, final open interval, usize::MAX, clone, append, truncate and replacement passed.
- New HTTP integration: shuffled spending inputs preserve original output order and correct vin/txid/height; scalar and batch responses agree; conditional GET returns 304; replacement fork returns 404 rather than reusing stale content.
- Existing transaction handoff/publication regression: passed.
- git diff --check: passed.

Two overly broad test invocations were corrected: combining all features across query packages exposed missing Tokio time/macros features, while forcing every ignored query test included a catalog-snapshot diagnostic requiring SEARCH_CATALOG_SNAPSHOT. No unrelated feature manifest or snapshot was changed. The normal query suite and intended all-feature vecdb suite passed.

## Reproduce

```sh
cargo test --locked -p vecdb --features pco --test mutable_lookup -- --include-ignored --nocapture
cargo test --locked -p brk_types --test range_map -- --include-ignored --nocapture
CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_TEST_LTO=false cargo test --locked -p bitview_query --lib benchmark_spending_positions -- --ignored --nocapture
cargo test --locked -p bitview_compute --test chart_lookup_profile -- --ignored --nocapture
CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_TEST_LTO=false cargo test --locked -p bitview_server --lib transaction_publication
```

HTTP fixtures need temporary loopback listeners.

## Raw paired output

### Outspend positions

```text
Finished `test` profile [optimized] target(s) in 2.85s
     Running unittests src/lib.rs (/private/tmp/bitview-view-verification.E4UAhn/debug/deps/bitview_query-4a616695950cde27)

running 1 test
one: cursors 10.292µs, sorted batch 10.292µs
two: cursors 8.708µs, sorted batch 8.708µs
clustered: cursors 8.917µs, sorted batch 9µs
shuffled32: cursors 445.5µs, sorted batch 422.083µs
shuffled4096: cursors 23.262292ms, sorted batch 565.333µs
dense4096: cursors 28.25µs, sorted batch 36.125µs
test r#impl::tx::outspend::tests::benchmark_spending_positions ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 62 filtered out; finished in 0.37s
```

### Mutable lookups

```text
running 1 test
writer/one/new: 84ns
writer/one/old: 83ns
writer/spread/new: 1.125µs
writer/spread/old: 1.125µs
writer/dense/new: 13.167µs
writer/dense/old: 172.75µs
writer/duplicates/new: 10.417µs
writer/duplicates/old: 158.875µs
reader/one/new: 42ns
reader/one/old: 42ns
reader/spread/new: 625ns
reader/spread/old: 666ns
reader/dense/new: 12.417µs
reader/dense/old: 89.375µs
reader/duplicates/new: 11.584µs
reader/duplicates/old: 82.292µs
test benchmark_mutable_lookup ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.01s
```

### RangeMap

```text
running 1 test
one: shared 83ns, cursor 42ns
same_block: shared 77.708µs, cursor 4.917µs
sequential: shared 77.75µs, cursor 5.083µs
random: shared 172.167µs, cursor 191.667µs
test benchmark_range_map_cursor ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 1 filtered out; finished in 0.02s
```

