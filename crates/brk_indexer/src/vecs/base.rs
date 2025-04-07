use std::{
    cmp::Ordering,
    fmt::Debug,
    io,
    path::{Path, PathBuf},
};

use brk_vec::{
    AnyStorableVec, Compressed, Error, MAX_CACHE_SIZE, MAX_PAGE_SIZE, Result, StorableVec,
    StoredIndex, StoredType, Value, Version,
};

use super::Height;

#[derive(Debug)]
pub struct IndexedVec<I, T> {
    height: Option<Height>,
    vec: StorableVec<I, T>,
}

impl<I, T> IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub const SIZE_OF_T: usize = size_of::<T>();
    pub const PER_PAGE: usize = MAX_PAGE_SIZE / Self::SIZE_OF_T;
    pub const PAGE_SIZE: usize = Self::PER_PAGE * Self::SIZE_OF_T;
    pub const CACHE_LENGTH: usize = MAX_CACHE_SIZE / Self::PAGE_SIZE;

    pub fn forced_import(
        path: &Path,
        version: Version,
        compressed: Compressed,
    ) -> brk_vec::Result<Self> {
        let mut vec = StorableVec::forced_import(path, version, compressed)?;

        vec.enable_large_cache();

        Ok(Self {
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
            vec,
        })
    }

    #[inline]
    pub fn get(&self, index: I) -> Result<Option<Value<'_, T>>> {
        self.get_(index.to_usize()?)
    }
    fn get_(&self, index: usize) -> Result<Option<Value<'_, T>>> {
        match self.vec.index_to_pushed_index(index) {
            Ok(index) => {
                if let Some(index) = index {
                    return Ok(self.vec.pushed().get(index).map(|v| Value::Ref(v)));
                }
            }
            Err(Error::IndexTooHigh) => return Ok(None),
            Err(Error::IndexTooLow) => {}
            Err(error) => return Err(error),
        }

        let large_cache_len = self.vec.large_cache_len();
        if large_cache_len != 0 {
            let page_index = Self::index_to_page_index(index);
            let last_index = self.vec.stored_len() - 1;
            let max_page_index = Self::index_to_page_index(last_index);
            let min_page_index = (max_page_index + 1) - large_cache_len;

            if page_index >= min_page_index {
                let values = self
                    .vec
                    .pages()
                    .unwrap()
                    .get(page_index - min_page_index)
                    .ok_or(Error::MmapsVecIsTooSmall)?
                    .get_or_init(|| self.vec.decode_page(page_index).unwrap());

                return Ok(values.get(index)?.map(|v| Value::Ref(v)));
            }
        }

        Ok(self.vec.read_(index)?.map(|v| Value::Owned(v)))
    }

    pub fn iter_from<F>(&mut self, index: I, f: F) -> Result<()>
    where
        F: FnMut((I, &T)) -> Result<()>,
    {
        self.vec.iter_from(index, f)
    }

    #[inline(always)]
    fn index_to_page_index(index: usize) -> usize {
        index / Self::PER_PAGE
    }

    #[inline]
    pub fn push_if_needed(&mut self, index: I, value: T) -> Result<()> {
        match self.vec.len().cmp(&index.to_usize()?) {
            Ordering::Greater => {
                // dbg!(len, index, &self.pathbuf);
                // panic!();
                Ok(())
            }
            Ordering::Equal => {
                self.vec.push(value);
                Ok(())
            }
            Ordering::Less => {
                dbg!(index, value, self.vec.len(), self.path_height());
                Err(Error::IndexTooHigh)
            }
        }
    }

    pub fn truncate_if_needed(&mut self, index: I, height: Height) -> brk_vec::Result<()> {
        if self.height.is_none_or(|self_height| self_height != height) {
            height.write(&self.path_height())?;
        }
        self.vec.truncate_if_needed(index)?;
        Ok(())
    }

    pub fn flush(&mut self, height: Height) -> io::Result<()> {
        height.write(&self.path_height())?;
        self.vec.flush()
    }

    pub fn vec(&self) -> &StorableVec<I, T> {
        &self.vec
    }

    pub fn mut_vec(&mut self) -> &mut StorableVec<I, T> {
        &mut self.vec
    }

    pub fn any_vec(&self) -> &dyn AnyStorableVec {
        &self.vec
    }

    pub fn len(&self) -> usize {
        self.vec.len()
    }

    pub fn is_empty(&self) -> bool {
        self.vec.is_empty()
    }

    #[inline]
    pub fn hasnt(&self, index: I) -> Result<bool> {
        self.vec.has(index).map(|b| !b)
    }

    pub fn height(&self) -> brk_core::Result<Height> {
        Height::try_from(self.path_height().as_path())
    }
    fn path_height(&self) -> PathBuf {
        Self::path_height_(self.vec.path())
    }
    fn path_height_(path: &Path) -> PathBuf {
        path.join("height")
    }
}

impl<I, T> Clone for IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn clone(&self) -> Self {
        Self {
            height: self.height,
            vec: self.vec.clone(),
        }
    }
}

pub trait AnyIndexedVec: Send + Sync {
    fn height(&self) -> brk_core::Result<Height>;
    fn flush(&mut self, height: Height) -> io::Result<()>;
}

impl<I, T> AnyIndexedVec for IndexedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn height(&self) -> brk_core::Result<Height> {
        self.height()
    }

    fn flush(&mut self, height: Height) -> io::Result<()> {
        self.flush(height)
    }
}
