use std::sync::{
    Arc,
    atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
};

use parking_lot::Mutex;
use vecdb::{BudgetedCachedVec, CachedVecBudget, TypedVec};

const MAX_BYTES: usize = 2 * 1024 * 1024 * 1024;

pub struct CacheBudget {
    remaining_bytes: AtomicUsize,
    clock: AtomicU64,
    caches: Mutex<Vec<CacheEntry>>,
}

impl CacheBudget {
    const fn new() -> Self {
        Self {
            remaining_bytes: AtomicUsize::new(MAX_BYTES),
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
        self.caches.lock().push(CacheEntry {
            last_access,
            resident_bytes,
            invalidate: Arc::new(cached.weak_invalidator()),
        });
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
        if bytes > MAX_BYTES {
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
        debug_assert!(previous + bytes <= MAX_BYTES);
    }
}

pub static CACHE_BUDGET: CacheBudget = CacheBudget::new();

#[cfg(test)]
mod tests {
    use super::*;
    use vecdb::{AnyStoredVec, BytesVec, Database, ImportableVec, StoredVec, Version, WritableVec};

    #[test]
    #[ignore = "synthetic registry pressure without allocating resident payloads"]
    fn benchmark_eviction_pressure() {
        for registrations in [1000, 10000] {
            for victims in [1, 64, 512] {
                let mut samples = [Vec::new(), Vec::new()];
                for round in 0..30 {
                    let variant = round % 2;
                    let budget = Arc::new(CacheBudget::new());
                    for i in 0..registrations {
                        let resident_bytes = Arc::new(AtomicUsize::new(usize::from(i < 512)));
                        let resident = resident_bytes.clone();
                        let weak = Arc::downgrade(&budget);
                        budget.caches.lock().push(CacheEntry {
                            last_access: Arc::new(AtomicU64::new(i as u64)),
                            resident_bytes,
                            invalidate: Arc::new(move || {
                                let bytes = resident.swap(0, Relaxed);
                                weak.upgrade().unwrap().release(bytes);
                                true
                            }),
                        });
                    }
                    budget.remaining_bytes.store(0, Relaxed);
                    let started = std::time::Instant::now();
                    if variant == 0 {
                        // Previous implementation, retained only as a benchmark control.
                        while !budget.remaining_bytes.try_reserve(victims) {
                            assert!(budget.evict_one());
                        }
                    } else {
                        assert!(budget.try_reserve(victims));
                    }
                    samples[variant].push(started.elapsed());
                    assert_eq!(budget.remaining_bytes.load(Relaxed), 0);
                    let entries = budget.caches.lock();
                    assert!(
                        entries[..victims]
                            .iter()
                            .all(|entry| entry.resident_bytes.load(Relaxed) == 0)
                    );
                    assert!(
                        entries[victims..512]
                            .iter()
                            .all(|entry| entry.resident_bytes.load(Relaxed) == 1)
                    );
                }
                for times in &mut samples {
                    times.sort();
                }
                eprintln!(
                    "eviction registrations={registrations} victims={victims} previous={:?} batched={:?}",
                    samples[0][7], samples[1][7]
                );
            }
        }
    }

    #[test]
    fn reservation_evicts_oldest_and_keeps_accounting_symmetric() {
        let directory = tempfile::tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let budget = Box::leak(Box::new(CacheBudget::new()));
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
        let directory = tempfile::tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let budget = Box::leak(Box::new(CacheBudget::new()));
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
        let directory = tempfile::tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let budget = Box::leak(Box::new(CacheBudget::new()));
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

        let directory = tempfile::tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let budget: &'static CacheBudget = Box::leak(Box::new(CacheBudget::new()));
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
        std::thread::scope(|scope| {
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
                        std::thread::yield_now();
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
                std::thread::yield_now();
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
