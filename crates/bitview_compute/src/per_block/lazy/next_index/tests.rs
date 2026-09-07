use brk_types::{Height, StoredU64, Version};
use vecdb::{
    AnyStoredVec, Database, EagerVec, ImportableVec, PcoVec, ReadBounds, ReadableCloneableVec,
    ReadableVec, WritableVec,
};

use super::{LazyCumulativeIndexVec, LazyIndexCountVec};

#[test]
fn next_boundaries_produce_cumulative_and_per_item_counts() {
    let directory = tempfile::tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut first: EagerVec<PcoVec<Height, Height>> =
        EagerVec::forced_import(&db, "first", Version::ONE).unwrap();
    let mut terminal: EagerVec<PcoVec<Height, StoredU64>> =
        EagerVec::forced_import(&db, "terminal", Version::ONE).unwrap();

    for value in [0, 2, 5] {
        first.push(Height::new(value));
    }
    for value in 0_u64..6 {
        terminal.push(StoredU64::from(value));
    }
    first.write().unwrap();
    terminal.write().unwrap();

    let cumulative = LazyCumulativeIndexVec::new(
        "cumulative",
        Version::ONE,
        first.read_only_boxed_clone(),
        terminal.read_only_boxed_clone(),
    );
    let count = LazyIndexCountVec::new(
        "count",
        Version::ONE,
        first.read_only_boxed_clone(),
        terminal.read_only_boxed_clone(),
    );

    assert_eq!(
        cumulative.collect_range(Height::ZERO, Height::new(3)),
        [2_u64, 5, 6].map(StoredU64::from)
    );
    assert_eq!(
        count.collect_range(Height::ZERO, Height::new(3)),
        [2_u64, 3, 1].map(StoredU64::from)
    );
    assert_eq!(
        count.collect_range(Height::new(1), Height::new(3)),
        [3_u64, 1].map(StoredU64::from)
    );
    assert_eq!(
        cumulative.collect_one(Height::new(2)),
        Some(StoredU64::new(6))
    );
    assert_eq!(count.collect_one(Height::new(2)), Some(StoredU64::new(1)));
    assert_eq!(
        cumulative.read_sorted_at(&[0, 2, 2, 3]),
        [2_u64, 6, 6].map(StoredU64::from)
    );
    assert_eq!(
        count.read_sorted_at(&[0, 2, 2, 3]),
        [2_u64, 1, 1].map(StoredU64::from)
    );

    let mut bounds = ReadBounds::new();
    bounds.set("height", 5);
    bounds.scope(|| {
        assert_eq!(
            cumulative.collect_one(Height::new(2)),
            Some(StoredU64::new(5))
        );
        assert_eq!(count.collect_one(Height::new(2)), Some(StoredU64::new(0)));
        assert_eq!(
            count.read_sorted_at(&[0, 2, usize::MAX]),
            [2_u64, 0].map(StoredU64::from)
        );
        assert_eq!(
            cumulative.read_sorted_at(&[0, 2, usize::MAX]),
            [2_u64, 5].map(StoredU64::from)
        );
    });
    first.truncate_if_needed_at(1).unwrap();
    first.push(Height::new(1));
    first.push(Height::new(4));
    first.write().unwrap();
    terminal.truncate_if_needed_at(4).unwrap();
    terminal.write().unwrap();
    assert_eq!(
        count.read_sorted_at(&[0, 2, usize::MAX]),
        [1_u64, 0].map(StoredU64::from)
    );
    assert_eq!(
        cumulative.read_sorted_at(&[0, 2, usize::MAX]),
        [1_u64, 4].map(StoredU64::from)
    );
    assert!(count.read_sorted_at(&[]).is_empty());
    assert!(cumulative.read_sorted_at(&[usize::MAX]).is_empty());
}
