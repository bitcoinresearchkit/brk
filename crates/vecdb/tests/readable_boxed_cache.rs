#![cfg(feature = "pco")]

use std::sync::Arc;

use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, CachedVec, Database, EagerVec, ImportableVec, ReadableBoxedVec,
    ReadableCloneableVec, ReadableVec, Version, WritableVec,
};
use vecdb::{PcoVec, ReadOnlyClone, Stamp};

#[test]
fn cached_stored_reader_capture_shares_snapshots_and_invalidation() {
    fn capture(source: &impl ReadableCloneableVec<usize, u64>) -> ReadableBoxedVec<usize, u64> {
        source.read_only_boxed_clone()
    }

    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut values =
        CachedVec::wrap(PcoVec::<usize, u64>::import(&db, "values", Version::ONE).unwrap());
    values.push(10);
    values.push(20);
    values.write().unwrap();

    let captured = capture(&values);
    let snapshot = values.snapshot();
    assert!(Arc::ptr_eq(&snapshot, &captured.snapshot()));

    values.truncate_if_needed_at(1).unwrap();
    values.push(30);
    values.write().unwrap();
    assert_eq!(captured.collect(), [10, 30]);
    assert_eq!(snapshot.as_slice(), [10, 20]);
    assert!(Arc::ptr_eq(&values.snapshot(), &captured.snapshot()));
}

#[test]
fn truncation_invalidates_same_length_replacement_for_read_only_consumers() {
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut values = CachedVec::wrap(
        EagerVec::<PcoVec<usize, u64>>::import(&db, "prices", Version::ONE).unwrap(),
    );
    values.inner.push(10);
    values.inner.push(20);
    values.inner.write().unwrap();
    let read_only = values.read_only_clone();
    assert!(read_only.cached_snapshot().is_none());
    assert!(read_only.cached_snapshot().is_none());
    let previous = read_only.snapshot();
    assert!(Arc::ptr_eq(
        &previous,
        &read_only.cached_snapshot().unwrap()
    ));
    assert_eq!(previous.as_slice(), [10, 20]);
    for length in [2, 3] {
        values.truncate_if_needed_at(length).unwrap();
        assert!(Arc::ptr_eq(&previous, &read_only.snapshot()));
    }
    let stamp = Stamp::from(42u64);
    values.truncate_if_needed_with_stamp(2, stamp).unwrap();
    assert_eq!(values.inner.stamp(), stamp);
    assert!(Arc::ptr_eq(&previous, &read_only.snapshot()));
    values.truncate_if_needed_at(1).unwrap();
    assert!(read_only.cached_snapshot().is_none());
    values.inner.push(30);
    values.inner.write().unwrap();
    assert_eq!(read_only.inner.collect_range_at(0, 2), [10, 30]);
    assert_eq!(read_only.collect_range_at(0, 2), [10, 30]);
    assert_eq!(previous.as_slice(), [10, 20]);
    values.inner.push(40);
    values.inner.write().unwrap();
    assert!(read_only.cached_snapshot().is_none());
}
