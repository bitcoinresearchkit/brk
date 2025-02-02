use std::{
    cmp::Ordering,
    fmt::Debug,
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    marker::PhantomData,
    mem,
    ops::Range,
    path::{Path, PathBuf},
    sync::OnceLock,
};

use memmap2::{Mmap, MmapOptions};
use unsafe_slice_serde::UnsafeSliceSerde;

mod any;
// mod bytes;
mod error;
mod index;
mod type_;
mod value;
mod version;

pub use any::*;
// pub use bytes::*;
pub use error::*;
pub use index::*;
pub use type_::*;
pub use value::*;
pub use version::*;

///
/// A very small, fast, efficient and simple storable Vec
///
/// Reads (imports of Mmap) are lazy
///
/// Stores only raw data without any overhead, and doesn't even have a header (TODO: which it should, at least to Err if wrong endian)
///
/// The file isn't portable for speed reasons (TODO: but could be ?)
///
/// If you don't call `.flush()` it just acts as a normal Vec
///
#[derive(Debug)]
pub struct StorableVec<I, T> {
    pathbuf: PathBuf,
    unsafe_file: File,
    cache: Vec<OnceLock<Box<Mmap>>>, // Boxed Mmap to reduce the size of the Lock (from 24 to 16)
    disk_len: usize,
    pushed: Vec<T>,
    // updated: BTreeMap<usize, T>,
    // inserted: BTreeMap<usize, T>,
    // removed: BTreeSet<usize>,
    // min: AtomicUsize,
    // opened_mmaps: AtomicUsize,
    phantom: PhantomData<I>,
}

/// In bytes
const MAX_PAGE_SIZE: usize = 4 * 4096;
const ONE_MB: usize = 1000 * 1024;
const MAX_CACHE_SIZE: usize = 100 * ONE_MB;
// const MAX_CACHE_SIZE: usize = 100 * ONE_MB;

impl<I, T> StorableVec<I, T>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    pub const SIZE_OF_T: usize = size_of::<T>();
    pub const PER_PAGE: usize = MAX_PAGE_SIZE / Self::SIZE_OF_T;
    /// In bytes
    pub const PAGE_SIZE: usize = Self::PER_PAGE * Self::SIZE_OF_T;
    pub const CACHE_LENGTH: usize = MAX_CACHE_SIZE / Self::PAGE_SIZE;

    pub fn import(path: &Path, version: Version) -> Result<Self, io::Error> {
        fs::create_dir_all(path)?;

        let path_version = Self::path_version_(path);
        let is_same_version =
            Version::try_from(path_version.as_path()).is_ok_and(|prev_version| version == prev_version);
        if !is_same_version {
            fs::remove_dir_all(path)?;
        }
        version.write(&path_version)?;

        let unsafe_file = Self::open_file_(&Self::path_vec_(path))?;

        let mut this = Self {
            pathbuf: path.to_owned(),
            disk_len: Self::disk_len(&unsafe_file)?,
            unsafe_file,
            cache: vec![],
            pushed: vec![],
            // updated: BTreeMap::new(),
            // inserted: BTreeMap::new(),
            // removed: BTreeSet::new(),
            phantom: PhantomData,
            // min: AtomicUsize::new(usize::MAX),
            // opened_mmaps: AtomicUsize::new(0),
        };

        // TODO: Only if write mode
        this.reset_cache();

        Ok(this)
    }

    pub fn disk_len(file: &File) -> io::Result<usize> {
        Ok(Self::byte_index_to_index(file.metadata()?.len() as usize))
    }

    pub fn reset_cache(&mut self) {
        // par_iter_mut ?
        self.cache.iter_mut().for_each(|lock| {
            lock.take();
        });

        let len = (self.disk_len as f64 / Self::PER_PAGE as f64).ceil() as usize;
        let len = Self::CACHE_LENGTH.min(len);

        if self.cache.len() != len {
            self.cache.resize_with(len, Default::default);
            self.cache.shrink_to_fit();
        }
    }

    fn open_file(&self) -> Result<File, Error> {
        Self::open_file_(&self.path_vec()).map_err(Error::IO)
    }
    fn open_file_(path: &Path) -> Result<File, io::Error> {
        OpenOptions::new()
            .read(true)
            .create(true)
            .truncate(false)
            .append(true)
            .open(path)
    }

    #[inline]
    fn index_to_byte_range(index: usize) -> Range<usize> {
        let index = Self::index_to_byte_index(index) % Self::PAGE_SIZE;
        index..(index + Self::SIZE_OF_T)
    }

    #[inline]
    fn index_to_byte_index(index: usize) -> usize {
        index * Self::SIZE_OF_T
    }

    #[inline]
    fn byte_index_to_index(byte_index: usize) -> usize {
        byte_index / Self::SIZE_OF_T
    }

    fn index_to_pushed_index(&self, index: usize) -> Result<Option<usize>> {
        if index >= self.disk_len {
            let index = index - self.disk_len;
            if index >= self.pushed.len() {
                Err(Error::IndexTooHigh)
            } else {
                Ok(Some(index))
            }
        } else {
            Ok(None)
        }
    }

    #[inline]
    pub fn cached_get(&self, index: I) -> Result<Option<Value<'_, T>>> {
        self.cached_get_(index.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)?)
    }
    fn cached_get_(&self, index: usize) -> Result<Option<Value<'_, T>>> {
        match self.index_to_pushed_index(index) {
            Ok(index) => {
                if let Some(index) = index {
                    return Ok(self.pushed.get(index).map(|v| Value::Ref(v)));
                }
            }
            Err(Error::IndexTooHigh) => return Ok(None),
            Err(error) => return Err(error),
        }

        // if !self.updated.is_empty() {
        //     if let Some(v) = self.updated.get(&index) {
        //         return Ok(Some(v));
        //     }
        // }

        let page_index = index / Self::PER_PAGE;
        let last_index = self.disk_len - 1;
        let max_page_index = last_index / Self::PER_PAGE;
        let min_page_index = (max_page_index + 1).checked_sub(self.cache.len()).unwrap_or_default();

        // let min_open_page = self.min.load(AtomicOrdering::SeqCst);

        // if self.min.load(AtomicOrdering::SeqCst) {
        //     self.min.set(value)
        // }

        if page_index >= min_page_index {
            let mmap = &**self
                .cache
                .get(page_index - min_page_index)
                .ok_or(Error::MmapsVecIsTooSmall)?
                .get_or_init(|| {
                    Box::new(unsafe {
                        MmapOptions::new()
                            .len(Self::PAGE_SIZE)
                            .offset((page_index * Self::PAGE_SIZE) as u64)
                            .map(&self.unsafe_file)
                            .unwrap()
                    })
                });

            let range = Self::index_to_byte_range(index);

            let slice = &mmap[range];

            Ok(Some(Value::Ref(
                T::unsafe_try_from_slice(slice).map_err(Error::UnsafeSliceSerde)?,
            )))
        } else {
            let (mut file, mut buf) = self.prepare_to_read()?;
            Self::seek_(&mut file, index)?;
            let value = self.read_exact(&mut file, &mut buf)?;
            Ok(Some(Value::Owned(value.to_owned())))
        }
    }

    pub fn get_or_default(&self, index: I) -> Result<T>
    where
        T: Default + Clone,
    {
        Ok(self
            .cached_get(index)?
            .map(|v| (*v).clone())
            .unwrap_or(Default::default()))
    }

    pub fn seek(file: &mut File, index: I) -> Result<()> {
        Self::seek_(file, index.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)?)
    }

    pub fn seek_(file: &mut File, index: usize) -> Result<()> {
        let byte_index = Self::index_to_byte_index(index);
        file.seek(SeekFrom::Start(byte_index as u64)).map_err(Error::IO)?;
        Ok(())
    }

    pub fn iter<F>(&self, f: F) -> Result<()>
    where
        F: FnMut((I, &T)) -> Result<()>,
    {
        self.iter_from(I::from(0_usize), f)
    }

    pub fn prepare_to_read(&self) -> Result<(File, Vec<u8>)> {
        let file = self.open_file()?;
        let buf = vec![0; Self::SIZE_OF_T];
        Ok((file, buf))
    }

    pub fn prepare_to_read_at(&self, index: I) -> Result<(File, Vec<u8>)> {
        self.prepare_to_read_at_(index.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)?)
    }
    pub fn prepare_to_read_at_(&self, index: usize) -> Result<(File, Vec<u8>)> {
        let (mut file, buf) = self.prepare_to_read()?;
        Self::seek_(&mut file, index)?;
        Ok((file, buf))
    }

    pub fn read_exact<'a>(&self, file: &'a mut File, buf: &'a mut [u8]) -> Result<&'a T> {
        file.read_exact(buf).map_err(Error::IO)?;
        let v = T::unsafe_try_from_slice(&buf[..]).map_err(Error::UnsafeSliceSerde)?;
        Ok(v)
    }

    pub fn iter_from<F>(&self, index: I, mut f: F) -> Result<()>
    where
        F: FnMut((I, &T)) -> Result<()>,
    {
        let (mut file, mut buf) = self.prepare_to_read()?;
        let disk_len = Self::disk_len(&file).map_err(Error::IO)?;
        Self::seek(&mut file, index)?;

        let mut i: usize = index.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)?;
        while i < disk_len {
            let v = self.read_exact(&mut file, &mut buf)?;
            f((I::from(i), v))?;
            i += 1;
        }
        i = 0;
        while i < self.pushed_len() {
            f((I::from(i + disk_len), self.pushed.get(i).as_ref().unwrap()))?;
            i += 1;
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn first(&self) -> Result<Option<Value<'_, T>>> {
        self.cached_get_(0)
    }

    #[allow(unused)]
    pub fn last(&self) -> Result<Option<Value<'_, T>>> {
        let len = self.len();
        if len == 0 {
            return Ok(None);
        }
        self.cached_get_(len - 1)
    }

    pub fn push(&mut self, value: T) {
        self.pushed.push(value)
    }

    pub fn push_if_needed(&mut self, index: I, value: T) -> Result<()> {
        self.push_if_needed_(index.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)?, value)
    }
    fn push_if_needed_(&mut self, index: usize, value: T) -> Result<()> {
        let len = self.len();
        match len.cmp(&index) {
            Ordering::Greater => {
                // dbg!(len, index, &self.pathbuf);
                // panic!();
                Ok(())
            }
            Ordering::Equal => {
                self.push(value);
                Ok(())
            }
            Ordering::Less => {
                dbg!(index, value);
                Err(Error::IndexTooHigh)
            }
        }
    }

    // pub fn update(&mut self, index: I, value: T) -> Result<()> {
    //     self._update(index.into(), value)
    // }
    // pub fn update_(&mut self, index: usize, value: T) -> Result<()> {
    //     if let Some(index) = self.index_to_pushed_index(index) {
    //         self.pushed[index] = value;
    //     } else {
    //         self.updated.insert(index, value);
    //     }
    //     Ok(())
    // }

    // pub fn fetch_update(&mut self, index: I, value: T) -> Result<T>
    // where
    //     T: Clone,
    // {
    //     self._fetch_update(index.into(), value)
    // }
    // pub fn fetch_update_(&mut self, index: usize, value: T) -> Result<T>
    // where
    //     T: Clone,
    // {
    //     let prev_opt = self.updated.insert(index, value);
    //     if let Some(prev) = prev_opt {
    //         Ok(prev)
    //     } else {
    //         Ok(self
    //             ._get(index)?
    //             .ok_or(Error::ExpectFileToHaveIndex)?
    //             .clone())
    //     }
    // }

    // pub fn remove(&mut self, index: I) {
    //     self._remove(index.into())
    // }
    // pub fn remove_(&mut self, index: usize) {
    //     self.removed.insert(index);
    // }

    pub fn len(&self) -> usize {
        self.disk_len + self.pushed_len()
    }

    pub fn pushed_len(&self) -> usize {
        self.pushed.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn has(&self, index: I) -> Result<bool> {
        Ok(self.has_(index.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)?))
    }
    fn has_(&self, index: usize) -> bool {
        index < self.len()
    }

    pub fn hasnt(&self, index: I) -> Result<bool> {
        Ok(self.hasnt_(index.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)?))
    }
    fn hasnt_(&self, index: usize) -> bool {
        !self.has_(index)
    }

    // pub fn flush(&mut self) -> io::Result<()>
    // where
    //     T: Bytes,
    // {
    //     self.flush_(|bytes, v| bytes.extend_from_slice(&v.to_bytes()))
    // }
    pub fn flush(&mut self) -> io::Result<()> {
        //     self.flush_(|bytes, v| bytes.extend_from_slice(v.unsafe_as_slice()))
        // }
        // fn flush_<F>(&mut self, mut extend: F) -> io::Result<()>
        // where
        //     F: FnMut(&mut Vec<u8>, T),
        // {
        self.reset_cache();

        if self.pushed.is_empty() {
            return Ok(());
        }

        self.disk_len += self.pushed.len();

        let mut bytes: Vec<u8> = vec![];

        mem::take(&mut self.pushed)
            .into_iter()
            .for_each(|v| bytes.extend_from_slice(v.unsafe_as_slice()));
        // .for_each(|v| extend(&mut bytes, v));

        self.unsafe_file.write_all(&bytes)?;

        Ok(())
    }

    pub fn path(&self) -> &Path {
        &self.pathbuf
    }

    fn path_vec(&self) -> PathBuf {
        Self::path_vec_(&self.pathbuf)
    }
    fn path_vec_(path: &Path) -> PathBuf {
        path.join("vec")
    }

    fn path_version_(path: &Path) -> PathBuf {
        path.join("version")
    }
}
