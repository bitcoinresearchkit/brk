use std::sync::Arc;

use parking_lot::{Mutex, RwLock};

mod any_stored_vec;
pub mod any_vec;
pub mod budget;
pub mod budgeted;
pub mod clone;
pub mod cloneable;
mod deref;
pub mod pinned;
pub mod read_only_clone;
pub mod readable;
mod readable_cloneable;
pub mod strategy;
pub mod typed;
pub mod writable;

pub use budget::CachedVecBudget;
pub use budgeted::{Budgeted, BudgetedCachedVec};
pub use cloneable::{CachedBoxedVec, CachedReadableVec};
pub use pinned::Pinned as NoBudget;
pub use pinned::{Pinned, PinnedCachedVec};
pub use strategy::CachedVecStrategy;

use crate::{ReadOnlyClone, ReadableVec, StoredVec, TypedVec, VecIndex, Version};

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
/// Use [`PinnedCachedVec`] or [`BudgetedCachedVec`] to make the policy explicit.
/// The default strategy is [`Pinned`].
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
pub struct CachedVec<V: TypedVec, S: CachedVecStrategy = Pinned> {
    pub inner: V,
    cache: Arc<RwLock<CacheState<V::T>>>,
    materialize: Arc<Mutex<()>>,
    strategy: S,
}

impl<V: TypedVec, S: CachedVecStrategy> CachedVec<V, S> {
    fn with_strategy(inner: V, strategy: S) -> Self {
        Self {
            inner,
            cache: Arc::new(RwLock::new(CacheState::empty())),
            materialize: Arc::new(Mutex::new(())),
            strategy,
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
            self.strategy.take_resident_bytes()
        };
        self.strategy.release(released_bytes);
    }
}

impl<V: TypedVec + ReadableVec<V::I, V::T>, S: CachedVecStrategy> CachedVec<V, S> {
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
            let version = self.inner.snapshot_version();
            let cache_is_empty = {
                let cache = self.cache.read();
                if let Some(data) = cache.matching_data(len, version) {
                    self.record_cache_access();
                    return Some(data);
                }
                cache.data.is_none()
            };
            let admitted = *admitted.get_or_insert_with(|| self.strategy.admit(cache_worthy()));
            if cache_is_empty && !admitted {
                return None;
            }

            let _materialize = self.materialize.lock();

            let len = self.inner.len();
            let version = self.inner.snapshot_version();
            let (generation, released_bytes) = {
                let mut cache = self.cache.write();
                if let Some(data) = cache.matching_data(len, version) {
                    self.record_cache_access();
                    return Some(data);
                }
                cache.invalidate();
                (cache.generation, self.strategy.take_resident_bytes())
            };
            self.strategy.release(released_bytes);
            if !admitted {
                return None;
            }

            let bytes = len.checked_mul(size_of::<V::T>())?;
            if bytes > 0 && !self.strategy.try_reserve(bytes) {
                return None;
            }

            let data = self.inner.collect_range_dyn(0, len);
            let mut cache = self.cache.write();
            if cache.generation != generation
                || self.inner.len() != len
                || self.inner.snapshot_version() != version
            {
                self.strategy.release(bytes);
                continue;
            }
            debug_assert_eq!(data.len(), len);
            debug_assert!(size_of::<V::T>() == 0 || data.capacity() == len);

            let data = Arc::new(data);
            self.record_cache_access();
            self.strategy.set_resident_bytes(bytes);
            cache.replace(len, version, data.clone());

            return Some(data);
        }
    }

    #[inline(always)]
    fn record_cache_access(&self) {
        self.strategy.record_access();
    }
}

impl<V: StoredVec, S: CachedVecStrategy> CachedVec<V, S> {
    /// Boxes a read-only clone for use with type-erased APIs (e.g. LazyVec).
    #[inline]
    pub fn read_only_boxed_clone(&self) -> crate::ReadableBoxedVec<V::I, V::T> {
        crate::ReadableBoxedVec::new(self.read_only_clone())
    }
}

#[cfg(test)]
#[path = "../../../tests/unit/variants/cached.rs"]
mod tests;
