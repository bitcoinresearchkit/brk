use std::sync::{
    Arc,
    atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
};

use parking_lot::Mutex;
use vecdb::{CachedVec, CachedVecBudget, ReadableBoxedVec, VecIndex, VecValue};

const MAX_CACHED: usize = 256;
const MIN_ACCESSES: u64 = 2;

struct LruBudget {
    remaining: AtomicUsize,
}

impl LruBudget {
    fn try_decrement(&self) -> bool {
        self.remaining
            .fetch_update(Relaxed, Relaxed, |n| if n > 0 { Some(n - 1) } else { None })
            .is_ok()
    }
}

impl CachedVecBudget for LruBudget {
    fn try_reserve(&self, access_count: u64) -> bool {
        if access_count < MIN_ACCESSES {
            return false;
        }
        if self.try_decrement() {
            return true;
        }
        // Only evict if we're more popular than the least popular cached entry.
        if evict_less_popular_than(access_count) {
            self.try_decrement()
        } else {
            false
        }
    }
}

struct CacheEntry {
    access_count: Arc<AtomicU64>,
    clear: Box<dyn Fn() + Send + Sync>,
}

static BUDGET: LruBudget = LruBudget {
    remaining: AtomicUsize::new(MAX_CACHED),
};
static CACHES: Mutex<Vec<CacheEntry>> = Mutex::new(Vec::new());

fn evict_less_popular_than(threshold: u64) -> bool {
    let caches = CACHES.lock();
    if let Some((idx, _)) = caches
        .iter()
        .enumerate()
        .filter(|(_, e)| {
            let c = e.access_count.load(Relaxed);
            c >= MIN_ACCESSES && c < threshold
        })
        .min_by_key(|(_, e)| e.access_count.load(Relaxed))
    {
        (caches[idx].clear)();
        BUDGET.remaining.fetch_add(1, Relaxed);
        true
    } else {
        false
    }
}

/// Wraps a boxed source in a budgeted [`CachedVec`] and registers it for eviction.
pub fn cache_wrap<I: VecIndex, T: VecValue>(source: ReadableBoxedVec<I, T>) -> CachedVec<I, T> {
    let access_count = Arc::new(AtomicU64::new(0));
    let cached = CachedVec::new_budgeted(source, &BUDGET, access_count.clone());
    let clone = cached.clone();
    CACHES.lock().push(CacheEntry {
        access_count,
        clear: Box::new(move || clone.clear()),
    });
    cached
}

/// Clears all cached vecs and resets the budget.
pub fn cache_clear_all() {
    for entry in CACHES.lock().iter() {
        (entry.clear)();
    }
    BUDGET.remaining.store(MAX_CACHED, Relaxed);
}
