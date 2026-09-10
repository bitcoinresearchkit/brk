use std::fmt::Debug;

use bitview_vecs::{LazyDateVec, LazyFirstHeightVec};
use brk_types::{Date, Day1, Height, Timestamp};
use tempfile::tempdir;
use vecdb::{Database, READ_CHUNK_SIZE, ReadableVec, VecValue};

#[allow(dead_code)]
mod common;

fn check<T: VecValue + PartialEq + Debug>(view: &impl ReadableVec<Day1, T>, expected: &[T]) {
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
