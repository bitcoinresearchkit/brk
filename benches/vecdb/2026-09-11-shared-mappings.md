# Shared mapping follow-ups — 2026-09-11

## Retained changes

1. Investing's daily projections use the shared height-to-day mapping. Class cost-basis views find their starting height directly in the resident day-to-first-height boundaries instead of binary-searching reverse day values. Reads remain capped at the source's published height.
2. Public day, week, month, quarter, half-year, year, and decade mappings reuse the existing resolution boundaries. Timestamp conversions remain private inputs for building/updating those boundaries, avoiding a circular dependency. Mining queries also pick up the public day mapping.
3. Difficulty epochs and halvings use metadata-backed `IndexVec` height arithmetic, without reading timestamp values. Fixed-duration timestamp arithmetic is retained for 10/30 minutes, 1/4/12 hours, and 3 days: the comparison below found reverse maps slower for those conversions.
4. Distribution borrows the shared transaction-to-height boundaries once per compute batch. A local 1,024-slot cursor reuses the existing `RangeMap::get` cache algorithm; the duplicate boundary vector and its append/recovery maintenance are removed. The lightweight single-interval cursor remains available for sequential reads.

## Method and limits

Local `aarch64-apple-darwin`, Rust 1.98.1, workspace test profile: optimization level 2, thin LTO, native CPU, debug assertions enabled. These are same-process paired synthetic lookup benchmarks, not production indexer throughput or HTTP latency. Other machine activity was not stopped. Output allocation is timed; exact output-array equality is checked after every timed read. No live server was restarted or queried for this work.

### Public reverse mappings

- 966,505 synthetic monotonic timestamps spanning 6,463 UTC days; fully resident timestamp input.
- Full reads return all heights. Sparse reads request every 149th height: 6,487 results.
- Two warmup rounds followed by 12 retained rounds, alternating timestamp/map order.
- Resolution construction is outside timing because these resident boundaries already exist. Storage decoding, serialization, and HTTP are excluded. Fully resident input favors the timestamp baseline; disk-backed crossover costs are not established here.

All timings below are medians in milliseconds. Each cell is timestamp conversion → resident reverse map.

| Mapping | Full read | Sparse read | Retained implementation |
|---|---:|---:|---|
| 10 minutes | 0.901 → 18.008 | 0.011 → 0.156 | Timestamp arithmetic |
| 30 minutes | 0.899 → 7.100 | 0.011 → 0.135 | Timestamp arithmetic |
| 1 hour | 0.905 → 3.810 | 0.011 → 0.127 | Timestamp arithmetic |
| 4 hours | 0.920 → 1.881 | 0.012 → 0.111 | Timestamp arithmetic |
| 12 hours | 0.916 → 1.366 | 0.011 → 0.091 | Timestamp arithmetic |
| 1 day | 15.775 → 1.132 | 0.112 → 0.075 | Shared reverse map |
| 3 days | 0.827 → 1.011 | 0.011 → 0.031 | Timestamp arithmetic |
| 1 week | 16.248 → 0.993 | 0.116 → 0.017 | Shared reverse map |
| 1 month | 25.887 → 0.975 | 0.175 → 0.011 | Shared reverse map |
| 3 months | 27.071 → 1.034 | 0.179 → 0.009 | Shared reverse map |
| 6 months | 26.644 → 0.961 | 0.175 → 0.008 | Shared reverse map |
| 1 year | 26.129 → 1.025 | 0.177 → 0.008 | Shared reverse map |
| 10 years | 28.192 → 1.036 | 0.186 → 0.008 | Shared reverse map |

The retained calendar conversions are approximately 14–28× faster for full reads in this fixture. A blanket reverse-map replacement would regress the cheap fixed-duration conversions, so it was not retained. This does not claim a 14–28× endpoint speedup.

### Distribution transaction-to-height lookup

- 966,505 block boundaries with deterministic variable transaction counts (1–3,000 transactions per block).
- 262,144 spending-lookup requests per pass. Each chosen block is represented by its first transaction; this is a locality model, not a captured UTXO-spend trace.
- Random: uniformly selected blocks. Recent mix: 80% from the last 4,096 blocks. Repeated transaction: runs of 16 identical requests. Interleaved hot: cycle through 256 recent transactions.
- Two warmup rounds followed by 20 retained rounds, rotating all four variants.
- The owned baseline keeps its existing cache between passes. Shared variants include one read-lock acquisition per pass; cursors start fresh each pass. Boundary-vector creation/cloning is outside timing.
- The removed duplicate boundary payload at this fixture size is 3,866,020 bytes (about 3.69 MiB); this is not an RSS measurement.

Owned cached → shared cached timings, in milliseconds:

| Pattern | Median | Mean |
|---|---:|---:|
| Random | 10.553 → 9.713 | 10.624 → 9.770 |
| Recent mix | 7.429 → 6.348 | 7.556 → 6.398 |
| Repeated transaction | 2.118 → 1.541 | 2.209 → 1.564 |
| Interleaved hot | 1.289 → 1.278 | 1.302 → 1.293 |

The initial comparison found the single-interval shared cursor materially slower for interleaved hot requests (5.087 ms versus 1.302 ms for the owned cache). Sharing boundaries therefore retains the existing direct-mapped cache algorithm rather than replacing it with the single-interval cursor or uncached binary search. The small final interleaved difference should be treated as parity, not a meaningful speedup.

## Reproduction

```sh
cargo test --offline -p bitview_plugin_mappings --lib reverse_mapping_read_costs -- --ignored --nocapture
cargo test --offline -p bitview_plugin_mappings --lib distribution_height_lookups -- --ignored --nocapture
```

Both benchmarks print means and medians and validate every output. They are ignored in ordinary test runs.

Regression coverage includes published-height bounds; empty, skipped, and future days; source append/reorg/truncation through retained clones; direct/sorted/chunk/fold reads; cache collisions and terminal ranges; investing daily projections; and server-level mapping/derived-series checks through append and reorg. Transaction-publication and concurrent cache-eviction/reorg fixtures exercise distribution's shared map with non-coinbase spends.

Validation passed: 143 tests across the five changed packages, three vecdb index/bounds tests, and four server integration tests (150 total). Both paired benchmarks above passed separately. All-target checking, Clippy with the existing allowance below, formatting, and diff whitespace checks also passed. Six manual benchmarks were skipped by the ordinary suite; this work explicitly ran the two paired benchmarks listed above.

```sh
cargo check --offline -p bitview_vecs -p bitview_plugin_mappings -p bitview_plugin_investing -p bitview_plugin_distribution -p rangeindex --all-targets --features bitview_vecs/diagnostics
cargo test --offline -p bitview_vecs -p bitview_plugin_mappings -p bitview_plugin_investing -p bitview_plugin_distribution -p rangeindex --lib --tests --features bitview_vecs/diagnostics
cargo test --offline -p vecdb --all-features --test index_vec --test read_bounds
cargo test --offline -p bitview_server --lib resident_resolution_mappings_preserve_last_values_through_append_and_reorg -- --nocapture
cargo test --offline -p bitview_server --lib transaction_publication -- --nocapture
cargo test --offline -p bitview_server --lib chart_reads_survive_concurrent_cache_eviction_and_reorgs -- --nocapture
cargo clippy --offline -p bitview_vecs -p bitview_plugin_mappings -p bitview_plugin_investing -p bitview_plugin_distribution -p rangeindex --all-targets --features bitview_vecs/diagnostics -- -D warnings -A clippy::single_range_in_vec_init
cargo fmt --all -- --check
git diff --check
```

The Clippy allowance is for an existing unrelated vecdb test lint. The transaction-publication and cache-pressure reruns used the freshly built server test binary directly, with the same filters above, to avoid another build. Server fixtures require temporary localhost listeners.
