use std::sync::Arc;

use bitview_traversable::Traversable;
use bitview_vecs::{DailyMappings, DailyMetric, PerBlock};
use brk_exit::Exit;
use brk_types::{Day1, Height, StoredU64, Version};
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, Budgeted, CachedVec, Database, EagerVec, PcoVec, Pinned, ReadOnlyClone,
    ReadableVec, Rw, TypedVec, WritableVec,
};

use crate::common::CACHE_BUDGET;

#[allow(dead_code)]
mod common;

fn is_budgeted<V: TypedVec>(_: &CachedVec<V, Budgeted>) {}
fn is_pinned<V: TypedVec>(_: &CachedVec<V, Pinned>) {}

#[test]
fn compact_ratio_reads_do_not_retain_an_expanded_cumulative_history() {
    use bitview_transforms::RatioU64;
    use bitview_vecs::{CumulativeCountVec, LazyPercentPerBlock};
    use brk_types::{PartsPerMillion32, StoredU16};
    use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
    use vecdb::{LazyVec, ReadableCloneableVec};

    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = common::indexes(&db);
    let counts = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "compact_counts",
        [StoredU16::from(3u16); 16],
    ));
    let compact = CumulativeCountVec::new(&counts);
    static READS: AtomicUsize = AtomicUsize::new(0);
    let counted = LazyVec::init(
        "counted",
        Version::ONE,
        compact.read_only_boxed_clone(),
        |_, value| {
            READS.fetch_add(1, Relaxed);
            value
        },
    );
    let denominator = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "denominator",
        (1..=16).map(|i| StoredU64::from(i * 7u64)),
    ));
    let ratio = LazyPercentPerBlock::from_ratio::<_, _, RatioU64<PartsPerMillion32>>(
        "share",
        Version::ONE,
        &counted,
        &denominator,
        &indexes,
    );
    for _ in 0..2 {
        READS.store(0, Relaxed);
        assert_eq!(
            ratio.ppm.height.collect(),
            vec![PartsPerMillion32::from(3.0 / 7.0); 16]
        );
        assert!(
            READS.load(Relaxed) > 0,
            "compact source acquired an expanded cache"
        );
    }
}

#[test]
fn generic_clones_share_source_snapshots_but_do_not_retain_derived_histories() {
    use crate::common::CACHE_BUDGET;
    use bitview_vecs::Resolutions;
    use vecdb::{CachedReadableVec, LazyVec, ReadableBoxedVec, ReadableCloneableVec};

    fn snapshot<V: ReadableVec<Height, StoredU64> + Clone>(source: &V) -> Arc<Vec<StoredU64>> {
        source.clone().snapshot()
    }

    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let raw =
        common::stored::<Height, _>(&db, "generic_snapshot", (0..4096u64).map(StoredU64::from));
    let pinned = CachedVec::wrap(raw.read_only_clone());
    let expected = pinned.snapshot();
    let erased = ReadableBoxedVec::new(pinned.cached_boxed_clone());
    assert!(Arc::ptr_eq(&expected, &snapshot(&pinned)));
    assert!(Arc::ptr_eq(&expected, &snapshot(&erased)));
    let indexes = common::indexes(&db);
    let resolutions =
        Resolutions::from_source("generic_resolutions", &pinned, Version::ONE, &indexes);
    assert!(Arc::ptr_eq(
        &expected,
        &resolutions.height_source().snapshot()
    ));

    let budgeted = CACHE_BUDGET.wrap(raw.read_only_clone());
    let retained = budgeted.snapshot();
    assert!(Arc::ptr_eq(&retained, &snapshot(&budgeted)));
    assert!(Arc::ptr_eq(
        &retained,
        &snapshot(&budgeted.read_only_boxed_clone())
    ));

    let derived = LazyVec::init("double", Version::ONE, erased, |_: Height, value| {
        StoredU64::from(u64::from(value) * 2)
    });
    let first = snapshot(&derived);
    let second = snapshot(&derived);
    assert_eq!(first, second);
    assert_eq!(first[4095], StoredU64::from(8190u64));
    assert!(
        !Arc::ptr_eq(&first, &second),
        "the view retained a derived history"
    );
    assert!(Arc::ptr_eq(&expected, &pinned.snapshot()));

    let uncached = raw.read_only_clone();
    let first = snapshot(&uncached);
    let second = snapshot(&uncached);
    assert_eq!(first, second);
    assert!(
        !Arc::ptr_eq(&first, &second),
        "generic reads introduced a cache"
    );
}

#[test]
fn height_owner_catalog_and_read_only_clone_share_one_budgeted_cache() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = common::indexes(&db);
    let mut metric = PerBlock::<StoredU64>::forced_import(
        &CACHE_BUDGET,
        &db,
        "source_cache_height",
        Version::ONE,
        &indexes,
    )
    .unwrap();
    is_budgeted(&metric.height);
    for i in 0..4096 {
        metric.height.push(StoredU64::from(i as u64));
    }
    metric.height.write().unwrap();
    let reader = metric.read_only_clone();
    is_budgeted(&reader.height);
    // Eager dependency versions intentionally differ from lean stored versions.
    assert_ne!(metric.height.version(), reader.height.version());
    assert_eq!(
        metric.height.snapshot_version(),
        reader.height.snapshot_version()
    );
    assert!(metric.height.cached_snapshot().is_none());
    assert_eq!(reader.height.collect_last(), Some(StoredU64::from(4095u64)));
    assert!(
        metric.height.cached_snapshot().is_none(),
        "tail reads must not fill history"
    );

    // Exercise the real export boundary, not just the concrete wrapper API.
    let mut json = Vec::new();
    reader
        .iter_any_exportable()
        .find(|v| v.name() == "source_cache_height")
        .unwrap()
        .write_json(None, None, &mut json)
        .unwrap();
    assert!(json.starts_with(b"[0,1,2,"));
    let snapshot = metric
        .height
        .cached_snapshot()
        .expect("catalog bypassed source cache");
    assert!(Arc::ptr_eq(&snapshot, &reader.height.snapshot()));

    metric.height.truncate_if_needed_at(4096).unwrap();
    assert!(
        Arc::ptr_eq(&snapshot, &reader.height.snapshot()),
        "no-op truncation evicted cache"
    );
    metric.height.push(StoredU64::from(4096u64));
    metric.height.write().unwrap();
    assert_eq!(reader.height.collect_last(), Some(StoredU64::from(4096u64)));
    assert!(
        metric.height.cached_snapshot().is_none(),
        "append rebuilt full history"
    );

    reader.height.snapshot();
    metric.height.truncate_if_needed_at(4095).unwrap();
    metric.height.push(StoredU64::from(9000u64));
    metric.height.push(StoredU64::from(9001u64));
    metric.height.write().unwrap();
    assert!(reader.height.cached_snapshot().is_none());
    assert_eq!(reader.height.collect_last(), Some(StoredU64::from(9001u64)));
    assert_eq!(
        snapshot[4095],
        StoredU64::from(4095u64),
        "held snapshots must remain immutable"
    );
}

#[test]
fn pinned_policy_survives_owner_cloning_and_inner_compute_access_invalidates() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = common::indexes(&db);
    let mut metric = PerBlock::<StoredU64, Rw, Pinned>::forced_import(
        &CACHE_BUDGET,
        &db,
        "source_cache_pinned",
        Version::ONE,
        &indexes,
    )
    .unwrap();
    is_pinned(&metric.height);
    metric.height.push(StoredU64::from(1u64));
    metric.height.write().unwrap();
    let reader = metric.read_only_clone();
    is_pinned(&reader.height);
    let snapshot = reader.height.snapshot();
    // Mutable coercion is also used by compute helpers taking &mut EagerVec.
    let inner: &mut EagerVec<PcoVec<Height, StoredU64>> = &mut metric.height;
    inner.truncate_if_needed_at(0).unwrap();
    inner.push(StoredU64::from(2u64));
    inner.write().unwrap();
    assert!(reader.height.cached_snapshot().is_none());
    assert_eq!(reader.height.collect_last(), Some(StoredU64::from(2u64)));
    assert_eq!(&**snapshot, &[StoredU64::from(1u64)]);
}

#[test]
fn daily_owner_is_budgeted_and_its_catalog_uses_the_same_cache() {
    use vecdb::ReadableCloneableVec;

    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut indexes = common::indexes(&db);
    indexes.height_day1 = common::stored(
        &db,
        "daily_view_mapping",
        [Day1::from(0usize), Day1::from(4095usize)],
    )
    .read_only_boxed_clone();
    let mappings = DailyMappings::new(&indexes);
    let mut metric = DailyMetric::<StoredU64>::forced_import(
        &CACHE_BUDGET,
        &db,
        "source_cache_day",
        Version::ONE,
        &mappings,
    )
    .unwrap();
    is_budgeted(&metric.day1);
    for i in 0..4096 {
        metric.day1.push(StoredU64::from(i as u64));
    }
    metric.day1.write().unwrap();
    assert!(metric.day1.cached_snapshot().is_none());
    assert_eq!(
        metric.views.height.collect(),
        [Some(StoredU64::from(0u64)), Some(StoredU64::from(4095u64))]
    );
    let view_snapshot = metric
        .day1
        .cached_snapshot()
        .expect("daily view bypassed cache");
    let reader = metric.read_only_clone();
    is_budgeted(&reader.day1);
    let mut json = Vec::new();
    reader
        .iter_any_exportable()
        .find(|v| v.name() == "source_cache_day")
        .unwrap()
        .write_json(None, None, &mut json)
        .unwrap();
    let snapshot = metric
        .day1
        .cached_snapshot()
        .expect("daily catalog bypassed cache");
    assert!(Arc::ptr_eq(&snapshot, &reader.day1.snapshot()));
    assert!(Arc::ptr_eq(&snapshot, &view_snapshot));
    metric
        .day1
        .truncate_if_needed(Day1::from(4095usize))
        .unwrap();
    metric.day1.push(StoredU64::from(7000u64));
    metric.day1.write().unwrap();
    assert_eq!(reader.day1.collect_last(), Some(StoredU64::from(7000u64)));
    assert_eq!(
        reader.views.height.collect_last(),
        Some(Some(StoredU64::from(7000u64)))
    );
}

#[test]
fn incremental_compute_reads_only_the_new_tail_and_invalidates_rewrites() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let indexes = common::indexes(&db);
    let mut source = PerBlock::<StoredU64>::forced_import(
        &CACHE_BUDGET,
        &db,
        "incremental_source",
        Version::ONE,
        &indexes,
    )
    .unwrap();
    let mut target = PerBlock::<StoredU64>::forced_import(
        &CACHE_BUDGET,
        &db,
        "incremental_target",
        Version::ONE,
        &indexes,
    )
    .unwrap();
    for i in 0..4096u64 {
        source.height.push(StoredU64::from(i));
    }
    source.height.write().unwrap();
    let exit = Exit::new();
    let compute = |from: usize, target: &mut PerBlock<StoredU64>, source: &PerBlock<StoredU64>| {
        let mut count = 0;
        target
            .height
            .compute_transform(
                Height::from(from),
                &source.height,
                |(height, value, _)| {
                    count += 1;
                    (height, value)
                },
                &exit,
            )
            .unwrap();
        count
    };
    assert_eq!(compute(0, &mut target, &source), 4096);
    let reader = target.read_only_clone();
    let original = reader.height.snapshot();
    assert!(source.height.cached_snapshot().is_some());

    source.height.push(StoredU64::from(4096u64));
    source.height.write().unwrap();
    assert_eq!(compute(4096, &mut target, &source), 1);
    assert!(
        source.height.cached_snapshot().is_none(),
        "tail update filled input history"
    );
    assert!(reader.height.cached_snapshot().is_none());
    assert_eq!(reader.height.collect_last(), Some(StoredU64::from(4096u64)));

    reader.height.snapshot();
    source.height.truncate_if_needed_at(4095).unwrap();
    source.height.push(StoredU64::from(8000u64));
    source.height.push(StoredU64::from(8001u64));
    source.height.write().unwrap();
    assert_eq!(compute(4095, &mut target, &source), 2);
    assert!(reader.height.cached_snapshot().is_none());
    assert_eq!(reader.height.collect_last(), Some(StoredU64::from(8001u64)));
    assert_eq!(original[4095], StoredU64::from(4095u64));
}
