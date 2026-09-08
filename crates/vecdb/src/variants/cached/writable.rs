use crate::{Result, Stamp, TypedVec, VecIndex, WritableVec};

use super::{CachedVec, CachedVecStrategy};

impl<V: crate::StoredVec, S: CachedVecStrategy> WritableVec<V::I, V::T> for CachedVec<V, S> {
    #[inline]
    fn push(&mut self, value: V::T) {
        self.inner.push(value);
    }

    #[inline]
    fn pushed(&self) -> &[V::T] {
        self.inner.pushed()
    }

    #[inline]
    fn truncate_if_needed_at(&mut self, index: usize) -> crate::Result<()> {
        CachedVec::truncate_if_needed_at(self, index)
    }

    #[inline]
    fn reset(&mut self) -> crate::Result<()> {
        self.invalidate();
        self.inner.reset()
    }

    #[inline]
    fn reset_unsaved(&mut self) {
        self.invalidate();
        self.inner.reset_unsaved()
    }

    #[inline]
    fn is_dirty(&self) -> bool {
        self.inner.is_dirty()
    }

    #[inline]
    fn stamped_write_with_changes(&mut self, stamp: Stamp) -> crate::Result<()> {
        self.inner.stamped_write_with_changes(stamp)
    }

    #[inline]
    fn rollback(&mut self) -> crate::Result<()> {
        self.invalidate();
        self.inner.rollback()
    }

    fn find_rollback_files(
        &self,
    ) -> crate::Result<std::collections::BTreeMap<Stamp, std::path::PathBuf>> {
        self.inner.find_rollback_files()
    }

    fn save_rollback_state(&mut self) {
        self.inner.save_rollback_state()
    }
}

impl<V: TypedVec + WritableVec<V::I, V::T>, S: CachedVecStrategy> CachedVec<V, S> {
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
