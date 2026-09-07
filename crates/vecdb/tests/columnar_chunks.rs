use std::{
    array,
    sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering},
    },
};

use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, BytesVec, CachedColumnarVec, CachedVec, ColumnId, ColumnarVec, Database, Ident,
    ImportableVec, LazyColumnarVec, LazyVec, ReadableCloneableVec, ReadableColumnarVec,
    ReadableVec, UnaryTransform, VecValue, Version, WritableVec,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Column {
    First,
    Second,
}

struct Twice;
impl UnaryTransform<u64> for Twice {
    fn apply(value: u64) -> u64 {
        value * 2
    }
}

#[test]
fn lazy_columnar_rows_and_columns_match_across_boundaries() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut source =
        ColumnarVec::<BytesVec<usize, u64>, Column>::import(&db, "lazy_columns", Version::ONE)
            .unwrap();
    for i in 0..40_000u64 {
        source.push([i, i + 7]);
    }
    source.write().unwrap();
    let transformed =
        LazyColumnarVec::transformed::<Twice>("twice", Version::ONE, source.read_only_clone());
    let clone = transformed.clone();
    let sorted_sum = clone.sum_columns("sorted_sum", Version::ONE, [Column::Second, Column::First]);
    for indices in [
        vec![],
        vec![usize::MAX],
        vec![0, 0, 1023, 1024, 32_000, 39_999, 40_000, usize::MAX],
        (100..110).flat_map(|i| [i, i, i, i]).collect(),
    ] {
        let expected: Vec<_> = indices
            .iter()
            .filter(|&&i| i < 40_000)
            .map(|&i| (i as u64 * 2 + 7) * 2)
            .collect();
        let mut actual = vec![99];
        sorted_sum.read_sorted_into_at(&indices, &mut actual);
        assert_eq!(&actual[1..], expected);
    }
    for &column in Column::ALL {
        let projection = clone.column("sorted_projection", Version::ONE, column);
        for indices in [
            vec![],
            vec![usize::MAX],
            vec![0, 0, 1023, 1024, 32_000, 39_999, 40_000, usize::MAX],
            (100..110).flat_map(|i| [i, i, i, i]).collect(),
        ] {
            let expected: Vec<_> = indices
                .iter()
                .filter(|&&i| i < 40_000)
                .map(|&i| (i as u64 + if column == Column::Second { 7 } else { 0 }) * 2)
                .collect();
            let mut actual = vec![99];
            projection.read_sorted_into_at(&indices, &mut actual);
            assert_eq!(&actual[1..], expected);
        }
    }
    for (from, to) in [
        (0usize, 45_000usize),
        (16_380, 33_000),
        (39_990, 50_000),
        (7, 2),
        (usize::MAX, usize::MAX),
    ] {
        let end = to.min(40_000).max(from.min(40_000));
        let expected: Vec<_> = (from.min(40_000)..end)
            .map(|i| [i as u64 * 2, (i as u64 + 7) * 2])
            .collect();
        let mut actual = Vec::new();
        clone.for_each_chunk_at(from, to, &mut |at, rows| {
            assert_eq!(at, from + actual.len());
            actual.extend_from_slice(rows);
        });
        assert_eq!(actual, expected);
        assert_eq!(clone.collect_range_at(from, to), expected);
        assert_eq!(
            clone.fold_range_at(from, to, Vec::new(), |mut out, row| {
                out.push(row);
                out
            }),
            expected
        );
        for column in Column::ALL {
            let projection = clone.column("projection", Version::ONE, *column);
            assert_eq!(
                projection.collect_range_at(from, to),
                expected
                    .iter()
                    .map(|row| row[column.index()])
                    .collect::<Vec<_>>()
            );
        }
    }
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
fn column_chunks_borrow_cached_allocations_and_preserve_denial_and_rewrites() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut source =
        ColumnarVec::<BytesVec<usize, u64>, Column>::import(&db, "columns", Version::ONE).unwrap();
    for i in 0..40_000u64 {
        source.push([i, 7]);
    }
    source.write().unwrap();
    let budget = Box::leak(Box::new(AtomicUsize::new(40_000 * 16)));
    let columns = CachedColumnarVec::new(source.read_only_clone(), Version::ONE, |column| {
        CachedVec::wrap_budgeted(
            column,
            budget,
            Arc::new(AtomicU64::new(0)),
            Arc::new(AtomicUsize::new(0)),
        )
    });
    let snapshot = columns.cached_column(Column::First).snapshot();
    let projection = columns.column("first", Version::ONE, Column::First);
    assert_eq!(
        projection.read_sorted_at(&[0, 1024, 39_999, 40_000, usize::MAX]),
        [0, 1024, 39_999]
    );
    let nested = LazyVec::<usize, u64, usize, u64>::transformed::<Ident>(
        "nested",
        Version::ONE,
        projection.read_only_boxed_clone(),
    );
    let mut calls = 0;
    nested.for_each_chunk_at(17, 39_999, &mut |at, values| {
        calls += 1;
        assert_eq!(at, 17);
        assert_eq!(values.as_ptr(), snapshot[17..].as_ptr());
        assert_eq!(values.len(), 39_999 - 17);
    });
    assert_eq!(calls, 1);
    let sum = columns.sum_columns("sum", Version::ONE, Column::ALL.iter().copied());
    assert_eq!(
        sum.read_sorted_at(&[0, 1024, 39_999, 40_000, usize::MAX]),
        [7, 1031, 40_006]
    );
    assert_eq!(
        sum.collect_range_at(0, 40_000),
        (0..40_000u64).map(|i| i + 7).collect::<Vec<_>>()
    );
    columns.invalidate();
    budget.store(0, Ordering::Relaxed);
    assert_eq!(
        sum.read_sorted_at(&[0, 1024, 39_999, 40_000, usize::MAX]),
        [7, 1031, 40_006]
    );
    assert_eq!(
        projection.read_sorted_at(&[0, 1024, 39_999, 40_000, usize::MAX]),
        [0, 1024, 39_999]
    );
    let mut actual = Vec::new();
    nested.for_each_chunk_at(17, 40_010, &mut |at, values| {
        assert_eq!(at, 17 + actual.len());
        actual.extend_from_slice(values);
    });
    assert_eq!(actual, (17..40_000u64).collect::<Vec<_>>());
    assert!(
        columns
            .cached_column(Column::First)
            .cached_snapshot()
            .is_none()
    );
    assert_eq!(
        sum.collect_range_at(1023, 35_000),
        (1023..35_000u64).map(|i| i + 7).collect::<Vec<_>>()
    );
    source.truncate_if_needed_at(39_999).unwrap();
    source.push([123, 11]);
    source.write().unwrap();
    assert_eq!(nested.collect_range_at(39_999, 50_000), [123]);
    assert_eq!(
        projection.read_sorted_at(&[0, 1024, 39_999, 39_999, 40_000]),
        [0, 1024, 123, 123]
    );
    assert_eq!(sum.collect_range_at(39_999, 50_000), [134]);
    assert_eq!(
        sum.read_sorted_at(&[0, 1024, 39_999, 39_999, 40_000]),
        [7, 1031, 134, 134]
    );
    assert_eq!(snapshot[39_999], 39_999);
    for (from, to) in [(9, 2), (40_000, usize::MAX), (usize::MAX, usize::MAX)] {
        nested.for_each_chunk_at(from, to, &mut |_, _| panic!("empty range"));
    }
}
