use std::{borrow::Cow, cmp::Ordering, fmt::Debug, path::Path};

use brk_core::{Error, Height, Result, Version};

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BoxedVecIterator, CollectableVec, Format,
    GenericStoredVec, Header, Mmap, StoredIndex, StoredType, StoredVec,
};

use super::StoredVecIterator;

#[derive(Debug, Clone)]
pub struct IndexedVec<I, T>(StoredVec<I, T>);

impl<I, T> IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub fn forced_import(
        path: &Path,
        name: &str,
        version: Version,
        format: Format,
    ) -> Result<Self> {
        Ok(Self(
            StoredVec::forced_import(path, name, version, format).unwrap(),
        ))
    }

    #[inline]
    pub fn unwrap_read(&self, index: I, mmap: &Mmap) -> T {
        self.0.unwrap_read(index, mmap)
    }

    #[inline]
    pub fn get_or_read(&self, index: I, mmap: &Mmap) -> Result<Option<Cow<T>>> {
        self.0.get_or_read(index, mmap)
    }

    #[inline]
    pub fn update_or_push(&mut self, index: I, value: T) -> Result<()> {
        self.0.update_or_push(index, value)
    }

    #[inline]
    pub fn checked_push(&mut self, index: I, value: T) -> Result<()> {
        let len = self.0.len();
        match len.cmp(&index.to_usize()?) {
            Ordering::Greater => {
                dbg!(index, value, len, self.0.header());
                Err(Error::IndexTooLow)
            }
            Ordering::Equal => {
                self.0.push(value);
                Ok(())
            }
            Ordering::Less => {
                dbg!(index, value, len, self.0.header());
                Err(Error::IndexTooHigh)
            }
        }
    }

    #[inline]
    pub fn push_if_needed(&mut self, index: I, value: T) -> Result<()> {
        let len = self.0.len();
        match len.cmp(&index.to_usize()?) {
            Ordering::Greater => {
                // dbg!(len, index, &self.pathbuf);
                // panic!();
                Ok(())
            }
            Ordering::Equal => {
                self.0.push(value);
                Ok(())
            }
            Ordering::Less => {
                dbg!(index, value, len, self.0.header());
                Err(Error::IndexTooHigh)
            }
        }
    }

    #[inline]
    pub fn fill_first_hole_or_push(&mut self, value: T) -> Result<I> {
        self.0.fill_first_hole_or_push(value)
    }

    pub fn update(&mut self, index: I, value: T) -> Result<()> {
        self.0.update(index, value)
    }

    pub fn take(&mut self, index: I, mmap: &Mmap) -> Result<Option<T>> {
        self.0.take(index, mmap)
    }

    pub fn delete(&mut self, index: I) {
        self.0.delete(index)
    }

    fn update_height(&mut self, height: Height) {
        self.0.mut_header().update_height(height);
    }

    pub fn reset(&mut self) -> Result<()> {
        self.update_height(Height::ZERO);
        self.0.reset()
    }

    pub fn truncate_if_needed(&mut self, index: I, height: Height) -> Result<()> {
        self.update_height(height);
        self.0.truncate_if_needed(index)?;
        Ok(())
    }

    pub fn flush(&mut self, height: Height) -> Result<()> {
        self.update_height(height);
        self.0.flush()
    }

    pub fn header(&self) -> &Header {
        self.0.header()
    }

    pub fn create_mmap(&self) -> Result<Mmap> {
        self.0.create_mmap()
    }

    #[inline]
    pub fn hasnt(&self, index: I) -> Result<bool> {
        self.0.has(index).map(|b| !b)
    }
}

impl<I, T> AnyVec for IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn version(&self) -> Version {
        self.0.version()
    }

    #[inline]
    fn name(&self) -> &str {
        self.0.name()
    }

    #[inline]
    fn len(&self) -> usize {
        self.0.len()
    }

    #[inline]
    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    #[inline]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }
}

pub trait AnyIndexedVec: AnyVec {
    fn height(&self) -> Height;
    fn flush(&mut self, height: Height) -> Result<()>;
}

impl<I, T> AnyIndexedVec for IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn height(&self) -> Height {
        self.0.header().height()
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
    type Item = (I, Cow<'a, T>);
    type IntoIter = StoredVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
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
        from: Option<usize>,
        to: Option<usize>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
