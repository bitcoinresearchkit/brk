#![cfg(feature = "diagnostics")]

#[allow(dead_code)]
mod common;

use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use crate::common::CACHE_BUDGET;
use bitview_collections::{WindowId, Windows};
use bitview_transforms::{BlockCountTarget, RatioU64};
use bitview_traversable::Traversable;
use bitview_vecs::{
    CachedWindowStartVec, ConstantVecs, CumulativeCountVec, LazyPerBlock,
    LazyPercentCumulativeRolling, LazyPercentPerBlock, LazyWindowStartVec, PerBlock,
};
use brk_types::{Height, PartsPerMillion32, StoredU16, StoredU64, Timestamp, Version};
use vecdb::{
    AnyStoredVec, AnyVec, BinaryTransform, CachedVec, ColumnId, Database, Ident, IndexVec, LazyVec,
    ReadOnlyClone, ReadableCloneableVec, ReadableVec, UnaryTransform, WritableVec,
};

static TRANSFORMS: AtomicUsize = AtomicUsize::new(0);
static RATIOS: AtomicUsize = AtomicUsize::new(0);

struct CountRatio;

struct CountIdentity;

impl UnaryTransform<StoredU64, StoredU64> for CountIdentity {
    fn apply(value: StoredU64) -> StoredU64 {
        TRANSFORMS.fetch_add(1, Relaxed);
        value
    }
}

impl BinaryTransform<StoredU64, StoredU64, PartsPerMillion32> for CountRatio {
    fn apply(numerator: StoredU64, denominator: StoredU64) -> PartsPerMillion32 {
        RATIOS.fetch_add(1, Relaxed);
        RatioU64::<PartsPerMillion32>::apply(numerator, denominator)
    }
}

// One test keeps the process-wide decompression counters isolated.
#[test]
fn views_recompute_cheap_outputs_and_share_only_their_roots() {
    const N: usize = 32_768;
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut indexes = common::indexes(&db);
    let days: Vec<_> = (0..N).step_by(144).collect();
    let ends: Vec<_> = days
        .iter()
        .enumerate()
        .map(|(i, _)| days.get(i + 1).copied().unwrap_or(N) - 1)
        .collect();
    indexes.first_height.day1 = CachedVec::wrap(common::stored(
        &db,
        "days",
        days.iter().copied().map(Height::from),
    ))
    .read_only_boxed_clone();
    indexes.first_height.day1.snapshot();

    let mut stored = common::stored::<Height, _>(
        &db,
        "cumulative",
        (0..N).map(|i| StoredU64::from(3 * (i + 1) as u64)),
    );
    let root = CACHE_BUDGET.wrap(stored.read_only_clone());
    let view = LazyVec::init(
        "conversion",
        Version::ONE,
        root.read_only_boxed_clone(),
        |_, v| {
            TRANSFORMS.fetch_add(1, Relaxed);
            StoredU64::from(u64::from(v) * 2)
        },
    );
    let converted =
        LazyPerBlock::from_height_source::<Ident>("converted", Version::ONE, &view, &indexes);
    let alias = LazyPerBlock::from_lazy::<Ident, StoredU64>("alias", Version::ONE, &converted);
    let expected: Vec<_> = ends
        .iter()
        .map(|&i| Some(StoredU64::from(6 * (i + 1) as u64)))
        .collect();
    vecdb::diagnostics::take();
    assert_eq!(converted.day1.collect(), expected);
    assert_eq!(vecdb::diagnostics::take().0, N / 1024);
    for _ in 0..3 {
        TRANSFORMS.store(0, Relaxed);
        assert_eq!(alias.day1.collect(), expected);
        assert!(
            TRANSFORMS.load(Relaxed) > 0,
            "a view retained its output history"
        );
        assert_eq!(vecdb::diagnostics::take().0, 0, "root decoded again");
    }
    assert_eq!(
        converted.height.collect_range_at(1020, 1030),
        (1020..1030)
            .map(|i| StoredU64::from(6 * (i + 1) as u64))
            .collect::<Vec<_>>()
    );
    assert_eq!(converted.height.collect().len(), N);
    assert_eq!(vecdb::diagnostics::take().0, 0);

    // Stored constructors own a root. Conversions cannot accidentally select
    // an uncached height handle independently from their resolution handle.
    let mut metric = PerBlock::<StoredU64>::forced_import(
        &crate::common::CACHE_BUDGET,
        &db,
        "stored_metric",
        Version::ONE,
        &indexes,
    )
    .unwrap();
    for i in 0..N {
        metric.height.push(StoredU64::from(i as u64));
    }
    metric.height.write().unwrap();
    let units = LazyPerBlock::from_resolutions::<CountIdentity>("units", Version::ONE, &metric);
    let units_alias = LazyPerBlock::from_resolutions::<CountIdentity>(
        "units_alias",
        Version::ONE,
        &metric.resolutions,
    );
    metric.day1.collect();
    vecdb::diagnostics::take();
    TRANSFORMS.store(0, Relaxed);
    assert_eq!(
        units.height.collect(),
        (0..N)
            .map(|i| StoredU64::from(i as u64))
            .collect::<Vec<_>>()
    );
    assert_eq!(
        units_alias.day1.collect(),
        ends.iter()
            .map(|&i| Some(StoredU64::from(i as u64)))
            .collect::<Vec<_>>()
    );
    assert!(TRANSFORMS.load(Relaxed) >= N);
    assert_eq!(vecdb::diagnostics::take().0, 0);

    let denominator = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "denominator",
        (0..N).map(|i| StoredU64::from(7 * (i + 1) as u64)),
    ));
    denominator.read_only_cached_boxed_clone().snapshot();
    let timestamps = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "timestamps",
        (0..N).map(|i| Timestamp::from((i * 600) as u32)),
    ));
    let starts = WindowId::series(|id| {
        CachedWindowStartVec::new(LazyWindowStartVec::days(
            id.suffix(),
            Version::ONE,
            Windows::<()>::DAYS[id.index()] as u64,
            timestamps.read_only_cached_boxed_clone(),
        ))
    });
    let starts_ref = WindowId::series(|id| id.select(&starts));
    for id in WindowId::ALL {
        id.select(&starts).snapshot();
    }
    let ratios = LazyPercentCumulativeRolling::from_cumulative_ratio::<_, _, CountRatio>(
        "share",
        Version::ONE,
        &root,
        &denominator,
        &starts_ref,
        &indexes,
    );
    let expected_ratio = RatioU64::<PartsPerMillion32>::apply(3u64.into(), 7u64.into());
    let expected_ratios = vec![Some(expected_ratio); ends.len()];
    vecdb::diagnostics::take();
    for _ in 0..3 {
        RATIOS.store(0, Relaxed);
        assert_eq!(ratios.cumulative.ppm.day1.collect(), expected_ratios);
        for id in WindowId::ALL {
            assert_eq!(
                id.select(&ratios.rolling).ppm.day1.collect(),
                expected_ratios
            );
        }
        assert!(
            RATIOS.load(Relaxed) >= ends.len() * 5,
            "cumulative/rolling ratios must not retain output snapshots"
        );
        assert_eq!(vecdb::diagnostics::take().0, 0);
    }

    // A compact numerator must not acquire a second, expanded cumulative cache.
    // The counter stands in front of the compact state so implicit wrapping is observable.
    let counts = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "counts",
        (0..N).map(|_| StoredU16::from(3u16)),
    ));
    let compact = CumulativeCountVec::new(counts.read_only_cached_boxed_clone());
    let compact_view = LazyVec::init(
        "compact_view",
        Version::ONE,
        compact.read_only_boxed_clone(),
        |_, v| {
            TRANSFORMS.fetch_add(1, Relaxed);
            v
        },
    );
    let compact_ratio = LazyPercentPerBlock::from_ratio::<_, _, CountRatio>(
        "compact_share",
        Version::ONE,
        &compact_view,
        &denominator,
        &indexes,
    );
    assert_eq!(compact_ratio.ppm.day1.collect(), expected_ratios);
    TRANSFORMS.store(0, Relaxed);
    assert_eq!(compact_ratio.ppm.day1.collect(), expected_ratios);
    assert!(
        TRANSFORMS.load(Relaxed) > 0,
        "compact cumulative state was expanded"
    );

    // Same-length tail replacement must be visible after the normal compute invalidation.
    CACHE_BUDGET.invalidate();
    stored.truncate_if_needed(Height::from(N - 1)).unwrap();
    stored.push(StoredU64::from(3 * N as u64 + 21));
    stored.write().unwrap();
    vecdb::diagnostics::take();
    assert_eq!(
        alias.day1.collect().last().copied().flatten(),
        Some(StoredU64::from(6 * N as u64 + 42))
    );
    assert_eq!(vecdb::diagnostics::take().0, N / 1024);
    assert_eq!(
        ratios
            .cumulative
            .ppm
            .day1
            .collect()
            .last()
            .copied()
            .flatten(),
        Some(RatioU64::<PartsPerMillion32>::apply(
            StoredU64::from(3 * N as u64 + 21),
            StoredU64::from(7 * N as u64)
        ))
    );
    assert_eq!(vecdb::diagnostics::take().0, 0);

    // Height-only reads use source metadata, including growth/truncation, without decoding it.
    let heights = IndexVec::new(
        "heights",
        Version::ONE,
        stored.read_only_clone(),
        |height| StoredU64::from(u64::from(height) + 1),
    );
    vecdb::diagnostics::take();
    assert_eq!(
        heights.collect_range_at(1020, 1030),
        (1021..1031)
            .map(|i| StoredU64::from(i as u64))
            .collect::<Vec<_>>()
    );
    let mut selected = vec![StoredU64::ZERO];
    heights.read_sorted_into_at(&[0, 0, 1024, N - 1, N], &mut selected);
    assert_eq!(selected, [0, 1, 1, 1025, N as u64].map(StoredU64::from));
    assert!(heights.collect_range_at(N, N + 10).is_empty());
    assert!(heights.collect_range_at(10, 0).is_empty());
    assert_eq!(
        heights.fold_range_at(0, 3, 0u64, |a, v| a + u64::from(v)),
        6
    );
    let stopped: Result<(), ()> = heights.try_fold_range_at(0, N, (), |_, _| Err(()));
    assert!(stopped.is_err());
    assert_eq!(vecdb::diagnostics::take().0, 0);
    stored.push(StoredU64::from(123u64));
    stored.write().unwrap();
    vecdb::diagnostics::take();
    assert_eq!(heights.len(), N + 1);
    assert_eq!(
        heights.collect_one_at(N),
        Some(StoredU64::from(N as u64 + 1))
    );
    assert_eq!(vecdb::diagnostics::take().0, 0);
    stored.truncate_if_needed(Height::from(N - 1)).unwrap();
    stored.write().unwrap();
    vecdb::diagnostics::take();
    assert_eq!(heights.len(), N - 1);
    assert_eq!(heights.collect_one_at(N - 1), None);
    assert_eq!(vecdb::diagnostics::take().0, 0);

    // Constant resolution views use the same metadata-only primitive.
    let mut constant_indexes = common::indexes(&db);
    constant_indexes.height_minute10 = common::stored(
        &db,
        "constant_height",
        [brk_types::Minute10::from(0usize); 3],
    )
    .read_only_boxed_clone();
    constant_indexes.first_height.day1 = common::stored(
        &db,
        "constant_day",
        [Height::from(0usize), Height::from(2usize)],
    )
    .read_only_boxed_clone();
    constant_indexes.first_height.epoch =
        common::stored(&db, "constant_epoch", [Height::from(0usize)]).read_only_boxed_clone();
    let constants =
        ConstantVecs::new::<BlockCountTarget<1>>("target", Version::ONE, &constant_indexes);
    vecdb::diagnostics::take();
    assert_eq!(constants.height.collect(), [StoredU64::from(144u64); 3]);
    assert_eq!(constants.iter_any_exportable().count(), 16);
    assert_eq!(constants.day1.collect(), [StoredU64::from(144u64); 2]);
    assert_eq!(constants.epoch.collect(), [StoredU64::from(144u64)]);
    assert_eq!(
        constants.height.version(),
        Version::ONE + constant_indexes.height_minute10.version()
    );
    assert_eq!(vecdb::diagnostics::take().0, 0);
}
