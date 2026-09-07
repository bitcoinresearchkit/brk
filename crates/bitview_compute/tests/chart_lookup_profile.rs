#![cfg(feature = "diagnostics")]

#[allow(dead_code)]
mod common;

use std::{collections::BTreeSet, hint::black_box, sync::Arc, time::Instant};

use bitview_compute::CACHE_BUDGET;
use brk_types::{Day1, Height, Month1, StoredU64};
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, CachedVec, ColumnId, ColumnarVec, Database, DeltaSub, ImportableVec, LazyAggVec,
    LazyDeltaVec, PcoVec, ReadableCloneableVec, ReadableVec, VecValue, Version, WritableVec,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Column {
    First,
    Second,
}
impl ColumnId for Column {
    type Row<T: VecValue> = [T; 2];
    const VERSION: Version = Version::ONE;
    const ALL: &'static [Self] = &[Self::First, Self::Second];
    fn index(self) -> usize {
        self as usize
    }
    fn get<T: VecValue>(self, row: &Self::Row<T>) -> &T {
        &row[self.index()]
    }
    fn get_mut<T: VecValue>(self, row: &mut Self::Row<T>) -> &mut T {
        &mut row[self.index()]
    }
    fn from_fn<T: VecValue, F: FnMut(Self) -> T>(mut f: F) -> Self::Row<T> {
        std::array::from_fn(|i| f(Self::ALL[i]))
    }
    fn map<T: VecValue, U: VecValue, F: FnMut(T) -> U>(row: Self::Row<T>, f: F) -> Self::Row<U> {
        row.map(f)
    }
}

#[test]
#[ignore = "Chart lookup/cache reproduction, not a live server profile"]
fn profile_chart_lookups() {
    const N: usize = 1_000_000;
    const WINDOW: usize = 30 * 144;
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut columns =
        ColumnarVec::<PcoVec<Height, StoredU64>, Column>::import(&db, "cumulative", Version::ONE)
            .unwrap();
    for i in 0..N {
        columns.push([
            StoredU64::from((i as u64 + 1) * 3),
            StoredU64::from((i as u64 + 1) * 4),
        ]);
    }
    columns.write().unwrap();
    let source = CACHE_BUDGET.wrap(columns.sum_columns(
        "sum",
        Version::ONE,
        [Column::First, Column::Second],
    ));
    let starts = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "starts",
        (0..N).map(|i| Height::from(i.saturating_sub(WINDOW - 1))),
    ));
    let rolling_source = LazyDeltaVec::<Height, StoredU64, StoredU64, DeltaSub>::new(
        "rolling",
        Version::ONE,
        source.read_only_boxed_clone(),
        Version::ONE,
        move || starts.snapshot(),
    );
    let rolling = CACHE_BUDGET.wrap(rolling_source.clone());
    let days: Arc<Vec<Height>> = Arc::new((0..N).step_by(144).map(Height::from).collect());
    let months: Arc<Vec<Height>> = Arc::new((0..N).step_by(4320).map(Height::from).collect());
    let daily = LazyAggVec::<Day1, Option<StoredU64>, Height, Height, StoredU64>::new(
        "daily",
        Version::ONE,
        Version::ONE,
        rolling.read_only_boxed_clone(),
        {
            let days = days.clone();
            move || days.clone()
        },
    );
    let monthly = LazyAggVec::<Month1, Option<StoredU64>, Height, Height, StoredU64>::new(
        "monthly",
        Version::ONE,
        Version::ONE,
        rolling.read_only_boxed_clone(),
        {
            let months = months.clone();
            move || months.clone()
        },
    );
    // Candidate moves the rolling cache to the aggregate result, rather than
    // stacking another cache on the budgeted rolling source used above.
    let candidate_monthly = CACHE_BUDGET.wrap(LazyAggVec::<
        Month1,
        Option<StoredU64>,
        Height,
        Height,
        StoredU64,
    >::new(
        "candidate_monthly",
        Version::ONE,
        Version::ONE,
        rolling_source.read_only_boxed_clone(),
        {
            let months = months.clone();
            move || months.clone()
        },
    ));
    let candidate_daily = CACHE_BUDGET.wrap(LazyAggVec::<
        Day1,
        Option<StoredU64>,
        Height,
        Height,
        StoredU64,
    >::new(
        "candidate_daily",
        Version::ONE,
        Version::ONE,
        rolling_source.read_only_boxed_clone(),
        {
            let days = days.clone();
            move || days.clone()
        },
    ));
    // Warm required metadata, but not either metric snapshot.
    monthly.collect_range_at(months.len() - 120, months.len());
    fn profile_candidate<I: vecdb::VecIndex>(
        name: &str,
        count: usize,
        len: usize,
        candidate: &impl ReadableVec<I, Option<StoredU64>>,
    ) {
        let mut samples = Vec::new();
        let mut counts = Vec::new();
        for _ in 0..12 {
            vecdb::diagnostics::take();
            let started = Instant::now();
            let values = candidate.collect_range_at(len - count, len);
            samples.push(started.elapsed());
            counts.push(vecdb::diagnostics::take());
            assert_eq!(
                values,
                vec![Some(StoredU64::from((WINDOW * 7) as u64)); count]
            );
        }
        let first = samples.remove(0);
        samples.sort();
        eprintln!(
            "aggregate_candidate/{name}: first={first:?} warm={:?} first_counts={:?} warm_counts={:?} full_snapshot_bytes={}",
            samples[5],
            counts[0],
            counts[1],
            len * size_of::<Option<StoredU64>>()
        );
    }
    profile_candidate("monthly", 120, months.len(), &candidate_monthly);
    profile_candidate("daily", 365, days.len(), &candidate_daily);
    assert!(candidate_monthly.cached_snapshot().is_some());
    assert!(candidate_daily.cached_snapshot().is_none());
    assert!(source.cached_snapshot().is_none());
    assert!(rolling.cached_snapshot().is_none());
    CACHE_BUDGET.invalidate();
    for mode in ["normal_partial", "resident_cumulative", "resident_rolling"] {
        if mode == "resident_cumulative" {
            source.snapshot();
        }
        if mode == "resident_rolling" {
            rolling.snapshot();
        }
        for (name, count, mapping) in [("daily", 365, &days), ("monthly", 120, &months)] {
            let from = mapping.len() - count;
            let endpoints: Vec<_> = (from..mapping.len())
                .map(|i| mapping.get(i + 1).map_or(N, |h| usize::from(*h)) - 1)
                .collect();
            let chunk = source.cursor_chunk_size();
            let pages: BTreeSet<_> = endpoints
                .iter()
                .flat_map(|&i| [i / chunk, (i - WINDOW) / chunk])
                .collect();
            let mut samples = Vec::new();
            let mut counts = Vec::new();
            for round in 0..12 {
                vecdb::diagnostics::take();
                let start = Instant::now();
                let values = if name == "daily" {
                    daily.collect_range_at(from, mapping.len())
                } else {
                    monthly.collect_range_at(from, mapping.len())
                };
                let elapsed = start.elapsed();
                counts.push(vecdb::diagnostics::take());
                assert_eq!(
                    values,
                    vec![Some(StoredU64::from((WINDOW * 7) as u64)); count]
                );
                black_box(values);
                if round > 0 {
                    samples.push(elapsed);
                }
            }
            samples.sort();
            assert!(counts[1..].iter().all(|count| *count == counts[1]));
            if mode != "normal_partial" {
                assert_eq!(counts[1], (0, 0));
            } else {
                assert!(counts[1].0 > 0);
                assert_eq!(counts[1].1, 2);
            }
            eprintln!(
                "actual decode attempts / sparse column reads: first={:?}, warm={:?}",
                counts[0], counts[1]
            );
            eprintln!(
                "{mode}/{name}: median={:?}, endpoint_chunks={}, chunk_size={}, cumulative_resident={}, rolling_resident={}",
                samples[5],
                pages.len(),
                chunk,
                source.cached_snapshot().is_some(),
                rolling.cached_snapshot().is_some()
            );
        }
        let mut samples = Vec::new();
        for round in 0..12 {
            let start = Instant::now();
            let values = rolling.collect_range_at(N - 4096, N);
            let elapsed = start.elapsed();
            assert_eq!(values, vec![StoredU64::from((WINDOW * 7) as u64); 4096]);
            black_box(values);
            if round > 0 {
                samples.push(elapsed);
            }
        }
        samples.sort();
        eprintln!("{mode}/block4096: median={:?}", samples[5]);
    }
}
