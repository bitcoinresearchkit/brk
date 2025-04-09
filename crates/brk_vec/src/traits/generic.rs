use std::{
    fs::{File, OpenOptions},
    io::{self, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::Arc,
};

use arc_swap::{ArcSwap, Guard};
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
        file.set_len(len)?;
        file.seek(SeekFrom::End(0))?;
        self.update_mmap(file)
    }

    fn file_write_all(&mut self, buf: &[u8]) -> Result<()> {
        let mut file = self.open_file()?;
        file.write_all(buf)?;
        self.update_mmap(file)
    }

    #[inline]
    fn reset(&mut self) -> Result<()> {
        self.file_write_all(&[])?;
        Ok(())
    }

    fn mmap(&self) -> &ArcSwap<Mmap>;

    #[inline]
    fn new_guard(&self) -> Guard<Arc<Mmap>> {
        self.mmap().load()
    }
    fn guard(&self) -> &Option<Guard<Arc<Mmap>>>;
    fn mut_guard(&mut self) -> &mut Option<Guard<Arc<Mmap>>>;

    fn new_mmap(file: File) -> Result<Arc<Mmap>> {
        Ok(Arc::new(unsafe { Mmap::map(&file)? }))
    }

    fn update_mmap(&mut self, file: File) -> Result<()> {
        file.sync_all()?;
        let mmap = Self::new_mmap(file)?;
        self.mmap().store(mmap);
        if self.guard().is_some() {
            let guard = self.new_guard();
            self.mut_guard().replace(guard);
        } else {
            unreachable!()
        }
        Ok(())
    }

    #[inline]
    fn is_pushed_empty(&self) -> bool {
        self.pushed_len() == 0
    }

    #[inline]
    fn index_to_pushed_index(&self, index: usize) -> Result<Option<usize>> {
        let stored_len = self.stored_len();

        if index >= stored_len {
            let index = index - stored_len;
            if index >= self.pushed_len() {
                Err(Error::IndexTooHigh)
            } else {
                Ok(Some(index))
            }
        } else {
            Err(Error::IndexTooLow)
        }
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

    fn fix_i64(i: i64, len: usize, from: bool) -> usize {
        if i >= 0 {
            let v = i as usize;
            if v < len {
                v
            } else if from {
                len - 1
            } else {
                len
            }
        } else {
            let v = len as i64 + i;
            if v < 0 { 0 } else { v as usize }
        }
    }

    fn flush(&mut self) -> Result<()>;

    fn truncate_if_needed(&mut self, index: Self::I) -> Result<()>;

    fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<Self::T>>;

    #[inline]
    fn collect_range_axum_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Json<Vec<Self::T>>> {
        Ok(Json(self.collect_range(from, to)?))
    }

    #[inline]
    fn collect_range_serde_json(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<Value>> {
        self.collect_range(from, to)?
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
