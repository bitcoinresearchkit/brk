use std::fmt::Debug;

use brk_vec::{DynamicVec, GenericVec, StoredIndex, StoredType, StoredVec, Version};

#[derive(Debug)]
pub struct LazyVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    inner: StoredVec<I, T>,
}

impl<I, T> LazyVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    const SIZE_OF: usize = size_of::<T>();

    fn version(&self) -> Version {
        self.inner.version()
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }
    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn unwrap_cached_get(&mut self, index: I) -> Option<T> {
        self.inner.unwrap_cached_get(index)
    }
    #[inline]
    pub fn double_unwrap_cached_get(&mut self, index: I) -> T {
        self.inner.double_unwrap_cached_get(index)
    }

    // pub fn collect_inclusive_range(&self, from: I, to: I) -> Result<Vec<T>> {
    //     self.inner.collect_inclusive_range(from, to)
    // }
}

impl<I, T> Clone for LazyVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
        }
    }
}
