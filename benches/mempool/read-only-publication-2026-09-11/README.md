# Mempool immutable publication measurements — 2026-09-11

One writer now publishes the entire read state through one ArcSwap. These
measurements compare the original `brk_mempool` against the implementation on
the same workspace dependencies. They quantify the cost of retaining valid
read data while the writer changes its private state.

## Setup and scope

- Apple M3 Pro, 12 logical CPUs, 36 GiB RAM; Rust 1.98.1, release profile with
  fat LTO and one codegen unit. Exact hashes are in [environment.json](environment.json).
- The benchmark uses an instrumented `System` allocator in its test binary.
  Allocation counters add overhead; absolute times are not production promises.
- Deterministic pools start with 0, 100, 10,000, 50,000 or 200,000 transactions:
  one input, two outputs, roughly one distinct address per transaction,
  1,000 fee levels, one parent/child pair per ten transactions, and up to
  2,500 transactions in the Core template selection.
- Eleven rounds per workload, with the first two discarded. Tables show the
  median of nine measured rounds. Raw p95/p99 values are observed sample tails,
  not statistically precise tail estimates.
- The final before/after pair ran after compilation and integration tests
  finished; the local daemon remained running.
- Timed stages execute the real native applier, input-fill/address updates,
  graph builder, completeness validation, freeze and publication. Fetch/JSON
  decoding, external RPC latency and HTTP serialization are excluded. RPC and
  HTTP correctness are exercised by the server suite separately.
- Workloads run sequentially. Churn, replacement and fill rounds change 1% of
  initial membership. Eviction removes up to 2,500 transactions each round;
  the later read workload therefore has 22,500 and 172,500 live transactions
  for the 50k and 200k starting cases. Earlier small cases become empty.

## Native update cost

Milliseconds per apply/build/publish cycle, including destruction caused by
replacing the current version. This is additional native processing, not an
end-to-end Bitcoin Core RPC measurement.

| Initial transactions | Workload | Before p50 ms | After p50 ms |
| ---: | --- | ---: | ---: |
| 50,000 | noop | 0.206 | 0.182 |
| 50,000 | fee | 14.917 | 15.385 |
| 50,000 | churn | 15.780 | 18.356 |
| 50,000 | replacement | 16.232 | 19.355 |
| 50,000 | fills | 15.858 | 17.720 |
| 50,000 | eviction | 14.993 | 18.810 |
| 50,000 | expiry | 0.158 | 0.153 |
| 200,000 | noop | 0.941 | 0.746 |
| 200,000 | fee | 40.914 | 40.327 |
| 200,000 | churn | 43.650 | 52.668 |
| 200,000 | replacement | 44.044 | 53.614 |
| 200,000 | fills | 41.252 | 48.764 |
| 200,000 | eviction | 39.929 | 50.585 |
| 200,000 | expiry | 0.984 | 0.882 |

The retained-data cost is intentional: the original implementation could reject
reads during these updates. Unchanged pools do not copy their containers. A
fee/template or tip change shares membership stores, and graveyard-only changes
share the live stores. Transaction bodies remain shared; address records copy
only when their contents change.

## Publication and retained memory

Initial freeze shares bodies and address records and copies the lookup maps.
Live bytes below are allocator-requested bytes above the test's starting state,
including private writer state and the current publication.

| Initial transactions | Before live MiB | After live MiB | Initial freeze ms | Freeze allocations |
| ---: | ---: | ---: | ---: | ---: |
| 50,000 | 59.0 | 73.4 | 1.624 | 179 |
| 200,000 | 235.5 | 293.0 | 7.227 | 179 |

The retention stress holds seven acquired versions while subsequent updates
publish. The baseline holds seven graph snapshots because its mutable live
pool could not be retained. These are different capabilities, so the difference
is the cost of the new retention guarantee. Normal HTTP selections retain
only needed bodies/histograms rather than the entire root.

| Initial transactions | Before retained live MiB | After retained live MiB | After peak MiB | Drop seven versions ms | Drop writer ms |
| ---: | ---: | ---: | ---: | ---: | ---: |
| 50,000 | 103.1 | 293.2 | 320.3 | 16.442 | 5.236 |
| 200,000 | 458.2 | 974.5 | 1037.6 | 56.902 | 42.549 |

There is no two-buffer memory bound. A final Arc drop can free a large old
version on its owning thread. The existing request/body admission limits remain
in force, and the implementation does not add hidden generation caches or a
reclamation worker. OS RSS/footprint counters in the raw output include allocator
rounding, fragmentation and the full sequence of test cases.

## Reads during updates

Four reader threads execute histogram, information, txid-hash and transaction
queries while ten updates run. A success requires all four queries to succeed.
Latency samples include every hundredth successful read; failures are counted
separately. These counts describe this bounded workload, not throughput targets.

| Initial transactions | Before successes / failures | After successes / failures |
| ---: | ---: | ---: |
| 50,000 | 4 / 21,335 | 23,005 / 0 |
| 200,000 | 2 / 77,021 | 87,499 / 0 |

## Measured design changes

The first correct implementation eagerly cloned address records and their
nested txid sets. The [intermediate measurement](eager-address.txt) at 200k
transactions required 200,179 initial-freeze allocations and approximately
117.5 MB. Sharing unchanged address records reduced this to 179 allocations
and approximately 59.6 MB. The intermediate initial freeze took about 63 ms;
that run overlapped compilation, so its timing is not directly comparable with
the final quiet run above. The allocation reduction is the useful comparison.

A separate earlier measurement found a fee-only refresh cloning about 118 MB
of unchanged membership data. Component sharing removed that copy. The retained
implementation uses ordinary Arc ownership, one content revision for the live
stores, and one graveyard revision; it adds no persistent collections or cache
invalidation framework.

The remaining map-copy and retention costs are accepted for uninterrupted,
immutable reads. The larger synthetic updates still fit comfortably inside
the existing one-second cycle period on this machine. RPC/JSON time is extra;
this result does not promise that a real 200k pool always finishes within one
second or that an arbitrarily old acquired version is cheap to retain.

## Live local daemon observations

The [live baseline](live-before.json) was taken at 2026-09-11 14:41:27 UTC from
the already-running version 0.12.2, indexed height 966517. Its binary hash and
full indexed block hash are recorded. It predates this refactor and was not
restarted. Of 32 requests each, the price/histogram routes returned 27–28
HTTP 503s; the remaining responses were 200 or correctly revalidated 304s.
This is evidence of the original problem, not a live deployment check of the
new binary.

A separate `cargo dev` invocation launched the new binary during validation.
The [after sample](live-after.json), at 15:05:28 UTC and height 966522, returned
one HTTP 200 and 31 revalidated 304s on each of the four routes: zero failures
across 128 requests. Its health response, start time, binary hash and full tip
are recorded. This task did not launch or restart that daemon.

The observations are short samples at different tips and cache states; they
support availability, not a controlled production latency comparison or a
promise of zero future 503s. Full-tip mismatch and unresolved confirmed data
remain legitimate failures. Isolated RPC/HTTP tests cover these boundaries.

## Reproduction and raw results

```sh
cargo test -p brk_mempool --release publication_benchmark -- --ignored --nocapture --test-threads=1
```

Run the benchmark alone to avoid compiler/test contention. To reproduce the
baseline, extract `crates/brk_mempool` from the recorded HEAD, retain the same
workspace dependency versions, copy the current benchmark, and apply
[baseline-adapter.patch](baseline-adapter.patch). Add its test module under
`driver.rs` and expose `TxGraveyard::shift_oldest_back` as `pub(crate)` under
`cfg(test)`. These adapters change test ownership plumbing only.

- [Original implementation](before.txt)
- [Final implementation](after.txt)
- [Intermediate eager address copies](eager-address.txt)
- [Environment and source identities](environment.json)


## Follow-up KISS review

These measurements compare the completed read-only implementation above with
its subsequent simplification, not with the original lock-based implementation.
The [review patch](review.patch) and [source/binary identities](review-environment.json)
record that exact boundary. Earlier results are preserved.

The review removed 49 net lines of production code:

- Use the existing transaction content revision as the single membership/body
  change signal for graph reuse; remove the duplicate boolean and its helper.
  Check that revision before scanning the template order.
- Remove obsolete lock scopes, redundant reference borrows and an unreachable
  second empty-fill check. Private insertion helpers are named as insertions.
- Restore the previous graph in place after a panic, preserving the rebuild
  counter rather than constructing a fresh counter alongside it.

The RBF rate pass remains separate because it avoids rate lookups for rejected
or over-limit trees. The map layout and ownership types remain simple; the
review adds no storage structure, cache or reclamation machinery.

Four runs used the same fixtures in before/after/after/before order, after
compilation finished. Each table entry is the range of the two per-run medians
(nine measured rounds in each run). This is a noise check, not a claim of a
statistically significant speedup.

| Initial transactions | Workload | Before review p50 ms | After review p50 ms |
| ---: | --- | ---: | ---: |
| 50,000 | noop | 0.180–0.198 | 0.179–0.181 |
| 50,000 | fee | 15.255–15.512 | 14.932–15.175 |
| 50,000 | churn | 18.827–18.852 | 18.848–18.875 |
| 50,000 | replacement | 18.906–19.867 | 18.699–19.814 |
| 50,000 | fills | 17.212–17.911 | 17.269–17.814 |
| 50,000 | eviction | 17.800–18.333 | 17.714–18.791 |
| 50,000 | expiry | 0.141–0.144 | 0.148–0.192 |
| 200,000 | noop | 0.749–0.753 | 0.747–0.754 |
| 200,000 | fee | 40.485–40.721 | 40.056–40.615 |
| 200,000 | churn | 51.786–52.204 | 52.166–52.645 |
| 200,000 | replacement | 54.240–55.220 | 53.519–53.651 |
| 200,000 | fills | 50.579–51.704 | 48.466–48.525 |
| 200,000 | eviction | 51.772–53.662 | 49.445–51.700 |
| 200,000 | expiry | 0.904–0.940 | 0.893–0.924 |

The large-pool update costs are effectively unchanged: 200k churn is about
52 ms in both versions. Some individual timings vary by several percent; the
small 50k expiry case includes an after-run increase of about 48 microseconds.
There is no broad throughput-speedup claim. Initial live bytes, initial-freeze
allocation counts and retained-version live/peak bytes match exactly. Both
versions served concurrent reads with zero failures at 50k and 200k.

Raw review runs: [before 1](review-before-1.txt), [after 1](review-after-1.txt),
[after 2](review-after-2.txt), [before 2](review-before-2.txt).

Validation after the review: 117 native tests and three doctests pass; the
exact real RPC/HTTP publication/reorg regression passes; all-target checks for
`brk_mempool`, `bitview_query`, `bitview`, `bitviewd` and `mmpl` pass. The test
commands and scope are recorded in the publication plan's follow-up section.
