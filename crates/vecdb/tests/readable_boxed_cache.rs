#[cfg(feature = "pco")]
use std::sync::Arc;

use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, BytesVec, CachedVec, Database, EagerVec, ImportableVec, LazyVec,
    ReadableBoxedVec, ReadableVec, StoredVec, Version, WritableVec,
};
#[cfg(feature = "pco")]
use vecdb::{PcoVec, ReadOnlyClone, Stamp};

#[test]
fn boxed_vec_detects_only_actual_cache_layers() {
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut values: EagerVec<BytesVec<usize, u64>> =
        EagerVec::import(&db, "values", Version::ONE).unwrap();

    for value in 0..8 {
        values.push(value);
    }
    values.write().unwrap();

    let read_only = StoredVec::read_only_clone(&values);
    let uncached = ReadableBoxedVec::new(read_only.clone());
    assert!(!uncached.has_cache_layer());

    let cached = ReadableBoxedVec::new(CachedVec::wrap(read_only));
    assert!(cached.has_cache_layer());
    assert!(cached.clone().has_cache_layer());
    assert_eq!(cached.read_sorted_at(&[0, 2, 7]), [0, 2, 7]);

    let lazy =
        LazyVec::<usize, u64, usize, u64>::init("lazy_values", Version::ONE, cached, |_, value| {
            value
        });
    assert!(!lazy.has_cache_layer());
}

#[test]
#[cfg(feature = "pco")]
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
