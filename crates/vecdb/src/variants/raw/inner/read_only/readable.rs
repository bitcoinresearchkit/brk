use std::convert::Infallible;
use std::{result::Result, slice};

use super::{super::RawStrategy, ReadOnlyRawVec};
use crate::{
    HEADER_OFFSET, RawIoSource, ReadWriteRawVec, ReadableVec, VecIndex, VecValue,
    cache::{CachePolicy, Request},
    traits::chunk_folds::for_each_chunk,
};

impl<I, T, S, C: CachePolicy> ReadableVec<I, T> for ReadOnlyRawVec<I, T, S, C>
where
    I: VecIndex,
    T: VecValue,
    S: RawStrategy<T>,
{
    #[inline(always)]
    fn collect_one_at(&self, index: usize) -> Option<T> {
        if let Some(cache) = C::cache(&self.cache) {
            return cache.read_scope(|| {
                if index >= self.base.len() {
                    return None;
                }
                let stored = self.base.len();
                Some(cache.get(index, |ranges| {
                    ReadWriteRawVec::<I, T, S, C>::load_cache_ranges(
                        self.base.region(),
                        stored,
                        ranges,
                    )
                }))
            });
        }
        let len = self.base.len();
        if index >= len {
            return None;
        }
        Some(self.base.region().with_read_bytes(|bytes| unsafe {
            S::read_from_ptr(bytes.as_ptr().add(HEADER_OFFSET), index * size_of::<T>())
        }))
    }

    fn data_revision(&self) -> Option<u64> {
        C::cache(&self.cache).map(|cache| cache.revision())
    }

    fn read_cached_into_at(&self, from: usize, to: usize, out: &mut Vec<T>) -> bool {
        C::cache(&self.cache)
            .is_some_and(|cache| cache.try_read_range(from, to, || self.base.len(), out))
    }

    #[inline(always)]
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
        if let Some(cache) = C::cache(&self.cache) {
            return cache.read_scope(|| {
                let stored = self.base.len();
                let to = to.min(self.base.len());
                cache.read(Request::Range(from, to).clamp(stored), buf, |ranges| {
                    ReadWriteRawVec::<I, T, S, C>::load_cache_ranges(
                        self.base.region(),
                        stored,
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
        if S::IS_NATIVE_LAYOUT {
            let offset = HEADER_OFFSET + from * size_of::<T>();
            let bytes = (to - from) * size_of::<T>();
            if self.base.region().prefers_mmap(offset, bytes) {
                let reader = self.base.region().create_reader();
                let src = unsafe {
                    slice::from_raw_parts(
                        reader
                            .prefixed(HEADER_OFFSET)
                            .as_ptr()
                            .add(from * size_of::<T>()) as *const T,
                        to - from,
                    )
                };
                buf.extend_from_slice(src);
            } else {
                RawIoSource::<I, T, S>::new_from_parts(self.base.region(), len, from, to)
                    .read_into(buf);
            }
        } else {
            self.fold_source(from, to, len, (), |(), v| buf.push(v));
        }
    }

    #[inline]
    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<T>) {
        if let Some(cache) = C::cache(&self.cache) {
            return cache.read_scope(|| {
                let stored = self.base.len();
                let indices = &indices[..indices.partition_point(|&i| i < self.base.len())];
                let split = indices.partition_point(|&i| i < stored);
                cache.read(Request::Sorted(&indices[..split]), out, |ranges| {
                    ReadWriteRawVec::<I, T, S, C>::load_cache_ranges(
                        self.base.region(),
                        stored,
                        ranges,
                    )
                });
            });
        }
        let reader = self.reader();
        out.reserve(indices.len());
        for &index in indices {
            if let Some(value) = reader.try_get_at(index) {
                out.push(value);
            }
        }
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
                        ReadWriteRawVec::<I, T, S, C>::load_cache_ranges(
                            self.base.region(),
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
                let stored = self.base.len();
                let to = to.min(self.base.len());
                cache.try_fold(
                    from,
                    to,
                    init,
                    |ranges| {
                        ReadWriteRawVec::<I, T, S, C>::load_cache_ranges(
                            self.base.region(),
                            stored,
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
