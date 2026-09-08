#[allow(dead_code)]
mod common;

use std::{array, fmt::Debug, hint::black_box, time::Instant};

use bitview_vecs::LazyWindowStartVec;
use brk_types::{Height, Sats, StoredU64, Timestamp};
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, CachedColumnarVec, CachedVec, ColumnId, ColumnarVec, Database, ImportableVec,
    LazyColumnarVec, OverflowVec, PcoVec, ReadableBoxedVec, ReadableColumnarVec, ReadableVec,
    UnaryTransform, VecValue, Version, WritableVec,
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

fn measure<T: Debug + PartialEq>(name: &str, expected: &[T], mut read: impl FnMut() -> Vec<T>) {
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

fn window_search(timestamps: &[Timestamp], indices: &[usize], mode: &str) -> Vec<Height> {
    let expired = |current: usize, older: usize| {
        u64::from(timestamps[current]).saturating_sub(u64::from(timestamps[older])) >= 14 * 86400
    };
    let first = indices[0];
    let mut start = timestamps[..=first].partition_point(|&older| {
        u64::from(timestamps[first]).saturating_sub(u64::from(older)) >= 14 * 86400
    });
    let mut out = Vec::with_capacity(indices.len());
    for &current in indices {
        if mode == "linear" {
            while start < current && expired(current, start) {
                start += 1;
            }
        } else if start < current && expired(current, start) {
            let mut end = current;
            if mode == "galloping" {
                let mut step = 1;
                while step < current - start && expired(current, start + step) {
                    step *= 2;
                }
                end = (start + step + 1).min(current);
            }
            start += timestamps[start..end].partition_point(|&older| {
                u64::from(timestamps[current]).saturating_sub(u64::from(older)) >= 14 * 86400
            });
        }
        out.push(Height::from(start));
    }
    out
}

fn cursor_read<T: VecValue>(view: &impl ReadableVec<Height, T>, indices: &[usize]) -> Vec<T> {
    let mut cursor = vecdb::Cursor::new(view);
    let mut out = Vec::with_capacity(indices.len());
    out.extend(indices.iter().filter_map(|&i| cursor.get(i)));
    out
}

#[test]
#[ignore = "Window search, column sum and overflow sorted reads; validates all outputs"]
fn benchmark_lookup_tail() {
    const N: usize = 262_144;
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let requests = [
        ("one", vec![N - 3]),
        ("clustered", (N - 64..N - 32).collect()),
        ("spread", (0..32).map(|i| i * (N - 1) / 31).collect()),
        ("dense", (N - 4096..N).collect()),
        ("duplicates", (0..4096).map(|i| N - 4096 + i / 4).collect()),
    ];
    let timestamps: Vec<_> = (0..N)
        .map(|i| Timestamp::from((i / 2 * 600) as u32))
        .collect();
    let cached = CachedVec::wrap(common::stored::<Height, _>(
        &db,
        "timestamps",
        timestamps.iter().copied(),
    ));
    cached.snapshot();
    let window = LazyWindowStartVec::days(
        "window",
        Version::ONE,
        14,
        cached.read_only_cached_boxed_clone(),
    );
    for (pattern, indices) in &requests {
        let expected: Vec<_> = indices
            .iter()
            .map(|&i| {
                Height::from(timestamps[..=i].partition_point(|&older| {
                    u64::from(timestamps[i]).saturating_sub(u64::from(older)) >= 14 * 86400
                }))
            })
            .collect();
        measure(&format!("window/production/{pattern}"), &expected, || {
            window.read_sorted_at(black_box(indices))
        });
        for mode in ["linear", "binary", "galloping"] {
            measure(&format!("window/{mode}/{pattern}"), &expected, || {
                window_search(black_box(&timestamps), black_box(indices), mode)
            });
        }
    }
    let mut columns =
        ColumnarVec::<PcoVec<Height, StoredU64>, Column>::import(&db, "columns", Version::ONE)
            .unwrap();
    for i in 0..N {
        columns.push([StoredU64::from(i as u64 * 3), StoredU64::from(i as u64 * 5)]);
    }
    columns.write().unwrap();
    let cached_columns = CachedColumnarVec::new(columns.read_only_clone(), Version::ONE, |v| {
        let c = CachedVec::wrap(v);
        c.snapshot();
        c
    });
    for (name, view) in [
        (
            "pco",
            ReadableBoxedVec::new(columns.read_only_clone().sum_columns(
                "sum",
                Version::ONE,
                Column::ALL.iter().copied(),
            )),
        ),
        (
            "cached",
            ReadableBoxedVec::new(cached_columns.sum_columns(
                "sum",
                Version::ONE,
                Column::ALL.iter().copied(),
            )),
        ),
        (
            "transformed",
            ReadableBoxedVec::new(
                LazyColumnarVec::transformed::<Twice>("twice", Version::ONE, cached_columns)
                    .sum_columns("sum", Version::ONE, Column::ALL.iter().copied()),
            ),
        ),
    ] {
        for (pattern, indices) in &requests {
            let expected: Vec<_> = indices
                .iter()
                .map(|&i| StoredU64::from(i as u64 * if name == "transformed" { 16 } else { 8 }))
                .collect();
            measure(&format!("sum/{name}/{pattern}"), &expected, || {
                view.read_sorted_at(black_box(indices))
            });
            measure(&format!("sum_cursor/{name}/{pattern}"), &expected, || {
                cursor_read(&view, black_box(indices))
            });
        }
    }
    for (name, every) in [("inline", None), ("mixed", Some(64)), ("overflow", Some(1))] {
        let mut source = OverflowVec::<Height, Sats>::import(&db, name, Version::ONE).unwrap();
        let values: Vec<_> = (0..N)
            .map(|i| {
                Sats::from(
                    i as u64
                        + if every.is_some_and(|n| i % n == 0) {
                            1u64 << 40
                        } else {
                            0
                        },
                )
            })
            .collect();
        for &value in &values {
            source.push(value);
        }
        source.write().unwrap();
        let reader = source.read_only_clone();
        for (pattern, indices) in &requests {
            let expected: Vec<_> = indices.iter().map(|&i| values[i]).collect();
            measure(
                &format!("overflow_writer/{name}/{pattern}"),
                &expected,
                || source.read_sorted_at(black_box(indices)),
            );
            measure(
                &format!("overflow_reader/{name}/{pattern}"),
                &expected,
                || reader.read_sorted_at(black_box(indices)),
            );
            measure(
                &format!("overflow_writer_cursor/{name}/{pattern}"),
                &expected,
                || cursor_read(&source, black_box(indices)),
            );
            measure(
                &format!("overflow_reader_cursor/{name}/{pattern}"),
                &expected,
                || cursor_read(&reader, black_box(indices)),
            );
        }
    }
}
