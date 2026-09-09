use std::sync::{
    Arc,
    atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
};

use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, BudgetedCachedVec, BytesVec, CachedReadableVec, Database, ImportableVec, Pinned,
    PinnedCachedVec, ReadOnlyClone, ReadableVec, Version, WritableVec,
};

#[test]
fn aliases_preserve_policy_and_share_snapshots_through_clones() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut source = BytesVec::<usize, u64>::forced_import(&db, "budgeted", Version::ONE).unwrap();
    source.push(10);
    source.push(20);
    source.write().unwrap();
    let budget = Box::leak(Box::new(AtomicUsize::new(64)));
    let resident = Arc::new(AtomicUsize::new(0));
    let cached: BudgetedCachedVec<_> = BudgetedCachedVec::wrap_budgeted(
        source,
        budget,
        Arc::new(AtomicU64::new(0)),
        resident.clone(),
    );
    let read_only: BudgetedCachedVec<_> = cached.read_only_clone();
    // Weak cache + resident-byte counter + shared budget trait object.
    // Eviction does not need to retain the recency counter as well.
    assert_eq!(
        size_of_val(&cached.weak_invalidator()),
        4 * size_of::<usize>()
    );
    let cloned: BudgetedCachedVec<_> = read_only.clone();
    let boxed = cloned.cached_boxed_clone();
    let snapshot = cached.snapshot();
    assert!(Arc::ptr_eq(&snapshot, &read_only.snapshot()));
    assert!(Arc::ptr_eq(&snapshot, &boxed.snapshot()));
    assert_eq!(resident.load(Relaxed), 16);
    assert_eq!(budget.load(Relaxed), 48);
    cloned.invalidate();
    assert!(read_only.cached_snapshot().is_none());
    assert_eq!(resident.load(Relaxed), 0);
    assert_eq!(budget.load(Relaxed), 64);
    assert_eq!(cached.collect(), [10, 20]);
    cached.invalidate();

    let source = BytesVec::<usize, u64>::forced_import(&db, "pinned", Version::ONE).unwrap();
    let pinned: PinnedCachedVec<_> = PinnedCachedVec::wrap(source);
    let read_only: PinnedCachedVec<_> = pinned.read_only_clone();
    assert!(Arc::ptr_eq(&pinned.snapshot(), &read_only.snapshot()));
    assert_eq!(size_of::<Pinned>(), 0);
    assert!(size_of_val(&read_only) < size_of_val(&cloned));
}
