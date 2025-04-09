use std::{
    fs,
    path::Path,
    sync::{Arc, OnceLock},
};

use arc_swap::{ArcSwap, Guard};
use axum::Json;
use memmap2::Mmap;

use crate::{
    AnyVec, CompressedPagesMetadata, Error, RawVec, Result, StoredIndex, StoredType, Value, Version,
};

#[derive(Debug)]
pub struct CompressedVec<I, T> {
    inner: RawVec<I, T>,
    decoded_page: Option<(usize, Box<[T]>)>,
    pages_meta: CompressedPagesMetadata,
    decoded_pages: Option<Vec<OnceLock<Box<[T]>>>>,
    // pages: Option<Vec<OnceLock<Values<T>>>>,
    // page: Option<(usize, Values<T>)>,
    // length: Length
}

impl<I, T> CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    /// Same as import but will reset the folder under certain errors, so be careful !
    pub fn forced_import(path: &Path, version: Version) -> Result<Self> {
        let res = Self::import(path, version);
        match res {
            Err(Error::WrongEndian)
            | Err(Error::DifferentVersion { .. })
            | Err(Error::DifferentCompressionMode) => {
                fs::remove_dir_all(path)?;
                Self::import(path, version)
            }
            _ => res,
        }
    }

    pub fn import(path: &Path, version: Version) -> Result<Self> {
        Ok(Self {
            inner: RawVec::import(path, version)?,
            decoded_page: None,
            decoded_pages: None,
            pages_meta: CompressedPagesMetadata::read(path)?,
        })
    }
}

impl<I, T> AnyVec<I, T> for CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn get_(&mut self, index: usize) -> Result<Option<Value<T>>> {
        self.inner.get_(index)
    }

    fn iter_from<F>(&mut self, _index: I, _f: F) -> Result<()>
    where
        F: FnMut((I, T, &mut Self)) -> Result<()>,
    {
        todo!()
        // self.inner.iter_from(index, f)
    }

    fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Json<Vec<T>>> {
        self.inner.collect_range(from, to)
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }

    fn truncate_if_needed(&mut self, index: I) -> Result<()> {
        self.inner.truncate_if_needed(index)
    }

    #[inline]
    fn mmap(&self) -> &ArcSwap<Mmap> {
        self.inner.mmap()
    }

    #[inline]
    fn guard(&self) -> &Option<Guard<Arc<Mmap>>> {
        self.inner.guard()
    }
    #[inline]
    fn mut_guard(&mut self) -> &mut Option<Guard<Arc<Mmap>>> {
        self.inner.mut_guard()
    }

    #[inline]
    fn pushed(&self) -> &[T] {
        self.inner.pushed()
    }
    #[inline]
    fn mut_pushed(&mut self) -> &mut Vec<T> {
        self.inner.mut_pushed()
    }

    #[inline]
    fn path(&self) -> &Path {
        self.inner.path()
    }

    #[inline]
    fn version(&self) -> Version {
        self.inner.version()
    }
}
