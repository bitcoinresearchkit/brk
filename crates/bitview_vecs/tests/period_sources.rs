use std::fmt::Debug;

use bitview_vecs::{LazyDateVec, LazyFirstHeightVec};
#[cfg(feature = "diagnostics")]
use brk_types::Version;
use brk_types::{Date, Day1, Height, Timestamp};
use tempfile::tempdir;
use vecdb::{Database, READ_CHUNK_SIZE, ReadableVec, VecValue};

#[cfg(feature = "diagnostics")]
use vecdb::{
    AnyStoredVec, Budgeted, CacheBudget, ImportOptions, ImportableVec, PcoVec, WritableVec,
    diagnostics,
};

#[allow(dead_code)]
mod common;

fn check<T: VecValue + PartialEq + Debug>(view: &impl ReadableVec<Day1, T>, expected: &[T]) {
    let indices = [0, 1, 1, expected.len() - 1, expected.len(), usize::MAX];
    let mut selected = vec![expected[0].clone()];
    view.read_sorted_into_at(&indices, &mut selected);
    assert_eq!(
        selected,
        [
            expected[0].clone(),
            expected[0].clone(),
            expected[1].clone(),
            expected[1].clone(),
            expected.last().unwrap().clone()
        ]
    );
    for (from, to) in [
        (0, expected.len()),
        (1, 4),
        (expected.len() / 2, expected.len() + 10),
        (expected.len(), expected.len() + 10),
        (4, 1),
    ] {
        let expected = if from < to.min(expected.len()) {
            &expected[from..to.min(expected.len())]
        } else {
            &[]
        };
        assert_eq!(view.collect_range_at(from, to), expected);
        let mut chunks = Vec::new();
        view.for_each_chunk_at(from, to, &mut |at, values| {
            assert_eq!(at, from + chunks.len());
            chunks.extend_from_slice(values);
        });
        assert_eq!(chunks, expected);
        assert_eq!(
            view.fold_range_at(from, to, Vec::new(), |mut values, value| {
                values.push(value);
                values
            }),
            expected
        );
    }
    let mut calls = 0;
    assert_eq!(
        view.try_fold_range_at(1, expected.len(), (), |(), _| {
            calls += 1;
            if calls == 3 { Err("stop") } else { Ok(()) }
        }),
        Err("stop")
    );
    assert_eq!(calls, 3);
}

#[cfg(feature = "diagnostics")]
#[test]
fn latest_period_lookups_seek_without_scanning_source_history() {
    const N: usize = 65_536;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mapping =
        common::stored::<Height, _>(&db, "periods", (0..N).map(|height| Day1::from(height / 16)));
    let timestamps = common::stored::<Height, _>(
        &db,
        "timestamps",
        (0..N).map(|height| Timestamp::from(Date::from(Day1::from(height / 16)))),
    );
    let first = LazyFirstHeightVec::new(&mapping);
    let dates = LazyDateVec::new(
        &timestamps,
        |timestamp| Day1::try_from(Date::from(timestamp)).unwrap(),
        |_, timestamp| Date::from(timestamp),
    );
    common::CACHE_BUDGET.clear();
    diagnostics::take();
    assert_eq!(first.collect_one_at(N / 16 - 1), Some(Height::from(N - 16)));
    let decoded = diagnostics::take();
    assert!(decoded <= 18, "first-height lookup decoded {decoded} pages");
    assert_eq!(
        dates.collect_one_at(N / 16 - 1),
        Some(Date::from(Day1::from(N / 16 - 1)))
    );
    let decoded = diagnostics::take();
    assert!(decoded <= 18, "date lookup decoded {decoded} pages");
}

#[cfg(feature = "diagnostics")]
fn check_sorted_period_work<T: VecValue + PartialEq + Debug>(
    view: &impl ReadableVec<Day1, T>,
    cache: &CacheBudget,
    pages: usize,
    expected: impl Fn(usize) -> T,
) {
    let len = view.len();
    for indices in [
        (0..len)
            .flat_map(|index| [index, index])
            .collect::<Vec<_>>(),
        (0..len).step_by(7).collect(),
        vec![0, 0, len / 2, len / 2, len - 1],
    ] {
        cache.clear();
        diagnostics::take();
        let mut actual = vec![expected(0)];
        view.read_sorted_into_at(&indices, &mut actual);
        let decoded = diagnostics::take();
        assert_eq!(
            actual,
            [
                vec![expected(0)],
                indices.iter().copied().map(&expected).collect()
            ]
            .concat()
        );
        let limit = if indices.len() > 5 { pages + 1 } else { 40 };
        assert!(
            decoded <= limit,
            "{} sorted values decoded {decoded} pages (limit {limit})",
            indices.len()
        );
        diagnostics::take();
        assert_eq!(view.read_sorted_at(&indices), actual[1..]);
        assert_eq!(diagnostics::take(), 0, "warm sorted read decoded pages");
    }
}

#[cfg(feature = "diagnostics")]
#[test]
fn sorted_period_reads_batch_dense_requests_duplicates_and_large_gaps() {
    const N: usize = 65_536;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let cache = Box::leak(Box::new(CacheBudget::new(2 * 1024 * 1024)));
    let mut mapping = PcoVec::<Height, Day1, Budgeted>::import_with(
        ImportOptions::new(&db, "periods", Version::ONE).with_cache_budget(cache),
    )
    .unwrap();
    let mut timestamps = PcoVec::<Height, Timestamp, Budgeted>::import_with(
        ImportOptions::new(&db, "timestamps", Version::ONE).with_cache_budget(cache),
    )
    .unwrap();
    for height in 0..N {
        let day = Day1::from(height / 16);
        mapping.push(day);
        timestamps.push(Timestamp::from(Date::from(day)));
    }
    mapping.write().unwrap();
    timestamps.write().unwrap();
    check_sorted_period_work(
        &LazyFirstHeightVec::new(&mapping),
        cache,
        N.div_ceil(mapping.cursor_chunk_size()),
        |index| Height::from(index * 16),
    );
    check_sorted_period_work(
        &LazyDateVec::new(
            &timestamps,
            |timestamp| Day1::try_from(Date::from(timestamp)).unwrap(),
            |_, timestamp| Date::from(timestamp),
        ),
        cache,
        N.div_ceil(timestamps.cursor_chunk_size()),
        |index| Date::from(Day1::from(index)),
    );
}

#[test]
fn period_readers_preserve_gaps_ranges_and_early_stop_across_source_chunks() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let periods: Vec<_> = (0..READ_CHUNK_SIZE + 9)
        .map(|height| Day1::from(height / 3 * 2 + 2))
        .collect();
    let mapping = common::stored::<Height, _>(&db, "periods", periods.iter().copied());
    let timestamps = common::stored::<Height, _>(
        &db,
        "timestamps",
        periods.iter().map(|&day| Timestamp::from(Date::from(day))),
    );
    let len = usize::from(*periods.last().unwrap()) + 1;
    let first: Vec<_> = (0..len)
        .map(|period| periods.partition_point(|day| usize::from(*day) < period))
        .collect();
    let dates = LazyDateVec::new(
        &timestamps,
        |timestamp| Day1::try_from(Date::from(timestamp)).unwrap(),
        |day, _| Date::from(day),
    );
    let first_dates = LazyDateVec::new(
        &timestamps,
        |timestamp| Day1::try_from(Date::from(timestamp)).unwrap(),
        |_, timestamp| Date::from(timestamp),
    );
    check(
        &LazyFirstHeightVec::new(&mapping),
        &first.iter().copied().map(Height::from).collect::<Vec<_>>(),
    );
    check(
        &dates,
        &(0..len)
            .map(|day| Date::from(Day1::from(day)))
            .collect::<Vec<_>>(),
    );
    check(
        &first_dates,
        &first
            .iter()
            .map(|&height| Date::from(periods[height]))
            .collect::<Vec<_>>(),
    );
}
