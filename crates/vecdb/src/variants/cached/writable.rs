use crate::{Result, Stamp, TypedVec, VecIndex, WritableVec};

use super::CachedVec;

impl<V: TypedVec + WritableVec<V::I, V::T>> CachedVec<V> {
    /// Invalidate shared snapshots before a truncation can replace existing rows.
    /// A no-op keeps the warm snapshot intact.
    pub fn truncate_if_needed_at(&mut self, index: usize) -> Result<()> {
        if index < self.inner.len() {
            self.invalidate();
            self.inner.truncate_if_needed_at(index)?;
        }
        Ok(())
    }

    /// Update the stamp even when no rows need truncation, as for the inner vec.
    pub fn truncate_if_needed_with_stamp(&mut self, index: V::I, stamp: Stamp) -> Result<()> {
        self.inner.update_stamp(stamp);
        self.truncate_if_needed_at(index.to_usize())
    }
}
