use std::{
    fs::{File, OpenOptions},
    io::{self, Seek, SeekFrom, Write},
    path::{Path, PathBuf},
    sync::Arc,
    time::{self, Duration},
};

use arc_swap::ArcSwap;
use brk_core::{Result, Value};
use memmap2::Mmap;

use super::{StoredIndex, StoredType};

pub trait GenericStoredVec<I, T>: Send + Sync
where
    I: StoredIndex,
    T: StoredType,
{
    const SIZE_OF_T: usize = size_of::<T>();

    #[inline]
    fn read(&self, index: I, mmap: &Mmap) -> Result<Option<T>> {
        self.read_(index.to_usize()?, mmap)
    }
    fn read_(&self, index: usize, mmap: &Mmap) -> Result<Option<T>>;

    #[inline]
    fn get_or_read(&self, index: I, mmap: &Mmap) -> Result<Option<Value<T>>> {
        self.get_or_read_(index.to_usize()?, mmap)
    }
    #[inline]
    fn get_or_read_(&self, index: usize, mmap: &Mmap) -> Result<Option<Value<T>>> {
        let stored_len = self.stored_len_(mmap);

        if index >= stored_len {
            let pushed = self.pushed();
            let j = index - stored_len;
            if j >= pushed.len() {
                return Ok(None);
            }
            Ok(pushed.get(j).map(Value::Ref))
        } else {
            Ok(self.read_(index, mmap)?.map(Value::Owned))
        }
    }

    #[inline]
    fn len_(&self) -> usize {
        self.stored_len() + self.pushed_len()
    }

    fn mmap(&self) -> &ArcSwap<Mmap>;

    fn stored_len(&self) -> usize;
    fn stored_len_(&self, mmap: &Mmap) -> usize;

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

    fn path(&self) -> &Path;

    // ---

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
        self.file_truncate_and_write_all(0, &[])
    }

    fn new_mmap(file: File) -> Result<Arc<Mmap>> {
        Ok(Arc::new(unsafe { Mmap::map(&file)? }))
    }

    fn update_mmap(&mut self, file: File) -> Result<()> {
        let mmap = Self::new_mmap(file)?;
        self.mmap().store(mmap);
        Ok(())
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
    fn name_(&self) -> String {
        self.path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
    }

    fn modified_time_(&self) -> Result<Duration> {
        Ok(self
            .path_vec()
            .metadata()?
            .modified()?
            .duration_since(time::UNIX_EPOCH)?)
    }
}
