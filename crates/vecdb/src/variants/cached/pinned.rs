use super::{CachedVec, CachedVecBudget, CachedVecStrategy};
use crate::TypedVec;

/// Retains snapshots without a memory budget or eviction accounting.
#[derive(Clone, Copy)]
pub struct Pinned;

pub type PinnedCachedVec<V> = CachedVec<V, Pinned>;

impl<V: TypedVec> CachedVec<V, Pinned> {
    pub fn wrap(inner: V) -> Self {
        Self::with_strategy(inner, Pinned)
    }
}

impl CachedVecBudget for Pinned {
    #[inline]
    fn admit(&self, _: bool) -> bool {
        true
    }

    #[inline]
    fn try_reserve(&self, _: usize) -> bool {
        true
    }

    #[inline]
    fn release(&self, _: usize) {}
}

impl CachedVecStrategy for Pinned {
    #[inline]
    fn set_resident_bytes(&self, _: usize) {}

    #[inline]
    fn take_resident_bytes(&self) -> usize {
        0
    }
}
