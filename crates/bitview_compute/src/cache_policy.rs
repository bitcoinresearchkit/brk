use vecdb::{Budgeted, CachedVec, CachedVecStrategy, Pinned, TypedVec};

use crate::CACHE_BUDGET;

/// Constructs a source cache with the application's shared budget or a pinned policy.
pub trait CachePolicy: CachedVecStrategy {
    fn wrap<V: TypedVec>(source: V) -> CachedVec<V, Self>;
}

impl CachePolicy for Budgeted {
    fn wrap<V: TypedVec>(source: V) -> CachedVec<V, Self> {
        CACHE_BUDGET.wrap(source)
    }
}

impl CachePolicy for Pinned {
    fn wrap<V: TypedVec>(source: V) -> CachedVec<V, Self> {
        CachedVec::wrap(source)
    }
}
