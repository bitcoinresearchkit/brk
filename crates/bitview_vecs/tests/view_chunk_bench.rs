//! Run unchanged before/after with --ignored --nocapture --test-threads=1.
use std::{array, hint::black_box, time::Instant};

use bitview_vecs::{LazyLookbackVec, LazyPreviousDeltaVec, LazySinceDayVec, LazyWindowVec};
use brk_types::{Day1, Height, StoredU64};
use tempfile::tempdir;
use vecdb::{
    AnySerializableVec, AnyStoredVec, BytesVec, CachedColumnarVec, CachedVec, ColumnId,
    ColumnarVec, Database, Ident, ImportableVec, LazyVec, ReadableBoxedVec, ReadableCloneableVec,
    ReadableColumnarVec, ReadableVec, VecValue, Version, WritableVec,
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

#[test]
#[ignore = "synthetic warm view benchmark, excludes HTTP and indexing"]
fn benchmark_views() {
    const N: usize = 3_014_656;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut source = BytesVec::<Height, StoredU64>::import(&db, "source", Version::ONE).unwrap();
    let mut days = BytesVec::<Height, Day1>::import(&db, "days", Version::ONE).unwrap();
    let mut starts = BytesVec::<Height, Height>::import(&db, "starts", Version::ONE).unwrap();
    let mut columns =
        ColumnarVec::<BytesVec<Height, StoredU64>, Column>::import(&db, "columns", Version::ONE)
            .unwrap();
    for i in 0..N {
        let value = StoredU64::from((i as u64 + 1) * 3);
        source.push(value);
        days.push(Day1::from(i / 144));
        starts.push(Height::from(i.saturating_sub(2016)));
        columns.push([value, StoredU64::from(7u64)]);
    }
    source.write().unwrap();
    days.write().unwrap();
    starts.write().unwrap();
    columns.write().unwrap();
    let source = CachedVec::wrap(source);
    let days = CachedVec::wrap(days);
    let starts = CachedVec::wrap(starts);
    let columns = CachedColumnarVec::new(columns.read_only_clone(), Version::ONE, CachedVec::wrap);
    let factor = black_box(3u64);
    let cases: Vec<(&str, ReadableBoxedVec<Height, StoredU64>)> = vec![
        (
            "column",
            columns
                .column("column", Version::ONE, Column::First)
                .read_only_boxed_clone(),
        ),
        (
            "column_sum",
            columns
                .sum_columns("sum", Version::ONE, Column::ALL.iter().copied())
                .read_only_boxed_clone(),
        ),
        (
            "since_day",
            LazySinceDayVec::new(
                "since",
                Version::ONE,
                source.read_only_boxed_clone(),
                days.read_only_cached_boxed_clone(),
                Day1::from(100usize),
                move |a: StoredU64, b: StoredU64| {
                    StoredU64::from((u64::from(a) - u64::from(b)) * factor)
                },
            )
            .read_only_boxed_clone(),
        ),
        (
            "window",
            LazyWindowVec::new(
                "window",
                Version::ONE,
                source.read_only_boxed_clone(),
                starts.read_only_cached_boxed_clone(),
                true,
                move |a: StoredU64, b: StoredU64, n| {
                    StoredU64::from((u64::from(a) - u64::from(b)) * factor / n as u64)
                },
            )
            .read_only_boxed_clone(),
        ),
        (
            "lookback",
            LazyLookbackVec::new(
                "lookback",
                Version::ONE,
                source.read_only_boxed_clone(),
                2016,
                |a: StoredU64, b: Option<StoredU64>| {
                    StoredU64::from(u64::from(a) - b.map(u64::from).unwrap_or(0))
                },
            )
            .read_only_boxed_clone(),
        ),
        (
            "previous_delta",
            LazyPreviousDeltaVec::new("delta", Version::ONE, source.read_only_boxed_clone())
                .read_only_boxed_clone(),
        ),
    ];
    let cases: Vec<_> = cases
        .into_iter()
        .map(|(name, view)| {
            (
                name,
                LazyVec::<Height, StoredU64, Height, StoredU64>::transformed::<Ident>(
                    "nested",
                    Version::ONE,
                    view,
                ),
            )
        })
        .collect();
    let mut times = vec![vec![Vec::new(); 3]; cases.len()];
    let mut expected_json = Vec::new();
    for (name, view) in &cases {
        let expected: Vec<_> = (0..N)
            .map(|i| {
                StoredU64::from(match *name {
                    "column" => (i as u64 + 1) * 3,
                    "column_sum" => (i as u64 + 1) * 3 + 7,
                    "since_day" => {
                        if i < 14_400 {
                            0
                        } else {
                            (i as u64 + 1 - 14_400) * 9
                        }
                    }
                    "window" => 9,
                    "lookback" => (i + 1).min(2016) as u64 * 3,
                    "previous_delta" => 3,
                    _ => unreachable!(),
                })
            })
            .collect();
        assert_eq!(view.collect_range_at(0, N), expected);
        expected_json.push(serde_json::to_vec(&expected).unwrap());
    }
    for round in 0..10 {
        for offset in 0..cases.len() {
            let index = (round + offset) % cases.len();
            let (_, view) = &cases[index];
            for mode in 0..3 {
                let mode = (mode + round) % 3;
                let start = Instant::now();
                let json = match mode {
                    0 => {
                        black_box(view.collect_range_at(black_box(0), black_box(N)));
                        None
                    }
                    1 => {
                        black_box(view.fold_range_at(
                            black_box(0),
                            black_box(N),
                            0u64,
                            |sum, v| sum + u64::from(v),
                        ));
                        None
                    }
                    _ => {
                        let mut out = Vec::new();
                        view.write_json(Some(0), Some(N), &mut out).unwrap();
                        Some(out)
                    }
                };
                let elapsed = start.elapsed();
                if let Some(json) = json {
                    assert_eq!(json, expected_json[index]);
                    black_box(json);
                }
                if round > 0 {
                    times[index][mode].push(elapsed);
                }
            }
        }
    }
    for ((name, _), modes) in cases.iter().zip(&mut times) {
        for (label, samples) in ["read", "fold", "json"].into_iter().zip(modes) {
            samples.sort();
            eprintln!(
                "{name}/{label}: median={:?} min={:?} max={:?}",
                samples[4], samples[0], samples[8]
            );
        }
    }
}
