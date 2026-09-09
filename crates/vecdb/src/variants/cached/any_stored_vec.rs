use std::path::PathBuf;

use rawdb::{Database, Region};

use super::{CachedVec, CachedVecStrategy};
use crate::{AnyStoredVec, Header, Result, Stamp, StoredVec, WritableVec};

impl<V, S: CachedVecStrategy> AnyStoredVec for CachedVec<V, S>
where
    V: StoredVec,
{
    #[inline]
    fn db_path(&self) -> PathBuf {
        self.inner.db_path()
    }

    #[inline]
    fn region(&self) -> &Region {
        self.inner.region()
    }

    #[inline]
    fn header(&self) -> &Header {
        self.inner.header()
    }

    #[inline]
    fn mut_header(&mut self) -> &mut Header {
        self.inner.mut_header()
    }

    #[inline]
    fn saved_stamped_changes(&self) -> u16 {
        self.inner.saved_stamped_changes()
    }

    #[inline]
    fn write(&mut self) -> Result<bool> {
        self.inner.write()
    }

    #[inline]
    fn stored_len(&self) -> usize {
        self.inner.stored_len()
    }

    #[inline]
    fn real_stored_len(&self) -> usize {
        self.inner.real_stored_len()
    }

    #[inline]
    fn serialize_changes(&self) -> Result<Vec<u8>> {
        self.inner.serialize_changes()
    }

    #[inline]
    fn db(&self) -> Database {
        self.inner.db()
    }

    fn any_stamped_write_with_changes(&mut self, stamp: Stamp) -> Result<()> {
        self.inner.stamped_write_with_changes(stamp)
    }

    fn any_save_rollback_state(&mut self) {
        self.inner.save_rollback_state()
    }

    fn remove(self) -> Result<()> {
        self.invalidate();
        self.inner.remove()
    }

    fn any_truncate_if_needed_at(&mut self, index: usize) -> Result<()> {
        self.truncate_if_needed_at(index)
    }

    fn any_reset(&mut self) -> Result<()> {
        self.reset()
    }
}
