#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

mod enums;
mod structs;
mod traits;
mod variants;

use std::{path::Path, time::Duration};

use arc_swap::ArcSwap;
pub use enums::*;
pub use memmap2::Mmap;
pub use structs::*;
pub use traits::*;
use variants::*;

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
    pub fn forced_import(path: &Path, version: Version, compressed: Compressed) -> Result<Self> {
        if *compressed {
            Ok(Self::Compressed(CompressedVec::forced_import(
                path, version,
            )?))
        } else {
            Ok(Self::Raw(RawVec::forced_import(path, version)?))
        }
    }

    pub fn iter(&self) -> StoredVecIterator<'_, I, T> {
        self.into_iter()
    }

    pub fn iter_at(&self, i: I) -> StoredVecIterator<'_, I, T> {
        let mut iter = self.into_iter();
        iter.set(i);
        iter
    }
}

impl<I, T> DynamicVec for StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type I = I;
    type T = T;

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
}

impl<I, T> GenericVec<I, T> for StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn collect_range(&self, from: Option<usize>, to: Option<usize>) -> Result<Vec<Self::T>> {
        match self {
            StoredVec::Raw(v) => v.collect_range(from, to),
            StoredVec::Compressed(v) => v.collect_range(from, to),
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

    #[inline]
    fn version(&self) -> Version {
        match self {
            StoredVec::Raw(v) => v.version(),
            StoredVec::Compressed(v) => v.version(),
        }
    }
}

impl<I, T> AnyVec for StoredVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn len(&self) -> usize {
        DynamicVec::len(self)
    }

    #[inline]
    fn index_type_to_string(&self) -> &str {
        GenericVec::index_type_to_string(self)
    }

    #[inline]
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        GenericVec::collect_range_serde_json(self, from, to)
    }

    #[inline]
    fn modified_time(&self) -> Result<Duration> {
        GenericVec::modified_time(self)
    }

    fn name(&self) -> String {
        GenericVec::name(self)
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
