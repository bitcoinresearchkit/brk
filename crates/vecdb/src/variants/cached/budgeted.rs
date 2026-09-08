use std::sync::{
    Arc,
    atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
};

use super::{CacheBudget, CachedVec, CachedVecBudget, CachedVecStrategy};
use crate::TypedVec;

/// Admission and eviction accounting for one cache in a shared budget.
#[derive(Clone)]
pub struct Budgeted {
    budget: &'static dyn CachedVecBudget,
    last_access: Arc<AtomicU64>,
    resident_bytes: Arc<AtomicUsize>,
}

pub type BudgetedCachedVec<V> = CachedVec<V, Budgeted>;

impl<V: TypedVec> CachedVec<V, Budgeted> {
    /// Eviction callback that retains neither the vector nor its source.
    /// Returns false once every reader has dropped its shared cache.
    pub fn weak_invalidator(&self) -> impl Fn() -> bool + Send + Sync + 'static {
        let cache = Arc::downgrade(&self.cache);
        let resident_bytes = self.strategy.resident_bytes.clone();
        let budget = self.strategy.budget;
        move || {
            let cache = cache.upgrade();
            let released = if let Some(cache) = &cache {
                let mut state = cache.write();
                state.invalidate();
                resident_bytes.swap(0, Relaxed)
            } else {
                resident_bytes.swap(0, Relaxed)
            };
            budget.release(released);
            cache.is_some()
        }
    }

    pub fn wrap_budgeted(
        inner: V,
        budget: &'static dyn CachedVecBudget,
        last_access: Arc<AtomicU64>,
        resident_bytes: Arc<AtomicUsize>,
    ) -> Self {
        Self::with_strategy(
            inner,
            Budgeted {
                budget,
                last_access,
                resident_bytes,
            },
        )
    }
}

impl CachedVecBudget for Budgeted {
    #[inline]
    fn admit(&self, cache_worthy: bool) -> bool {
        self.budget.admit(cache_worthy)
    }

    #[inline]
    fn record_access(&self) -> u64 {
        let last_access = &*self.last_access;
        let access = self.budget.record_access();
        last_access.store(access, Relaxed);
        access
    }

    #[inline]
    fn try_reserve(&self, bytes: usize) -> bool {
        self.budget.try_reserve(bytes)
    }

    #[inline]
    fn release(&self, bytes: usize) {
        self.budget.release(bytes);
    }
}

impl CachedVecStrategy for Budgeted {
    fn wrap<V: TypedVec>(source: V, budget: &'static CacheBudget) -> CachedVec<V, Self> {
        budget.wrap(source)
    }

    #[inline]
    fn set_resident_bytes(&self, bytes: usize) {
        self.resident_bytes.store(bytes, Relaxed);
    }

    #[inline]
    fn take_resident_bytes(&self) -> usize {
        self.resident_bytes.swap(0, Relaxed)
    }
}
