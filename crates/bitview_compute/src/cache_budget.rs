use std::sync::{
    Arc,
    atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
};

use parking_lot::Mutex;
use vecdb::{
    CachedVec, CachedVecBudget, ReadableBoxedVec, ReadableVec, TypedVec, VecIndex, VecValue,
};

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
    pub fn wrap<V>(&'static self, source: V) -> CachedVec<V>
    where
        V: TypedVec + ReadableVec<V::I, V::T> + Clone + 'static,
    {
        let last_access = Arc::new(AtomicU64::new(0));
        let resident_bytes = Arc::new(AtomicUsize::new(0));
        let cached =
            CachedVec::wrap_budgeted(source, self, last_access.clone(), resident_bytes.clone());
        self.caches.lock().push(CacheEntry {
            last_access,
            resident_bytes,
            invalidate: Arc::new(cached.weak_invalidator()),
        });
        cached
    }

    /// Adds this budget's cache unless the type-erased source is already cached.
    pub fn wrap_boxed<I, T>(&'static self, source: ReadableBoxedVec<I, T>) -> ReadableBoxedVec<I, T>
    where
        I: VecIndex,
        T: VecValue,
    {
        if source.has_cache_layer() {
            source
        } else {
            ReadableBoxedVec::new(self.wrap(source))
        }
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
        loop {
            if self.remaining_bytes.try_reserve(bytes) {
                return true;
            }
            if !self.evict_one() {
                return false;
            }
        }
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
}
