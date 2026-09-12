use std::ops::Range;

use rawdb::Region;

use crate::{Result, VecIndex, VecValue, cache::CachePolicy};

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
        C::update(self.cache.clone(), from, || write(self))
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
                Self::read_stored_into(region, len, range.start, range.end, &mut values);
                (range.start, values)
            })
            .collect()
    }
}
