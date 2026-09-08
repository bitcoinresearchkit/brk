#[allow(dead_code)]
mod common;

use std::{array, hint::black_box, time::Instant};

use bitview_vecs::{CumulativeCountVec, LazyIndexedVec, LazyRollingRatioVec};
use brk_types::{Height, StoredU16, StoredU64};
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, BinaryTransform, BytesVec, CachedColumnarVec, CachedVec, ColumnId, ColumnarVec,
    Database, ImportableVec, LazyColumnarVec, LazyVec, ReadableBoxedVec, ReadableCloneableVec,
    ReadableColumnarVec, ReadableVec, UnaryTransform, VecValue, Version, WritableVec,
};

struct Ratio;
impl BinaryTransform<StoredU64, StoredU64, StoredU64> for Ratio {
    fn apply(a: StoredU64, b: StoredU64) -> StoredU64 {
        StoredU64::from(u64::from(a) / u64::from(b))
    }
}
struct Twice;
impl UnaryTransform<StoredU64> for Twice {
    fn apply(value: StoredU64) -> StoredU64 {
        StoredU64::from(u64::from(value) * 2)
    }
}

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
        array::from_fn(|i| f(Self::ALL[i]))
    }
    fn map<T: VecValue, U: VecValue, F: FnMut(T) -> U>(row: Self::Row<T>, f: F) -> Self::Row<U> {
        row.map(f)
    }
}

#[test]
#[ignore = "synthetic remaining-view benchmark; run unchanged before and after"]
fn benchmark_remaining_views() {
    const N: usize = 262_144;
    const WINDOW: usize = 131_072;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let source = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "source",
        (0..N).map(|i| StoredU64::from((i as u64 + 1) * 3)),
    ));
    let cached = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "cached",
        (0..N).map(|i| StoredU64::from(i as u64 + 1)),
    ));
    let starts = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "starts",
        (0..N).map(|i| Height::from(i.saturating_sub(WINDOW))),
    ));
    let blocks = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "blocks",
        (0..N).map(|_| StoredU16::from(1u16)),
    ));
    let denominator = CumulativeCountVec::new(blocks.read_only_cached_boxed_clone());
    source.snapshot();
    cached.snapshot();
    starts.snapshot();
    let mut columns =
        ColumnarVec::<BytesVec<Height, StoredU64>, Column>::import(&db, "columns", Version::ONE)
            .unwrap();
    for i in 0..N {
        columns.push([StoredU64::from(i as u64 * 3), StoredU64::from(7u64)]);
    }
    columns.write().unwrap();
    let columns = CachedColumnarVec::new(columns.read_only_clone(), Version::ONE, CachedVec::wrap);
    let transformed = LazyColumnarVec::transformed::<Twice>("transformed", Version::ONE, columns);
    let cases: Vec<(&str, ReadableBoxedVec<Height, StoredU64>)> = vec![
        (
            "rolling_ratio",
            LazyRollingRatioVec::<StoredU64, StoredU64, StoredU64, Ratio>::new(
                "ratio",
                Version::ONE,
                source.read_only_boxed_clone(),
                cached.read_only_cached_boxed_clone(),
                starts.read_only_cached_boxed_clone(),
            )
            .read_only_boxed_clone(),
        ),
        (
            "block_ratio",
            LazyIndexedVec::new(
                "block_ratio",
                Version::ONE,
                &denominator,
                &source,
                |_, count, numerator| Ratio::apply(numerator, count),
            )
            .read_only_boxed_clone(),
        ),
        (
            "rolling_block_ratio",
            LazyRollingRatioVec::<
                StoredU64,
                StoredU64,
                StoredU64,
                vecdb::ReverseOperands<Ratio>,
            >::new(
                "rolling_block_ratio",
                Version::ONE,
                denominator.read_only_boxed_clone(),
                source.read_only_boxed_clone(),
                starts.read_only_cached_boxed_clone(),
            )
            .read_only_boxed_clone(),
        ),
        (
            "lazy_column",
            transformed
                .column("column", Version::ONE, Column::First)
                .read_only_boxed_clone(),
        ),
    ];
    for (name, view) in cases {
        let nested = LazyVec::<Height, StoredU64, Height, StoredU64>::transformed::<Twice>(
            "nested",
            Version::ONE,
            view.clone(),
        );
        let expected: Vec<_> = (0..N)
            .map(|i| {
                StoredU64::from(if name == "lazy_column" {
                    i as u64 * 6
                } else {
                    3
                })
            })
            .collect();
        assert_eq!(view.collect_range_at(0, N), expected);
        let mut times = vec![Vec::new(); 4];
        for round in 0..10 {
            for offset in 0..4 {
                let mode = (round + offset) % 4;
                let from = if mode < 2 { N - 64 } else { 0 };
                let start = Instant::now();
                let actual = if mode % 2 == 0 {
                    view.collect_range_at(black_box(from), black_box(N))
                } else {
                    nested.collect_range_at(black_box(from), black_box(N))
                };
                let elapsed = start.elapsed();
                let expected: Vec<_> = expected[from..]
                    .iter()
                    .map(|v| StoredU64::from(u64::from(*v) * if mode % 2 == 0 { 1 } else { 2 }))
                    .collect();
                assert_eq!(actual, expected);
                black_box(actual);
                if round > 0 {
                    times[mode].push(elapsed);
                }
            }
        }
        for (label, samples) in ["short_direct", "short_nested", "full_direct", "full_nested"]
            .into_iter()
            .zip(&mut times)
        {
            samples.sort();
            eprintln!(
                "{name}/{label}: median={:?} min={:?} max={:?}",
                samples[4], samples[0], samples[8]
            );
        }
    }
    let mut times = Vec::new();
    for round in 0..10 {
        let start = Instant::now();
        let rows = transformed.collect_range_at(0, N);
        let elapsed = start.elapsed();
        assert!(
            rows.iter()
                .enumerate()
                .all(|(i, row)| *row == [StoredU64::from(i as u64 * 6), StoredU64::from(14u64)])
        );
        black_box(rows);
        if round > 0 {
            times.push(elapsed);
        }
    }
    times.sort();
    eprintln!(
        "lazy_column/rows: median={:?} min={:?} max={:?}",
        times[4], times[0], times[8]
    );
}
