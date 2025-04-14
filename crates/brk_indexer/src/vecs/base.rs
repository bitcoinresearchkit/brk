use std::{
    cmp::Ordering,
    fmt::Debug,
    path::{Path, PathBuf},
};

use brk_vec::{
    Compressed, DynamicVec, Error, GenericVec, Result, StoredIndex, StoredType, StoredVec, Value,
    Version,
};

use super::Height;

#[derive(Debug, Clone)]
pub struct IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    height: Option<Height>,
    inner: StoredVec<I, T>,
}

impl<I, T> IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub fn forced_import(
        path: &Path,
        version: Version,
        compressed: Compressed,
    ) -> brk_vec::Result<Self> {
        let mut inner = StoredVec::forced_import(path, version, compressed)?;

        inner.enable_large_cache_if_needed();

        Ok(Self {
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
            inner,
        })
    }

    #[inline]
    pub fn get(&self, index: I) -> Result<Option<Value<'_, T>>> {
        self.inner.get(index)
    }
    #[inline]
    pub fn cached_get(&mut self, index: I) -> Result<Option<Value<'_, T>>> {
        self.inner.cached_get(index)
    }
    #[inline]
    pub fn cached_get_(&mut self, index: usize) -> Result<Option<Value<'_, T>>> {
        self.inner.cached_get_(index)
    }

    pub fn iter_from<F>(&mut self, index: I, f: F) -> Result<()>
    where
        F: FnMut((I, T, &mut dyn DynamicVec<I = I, T = T>)) -> Result<()>,
    {
        self.inner.iter_from(index, f)
    }

    #[inline]
    pub fn push_if_needed(&mut self, index: I, value: T) -> Result<()> {
        match self.inner.len().cmp(&index.to_usize()?) {
            Ordering::Greater => {
                // dbg!(len, index, &self.pathbuf);
                // panic!();
                Ok(())
            }
            Ordering::Equal => {
                self.inner.push(value);
                Ok(())
            }
            Ordering::Less => {
                dbg!(index, value, self.inner.len(), self.path_height());
                Err(Error::IndexTooHigh)
            }
        }
    }

    pub fn truncate_if_needed(&mut self, index: I, height: Height) -> brk_vec::Result<()> {
        if self.height.is_none_or(|self_height| self_height != height) {
            height.write(&self.path_height())?;
        }
        self.inner.truncate_if_needed(index)?;
        Ok(())
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        height.write(&self.path_height())?;
        self.inner.flush()
    }

    pub fn vec(&self) -> &StoredVec<I, T> {
        &self.inner
    }

    pub fn mut_vec(&mut self) -> &mut StoredVec<I, T> {
        &mut self.inner
    }

    pub fn any_vec(&self) -> &dyn brk_vec::AnyStoredVec {
        &self.inner
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    #[inline]
    pub fn hasnt(&self, index: I) -> Result<bool> {
        self.inner.has(index).map(|b| !b)
    }

    pub fn height(&self) -> brk_core::Result<Height> {
        Height::try_from(self.path_height().as_path())
    }
    fn path_height(&self) -> PathBuf {
        Self::path_height_(self.inner.path())
    }
    fn path_height_(path: &Path) -> PathBuf {
        path.join("height")
    }
}

pub trait AnyIndexedVec: Send + Sync {
    fn height(&self) -> brk_core::Result<Height>;
    fn flush(&mut self, height: Height) -> Result<()>;
}

impl<I, T> AnyIndexedVec for IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn height(&self) -> brk_core::Result<Height> {
        self.height()
    }

    fn flush(&mut self, height: Height) -> Result<()> {
        self.flush(height)
    }
}
