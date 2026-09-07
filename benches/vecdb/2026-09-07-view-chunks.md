# Borrowed chunks and batch transforms for vector views

Measured locally on 2026-09-07 with rustc 1.98.1. These are synthetic warm
in-process reads, not indexing throughput or HTTP latency.

The baseline was measured before these changes, against the working tree that
already included the earlier `CachedVec`, `LazyVec`, and `LazyIndexedVec` chunk
optimizations. It is not a comparison against an unmodified Git HEAD.

## Reproduction

```sh
cargo test --locked -p bitview_compute --test view_chunk_bench -- --ignored --nocapture --test-threads=1
```

The same benchmark workload ran before and after: 3,014,656 values, a nested
identity `LazyVec` around each view, resident input caches, one untimed validation
pass, one discarded timing round, and nine measured rounds. Case and operation
order rotate. The repository's optimized test profile is used (inherited dev
opt-level 2 / thin LTO), not a release build. Builds finished before timing.

Reads materialize the whole result; folds sum the values; JSON uses the actual
vector export method. Expected numeric values are checked independently, and
every timed JSON result is checked byte-for-byte against the expected output.

## Median milliseconds

| View | Read before | Read after | Fold before | Fold after | JSON before | JSON after |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Column projection | 0.937209 | 0.494334 | 1.531583 | 0.912792 | 21.032542 | 21.108083 |
| Two-column sum | 2.451417 | 1.487542 | 2.795292 | 2.165709 | 22.206291 | 22.645292 |
| Since-day | 6.195250 | 1.698583 | 6.977083 | 2.571750 | 22.168000 | 21.037042 |
| Window | 8.129542 | 5.226125 | 9.590375 | 6.037208 | 24.545833 | 24.243750 |
| Lookback | 6.947083 | 1.424458 | 7.822708 | 2.287375 | 23.892208 | 19.620833 |
| Previous delta | 2.811250 | 0.528250 | 3.362375 | 1.475458 | 18.935375 | 14.344792 |

All six read and fold cases improve. Column/sum/window JSON differences are small
relative to variability and should not be interpreted as a reliable export gain.
The benchmark prints min/max as well as medians for subsequent comparisons.

## Implementation boundaries

- Column projections forward borrowed cached slices, including through nested
  identity views. Column sums benefit from the same column-read path.
- Since-day, window, and lookback dispatch captured operations once per batch.
- Previous-delta processes adjacent slices; only the boundary value is carried
  between chunks, allowing the inner loop to optimize without a per-value carry.
- Window and lookback finish one buffered historical read before borrowing the
  current range. This preserves sparse emitted-value indexing and avoids recursive
  reads while a columnar source holds a non-reentrant publication lock.
- Output chunks reuse bounded scratch space; no output cache or retained input
  snapshot was added. Cache admission, budgets, and invalidation are unchanged.
- Concrete view fallible folds retain scalar transform/consumer ordering and
  first-error exit. Type-erased boxed folds still use their existing buffered path.
- Eager and generated storage wrappers forward chunk traversal. Their forwarding
  and chunk boundaries are tested; no independent wrapper speedup is claimed here.

During development, an initial per-value carry loop regressed previous-delta
reads; the adjacent-slice loop above replaced it. A faster nested-borrow experiment
for window/lookback was also removed because of the publication-lock risk. The
table reports the final concurrency-safe implementation, not those experiments.

## Validation

The final implementation passed `cargo test --locked -p vecdb -p bitview_compute`
and `cargo check --locked -p bitview_default -p bitview_server`. Additional tests
cover borrowed cache pointers, budget denial, source rewrites, chunk boundaries,
captured closures, early errors, sparse alignment, and non-reentrant sources.
