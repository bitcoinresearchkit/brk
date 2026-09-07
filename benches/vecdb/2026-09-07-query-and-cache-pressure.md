# Complete queries, decoder counts, and cache pressure

Measured on the local machine on 2026-09-07. Optimized test profile with
`CARGO_PROFILE_TEST_DEBUG=0 CARGO_PROFILE_TEST_LTO=false`, using the isolated
`/private/tmp/bitview-view-verification.E4UAhn` target directory.
These are synthetic fixtures, not production-server latency measurements.

## Complete outspends query

`benchmark_complete_outspends` builds a temporary indexed block containing a
4,096-output transaction and one transaction spending all its outputs. It runs
the public `Query::outspends` path, including plugin admission, transaction
resolution, spending-position lookup, transaction IDs, heights, block status,
and output construction. HTTP and JSON serialization are excluded. The input
order is either sequential or the permutation `(i * 2053) % 4096`.

Each median uses twelve measured rounds of 100 queries after three warm-up
rounds. Correctness checks independently map returned input positions back to
the original output slots. This fixture does not reproduce the previous
cross-page, many-transaction lookup benchmark.

The prior implementation is preserved in the temporary `outspends-baseline`
executable for repeat runs. Its initial ordered/shuffled medians were
104.051 / 211.310 microseconds. The updated resolver checks order while
collecting requests, visits positions directly into the output array, and
reuses the sorted index allocation for the second lookup.

After all builds and other integration tests finished, consecutive baseline and
updated runs measured:

| Complete query, 4,096 outputs | Prior implementation | Updated |
| --- | ---: | ---: |
| Ordered | 93.544 us | 91.274 us |
| Shuffled | 134.382 us | 133.511 us |

The complete-query improvement is small (about 2.4% ordered, 0.6% shuffled),
not the large speedup seen in isolated cross-page lookup benchmarks. The
ordered pre-scan and intermediate positions allocation are gone without a
measured full-query regression in this fixture. The larger differences in
earlier runs during other activity are not used as performance claims.

## Actual page and column operations

Run the ignored `chart_lookup_profile` integration test with
`-p bitview_compute --features diagnostics`. The opt-in feature counts calling-
thread page decode attempts (including raw pages) and read-only sparse column
reads. Default builds contain neither counters nor increments. Counters are
operation counts, not distinct page identities, and do not aggregate workers.

The fixture has one million cumulative rows in two independently compressed
columns, a cached rolling delta, and daily/monthly last-value aggregation.
Normal partial reads leave the cumulative and rolling snapshots nonresident.

| Query | Warm page decodes | Sparse column reads | Initial median |
| --- | ---: | ---: | ---: |
| 365 daily values, normal partial | 112 | 2 | 162.375 us |
| 120 monthly values, normal partial | 244 | 2 | 638.292 us |
| 365 daily values, cumulative resident | 0 | 0 | 3.708 us |
| 120 monthly values, cumulative resident | 0 | 0 | 1.250 us |
| 365 daily values, rolling resident | 0 | 0 | 1.166 us |
| 120 monthly values, rolling resident | 0 | 0 | 0.542 us |

The first daily read counted 601 page decodes, including initial metadata-cache
materialization; subsequent daily reads counted 112. All twelve monthly reads
counted 244. Value equality and stable warm counts are checked outside timing.
The resident cases demonstrate avoided source work, not a recommendation to
materialize every vector. They do not establish a live CPU-time percentage.

## Eviction pressure

`cache_budget::tests::benchmark_eviction_pressure` uses 1,000 or 10,000 registered
entries, of which 512 are resident. Atomic byte accounting and invalidation
callbacks are real; payload bytes are simulated to avoid gigabyte allocations.
Registry construction is excluded. The benchmark retains the previous repeated
minimum scan as an alternating same-process control.

The change preserves an allocation-free single-victim eviction. When that is
insufficient, it snapshots resident candidates once, sorts by last access, and
invalidates outside the registry lock until reservation succeeds. This changes
multi-victim work from repeated full scans to two scans plus sorting the resident
entries. Ordering is approximate LRU under concurrent accesses. If another
reservation consumes the available capacity, failure remains a safe uncached
fallback. The byte limit is unchanged.

Tests cover oldest-first selection, exact multi-victim stopping, released
capacity, dropped registrations, and oversized reservation rejection.

Final paired run, median of fifteen attempts per implementation:

| Registered entries | Victims | Previous repeated scan | Batched candidates |
| ---: | ---: | ---: | ---: |
| 1,000 | 1 | 0.875 us | 0.875 us |
| 1,000 | 64 | 42.042 us | 4.083 us |
| 1,000 | 512 | 305.542 us | 6.250 us |
| 10,000 | 1 | 9.042 us | 9.125 us |
| 10,000 | 64 | 569.292 us | 20.500 us |
| 10,000 | 512 | 4,687.208 us | 25.250 us |

## Verification

- Query library: 56 passed, 7 ignored diagnostics.
- vecdb library, all features: 21 passed, including cached concurrency and
  compressed sparse reads through both I/O and mmap.
- Cache budget: three correctness tests and the pressure benchmark passed.
- Server publication/reorg: both tests passed, including HTTP 200/304/404
  behavior across replacement and independent single/batch outspend checks.
- Diagnostic chart reproduction: passed with stable operation counts in a
  second run (normal monthly 608.833 us, 244 decodes, two sparse column reads).
- `git diff --check`: clean.

No running production process was restarted and no production database changed.
