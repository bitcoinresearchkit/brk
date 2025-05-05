use std::{
    cmp::Ordering,
    fmt::Debug,
    path::{Path, PathBuf},
    time::Duration,
};

use brk_core::Height;

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BoxedVecIterator, CollectableVec, Compressed, Error,
    GenericStoredVec, Mmap, Result, StoredIndex, StoredType, StoredVec, Value, Version,
};

use super::StoredVecIterator;

#[derive(Debug, Clone)]
pub struct IndexedVec<I, T> {
    height: Option<Height>,
    inner: StoredVec<I, T>,
}

impl<I, T> IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub fn forced_import(path: &Path, version: Version, compressed: Compressed) -> Result<Self> {
        Ok(Self {
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
            inner: StoredVec::forced_import(path, version, compressed)?,
        })
    }

    #[inline]
    pub fn get_or_read(&self, index: I, mmap: &Mmap) -> Result<Option<Value<T>>> {
        self.inner.get_or_read(index, mmap)
    }

    #[inline]
    pub fn push_if_needed(&mut self, index: I, value: T) -> Result<()> {
        let len = self.inner.len();
        match len.cmp(&index.to_usize()?) {
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
                dbg!(index, value, len, self.path_height());
                Err(Error::IndexTooHigh)
            }
        }
    }

    pub fn truncate_if_needed(&mut self, index: I, height: Height) -> Result<()> {
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

impl<I, T> AnyVec for IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn version(&self) -> Version {
        self.inner.version()
    }

    #[inline]
    fn name(&self) -> String {
        self.inner.name()
    }

    #[inline]
    fn len(&self) -> usize {
        self.inner.len()
    }

    #[inline]
    fn modified_time(&self) -> Result<Duration> {
        self.inner.modified_time()
    }

    #[inline]
    fn index_type_to_string(&self) -> &str {
        I::to_string()
    }
}

pub trait AnyIndexedVec: AnyVec {
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

impl<'a, I, T> IntoIterator for &'a IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Value<'a, T>);
    type IntoIter = StoredVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.inner.into_iter()
    }
}

impl<I, T> AnyIterableVec<I, T> for IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        T: 'a,
    {
        Box::new(self.into_iter())
    }
}

impl<I, T> AnyCollectableVec for IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
