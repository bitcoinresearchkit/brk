use std::{
    fmt::{Debug, Formatter, Result},
    sync::{
        Arc,
        atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
    },
};

use parking_lot::Mutex;

use crate::{BudgetedCachedVec, CachedVecBudget, TypedVec};

/// Shared memory limit and approximate LRU eviction for registered vector caches.
pub struct CacheBudget {
    max_bytes: usize,
    remaining_bytes: AtomicUsize,
    clock: AtomicU64,
    caches: Mutex<Vec<CacheEntry>>,
}

impl Debug for CacheBudget {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        f.debug_struct("CacheBudget")
            .field("max_bytes", &self.max_bytes)
            .field("remaining_bytes", &self.remaining_bytes.load(Relaxed))
            .finish_non_exhaustive()
    }
}

impl CacheBudget {
    pub const fn new(max_bytes: usize) -> Self {
        Self {
            max_bytes,
            remaining_bytes: AtomicUsize::new(max_bytes),
            clock: AtomicU64::new(0),
            caches: Mutex::new(Vec::new()),
        }
    }

    fn evict_one(&self) -> bool {
        let invalidate = self
            .caches
            .lock()
            .iter()
            .filter(|entry| entry.resident_bytes.load(Relaxed) > 0)
            .min_by_key(|entry| entry.last_access.load(Relaxed))
            .map(|entry| Arc::clone(&entry.invalidate));
        if let Some(invalidate) = invalidate {
            invalidate();
            true
        } else {
            false
        }
    }

    /// Wraps a source vec in this budget and registers it for eviction.
    pub fn wrap<V>(&'static self, source: V) -> BudgetedCachedVec<V>
    where
        V: TypedVec,
    {
        let last_access = Arc::new(AtomicU64::new(0));
        let resident_bytes = Arc::new(AtomicUsize::new(0));
        let cached = BudgetedCachedVec::wrap_budgeted(
            source,
            self,
            last_access.clone(),
            resident_bytes.clone(),
        );
        if self.max_bytes > 0 {
            self.caches.lock().push(CacheEntry {
                last_access,
                resident_bytes,
                invalidate: Arc::new(cached.weak_invalidator()),
            });
        }
        cached
    }

    /// Invalidates every registered vec.
    pub fn invalidate(&self) {
        self.caches.lock().retain(|entry| (entry.invalidate)());
    }
}

struct CacheEntry {
    last_access: Arc<AtomicU64>,
    resident_bytes: Arc<AtomicUsize>,
    invalidate: Arc<dyn Fn() -> bool + Send + Sync>,
}

impl CachedVecBudget for CacheBudget {
    #[inline]
    fn record_access(&self) -> u64 {
        self.clock.fetch_add(1, Relaxed) + 1
    }

    fn try_reserve(&self, bytes: usize) -> bool {
        if bytes > self.max_bytes {
            return false;
        }
        if self.remaining_bytes.try_reserve(bytes) {
            return true;
        }
        // Keep the common one-victim path allocation-free.
        if !self.evict_one() {
            return false;
        }
        if self.remaining_bytes.try_reserve(bytes) {
            return true;
        }
        // Snapshot eviction order once for multi-victim pressure. Invalidation
        // releases bytes outside the registry lock; concurrent touches make
        // this approximate LRU, just as they do for the single-victim path.
        let mut candidates: Vec<_> = self
            .caches
            .lock()
            .iter()
            .filter(|entry| entry.resident_bytes.load(Relaxed) > 0)
            .map(|entry| {
                (
                    entry.last_access.load(Relaxed),
                    Arc::clone(&entry.invalidate),
                )
            })
            .collect();
        candidates.sort_unstable_by_key(|(access, _)| *access);
        for (_, invalidate) in candidates {
            invalidate();
            if self.remaining_bytes.try_reserve(bytes) {
                return true;
            }
        }
        false
    }

    #[inline]
    fn release(&self, bytes: usize) {
        if bytes == 0 {
            return;
        }
        let previous = self.remaining_bytes.fetch_add(bytes, Relaxed);
        debug_assert!(previous + bytes <= self.max_bytes);
    }
}

#[cfg(test)]
mod tests {
    use std::thread;

    use tempfile::tempdir;

    use super::*;
    use crate::{AnyStoredVec, BytesVec, Database, ImportableVec, StoredVec, Version, WritableVec};

    const MAX_BYTES: usize = 1024;

    #[test]
    fn budgets_have_independent_configurable_limits() {
        let first = CacheBudget::new(17);
        let second = CacheBudget::new(31);
        let disabled = CacheBudget::new(0);
        assert!(first.try_reserve(17));
        assert!(!first.try_reserve(1));
        assert_eq!(second.remaining_bytes.load(Relaxed), 31);
        assert!(second.try_reserve(31));
        assert!(!disabled.try_reserve(1));
        first.release(17);
        second.release(31);
        assert_eq!(first.remaining_bytes.load(Relaxed), 17);
        assert_eq!(second.remaining_bytes.load(Relaxed), 31);
    }

    #[test]
    fn reservation_evicts_oldest_and_keeps_accounting_symmetric() {
        let directory = tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let budget = Box::leak(Box::new(CacheBudget::new(MAX_BYTES)));
        let mut sources = Vec::new();
        for name in ["first", "second"] {
            let mut source = BytesVec::<usize, u64>::import(&db, name, Version::ONE).unwrap();
            source.push(42);
            source.write().unwrap();
            sources.push(budget.wrap(source.read_only_clone()));
        }
        assert_eq!(&*sources[0].snapshot(), &[42]);
        assert_eq!(&*sources[1].snapshot(), &[42]);
        assert_eq!(budget.remaining_bytes.load(Relaxed), MAX_BYTES - 16);
        assert!(budget.try_reserve(MAX_BYTES - 8));
        let caches = budget.caches.lock();
        assert_eq!(caches[0].resident_bytes.load(Relaxed), 0);
        assert_eq!(caches[1].resident_bytes.load(Relaxed), 8);
        drop(caches);
        assert_eq!(budget.remaining_bytes.load(Relaxed), 0);
        budget.release(MAX_BYTES - 8);
        budget.invalidate();
        assert_eq!(budget.remaining_bytes.load(Relaxed), MAX_BYTES);
        assert!(!budget.try_reserve(MAX_BYTES + 1));
    }

    #[test]
    fn dropped_sources_release_registration_and_budget_on_invalidation() {
        let directory = tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let budget = Box::leak(Box::new(CacheBudget::new(MAX_BYTES)));
        let mut source = BytesVec::<usize, u64>::import(&db, "temporary", Version::ONE).unwrap();
        source.push(42);
        source.write().unwrap();
        let cached = budget.wrap(source.read_only_clone());
        assert_eq!(&*cached.snapshot(), &[42]);
        assert_eq!(budget.remaining_bytes.load(Relaxed), MAX_BYTES - 8);
        drop(cached);
        budget.invalidate();
        assert!(budget.caches.lock().is_empty());
        assert_eq!(budget.remaining_bytes.load(Relaxed), MAX_BYTES);
    }

    #[test]
    fn multi_victim_reservation_stops_after_enough_bytes() {
        let directory = tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let budget = Box::leak(Box::new(CacheBudget::new(MAX_BYTES)));
        let mut sources = Vec::new();
        for i in 0..8 {
            let mut source =
                BytesVec::<usize, u64>::import(&db, &format!("cache_{i}"), Version::ONE).unwrap();
            source.push(i);
            source.write().unwrap();
            let cached = budget.wrap(source.read_only_clone());
            cached.snapshot();
            sources.push(cached);
        }
        assert!(budget.try_reserve(MAX_BYTES - 16));
        let entries = budget.caches.lock();
        assert!(
            entries[..6]
                .iter()
                .all(|entry| entry.resident_bytes.load(Relaxed) == 0)
        );
        assert!(
            entries[6..]
                .iter()
                .all(|entry| entry.resident_bytes.load(Relaxed) == 8)
        );
        drop(entries);
        budget.release(MAX_BYTES - 16);
        budget.invalidate();
        assert_eq!(budget.remaining_bytes.load(Relaxed), MAX_BYTES);
    }

    #[test]
    fn concurrent_eviction_and_same_length_replacement_preserve_publications() {
        use parking_lot::RwLock;
        use std::sync::{Barrier, atomic::AtomicUsize};

        let directory = tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let budget: &'static CacheBudget = Box::leak(Box::new(CacheBudget::new(MAX_BYTES)));
        // Leave room for only two 64-byte resident snapshots, forcing eviction
        // while four readers traverse eight sources.
        assert!(budget.try_reserve(MAX_BYTES - 128));
        let mut sources = Vec::new();
        let mut readers = Vec::new();
        for i in 0..8 {
            let mut source =
                BytesVec::<usize, u64>::import(&db, &format!("stress_{i}"), Version::ONE).unwrap();
            for _ in 0..8 {
                source.push(0);
            }
            source.write().unwrap();
            readers.push(budget.wrap(source.read_only_clone()));
            sources.push(source);
        }
        let publication = RwLock::new(0u64);
        let start = Barrier::new(5);
        let reads = AtomicUsize::new(0);
        thread::scope(|scope| {
            for worker in 0..4 {
                let readers = &readers;
                let publication = &publication;
                let start = &start;
                let reads = &reads;
                scope.spawn(move || {
                    start.wait();
                    for turn in 0..128 {
                        let epoch = publication.read();
                        let snapshot = readers[(turn + worker) % readers.len()].snapshot();
                        assert_eq!(&*snapshot, &[*epoch; 8]);
                        reads.fetch_add(1, Relaxed);
                        drop(epoch);
                        thread::yield_now();
                    }
                });
            }
            start.wait();
            for epoch in 1..=32 {
                let mut published = publication.write();
                budget.invalidate();
                for source in &mut sources {
                    source.truncate_if_needed_at(0).unwrap();
                    for _ in 0..8 {
                        source.push(epoch);
                    }
                    source.write().unwrap();
                }
                *published = epoch;
                drop(published);
                thread::yield_now();
            }
        });
        assert_eq!(reads.load(Relaxed), 512);
        for reader in &readers {
            assert_eq!(&*reader.snapshot(), &[32; 8]);
        }
        budget.invalidate();
        assert_eq!(budget.remaining_bytes.load(Relaxed), 128);
        budget.release(MAX_BYTES - 128);
        assert_eq!(budget.remaining_bytes.load(Relaxed), MAX_BYTES);
    }
}
