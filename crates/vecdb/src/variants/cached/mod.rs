use std::sync::{
    Arc,
    atomic::{AtomicU64, AtomicUsize, Ordering::Relaxed},
};

use parking_lot::{Mutex, RwLock};

pub mod any_vec;
pub mod budget;
pub mod clone;
pub mod cloneable;
pub mod read_only_clone;
pub mod readable;
pub mod typed;
pub mod writable;

pub use budget::{CachedVecBudget, NoBudget};
pub use cloneable::{CachedBoxedVec, CachedReadableVec};

use crate::{ReadOnlyClone, ReadableVec, StoredVec, TypedVec, VecIndex, Version};

static NO_BUDGET: NoBudget = NoBudget;

struct CacheState<T> {
    len: usize,
    version: Version,
    generation: u64,
    data: Option<Arc<Vec<T>>>,
}

impl<T> CacheState<T> {
    fn empty() -> Self {
        Self {
            len: 0,
            version: Version::ZERO,
            generation: 0,
            data: None,
        }
    }

    fn matching_data(&self, len: usize, version: Version) -> Option<Arc<Vec<T>>> {
        if self.len == len && self.version == version {
            self.data.clone()
        } else {
            None
        }
    }

    fn invalidate(&mut self) {
        self.len = 0;
        self.version = Version::ZERO;
        self.generation = self.generation.wrapping_add(1);
        self.data = None;
    }

    fn replace(&mut self, len: usize, version: Version, data: Arc<Vec<T>>) {
        self.len = len;
        self.version = version;
        self.data = Some(data);
    }
}

/// Cached wrapper around any readable vec, refreshed when len or version changes.
///
/// Wraps a concrete vec `V` and adds an in-memory cache layer.
/// Reads always use a valid cache. Without a budget, the first miss materializes
/// the full snapshot. With a budget, an ordinary miss is retained only when that
/// read touches every source chunk; [`Self::snapshot`] explicitly requests the
/// complete snapshot.
///
/// Use the invalidating truncate methods when replacing existing rows.
/// Other writes through `inner` that preserve length and version require
/// [`Self::invalidate`] before dependent reads. Writers must exclude readers
/// throughout mutation and publication; invalidation is not a publication gate.
///
/// If the budget cannot retain a snapshot, reads fall through to the inner vec.
pub struct CachedVec<V: TypedVec> {
    pub inner: V,
    cache: Arc<RwLock<CacheState<V::T>>>,
    materialize: Arc<Mutex<()>>,
    budget: &'static dyn CachedVecBudget,
    last_access: Arc<AtomicU64>,
    resident_bytes: Arc<AtomicUsize>,
}

impl<V: TypedVec> CachedVec<V> {
    pub fn wrap(inner: V) -> Self {
        Self::wrap_budgeted(
            inner,
            &NO_BUDGET,
            Arc::new(AtomicU64::new(0)),
            Arc::new(AtomicUsize::new(0)),
        )
    }

    pub fn wrap_budgeted(
        inner: V,
        budget: &'static dyn CachedVecBudget,
        last_access: Arc<AtomicU64>,
        resident_bytes: Arc<AtomicUsize>,
    ) -> Self {
        Self {
            inner,
            cache: Arc::new(RwLock::new(CacheState::empty())),
            materialize: Arc::new(Mutex::new(())),
            budget,
            last_access,
            resident_bytes,
        }
    }

    #[inline(always)]
    pub fn version(&self) -> Version {
        self.inner.version()
    }

    pub fn invalidate(&self) {
        let released_bytes = {
            let mut cache = self.cache.write();
            cache.invalidate();
            self.resident_bytes.swap(0, Relaxed)
        };
        self.budget.release(released_bytes);
    }

    /// Invalidates this cache without retaining the vector or its source.
    /// Returns false once every reader has dropped its shared cache.
    pub fn weak_invalidator(&self) -> impl Fn() -> bool + Send + Sync + 'static {
        let cache = Arc::downgrade(&self.cache);
        let resident_bytes = self.resident_bytes.clone();
        let budget = self.budget;
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
}

impl<V: TypedVec + ReadableVec<V::I, V::T>> CachedVec<V> {
    /// Returns a full snapshot, retaining it when the budget allows.
    #[inline(always)]
    pub fn snapshot(&self) -> Arc<Vec<V::T>> {
        self.try_snapshot(|| true)
            .unwrap_or_else(|| Arc::new(self.inner.collect_range_dyn(0, self.inner.len())))
    }

    /// Returns the value at the given typed index.
    #[inline(always)]
    pub fn get(&self, index: V::I) -> Option<V::T> {
        self.get_at(index.to_usize())
    }

    /// Returns the value at the given raw index.
    #[inline(always)]
    pub fn get_at(&self, index: usize) -> Option<V::T> {
        self.collect_one_at(index)
    }

    /// Returns `None` when this read should not populate an empty budgeted cache
    /// or when the budget cannot retain the snapshot.
    fn try_snapshot(&self, cache_worthy: impl Fn() -> bool) -> Option<Arc<Vec<V::T>>> {
        let mut admitted = None;
        loop {
            let len = self.inner.len();
            let version = self.inner.version();
            let cache_is_empty = {
                let cache = self.cache.read();
                if let Some(data) = cache.matching_data(len, version) {
                    self.record_cache_access();
                    return Some(data);
                }
                cache.data.is_none()
            };
            let admitted = *admitted.get_or_insert_with(|| self.budget.admit(cache_worthy()));
            if cache_is_empty && !admitted {
                return None;
            }

            let _materialize = self.materialize.lock();

            let len = self.inner.len();
            let version = self.inner.version();
            let (generation, released_bytes) = {
                let mut cache = self.cache.write();
                if let Some(data) = cache.matching_data(len, version) {
                    self.record_cache_access();
                    return Some(data);
                }
                cache.invalidate();
                (cache.generation, self.resident_bytes.swap(0, Relaxed))
            };
            self.budget.release(released_bytes);
            if !admitted {
                return None;
            }

            let bytes = len.checked_mul(size_of::<V::T>())?;
            if bytes > 0 && !self.budget.try_reserve(bytes) {
                return None;
            }

            let data = self.inner.collect_range_dyn(0, len);
            let mut cache = self.cache.write();
            if cache.generation != generation
                || self.inner.len() != len
                || self.inner.version() != version
            {
                self.budget.release(bytes);
                continue;
            }
            debug_assert_eq!(data.len(), len);
            debug_assert!(size_of::<V::T>() == 0 || data.capacity() == len);

            let data = Arc::new(data);
            self.record_cache_access();
            self.resident_bytes.store(bytes, Relaxed);
            cache.replace(len, version, data.clone());

            return Some(data);
        }
    }

    #[inline(always)]
    fn record_cache_access(&self) {
        self.last_access.store(self.budget.record_access(), Relaxed);
    }
}

impl<V: StoredVec> CachedVec<V> {
    /// Boxes a read-only clone for use with type-erased APIs (e.g. LazyVec).
    #[inline]
    pub fn read_only_boxed_clone(&self) -> crate::ReadableBoxedVec<V::I, V::T> {
        crate::ReadableBoxedVec::new(self.read_only_clone())
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/variants/cached.rs"]
mod tests;
