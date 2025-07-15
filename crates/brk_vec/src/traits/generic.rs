use std::{
    borrow::Cow,
    fs::{File, OpenOptions},
    io::{self, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
};

use brk_core::Result;
use memmap2::Mmap;

use crate::{AnyVec, HEADER_OFFSET, Header};

use super::{StoredIndex, StoredType};

pub trait GenericStoredVec<I, T>: Send + Sync
where
    Self: AnyVec,
    I: StoredIndex,
    T: StoredType,
{
    const SIZE_OF_T: usize = size_of::<T>();

    #[inline]
    fn unwrap_read(&self, index: I, mmap: &Mmap) -> T {
        self.read(index, mmap).unwrap().unwrap()
    }
    #[inline]
    fn read(&self, index: I, mmap: &Mmap) -> Result<Option<T>> {
        self.read_(index.to_usize()?, mmap)
    }
    fn read_(&self, index: usize, mmap: &Mmap) -> Result<Option<T>>;

    #[inline]
    fn get_or_read(&self, index: I, mmap: &Mmap) -> Result<Option<Cow<T>>> {
        self.get_or_read_(index.to_usize()?, mmap)
    }
    #[inline]
    fn get_or_read_(&self, index: usize, mmap: &Mmap) -> Result<Option<Cow<T>>> {
        let stored_len = self.stored_len();

        if index >= stored_len {
            let pushed = self.pushed();
            let j = index - stored_len;
            if j >= pushed.len() {
                return Ok(None);
            }
            Ok(pushed.get(j).map(Cow::Borrowed))
        } else {
            Ok(self.read_(index, mmap)?.map(Cow::Owned))
        }
    }

    #[inline]
    fn len_(&self) -> usize {
        self.stored_len() + self.pushed_len()
    }

    fn index_to_name(&self) -> String {
        format!("{}_to_{}", I::to_string(), self.name())
    }

    fn stored_len(&self) -> usize;

    fn pushed(&self) -> &[T];
    #[inline]
    fn pushed_len(&self) -> usize {
        self.pushed().len()
    }
    fn mut_pushed(&mut self) -> &mut Vec<T>;
    #[inline]
    fn push(&mut self, value: T) {
        self.mut_pushed().push(value)
    }

    fn header(&self) -> &Header;
    fn mut_header(&mut self) -> &mut Header;

    fn parent(&self) -> &Path;

    fn folder(&self) -> PathBuf {
        self.parent().join(self.name())
    }

    fn folder_(parent: &Path, name: &str) -> PathBuf {
        parent.join(name)
    }

    fn path(&self) -> PathBuf {
        Self::path_(self.parent(), self.name())
    }

    fn path_(parent: &Path, name: &str) -> PathBuf {
        Self::folder_(parent, name).join(I::to_string())
    }

    // ---

    fn open_file(&self) -> io::Result<File> {
        Self::open_file_(&self.path())
    }
    fn open_file_(path: &Path) -> io::Result<File> {
        let mut file = OpenOptions::new()
            .read(true)
            .create(true)
            .write(true)
            .truncate(false)
            .open(path)?;
        file.seek(SeekFrom::End(0))?;
        Ok(file)
    }

    fn file_set_len(&mut self, file: &mut File, len: u64) -> Result<()> {
        Self::file_set_len_(file, len)?;
        Ok(())
    }
    fn file_set_len_(file: &mut File, len: u64) -> Result<()> {
        file.set_len(len)?;
        file.seek(SeekFrom::End(0))?;
        Ok(())
    }

    fn file_write_all(&mut self, file: &mut File, buf: &[u8]) -> Result<()> {
        file.write_all(buf)?;
        file.flush()?;
        Ok(())
    }

    fn file_truncate_and_write_all(&mut self, file: &mut File, len: u64, buf: &[u8]) -> Result<()> {
        Self::file_set_len_(file, len)?;
        self.file_write_all(file, buf)
    }

    fn reset(&mut self) -> Result<()>;

    #[inline]
    fn reset_(&mut self) -> Result<()> {
        let mut file = self.open_file()?;
        self.file_truncate_and_write_all(&mut file, HEADER_OFFSET as u64, &[])
    }

    #[inline]
    fn create_mmap(&self) -> Result<Mmap> {
        let file = self.open_file()?;
        unsafe { Mmap::map(&file).map_err(|e| e.into()) }
    }

    #[inline]
    fn is_pushed_empty(&self) -> bool {
        self.pushed_len() == 0
    }

    #[inline]
    fn has(&self, index: I) -> Result<bool> {
        Ok(self.has_(index.to_usize()?))
    }
    #[inline]
    fn has_(&self, index: usize) -> bool {
        index < self.len_()
    }

    fn flush(&mut self) -> Result<()>;

    fn truncate_if_needed(&mut self, index: I) -> Result<()>;
}
