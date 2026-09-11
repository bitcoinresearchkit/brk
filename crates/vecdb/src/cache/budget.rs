use std::{
    fmt,
    sync::{
        Weak,
        atomic::{AtomicUsize, Ordering::Relaxed},
    },
};

use parking_lot::Mutex;

use super::Charge;

pub(super) trait Reclaim: Send + Sync {
    fn evict_one(&self) -> bool;
    fn clear(&self);
}

#[derive(Default)]
struct Registry {
    caches: Vec<Weak<dyn Reclaim>>,
    next: usize,
}

/// Shared limit for retained buffers and a conservative range-directory allowance.
///
/// Charges follow allocation lifetimes, including buffers borrowed by active reads
/// after eviction. Decoder scratch and caller-owned results are not retained bytes.
pub struct CacheBudget {
    limit: usize,
    used: AtomicUsize,
    registry: Mutex<Registry>,
}

impl CacheBudget {
    pub const fn new(limit: usize) -> Self {
        Self {
            limit,
            used: AtomicUsize::new(0),
            registry: Mutex::new(Registry {
                caches: Vec::new(),
                next: 0,
            }),
        }
    }

    pub fn limit(&self) -> usize {
        self.limit
    }

    pub fn used(&self) -> usize {
        self.used.load(Relaxed)
    }

    pub(super) fn register(&self, cache: Weak<dyn Reclaim>) {
        let mut registry = self.registry.lock();
        // Prune before growing, not on every import: registering many live
        // sources stays amortized linear instead of repeatedly scanning them.
        if registry.caches.len() == registry.caches.capacity() {
            registry.caches.retain(|cache| cache.strong_count() != 0);
        }
        registry.caches.push(cache);
    }

    /// Evicts retained data without invalidating the underlying source generation.
    pub fn clear(&self) {
        let caches: Vec<_> = self
            .registry
            .lock()
            .caches
            .iter()
            .filter_map(Weak::upgrade)
            .collect();
        for cache in caches {
            cache.clear();
        }
    }

    pub(super) fn reserve(&'static self, bytes: usize) -> Option<Charge> {
        if bytes > self.limit {
            return None;
        }
        let claim = || {
            self.used
                .fetch_update(Relaxed, Relaxed, |used| {
                    used.checked_add(bytes).filter(|&next| next <= self.limit)
                })
                .ok()
                .map(|_| Charge::new(self, bytes))
        };
        if let Some(charge) = claim() {
            return Some(charge);
        }

        // Rotate across owners and their ranges. Never take a source/fill lock,
        // and never call a cache while holding the shared registry lock.
        let count = self.registry.lock().caches.len();
        let attempts = bytes.div_ceil(64).saturating_add(count).saturating_mul(4);
        let mut empty = 0;
        for _ in 0..attempts {
            let cache = {
                let mut registry = self.registry.lock();
                if registry.caches.is_empty() {
                    break;
                }
                let at = registry.next % registry.caches.len();
                registry.next = at + 1;
                registry.caches[at].upgrade()
            };
            if cache.is_some_and(|cache| cache.evict_one()) {
                empty = 0;
            } else {
                empty += 1;
            }
            if let Some(charge) = claim() {
                return Some(charge);
            }
            if empty >= count {
                break;
            }
        }
        None
    }

    pub(super) fn release(&self, bytes: usize) {
        let previous = self.used.fetch_sub(bytes, Relaxed);
        debug_assert!(previous >= bytes);
    }
}

impl fmt::Debug for CacheBudget {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("CacheBudget")
            .field("limit", &self.limit)
            .field("used", &self.used())
            .finish()
    }
}
