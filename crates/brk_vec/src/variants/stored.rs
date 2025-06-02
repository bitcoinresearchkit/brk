use std::{path::Path, time::Duration};

use arc_swap::ArcSwap;
use brk_core::{Result, Value, Version};
use memmap2::Mmap;

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator, BoxedVecIterator, CollectableVec,
    Format, GenericStoredVec, StoredIndex, StoredType,
};

use super::{CompressedVec, CompressedVecIterator, RawVec, RawVecIterator};

#[derive(Debug, Clone)]
pub enum StoredVec<I, T> {
    Raw(RawVec<I, T>),
    Compressed(CompressedVec<I, T>),
}

impl<I, T> StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub fn forced_import(
        path: &Path,
        value_name: &str,
        version: Version,
        format: Format,
    ) -> Result<Self> {
        let path = I::path(path, value_name);

        if version == Version::ZERO {
            dbg!(path, value_name);
            panic!("Version must be at least 1, can't verify endianess otherwise");
        }

        if format.is_compressed() {
            Ok(Self::Compressed(CompressedVec::forced_import(
                &path, version,
            )?))
        } else {
            Ok(Self::Raw(RawVec::forced_import(&path, version)?))
        }
    }
}

impl<I, T> GenericStoredVec<I, T> for StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn read_(&self, index: usize, guard: &Mmap) -> Result<Option<T>> {
        match self {
            StoredVec::Raw(v) => v.read_(index, guard),
            StoredVec::Compressed(v) => v.read_(index, guard),
        }
    }

    #[inline]
    fn mmap(&self) -> &ArcSwap<Mmap> {
        match self {
            StoredVec::Raw(v) => v.mmap(),
            StoredVec::Compressed(v) => v.mmap(),
        }
    }

    #[inline]
    fn stored_len(&self) -> usize {
        match self {
            StoredVec::Raw(v) => v.stored_len(),
            StoredVec::Compressed(v) => v.stored_len(),
        }
    }

    #[inline]
    fn pushed(&self) -> &[T] {
        match self {
            StoredVec::Raw(v) => v.pushed(),
            StoredVec::Compressed(v) => v.pushed(),
        }
    }
    #[inline]
    fn mut_pushed(&mut self) -> &mut Vec<T> {
        match self {
            StoredVec::Raw(v) => v.mut_pushed(),
            StoredVec::Compressed(v) => v.mut_pushed(),
        }
    }

    #[inline]
    fn path(&self) -> &Path {
        match self {
            StoredVec::Raw(v) => v.path(),
            StoredVec::Compressed(v) => v.path(),
        }
    }

    fn flush(&mut self) -> Result<()> {
        match self {
            StoredVec::Raw(v) => v.flush(),
            StoredVec::Compressed(v) => v.flush(),
        }
    }

    fn truncate_if_needed(&mut self, index: I) -> Result<()> {
        match self {
            StoredVec::Raw(v) => v.truncate_if_needed(index),
            StoredVec::Compressed(v) => v.truncate_if_needed(index),
        }
    }
}

impl<I, T> AnyVec for StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn version(&self) -> Version {
        match self {
            StoredVec::Raw(v) => v.version(),
            StoredVec::Compressed(v) => v.version(),
        }
    }

    #[inline]
    fn index_type_to_string(&self) -> String {
        I::to_string()
    }

    #[inline]
    fn len(&self) -> usize {
        self.pushed_len() + self.stored_len()
    }

    #[inline]
    fn modified_time(&self) -> Result<Duration> {
        match self {
            StoredVec::Raw(v) => v.modified_time(),
            StoredVec::Compressed(v) => v.modified_time(),
        }
    }

    fn name(&self) -> String {
        match self {
            StoredVec::Raw(v) => v.name(),
            StoredVec::Compressed(v) => v.name(),
        }
    }
}

#[derive(Debug)]
pub enum StoredVecIterator<'a, I, T> {
    Raw(RawVecIterator<'a, I, T>),
    Compressed(CompressedVecIterator<'a, I, T>),
}

impl<'a, I, T> Iterator for StoredVecIterator<'a, I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Value<'a, T>);
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Compressed(i) => i.next(),
            Self::Raw(i) => i.next(),
        }
    }
}

impl<I, T> BaseVecIterator for StoredVecIterator<'_, I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn mut_index(&mut self) -> &mut usize {
        match self {
            Self::Compressed(iter) => iter.mut_index(),
            Self::Raw(iter) => iter.mut_index(),
        }
    }

    fn len(&self) -> usize {
        match self {
            Self::Compressed(i) => i.len(),
            Self::Raw(i) => i.len(),
        }
    }
}

impl<'a, I, T> IntoIterator for &'a StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Value<'a, T>);
    type IntoIter = StoredVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        match self {
            StoredVec::Compressed(v) => StoredVecIterator::Compressed(v.into_iter()),
            StoredVec::Raw(v) => StoredVecIterator::Raw(v.into_iter()),
        }
    }
}

impl<I, T> AnyIterableVec<I, T> for StoredVec<I, T>
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

impl<I, T> AnyCollectableVec for StoredVec<I, T>
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
