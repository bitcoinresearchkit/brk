use std::{
    fs::{File, OpenOptions},
    io::{self, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use axum::{
    Json,
    response::{IntoResponse, Response},
};
use memmap2::Mmap;
use serde_json::Value;

use crate::{Error, Result, Version};

use super::{DynamicVec, StoredIndex, StoredType};

pub trait GenericVec<I, T>: DynamicVec<I = I, T = T>
where
    I: StoredIndex,
    T: StoredType,
{
    const SIZE_OF_T: usize = size_of::<Self::T>();

    fn open_file(&self) -> io::Result<File> {
        Self::open_file_(&self.path_vec())
    }
    fn open_file_(path: &Path) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .create(true)
            .truncate(false)
            .append(true)
            .open(path)
    }

    fn file_set_len(&mut self, len: u64) -> Result<()> {
        let mut file = self.open_file()?;
        Self::file_set_len_(&mut file, len)?;
        self.update_mmap(file)
    }
    fn file_set_len_(file: &mut File, len: u64) -> Result<()> {
        file.set_len(len)?;
        file.seek(SeekFrom::End(0))?;
        Ok(())
    }

    fn file_write_all(&mut self, buf: &[u8]) -> Result<()> {
        let mut file = self.open_file()?;
        file.write_all(buf)?;
        self.update_mmap(file)
    }

    fn file_truncate_and_write_all(&mut self, len: u64, buf: &[u8]) -> Result<()> {
        let mut file = self.open_file()?;
        Self::file_set_len_(&mut file, len)?;
        file.write_all(buf)?;
        self.update_mmap(file)
    }

    #[inline]
    fn reset(&mut self) -> Result<()> {
        self.file_write_all(&[])?;
        Ok(())
    }

    fn new_mmap(file: File) -> Result<Arc<Mmap>> {
        Ok(Arc::new(unsafe { Mmap::map(&file)? }))
    }

    fn update_mmap(&mut self, file: File) -> Result<()> {
        let mmap = Self::new_mmap(file)?;
        self.mmap().store(mmap);
        if self.guard().is_some() {
            let guard = self.new_guard();
            self.mut_guard().replace(guard);
        } else {
            unreachable!("This function shouldn't be called in a cloned instance")
        }
        Ok(())
    }

    #[inline]
    fn is_pushed_empty(&self) -> bool {
        self.pushed_len() == 0
    }

    #[inline]
    fn has(&self, index: Self::I) -> Result<bool> {
        Ok(self.has_(index.to_usize()?))
    }
    #[inline]
    fn has_(&self, index: usize) -> bool {
        index < self.len()
    }

    #[inline]
    fn index_type_to_string(&self) -> &str {
        Self::I::to_string()
    }

    #[inline]
    fn iter<F>(&mut self, f: F) -> Result<()>
    where
        F: FnMut(
            (
                Self::I,
                Self::T,
                &mut dyn DynamicVec<I = Self::I, T = Self::T>,
            ),
        ) -> Result<()>,
    {
        self.iter_from(Self::I::default(), f)
    }

    fn iter_from<F>(&mut self, index: Self::I, f: F) -> Result<()>
    where
        F: FnMut(
            (
                Self::I,
                Self::T,
                &mut dyn DynamicVec<I = Self::I, T = Self::T>,
            ),
        ) -> Result<()>;

    fn flush(&mut self) -> Result<()>;

    fn truncate_if_needed(&mut self, index: Self::I) -> Result<()>;

    fn collect_range(&self, from: Option<usize>, to: Option<usize>) -> Result<Vec<Self::T>>;

    #[inline]
    fn collect_inclusive_range(&self, from: I, to: I) -> Result<Vec<Self::T>> {
        self.collect_range(Some(from.to_usize()?), Some(to.to_usize()? + 1))
    }

    #[inline]
    fn i64_to_usize(i: i64, len: usize) -> usize {
        if i >= 0 {
            i as usize
        } else {
            let v = len as i64 + i;
            if v < 0 { 0 } else { v as usize }
        }
    }

    fn collect_signed_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<Self::T>> {
        let len = self.len();
        let from = from.map(|i| Self::i64_to_usize(i, len));
        let to = to.map(|i| Self::i64_to_usize(i, len));
        self.collect_range(from, to)
    }

    #[inline]
    fn collect_range_axum_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Json<Vec<Self::T>>> {
        Ok(Json(self.collect_signed_range(from, to)?))
    }

    #[inline]
    fn collect_range_serde_json(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<Value>> {
        self.collect_signed_range(from, to)?
            .into_iter()
            .map(|v| serde_json::to_value(v).map_err(Error::from))
            .collect::<Result<Vec<_>>>()
    }

    #[inline]
    fn collect_range_response(&self, from: Option<i64>, to: Option<i64>) -> Result<Response> {
        Ok(self.collect_range_axum_json(from, to)?.into_response())
    }

    fn path(&self) -> &Path;

    #[inline]
    fn path_vec(&self) -> PathBuf {
        Self::path_vec_(self.path())
    }
    #[inline]
    fn path_vec_(path: &Path) -> PathBuf {
        path.join("vec")
    }

    #[inline]
    fn path_version_(path: &Path) -> PathBuf {
        path.join("version")
    }

    #[inline]
    fn path_compressed_(path: &Path) -> PathBuf {
        path.join("compressed")
    }

    #[inline]
    fn file_name(&self) -> String {
        self.path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    }

    fn version(&self) -> Version;
}
