#[allow(dead_code)]
mod common;

use std::{array, hint::black_box, sync::Arc, time::Instant};

use bitview_compute::{
    CachedBlockCountReader, LazyCumulativeIndexVec, LazyIndexCountVec, LazyPreviousDeltaVec,
};
use brk_types::{Height, StoredU16, StoredU64};
use tempfile::tempdir;
use vecdb::ReadOnlyClone;
use vecdb::{
    AnyStoredVec, CachedColumnarVec, CachedVec, ColumnId, ColumnarVec, Database, DeltaSub,
    ImportableVec, LazyColumnarVec, LazyDeltaVec, PcoVec, ReadableBoxedVec, ReadableCloneableVec,
    ReadableColumnarVec, ReadableVec, UnaryTransform, VecValue, Version, WritableVec,
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
        array::from_fn(|i| f(Self::ALL[i]))
    }
    fn map<T: VecValue, U: VecValue, F: FnMut(T) -> U>(row: Self::Row<T>, f: F) -> Self::Row<U> {
        row.map(f)
    }
}

struct Twice;
impl UnaryTransform<StoredU64> for Twice {
    fn apply(value: StoredU64) -> StoredU64 {
        StoredU64::from(u64::from(value) * 2)
    }
}

fn measure(name: &str, expected: &[StoredU64], mut read: impl FnMut() -> Vec<StoredU64>) {
    let mut samples = Vec::new();
    for round in 0..12 {
        let start = Instant::now();
        let actual = read();
        let elapsed = start.elapsed();
        assert_eq!(actual, expected, "{name}");
        black_box(actual);
        if round > 0 {
            samples.push(elapsed);
        }
    }
    samples.sort();
    eprintln!(
        "{name}: median={:?} min={:?} max={:?}",
        samples[5], samples[0], samples[10]
    );
}

#[test]
#[ignore = "Sorted lookup followups: validate and benchmark unchanged before and after"]
fn benchmark_lookup_followups() {
    const N: usize = 262_144;
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let source = common::stored::<Height, _>(
        &db,
        "source",
        (0..N).map(|i| StoredU64::from((i as u64 + 1) * 3)),
    );
    let first = common::stored::<Height, _>(&db, "first", (0..N).map(|i| Height::from(i * 3)));
    let terminal =
        common::stored::<Height, _>(&db, "terminal", (0..N * 3).map(|_| StoredU16::new(1)));
    let counts = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "counts",
        (0..N).map(|i| StoredU16::new((i % 7) as u16)),
    ));
    let mut columns =
        ColumnarVec::<PcoVec<Height, StoredU64>, Column>::import(&db, "columns", Version::ONE)
            .unwrap();
    for i in 0..N {
        columns.push([StoredU64::from(i as u64 * 3), StoredU64::from(i as u64 * 5)]);
    }
    columns.write().unwrap();
    let requests = [
        ("one", vec![N - 3]),
        ("clustered", (N - 64..N - 32).collect()),
        ("spread", (0..32).map(|i| i * (N - 1) / 31).collect()),
        ("dense", (N - 4096..N).collect()),
        ("duplicates", (0..4096).map(|i| N - 4096 + i / 4).collect()),
    ];
    for resident in [false, true] {
        let source = if resident {
            let c = CachedVec::wrap(source.read_only_clone());
            c.snapshot();
            c.read_only_boxed_clone()
        } else {
            source.read_only_boxed_clone()
        };
        let first = if resident {
            let c = CachedVec::wrap(first.read_only_clone());
            c.snapshot();
            c.read_only_boxed_clone()
        } else {
            first.read_only_boxed_clone()
        };
        let starts = Arc::new(
            (0..N)
                .map(|i| Height::from(i.saturating_sub(2016)))
                .collect::<Vec<_>>(),
        );
        let delta = LazyDeltaVec::<Height, StoredU64, StoredU64, DeltaSub>::new(
            "delta",
            Version::ONE,
            source.clone(),
            Version::ONE,
            move || starts.clone(),
        );
        let previous = LazyPreviousDeltaVec::new("previous", Version::ONE, source);
        let cumulative = LazyCumulativeIndexVec::new(
            "next_cumulative",
            Version::ONE,
            first.clone(),
            terminal.read_only_boxed_clone(),
        );
        let count = LazyIndexCountVec::new(
            "next_count",
            Version::ONE,
            first,
            terminal.read_only_boxed_clone(),
        );
        let cached_columns = CachedColumnarVec::new(columns.read_only_clone(), Version::ONE, |v| {
            let c = CachedVec::wrap(v);
            c.snapshot();
            c
        });
        let projection: ReadableBoxedVec<Height, StoredU64> = if resident {
            ReadableBoxedVec::new(cached_columns.column("projection", Version::ONE, Column::Second))
        } else {
            ReadableBoxedVec::new(columns.read_only_clone().column(
                "projection",
                Version::ONE,
                Column::Second,
            ))
        };
        let transformed: ReadableBoxedVec<Height, StoredU64> = if resident {
            ReadableBoxedVec::new(
                LazyColumnarVec::transformed::<Twice>("twice", Version::ONE, cached_columns)
                    .column("twice_projection", Version::ONE, Column::Second),
            )
        } else {
            ReadableBoxedVec::new(
                LazyColumnarVec::transformed::<Twice>(
                    "twice",
                    Version::ONE,
                    columns.read_only_clone(),
                )
                .column("twice_projection", Version::ONE, Column::Second),
            )
        };
        for (name, view) in [
            ("delta", delta.read_only_boxed_clone()),
            ("previous", previous.read_only_boxed_clone()),
            ("next_cumulative", cumulative.read_only_boxed_clone()),
            ("next_count", count.read_only_boxed_clone()),
            ("column", projection),
            ("column_transformed", transformed),
            (
                "block_count",
                ReadableBoxedVec::new(CachedBlockCountReader::new(
                    counts.read_only_cached_boxed_clone(),
                )),
            ),
        ] {
            for (pattern, indices) in &requests {
                let expected: Vec<_> = indices
                    .iter()
                    .map(|&i| {
                        StoredU64::from(match name {
                            "delta" => (i + 1).min(2017) as u64 * 3,
                            "previous" | "next_count" => 3,
                            "next_cumulative" => (i as u64 + 1) * 3,
                            "column" => i as u64 * 5,
                            "column_transformed" => i as u64 * 10,
                            "block_count" => {
                                let n = i as u64 + 1;
                                n / 7 * 21 + (0..n % 7).sum::<u64>()
                            }
                            _ => unreachable!(),
                        })
                    })
                    .collect();
                measure(
                    &format!("{name}/cached={resident}/{pattern}"),
                    &expected,
                    || view.read_sorted_at(black_box(indices)),
                );
            }
        }
    }
}
