# bitview_vecs

Reusable metric vectors and views composed by Bitview plugins.

Plugins compose the vectors in this crate. `vecdb` supplies storage, cache,
and reader primitives; `bitview_compute` supplies stateful/range algorithms;
`bitview_transforms` supplies scalar operations; `bitview_collections` and
`bitview_cohort` supply typed shapes. None of those crates depends on this one.

## Layout

- `views`: single-source readers with aligned metadata and no retained result cache.
- `sources`: shared index and window-start readers.
- `resolutions`: composition of a height source into time and chain resolutions.
- `block`, `daily`, `tx`: metric families organized by their source index.
- `rolling`: rolling and delta compositions.
- `value`, `fiat`, `percent`, `ratio`: unit-specific compositions.
- `cohort`: cohort columns, aggregates, and typed count breakdowns.

Modules are private; the crate root exposes the supported types. Files are
named relative to their parent module (for example, `daily/metric.rs`, not
`daily/daily_metric.rs`) and define at most one public struct.
Small implementation-only helpers stay private to their owner or family.

## Composition

Larger families retain their components: `PriceWithRatio` owns a price and
its lazy ratio; cumulative families share `RollingTotals`/`RollingAmountTotals`;
`ValuePerBlockFull` adds distribution to the cumulative/rolling value family.
Traversal flattening preserves the public field layout without copying those
components' fields into every wrapper.

`Price` is the shared USD/cents/sats shape for stored, lazy, daily, and OHLC
views. Its field types retain integer versus fractional sats and each sampling
policy. `SpotPrice`, `OhlcPrice`, and `SplitPrice` own reusable price construction;
plugins retain provider policy. Investment-specific stacks retain the existing
`LazySpotValuePerBlock` instead of redeclaring its four unit fields.

`UTXOCoreColumns` owns the four disjoint age/epoch/class/entry axes.
`UTXOTypedColumns` composes that core with output types; `UTXOColumns` adds
output amounts. Each axis uses `ColumnarPerBlock`, with one varying cohort axis
and one metric per stored vector. Logical membership and rows remain in
`bitview_cohort`. Plugins keep these backing columns under `stored`, separate
from their public cohort views.

`ExactUTXOColumns` composes direct columns with `UTXOOverlappingColumns` for
independently calculated all/STH/LTH and threshold results. Additive metrics
sum canonical source columns instead; neither policy substitutes for the other.
Cumulative scalar and sats/cents families retain this composition and share
writer-checkpoint handling. Amount views compose the same block families.

`TypeCounts` composes a typed breakdown with `CountTotal`. Plugins select the
total's adjustment and retention, including coinbase exclusion; vector assembly
does not infer those business rules. OHLC, daily percentile prices, mapping
readers, and integer-cent SMA readers also live here; their plugins retain
provider, calendar, percentile-selection, and period-selection policy.

## Ownership

Source owners select cache retention using vecdb strategies. Constructors that
create budgeted caches receive an explicit shared `CacheBudget`; this crate
owns neither a global budget nor its limit/invalidation lifecycle. Application
composition passes the budget through plugin import resources.

 Views borrow cloneable readers at
construction and retain read-only clones; they do not create another cache
for each resolution or metric. Pinned metadata is shared with its owner.

`CumulativeCountVec` reconstructs cumulative `u64` counts from a shared `u16`
snapshot and one prefix checkpoint per 256 blocks. It implements the ordinary
reader interface, so rolling and ratio views use the same composition as other
cumulative sources. Ratio readers request only the needed ranges from both
operands, including on a cold budgeted cache. The owning count column
invalidates its block cache after changes.

Writer-only cumulative checkpoints use `StorageMode::WriteOnly`, so read-only
handles contain no unused row payload. Exposing mutable storage invalidates
their writer checkpoint. Count-total owners must also invalidate transformed
denominators when their sources are rewritten without changing length.

## Verification

```sh
cargo test -p bitview_collections -p bitview_transforms -p bitview_compute -p bitview_vecs --features bitview_vecs/diagnostics
cargo check -p bitviewd -p bitview_vecs --all-targets --features bitview_vecs/diagnostics
```

Tests cover calculation boundaries, storage resume/rewind, cache invalidation,
and exported values. Historical one-off benchmark results remain in `benches/vecdb`;
their comparison harnesses are no longer part of the test suite.
