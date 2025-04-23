use core::error;
use std::{
    cmp::Ordering,
    fmt::Debug,
    ops::Add,
    path::{Path, PathBuf},
};

use brk_core::{Bitcoin, CheckedSub, Close, Dollars, Height, Sats, Txindex};
use brk_exit::Exit;
use brk_vec::{
    Compressed, DynamicVec, Error, GenericVec, Result, StoredIndex, StoredType, StoredVec, Version,
};
use log::info;

const ONE_KIB: usize = 1024;
const ONE_MIB: usize = ONE_KIB * ONE_KIB;
const MAX_CACHE_SIZE: usize = 210 * ONE_MIB;

#[derive(Debug, Clone, Copy)]
enum Mode {
    Lazy,
    Eager,
}

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

    pub fn collect_inclusive_range(&self, from: I, to: I) -> Result<Vec<T>> {
        self.inner.collect_inclusive_range(from, to)
    }
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
