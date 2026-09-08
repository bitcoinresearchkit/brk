use crate::{ReadOnlyClone, StoredVec};

use super::{CachedVec, CachedVecStrategy};

impl<V: StoredVec, S: CachedVecStrategy> ReadOnlyClone for CachedVec<V, S> {
    type ReadOnly = CachedVec<V::ReadOnly, S>;

    #[inline]
    fn read_only_clone(&self) -> Self::ReadOnly {
        CachedVec {
            inner: self.inner.read_only_clone(),
            cache: self.cache.clone(),
            materialize: self.materialize.clone(),
            strategy: self.strategy.clone(),
        }
    }
}
