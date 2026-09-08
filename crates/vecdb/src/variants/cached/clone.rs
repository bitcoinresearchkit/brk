use crate::TypedVec;

use super::{CachedVec, CachedVecStrategy};

impl<V: TypedVec + Clone, S: CachedVecStrategy> Clone for CachedVec<V, S> {
    #[inline(always)]
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            cache: self.cache.clone(),
            materialize: self.materialize.clone(),
            strategy: self.strategy.clone(),
        }
    }
}
