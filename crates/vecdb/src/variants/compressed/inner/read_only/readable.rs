use std::convert::Infallible;
use std::result::Result;

use super::{
    super::{CompressionStrategy, ReadWriteCompressedVec},
    ReadOnlyCompressedVec,
};
use crate::{
    CompressedIoSource, ReadableVec, VecIndex, VecValue,
    cache::{CachePolicy, Request},
    traits::chunk_folds::for_each_chunk,
};

impl<I, T, S, C: CachePolicy> ReadableVec<I, T> for ReadOnlyCompressedVec<I, T, S, C>
where
    I: VecIndex,
    T: VecValue,
    S: CompressionStrategy<T>,
{
    fn collect_one_at(&self, index: usize) -> Option<T> {
        if let Some(cache) = C::cache(&self.cache) {
            return cache.read_scope(|| {
                if index >= self.base.len() {
                    return None;
                }
                let stored = self.base.len();
                Some(cache.get(index, |ranges| {
                    ReadWriteCompressedVec::<I, T, S, C>::load_cache_ranges(
                        self.base.region(),
                        &self.pages,
                        stored,
                        ranges,
                    )
                }))
            });
        }
        let mut values = Vec::with_capacity(1);
        self.read_into_at(index, index.saturating_add(1), &mut values);
        values.pop()
    }

    fn data_revision(&self) -> Option<u64> {
        C::cache(&self.cache).map(|cache| cache.revision())
    }

    fn read_cached_into_at(&self, from: usize, to: usize, out: &mut Vec<T>) -> bool {
        C::cache(&self.cache)
            .is_some_and(|cache| cache.try_read_range(from, to, || self.base.len(), out))
    }

    #[inline(always)]
    fn cursor_chunk_size(&self) -> usize {
        ReadWriteCompressedVec::<I, T, S>::PER_PAGE
    }

    #[inline(always)]
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
        if let Some(cache) = C::cache(&self.cache) {
            return cache.read_scope(|| {
                let len = self.base.len();
                cache.read(Request::Range(from, to).clamp(len), buf, |ranges| {
                    ReadWriteCompressedVec::<I, T, S, C>::load_cache_ranges(
                        self.base.region(),
                        &self.pages,
                        len,
                        ranges,
                    )
                });
            });
        }
        let len = self.base.len();
        let from = from.min(len);
        let to = to.min(len);
        if from >= to {
            return;
        }
        buf.reserve(to - from);

        if ReadWriteCompressedVec::<I, T, S>::prefers_mmap(
            self.base.region(),
            &self.pages,
            from,
            to,
        ) {
            let reader = self.base.region().create_reader();
            let pages = self.pages.read();
            ReadWriteCompressedVec::<I, T, S>::read_stored_pages_into(
                &reader, &pages, from, to, buf,
            );
        } else {
            CompressedIoSource::<I, T, S>::new_from_parts(
                self.base.region(),
                &self.pages,
                len,
                from,
                to,
            )
            .read_into(buf);
        }
    }

    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<T>) {
        if let Some(cache) = C::cache(&self.cache) {
            return cache.read_scope(|| {
                let len = self.base.len();
                cache.read(Request::Sorted(indices).clamp(len), out, |ranges| {
                    ReadWriteCompressedVec::<I, T, S, C>::load_cache_ranges(
                        self.base.region(),
                        &self.pages,
                        len,
                        ranges,
                    )
                });
            });
        }
        let len = self.base.len();
        let indices = &indices[..indices.partition_point(|&i| i < len)];
        out.reserve(indices.len());
        ReadWriteCompressedVec::<I, T, S>::read_sorted_stored_into(
            self.base.region(),
            &self.pages,
            len,
            indices,
            out,
        );
    }

    fn for_each_chunk_at(&self, from: usize, to: usize, f: &mut dyn FnMut(usize, &[T])) {
        let Some(cache) = C::cache(&self.cache) else {
            return for_each_chunk(self, from, to, f);
        };
        cache.read_scope(|| {
            let stored = self.base.len();
            let to = to.min(self.base.len());
            cache
                .try_for_each_chunk(
                    from,
                    to.min(stored),
                    |ranges| {
                        ReadWriteCompressedVec::<I, T, S, C>::load_cache_ranges(
                            self.base.region(),
                            &self.pages,
                            stored,
                            ranges,
                        )
                    },
                    |at, values| {
                        f(at, values);
                        Ok::<_, Infallible>(())
                    },
                )
                .unwrap();
        });
    }

    #[inline]
    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(T)) {
        self.fold_range_at(from, to, (), |(), v| f(v));
    }

    #[inline]
    fn fold_range_at<B, F: FnMut(B, T) -> B>(&self, from: usize, to: usize, init: B, f: F) -> B
    where
        Self: Sized,
    {
        if C::cache(&self.cache).is_some() {
            let mut f = f;
            return self
                .try_fold_range_at(from, to, init, |acc, value| {
                    Ok::<_, Infallible>(f(acc, value))
                })
                .unwrap();
        }
        let len = self.base.len();
        let from = from.min(len);
        let to = to.min(len);
        if from >= to {
            return init;
        }
        self.fold_source(from, to, len, init, f)
    }

    #[inline]
    fn try_fold_range_at<B, E, F: FnMut(B, T) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        f: F,
    ) -> Result<B, E>
    where
        Self: Sized,
    {
        if let Some(cache) = C::cache(&self.cache) {
            return cache.read_scope(|| {
                let len = self.base.len();
                cache.try_fold(
                    from,
                    to.min(len),
                    init,
                    |ranges| {
                        ReadWriteCompressedVec::<I, T, S, C>::load_cache_ranges(
                            self.base.region(),
                            &self.pages,
                            len,
                            ranges,
                        )
                    },
                    f,
                )
            });
        }
        let len = self.base.len();
        let from = from.min(len);
        let to = to.min(len);
        if from >= to {
            return Ok(init);
        }
        self.try_fold_source(from, to, len, init, f)
    }
}
