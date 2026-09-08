use std::{ops::Deref, sync::Arc};

use super::{CachedVec, CachedVecStrategy};
use crate::{AnyVec, ReadOnlyClone, ReadableVec, StoredVec, TypedVec, VecIndex, VecValue, Version};

pub trait CachedReadableVec<I, T>: ReadableVec<I, T>
where
    I: VecIndex,
    T: VecValue,
{
    fn invalidate(&self);
    fn cached_boxed_clone(&self) -> CachedBoxedVec<I, T>;
}

pub struct CachedBoxedVec<I, T>(Box<dyn CachedReadableVec<I, T>>)
where
    I: VecIndex,
    T: VecValue;

impl<I, T> CachedBoxedVec<I, T>
where
    I: VecIndex,
    T: VecValue,
{
    pub fn new(inner: impl CachedReadableVec<I, T> + 'static) -> Self {
        Self(Box::new(inner))
    }
}

impl<I, T> Clone for CachedBoxedVec<I, T>
where
    I: VecIndex,
    T: VecValue,
{
    fn clone(&self) -> Self {
        self.0.cached_boxed_clone()
    }
}

impl<I, T> Deref for CachedBoxedVec<I, T>
where
    I: VecIndex,
    T: VecValue,
{
    type Target = dyn CachedReadableVec<I, T>;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<I: VecIndex, T: VecValue> CachedReadableVec<I, T> for CachedBoxedVec<I, T> {
    fn invalidate(&self) {
        self.0.invalidate();
    }

    fn cached_boxed_clone(&self) -> Self {
        self.clone()
    }
}

impl<I, T> AnyVec for CachedBoxedVec<I, T>
where
    I: VecIndex,
    T: VecValue,
{
    #[inline(always)]
    fn version(&self) -> Version {
        self.0.version()
    }

    #[inline(always)]
    fn name(&self) -> &str {
        self.0.name()
    }

    #[inline(always)]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline(always)]
    fn is_mutable(&self) -> bool {
        self.0.is_mutable()
    }

    #[inline(always)]
    fn visible_len(&self) -> usize {
        self.0.visible_len()
    }

    #[inline(always)]
    fn index_type_to_string(&self) -> &'static str {
        self.0.index_type_to_string()
    }

    #[inline(always)]
    fn region_names(&self) -> Vec<String> {
        self.0.region_names()
    }

    #[inline(always)]
    fn value_type_to_size_of(&self) -> usize {
        self.0.value_type_to_size_of()
    }

    #[inline(always)]
    fn value_type_to_string(&self) -> &'static str {
        self.0.value_type_to_string()
    }
}

impl<I, T> TypedVec for CachedBoxedVec<I, T>
where
    I: VecIndex,
    T: VecValue,
{
    type I = I;
    type T = T;
}

impl<I: VecIndex, T: VecValue> ReadableVec<I, T> for CachedBoxedVec<I, T> {
    #[inline]
    fn fold_range_at<B, F: FnMut(B, T) -> B>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> B {
        let mut acc = Some(init);
        self.for_each_chunk_at(from, to, &mut |_, values| {
            acc = Some(values.iter().cloned().fold(acc.take().unwrap(), &mut f));
        });
        acc.unwrap()
    }

    #[inline]
    fn try_fold_range_at<B, E, F: FnMut(B, T) -> Result<B, E>>(
        &self,
        from: usize,
        to: usize,
        init: B,
        mut f: F,
    ) -> Result<B, E> {
        let to = to.min(self.len());
        let chunk_size = self.cursor_chunk_size().max(1);
        let mut buf = Vec::with_capacity(chunk_size.min(to.saturating_sub(from)));
        let mut acc = init;

        let mut start = from;
        while start < to {
            let end = start.saturating_add(chunk_size).min(to);
            self.read_into_at(start, end, &mut buf);
            for value in buf.drain(..) {
                acc = f(acc, value)?;
            }
            start = end;
        }

        Ok(acc)
    }

    fn for_each_range_dyn_at(&self, from: usize, to: usize, f: &mut dyn FnMut(T)) {
        self.0.for_each_range_dyn_at(from, to, f);
    }

    fn snapshot(&self) -> Arc<Vec<T>> {
        self.0.snapshot()
    }

    fn snapshot_version(&self) -> Version {
        self.0.snapshot_version()
    }

    fn cursor_chunk_size(&self) -> usize {
        self.0.cursor_chunk_size()
    }

    fn read_into_at(&self, from: usize, to: usize, buf: &mut Vec<T>) {
        self.0.read_into_at(from, to, buf);
    }

    fn for_each_chunk_at(&self, from: usize, to: usize, f: &mut dyn FnMut(usize, &[T])) {
        self.0.for_each_chunk_at(from, to, f);
    }

    fn collect_one_at(&self, index: usize) -> Option<T> {
        self.0.collect_one_at(index)
    }

    fn read_sorted_into_at(&self, indices: &[usize], out: &mut Vec<T>) {
        self.0.read_sorted_into_at(indices, out);
    }
}

impl<I, T, V, S> CachedReadableVec<I, T> for CachedVec<V, S>
where
    I: VecIndex,
    T: VecValue,
    V: TypedVec<I = I, T = T> + ReadableVec<I, T> + Clone + Send + Sync + 'static,
    S: CachedVecStrategy,
{
    fn invalidate(&self) {
        CachedVec::invalidate(self);
    }

    fn cached_boxed_clone(&self) -> CachedBoxedVec<I, T> {
        CachedBoxedVec::new(self.clone())
    }
}

impl<V, S: CachedVecStrategy> CachedVec<V, S>
where
    V: StoredVec,
    V::ReadOnly:
        TypedVec<I = V::I, T = V::T> + ReadableVec<V::I, V::T> + Clone + Send + Sync + 'static,
{
    pub fn read_only_cached_boxed_clone(&self) -> CachedBoxedVec<V::I, V::T> {
        CachedBoxedVec::new(self.read_only_clone())
    }
}
