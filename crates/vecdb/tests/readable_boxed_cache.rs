#![cfg(feature = "pco")]
use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, Budgeted, CacheBudget, Database, EagerVec, ImportOptions, ImportableVec, PcoVec,
    ReadOnlyClone, ReadableBoxedVec, ReadableCloneableVec, ReadableVec, Stamp, Version,
    WritableVec,
};

#[test]
fn captured_readers_share_ranges_and_source_owned_invalidation() {
    fn capture(source: &impl ReadableCloneableVec<usize, u64>) -> ReadableBoxedVec<usize, u64> {
        source.read_only_boxed_clone()
    }
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut values = PcoVec::<usize, u64, Budgeted>::import_with(
        ImportOptions::new(&db, "values", Version::ONE).with_cache_budget(&TEST_CACHE),
    )
    .unwrap();
    values.push(10);
    values.push(20);
    values.write().unwrap();
    let captured = capture(&values);
    let previous = values.collect();
    assert!(captured.read_cached_into_at(0, 2, &mut Vec::new()));
    values.truncate_if_needed_at(1).unwrap();
    assert!(captured.read_cached_into_at(0, 1, &mut Vec::new()));
    assert!(!captured.read_cached_into_at(1, 2, &mut Vec::new()));
    values.push(30);
    values.write().unwrap();
    assert_eq!(captured.collect(), [10, 30]);
    assert_eq!(previous, [10, 20], "caller-owned results do not change");
    assert!(values.read_cached_into_at(0, 2, &mut Vec::new()));
}

#[test]
fn append_and_noop_truncation_preserve_prefixes_and_revisions() {
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut values = EagerVec::<PcoVec<usize, u64, Budgeted>>::import_with(
        ImportOptions::new(&db, "prices", Version::ONE).with_cache_budget(&TEST_CACHE),
    )
    .unwrap();
    values.push(10);
    values.push(20);
    values.write().unwrap();
    let reader = values.read_only_clone();
    assert_eq!(reader.collect(), [10, 20]);
    let revision = reader.data_revision();
    for length in [2, 3] {
        values.truncate_if_needed_at(length).unwrap();
        assert!(reader.read_cached_into_at(0, 2, &mut Vec::new()));
        assert_eq!(reader.data_revision(), revision);
    }
    let stamp = Stamp::from(42_u64);
    values.truncate_if_needed_with_stamp(2, stamp).unwrap();
    assert_eq!(values.stamp(), stamp);
    values.push(40);
    values.write().unwrap();
    assert_eq!(reader.data_revision(), revision);
    assert!(reader.read_cached_into_at(0, 2, &mut Vec::new()));
    assert!(!reader.read_cached_into_at(2, 3, &mut Vec::new()));
    assert_eq!(reader.collect(), [10, 20, 40]);
    values.truncate_if_needed_at(1).unwrap();
    values.push(30);
    values.push(50);
    values.write().unwrap();
    assert_ne!(reader.data_revision(), revision);
    assert_eq!(reader.collect(), [10, 30, 50]);
}

static TEST_CACHE: CacheBudget = CacheBudget::new(64 * 1024 * 1024);
