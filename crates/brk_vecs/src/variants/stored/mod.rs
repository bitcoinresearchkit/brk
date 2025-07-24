use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    sync::Arc,
};

use brk_core::{Result, Version};

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator, BoxedVecIterator, CollectableVec,
    File, GenericStoredVec, Header, StoredIndex, StoredType, file::Reader,
};

use super::{CompressedVec, CompressedVecIterator, RawVec, RawVecIterator};

mod format;

pub use format::*;

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
        file: &Arc<File>,
        name: &str,
        version: Version,
        format: Format,
    ) -> Result<Self> {
        if version == Version::ZERO {
            dbg!(file, name);
            panic!("Version must be at least 1, can't verify endianess otherwise");
        }

        if format.is_compressed() {
            Ok(Self::Compressed(CompressedVec::forced_import(
                file, name, version,
            )?))
        } else {
            Ok(Self::Raw(RawVec::forced_import(file, name, version)?))
        }
    }
}

impl<I, T> GenericStoredVec<I, T> for StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn read_(&self, index: usize, reader: &Reader<'_>) -> Result<Option<T>> {
        match self {
            StoredVec::Raw(v) => v.read_(index, reader),
            StoredVec::Compressed(v) => v.read_(index, reader),
        }
    }

    #[inline]
    fn header(&self) -> &Header {
        match self {
            StoredVec::Raw(v) => v.header(),
            StoredVec::Compressed(v) => v.header(),
        }
    }

    #[inline]
    fn mut_header(&mut self) -> &mut Header {
        match self {
            StoredVec::Raw(v) => v.mut_header(),
            StoredVec::Compressed(v) => v.mut_header(),
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
    fn holes(&self) -> &BTreeSet<usize> {
        match self {
            StoredVec::Raw(v) => v.holes(),
            StoredVec::Compressed(v) => v.holes(),
        }
    }
    #[inline]
    fn mut_holes(&mut self) -> &mut BTreeSet<usize> {
        match self {
            StoredVec::Raw(v) => v.mut_holes(),
            StoredVec::Compressed(v) => v.mut_holes(),
        }
    }

    #[inline]
    fn updated(&self) -> &BTreeMap<usize, T> {
        match self {
            StoredVec::Raw(v) => v.updated(),
            StoredVec::Compressed(v) => v.updated(),
        }
    }
    #[inline]
    fn mut_updated(&mut self) -> &mut BTreeMap<usize, T> {
        match self {
            StoredVec::Raw(v) => v.mut_updated(),
            StoredVec::Compressed(v) => v.mut_updated(),
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

    fn reset(&mut self) -> Result<()> {
        match self {
            StoredVec::Raw(v) => v.reset(),
            StoredVec::Compressed(v) => v.reset(),
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
    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    #[inline]
    fn len(&self) -> usize {
        self.pushed_len() + self.stored_len()
    }

    fn name(&self) -> &str {
        match self {
            StoredVec::Raw(v) => v.name(),
            StoredVec::Compressed(v) => v.name(),
        }
    }

    #[inline]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }

    #[inline]
    fn file(&self) -> &File {
        match self {
            StoredVec::Raw(v) => v.file(),
            StoredVec::Compressed(v) => v.file(),
        }
    }

    #[inline]
    fn region_index(&self) -> usize {
        match self {
            StoredVec::Raw(v) => v.region_index(),
            StoredVec::Compressed(v) => v.region_index(),
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
    type Item = (I, Cow<'a, T>);
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

    #[inline]
    fn name(&self) -> &str {
        match self {
            Self::Compressed(i) => i.name(),
            Self::Raw(i) => i.name(),
        }
    }
}

impl<'a, I, T> IntoIterator for &'a StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Cow<'a, T>);
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
        from: Option<usize>,
        to: Option<usize>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
