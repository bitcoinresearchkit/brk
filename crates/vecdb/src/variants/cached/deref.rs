use std::ops::{Deref, DerefMut};

use super::{CachedVec, CachedVecStrategy};
use crate::StoredVec;

impl<V: StoredVec, S: CachedVecStrategy> Deref for CachedVec<V, S> {
    type Target = V;

    fn deref(&self) -> &V {
        &self.inner
    }
}

impl<V: StoredVec, S: CachedVecStrategy> DerefMut for CachedVec<V, S> {
    /// Inner compute APIs may rewrite values without changing length/version.
    /// Ordinary pushes and writes use the forwarding traits instead.
    fn deref_mut(&mut self) -> &mut V {
        self.invalidate();
        &mut self.inner
    }
}
