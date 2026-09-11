use std::ops::Range;

use rawdb::Region;

use crate::{
    AnyStoredVec, HEADER_OFFSET, RawIoSource, RawMmapSource, Result, VecIndex, VecValue,
    cache::CachePolicy,
};

use super::{RawStrategy, ReadWriteRawVec};

impl<I, T, S, C> ReadWriteRawVec<I, T, S, C>
where
    I: VecIndex,
    T: VecValue,
    S: RawStrategy<T>,
    C: CachePolicy,
{
    pub(super) fn with_cache_update<R>(
        &mut self,
        from: usize,
        write: impl FnOnce(&mut Self) -> Result<R>,
    ) -> Result<R> {
        match C::cache(&self.cache).cloned() {
            Some(cache) => cache.update(from, from < self.stored_len(), || write(self)),
            None => write(self),
        }
    }

    pub(super) fn load_cache_ranges(
        region: &Region,
        len: usize,
        missing: &[Range<usize>],
    ) -> Vec<(usize, Vec<T>)> {
        missing
            .iter()
            .map(|range| {
                let mut values = Vec::with_capacity(range.len());
                let offset = HEADER_OFFSET + range.start * size_of::<T>();
                let bytes = range.len() * size_of::<T>();
                if region.prefers_mmap(offset, bytes) {
                    RawMmapSource::<I, T, S>::new_from_parts(region, len, range.start, range.end)
                        .fold((), |(), value| values.push(value));
                } else {
                    RawIoSource::<I, T, S>::new_from_parts(region, len, range.start, range.end)
                        .read_into(&mut values);
                }
                (range.start, values)
            })
            .collect()
    }
}
