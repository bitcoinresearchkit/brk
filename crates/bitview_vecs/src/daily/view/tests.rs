use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};

use brk_types::{Day1, Height, StoredBool, StoredF64, Version};
use rangeindex::SharedRangeMap;
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, Database, EagerVec, ImportableVec, LazyVec, PcoVec, ReadBounds,
    ReadableCloneableVec, ReadableVec, WritableVec,
};

use super::{DailyView, DayStrategy, LastDay, RepeatDay};
use crate::RangeMapVec;

#[test]
fn repeat_uses_the_same_daily_value_throughout_the_day() {
    let mapping = [Day1::from(0), Day1::from(0), Day1::from(1)];

    assert_eq!(RepeatDay::source_index(&mapping, 0, 2), Some(0));
    assert_eq!(RepeatDay::source_index(&mapping, 1, 2), Some(0));
    assert_eq!(RepeatDay::source_index(&mapping, 2, 2), Some(1));
    assert_eq!(RepeatDay::source_index(&mapping, 2, 1), None);
}

#[test]
fn coarser_period_uses_its_last_available_day() {
    let mapping = [Day1::from(0), Day1::from(3), Day1::from(6)];

    assert_eq!(LastDay::source_index(&mapping, 0, 8), Some(2));
    assert_eq!(LastDay::source_index(&mapping, 1, 8), Some(5));
    assert_eq!(LastDay::source_index(&mapping, 2, 8), Some(7));
    assert_eq!(LastDay::source_index(&mapping, 1, 5), Some(4));
    assert_eq!(LastDay::source_index(&mapping, 2, 5), None);
    assert_eq!(LastDay::source_index(&mapping, 0, 0), None);
    assert_eq!(
        LastDay::source_index(&[Day1::from(0), Day1::from(0)], 0, 8),
        None
    );
}

#[test]
fn repeated_view_maps_ranges_and_preserves_missing_days() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();

    let mut source: EagerVec<PcoVec<Day1, StoredF64>> =
        EagerVec::forced_import(&db, "source", Version::ONE).unwrap();
    let mut mapping: EagerVec<PcoVec<Height, Day1>> =
        EagerVec::forced_import(&db, "mapping", Version::ONE).unwrap();
    for value in [10.0, 20.0, 30.0] {
        source.push(StoredF64::from(value));
    }
    for day in [0, 0, 1, 2, 3] {
        mapping.push(Day1::from(day));
    }
    source.write().unwrap();
    mapping.write().unwrap();

    let view =
        DailyView::<Height, StoredF64, RepeatDay>::new("test", Version::ONE, &source, &mapping);

    assert_eq!(
        view.collect_range_at(0, 5),
        vec![
            Some(StoredF64::from(10.0)),
            Some(StoredF64::from(10.0)),
            Some(StoredF64::from(20.0)),
            Some(StoredF64::from(30.0)),
            None,
        ]
    );
}

#[test]
fn repeated_view_supports_stored_booleans() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();

    let mut source: EagerVec<PcoVec<Day1, StoredBool>> =
        EagerVec::forced_import(&db, "source", Version::ONE).unwrap();
    let mut mapping: EagerVec<PcoVec<Height, Day1>> =
        EagerVec::forced_import(&db, "mapping", Version::ONE).unwrap();
    source.push(StoredBool::FALSE);
    source.push(StoredBool::TRUE);
    for day in [0, 0, 1, 1] {
        mapping.push(Day1::from(day));
    }
    source.write().unwrap();
    mapping.write().unwrap();

    let view =
        DailyView::<Height, StoredBool, RepeatDay>::new("test", Version::ONE, &source, &mapping);

    assert_eq!(
        view.collect_range_at(0, 4),
        vec![
            Some(StoredBool::FALSE),
            Some(StoredBool::FALSE),
            Some(StoredBool::TRUE),
            Some(StoredBool::TRUE),
        ]
    );
}

#[test]
fn sparse_reads_only_evaluate_requested_mapping_entries() {
    static READS: AtomicUsize = AtomicUsize::new(0);
    let source = RangeMapVec::<Day1, u64>::new(
        "values",
        Version::ONE,
        SharedRangeMap::new((0..100).collect()),
    );
    let mapping = RangeMapVec::<Height, Day1>::new(
        "days",
        Version::ONE,
        SharedRangeMap::new((0..10_001).map(|i| Day1::from(i / 100)).collect()),
    );
    let mapping = LazyVec::init(
        "days",
        Version::ZERO,
        mapping.read_only_boxed_clone(),
        |_, day| {
            READS.fetch_add(1, Relaxed);
            day
        },
    );
    let repeated =
        DailyView::<Height, u64, RepeatDay>::new("repeat", Version::ONE, &source, &mapping);
    assert_eq!(
        repeated.read_sorted_at(&[0, 0, 111, 111, 4095, 9999, 10_000, 10_001, usize::MAX]),
        [Some(0), Some(0), Some(1), Some(1), Some(40), Some(99), None],
    );
    assert_eq!(READS.swap(0, Relaxed), 5);

    let last = DailyView::<Height, u64, LastDay>::new("last", Version::ONE, &source, &mapping);
    assert_eq!(
        last.read_sorted_at(&[99, 99, 199, 200, 9999, 10_000, 10_001]),
        [Some(0), Some(0), Some(1), None, Some(99), None],
    );
    assert_eq!(READS.swap(0, Relaxed), 7);

    let mut bounds = ReadBounds::new();
    bounds.set("height", 200);
    bounds.set("day1", 2);
    bounds.scope(|| {
        assert_eq!(last.collect_one_at(200), None);
        assert_eq!(
            last.read_sorted_at(&[99, 100, 199, 200]),
            [Some(0), None, Some(1)]
        );
        assert_eq!(READS.swap(0, Relaxed), 4);
        assert_eq!(repeated.collect_range_at(198, 203), [Some(1), Some(1)]);
        assert_eq!(READS.swap(0, Relaxed), 2);
    });
}

#[test]
fn sparse_reads_match_scalar_reads_at_every_published_boundary() {
    check_sparse_reads::<RepeatDay>();
    check_sparse_reads::<LastDay>();
}

fn check_sparse_reads<S: DayStrategy>() {
    let source = RangeMapVec::<Day1, u64>::new(
        "values",
        Version::ONE,
        SharedRangeMap::new((0..10).collect()),
    );
    let days = [2usize, 2, 2, 4, 5, 5, 7, 8].map(Day1::from);
    let mapping =
        RangeMapVec::<Height, Day1>::new("days", Version::ONE, SharedRangeMap::new(days.to_vec()));
    let view = DailyView::<Height, u64, S>::new("test", Version::ONE, &source, &mapping);
    for source_len in [0, 1, 5, 10] {
        for mapping_len in 0..=days.len() {
            let mut bounds = ReadBounds::new();
            bounds.set("height", mapping_len);
            bounds.set("day1", source_len);
            bounds.scope(|| {
                let expected: Vec<_> = (0..mapping_len)
                    .map(|i| {
                        S::source_index(&days[..mapping_len], i, source_len).map(|day| day as u64)
                    })
                    .collect();
                assert_eq!(view.collect_range_at(0, 99), expected);
                for index in 0..=days.len() {
                    assert_eq!(view.collect_one_at(index), expected.get(index).copied());
                }
                for indices in [
                    vec![],
                    vec![0],
                    vec![0, 1, 2, 3],
                    vec![0, 0, 1, 1, 2, 3, 3, 6, 7, 8],
                    vec![1, 4, 6, 7, 99, usize::MAX],
                ] {
                    let mut out = vec![Some(99)];
                    view.read_sorted_into_at(&indices, &mut out);
                    let mut wanted = vec![Some(99)];
                    wanted.extend(indices.iter().filter_map(|&i| expected.get(i).copied()));
                    assert_eq!(out, wanted);
                }
            });
        }
    }
}
