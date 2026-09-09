use std::result::Result;

use rawdb::Region;

use super::{RawStrategy, ReadOnlyRawVec};
use crate::{
    HEADER_OFFSET, RawIoSource, RawMmapSource, RawRangeCursor, VecIndex, VecReader, VecValue,
};

pub mod any_vec;
pub mod readable;
pub mod typed;

impl<I, T, S> ReadOnlyRawVec<I, T, S>
where
    I: VecIndex,
    T: VecValue,
    S: RawStrategy<T>,
{
    pub fn reader(&self) -> VecReader<I, T, S> {
        VecReader::from_read_only(self)
    }

    /// Creates a forward cursor over a bounded persisted range.
    #[inline]
    pub fn range_cursor_at(&self, from: usize, to: usize) -> RawRangeCursor<'_, I, T, S> {
        RawRangeCursor::new(self.region(), self.stored_len(), from, to)
    }

    pub fn region(&self) -> &Region {
        self.base.region()
    }
    pub fn stored_len(&self) -> usize {
        self.base.stored_len()
    }
    #[inline(always)]
    fn fold_source<B, F: FnMut(B, T) -> B>(
        &self,
        from: usize,
        to: usize,
        len: usize,
        init: B,
        f: F,
    ) -> B {
        let offset = HEADER_OFFSET + from * size_of::<T>();
        let bytes = (to - from) * size_of::<T>();
        if self.base.region().prefers_mmap(offset, bytes) {
            RawMmapSource::<I, T, S>::new_from_parts(self.base.region(), len, from, to)
                .fold(init, f)
        } else {
            RawIoSource::<I, T, S>::new_from_parts(self.base.region(), len, from, to).fold(init, f)
        }
    }
    #[inline(always)]
    fn try_fold_source<B, E, F: FnMut(B, T) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        len: usize,
        init: B,
        f: F,
    ) -> Result<B, E> {
        let offset = HEADER_OFFSET + from * size_of::<T>();
        let bytes = (to - from) * size_of::<T>();
        if self.base.region().prefers_mmap(offset, bytes) {
            RawMmapSource::<I, T, S>::new_from_parts(self.base.region(), len, from, to)
                .try_fold(init, f)
        } else {
            RawIoSource::<I, T, S>::new_from_parts(self.base.region(), len, from, to)
                .try_fold(init, f)
        }
    }
}
