use std::convert::Infallible;
use std::{result::Result, slice};

use super::{super::RawStrategy, ReadWriteRawVec};
use crate::{
    AnyStoredVec, HEADER_OFFSET, RawIoSource, ReadableVec, VecIndex, VecValue, cache::CachePolicy,
    cache::Request, traits::chunk_folds::for_each_chunk,
};

impl<I, T, S, C: CachePolicy> ReadableVec<I, T> for ReadWriteRawVec<I, T, S, C>
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
                let stored = self.stored_len();
                if index >= stored {
                    return self.base.pushed().get(index - stored).cloned();
                }
                Some(cache.get(index, |ranges| {
                    Self::load_cache_ranges(self.region(), stored, ranges)
                }))
            });
        }
        let len = self.base.len();
        if index >= len {
            return None;
        }

        let stored_len = self.stored_len();
        if index >= stored_len {
            return self.base.pushed().get(index - stored_len).cloned();
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
            .is_some_and(|cache| cache.try_read_range(from, to, || self.base.stored_len(), out))
    }

    #[inline(always)]
    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
        if let Some(cache) = C::cache(&self.cache) {
            return cache.read_scope(|| {
                let stored = self.stored_len();
                let to = to.min(self.base.len());
                cache.read(Request::Range(from, to).clamp(stored), buf, |ranges| {
                    Self::load_cache_ranges(self.region(), stored, ranges)
                });
                if to > stored && from < to {
                    buf.extend_from_slice(
                        &self.base.pushed()[from.max(stored) - stored..to - stored],
                    );
                }
            });
        }
        let len = self.base.len();
        let from = from.min(len);
        let to = to.min(len);
        if from >= to {
            return;
        }

        buf.reserve(to - from);

        let stored_len = self.stored_len();

        if from < stored_len {
            let stored_to = to.min(stored_len);
            if S::IS_NATIVE_LAYOUT {
                let offset = HEADER_OFFSET + from * Self::SIZE_OF_T;
                let bytes = (stored_to - from) * Self::SIZE_OF_T;
                if self.region().prefers_mmap(offset, bytes) {
                    let reader = self.raw_reader();
                    let src = unsafe {
                        slice::from_raw_parts(
                            reader
                                .prefixed(HEADER_OFFSET)
                                .as_ptr()
                                .add(from * Self::SIZE_OF_T)
                                as *const T,
                            stored_to - from,
                        )
                    };
                    buf.extend_from_slice(src);
                } else {
                    RawIoSource::new(self, from, stored_to).read_into(buf);
                }
            } else {
                self.fold_source(from, stored_to, (), |(), v| buf.push(v));
            }
        }

        if to > stored_len {
            let push_from = from.max(stored_len);
            let pushed = self.base.pushed();
            let start = push_from - stored_len;
            let end = (to - stored_len).min(pushed.len());
            buf.extend_from_slice(&pushed[start..end]);
        }
    }

    #[inline]
    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<T>) {
        if let Some(cache) = C::cache(&self.cache) {
            return cache.read_scope(|| {
                let stored = self.stored_len();
                let indices = &indices[..indices.partition_point(|&i| i < self.base.len())];
                let split = indices.partition_point(|&i| i < stored);
                cache.read(Request::Sorted(&indices[..split]), out, |ranges| {
                    Self::load_cache_ranges(self.region(), stored, ranges)
                });
                out.extend(
                    indices[split..]
                        .iter()
                        .map(|&i| self.base.pushed()[i - stored].clone()),
                );
            });
        }
        let reader = self.reader();
        let stored_len = reader.len();
        let pushed = self.base.pushed();

        out.reserve(indices.len());
        for &index in indices {
            if index < stored_len {
                out.push(reader.get_at(index));
            } else if let Some(value) = pushed.get(index - stored_len) {
                out.push(value.clone());
            }
        }
    }

    fn for_each_chunk_at(&self, from: usize, to: usize, f: &mut dyn FnMut(usize, &[T])) {
        let Some(cache) = C::cache(&self.cache) else {
            return for_each_chunk(self, from, to, f);
        };
        cache.read_scope(|| {
            let stored = self.stored_len();
            let to = to.min(self.base.len());
            cache
                .try_for_each_chunk(
                    from,
                    to.min(stored),
                    |ranges| Self::load_cache_ranges(self.region(), stored, ranges),
                    |at, values| {
                        f(at, values);
                        Ok::<_, Infallible>(())
                    },
                )
                .unwrap();
            if to > stored && from < to {
                let from = from.max(stored);
                f(from, &self.base.pushed()[from - stored..to - stored]);
            }
        });
    }

    #[inline]
    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(T)) {
        self.fold_range_at(from, to, (), |(), v| f(v));
    }

    #[inline]
    fn fold_range_at<B, F: FnMut(B, T) -> B>(&self, from: usize, to: usize, init: B, mut f: F) -> B
    where
        Self: Sized,
    {
        if C::cache(&self.cache).is_some() {
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

        let stored_len = self.stored_len();

        if to <= stored_len {
            return self.fold_source(from, to, init, f);
        }

        let mut acc = init;
        if from < stored_len {
            acc = self.fold_source(from, stored_len, acc, &mut f);
        }
        self.base.fold_pushed(from, to, acc, f)
    }

    #[inline]
    fn try_fold_range_at<B, E, F: FnMut(B, T) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> Result<B, E>
    where
        Self: Sized,
    {
        if let Some(cache) = C::cache(&self.cache) {
            return cache.read_scope(|| {
                let stored = self.stored_len();
                let to = to.min(self.base.len());
                let acc = cache.try_fold(
                    from,
                    to.min(stored),
                    init,
                    |ranges| Self::load_cache_ranges(self.region(), stored, ranges),
                    &mut f,
                )?;
                self.base.try_fold_pushed(from, to, acc, f)
            });
        }
        let len = self.base.len();
        let from = from.min(len);
        let to = to.min(len);
        if from >= to {
            return Ok(init);
        }

        let stored_len = self.stored_len();

        if to <= stored_len {
            return self.try_fold_source(from, to, init, f);
        }

        let mut acc = init;
        if from < stored_len {
            acc = self.try_fold_source(from, stored_len, acc, &mut f)?;
        }
        self.base.try_fold_pushed(from, to, acc, f)
    }
}
