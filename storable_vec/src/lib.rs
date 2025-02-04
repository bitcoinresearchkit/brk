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

mod enums;
mod structs;
mod traits;

pub use enums::*;
pub use structs::*;
pub use traits::*;

type Buffer = Vec<u8>;

/// Uses `Mmap` instead of `File`
///
/// Used in `/indexer`
pub const CACHED_GETS: u8 = 0;

/// Will use the same `File` for every read, so not thread safe
///
/// Used in `/computer`
pub const SINGLE_THREAD: u8 = 1;

/// Will spin up a new `File` for every read
///
/// Used in `/server`
pub const ASYNC_READ_ONLY: u8 = 2;

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
pub struct StorableVec<I, T, const MODE: u8> {
    pathbuf: PathBuf,
    file: File,
    /// **Number of values NOT number of bytes**
    file_len: usize,
    /// Only for SINGLE_THREAD
    file_position: u64,
    buf: Buffer,
    /// Only for CACHED_GETS
    cache: Vec<OnceLock<Box<Mmap>>>, // Boxed Mmap to reduce the size of the Lock (from 24 to 16)
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

impl<I, T, const MODE: u8> StorableVec<I, T, MODE>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    pub const SIZE_OF_T: usize = size_of::<T>();
    pub const PER_PAGE: usize = MAX_PAGE_SIZE / Self::SIZE_OF_T;
    /// In bytes
    pub const PAGE_SIZE: usize = Self::PER_PAGE * Self::SIZE_OF_T;
    pub const CACHE_LENGTH: usize = MAX_CACHE_SIZE / Self::PAGE_SIZE;

    /// Same as import but will remove the folder if the endian or the version is different, so be careful !
    pub fn forced_import(path: &Path, version: Version) -> Result<Self> {
        let res = Self::import(path, version);
        match res {
            Err(Error::WrongEndian) | Err(Error::DifferentVersion { found: _, expected: _ }) => {
                fs::remove_dir_all(path)?;
                Self::import(path, version)
            }
            _ => res,
        }
    }

    pub fn import(path: &Path, version: Version) -> Result<Self> {
        fs::create_dir_all(path)?;

        let path_version = Self::path_version_(path);

        if let Ok(prev_version) = Version::try_from(path_version.as_path()) {
            if prev_version != version {
                if prev_version.swap_bytes() == version {
                    return Err(Error::WrongEndian);
                }
                return Err(Error::DifferentVersion {
                    found: prev_version,
                    expected: version,
                });
            }
        }
        version.write(&path_version)?;

        let file = Self::open_file_(&Self::path_vec_(path))?;

        let mut slf = Self {
            pathbuf: path.to_owned(),
            file_position: 0,
            file_len: Self::read_disk_len_(&file)?,
            file,
            buf: Self::create_buffer(),
            cache: vec![],
            pushed: vec![],
            // updated: BTreeMap::new(),
            // inserted: BTreeMap::new(),
            // removed: BTreeSet::new(),
            phantom: PhantomData,
            // min: AtomicUsize::new(usize::MAX),
            // opened_mmaps: AtomicUsize::new(0),
        };

        slf.reset_disk_related_state()?;

        Ok(slf)
    }

    #[inline]
    fn create_buffer() -> Buffer {
        vec![0; Self::SIZE_OF_T]
    }

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

    fn read_disk_len(&self) -> io::Result<usize> {
        Self::read_disk_len_(&self.file)
    }
    fn read_disk_len_(file: &File) -> io::Result<usize> {
        Ok(Self::byte_index_to_index(file.metadata()?.len() as usize))
    }

    fn reset_disk_related_state(&mut self) -> io::Result<()> {
        self.file = self.open_file()?;
        self.file_len = self.read_disk_len()?;
        self.file_position = 0;
        self.reset_cache()
    }

    fn reset_cache(&mut self) -> io::Result<()> {
        match MODE {
            CACHED_GETS => {
                // par_iter_mut ?
                self.cache.iter_mut().for_each(|lock| {
                    lock.take();
                });

                let len = (self.file_len as f64 / Self::PER_PAGE as f64).ceil() as usize;
                let len = Self::CACHE_LENGTH.min(len);

                if self.cache.len() != len {
                    self.cache.resize_with(len, Default::default);
                    self.cache.shrink_to_fit();
                }

                Ok(())
            }
            _ => Ok(()),
        }
    }

    // #[inline]
    // fn open_file_at_then_read(&self, index: I) -> Result<T> {
    //     self.open_file_at_then_read_(Self::i_to_usize(index)?)
    // }
    fn open_file_at_then_read(&self, index: usize) -> Result<T> {
        // let (mut file, mut buf) = self.open_file_at(index)?;
        let mut file = self.open_file()?;
        let mut buf = Self::create_buffer();

        let byte_index = Self::index_to_byte_index(index);
        Self::seek(&mut file, byte_index)?;

        Ok(Self::read_exact(&mut file, &mut buf)?.to_owned())
    }
    // #[inline]
    // fn open_file_at(&self, index: I) -> Result<(File, Buffer)> {
    //     self.open_file_at_(Self::i_to_usize(index)?)
    // }
    // fn open_file_at(&self, index: usize) -> Result<(File, Buffer)> {
    //     let mut file = self.open_file()?;
    //     let buf = Self::create_buffer();
    //     let byte_index = Self::index_to_byte_index(index);
    //     Self::seek(&mut file, byte_index)?;
    //     Ok((file, buf))
    // }
    // #[inline]
    // fn seek_if_needed_(file: &mut File, index: I) -> Result<u64> {
    //     Self::seek_if_needed__(file, Self::i_to_usize(index)?).map_err(Error::IO)
    // }
    // #[inline]
    // fn seek_if_needed(file: &mut File, index: usize) -> io::Result<u64> {
    //     let byte_index = Self::index_to_byte_index(index);
    //     if file.stream_position()? != byte_index {
    //         Self::seek(file, byte_index)?;
    //     }
    //     Ok(byte_index)
    // }
    #[inline]
    fn seek(file: &mut File, byte_index: u64) -> io::Result<u64> {
        file.seek(SeekFrom::Start(byte_index))
    }
    fn read_exact<'a>(file: &'a mut File, buf: &'a mut [u8]) -> Result<&'a T> {
        file.read_exact(buf)?;
        let v = T::unsafe_try_from_slice(&buf[..])?;
        Ok(v)
    }

    #[inline]
    fn push_(&mut self, value: T) {
        self.pushed.push(value)
    }

    #[inline]
    fn push_if_needed_(&mut self, index: I, value: T) -> Result<()> {
        match self.pushed_len().cmp(&Self::i_to_usize(index)?) {
            Ordering::Greater => {
                // dbg!(len, index, &self.pathbuf);
                // panic!();
                Ok(())
            }
            Ordering::Equal => {
                self.pushed.push(value);
                Ok(())
            }
            Ordering::Less => {
                dbg!(index, value);
                Err(Error::IndexTooHigh)
            }
        }
        // Self::push_to_vec_if_needed(&mut self.pushed, index, value)
    }
    // #[inline]
    // fn push_if_needed__(&mut self, index: usize, value: T) -> Result<()> {
    //     Self::push_to_vec_if_needed_(&mut self.pushed, index, value)
    // }
    // #[inline]
    // fn push_to_vec_if_needed(vec: &mut Vec<T>, index: I, value: T) -> Result<()> {
    //     Self::push_to_vec_if_needed_(vec, Self::i_to_usize(index)?, value)
    // }
    // fn push_to_vec_if_needed_(vec: &mut Vec<T>, index: usize, value: T) -> Result<()> {
    //     let len = vec.len();
    //     match len.cmp(&index) {
    //         Ordering::Greater => {
    //             // dbg!(len, index, &self.pathbuf);
    //             // panic!();
    //             Ok(())
    //         }
    //         Ordering::Equal => {
    //             vec.push(value);
    //             Ok(())
    //         }
    //         Ordering::Less => {
    //             dbg!(index, value);
    //             Err(Error::IndexTooHigh)
    //         }
    //     }
    // }

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

    #[inline]
    pub fn len(&self) -> usize {
        self.file_len + self.pushed_len()
    }

    #[inline]
    pub fn pushed_len(&self) -> usize {
        self.pushed.len()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn has(&self, index: I) -> Result<bool> {
        Ok(self.has_(Self::i_to_usize(index)?))
    }
    #[inline]
    fn has_(&self, index: usize) -> bool {
        index < self.len()
    }

    #[inline]
    pub fn hasnt(&self, index: I) -> Result<bool> {
        self.has(index).map(|b| !b)
    }
    #[inline]
    fn hasnt_(&self, index: usize) -> bool {
        !self.has_(index)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.reset_disk_related_state()?;

        if self.pushed.is_empty() {
            return Ok(());
        }

        self.file_len += self.pushed.len();

        let mut bytes: Vec<u8> = vec![];

        mem::take(&mut self.pushed)
            .into_iter()
            .for_each(|v| bytes.extend_from_slice(v.unsafe_as_slice()));

        self.file.write_all(&bytes)?;

        Ok(())
    }

    #[inline]
    fn i_to_usize(index: I) -> Result<usize> {
        index.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)
    }

    #[inline]
    fn byte_index_to_index(byte_index: usize) -> usize {
        byte_index / Self::SIZE_OF_T
    }

    #[inline]
    fn index_to_byte_index(index: usize) -> u64 {
        (index * Self::SIZE_OF_T) as u64
    }

    #[inline]
    fn index_to_byte_range(index: usize) -> Range<usize> {
        let index = (Self::index_to_byte_index(index) as usize) % Self::PAGE_SIZE;
        index..(index + Self::SIZE_OF_T)
    }

    fn index_to_pushed_index(&self, index: usize) -> Result<Option<usize>> {
        if index >= self.file_len {
            let index = index - self.file_len;
            if index >= self.pushed.len() {
                Err(Error::IndexTooHigh)
            } else {
                Ok(Some(index))
            }
        } else {
            Err(Error::IndexTooLow)
        }
    }

    #[inline]
    pub fn path(&self) -> &Path {
        &self.pathbuf
    }

    #[inline]
    fn path_vec(&self) -> PathBuf {
        Self::path_vec_(&self.pathbuf)
    }
    #[inline]
    fn path_vec_(path: &Path) -> PathBuf {
        path.join("vec")
    }

    #[inline]
    fn path_version_(path: &Path) -> PathBuf {
        path.join("version")
    }
}

impl<I, T> StorableVec<I, T, CACHED_GETS>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    #[inline]
    pub fn get(&self, index: I) -> Result<Option<Value<'_, T>>> {
        self.get_(Self::i_to_usize(index)?)
    }
    fn get_(&self, index: usize) -> Result<Option<Value<'_, T>>> {
        match self.index_to_pushed_index(index) {
            Ok(index) => {
                if let Some(index) = index {
                    return Ok(self.pushed.get(index).map(|v| Value::Ref(v)));
                }
            }
            Err(Error::IndexTooHigh) => return Ok(None),
            Err(Error::IndexTooLow) => {}
            Err(error) => return Err(error),
        }

        // if !self.updated.is_empty() {
        //     if let Some(v) = self.updated.get(&index) {
        //         return Ok(Some(v));
        //     }
        // }

        let page_index = index / Self::PER_PAGE;
        let last_index = self.file_len - 1;
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
                            .map(&self.file)
                            .unwrap()
                    })
                });

            let range = Self::index_to_byte_range(index);
            let slice = &mmap[range];
            return Ok(Some(Value::Ref(T::unsafe_try_from_slice(slice)?)));
        }

        Ok(Some(Value::Owned(self.open_file_at_then_read(index)?.to_owned())))
    }

    pub fn get_or_default(&self, index: I) -> Result<T>
    where
        T: Default + Clone,
    {
        Ok(self.get(index)?.map(|v| (*v).clone()).unwrap_or(Default::default()))
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.push_(value)
    }

    #[inline]
    pub fn push_if_needed(&mut self, index: I, value: T) -> Result<()> {
        self.push_if_needed_(index, value)
    }
}

const FLUSH_EVERY: usize = 10_000;
impl<I, T> StorableVec<I, T, SINGLE_THREAD>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    pub fn get(&mut self, index: I) -> Result<&T> {
        self.get_(Self::i_to_usize(index)?)
    }
    fn get_(&mut self, index: usize) -> Result<&T> {
        let byte_index = Self::index_to_byte_index(index);
        if self.file_position != byte_index {
            self.file_position = Self::seek(&mut self.file, byte_index)?;
        }
        let res = Self::read_exact(&mut self.file, &mut self.buf);
        if res.is_ok() {
            self.file_position += Self::SIZE_OF_T as u64;
        }
        res
    }

    pub fn last(&mut self) -> Result<Option<&T>> {
        let len = self.len();
        if len == 0 {
            return Ok(None);
        }
        Ok(self.get_(len - 1).ok())
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.push_(value)
    }

    #[inline]
    pub fn push_if_needed(&mut self, index: I, value: T) -> Result<()> {
        self.push_if_needed_(index, value)?;

        if self.pushed_len() >= FLUSH_EVERY {
            Ok(self.flush()?)
        } else {
            Ok(())
        }
    }

    // #[inline]
    // fn seek_if_needed(&mut self, index: I) -> Result<()> {
    //     if self.file_position == Self::index_to_byte_index(Self::i_to_usize(index)?) {
    //         return Ok(());
    //     }
    //     self.file_position = Self::seek_if_needed_(&mut self.file, index)?;
    //     Ok(())
    // }

    pub fn iter<F>(&mut self, f: F) -> Result<()>
    where
        F: FnMut((I, &T)) -> Result<()>,
    {
        self.iter_from(I::default(), f)
    }

    pub fn iter_from<F>(&mut self, mut index: I, mut f: F) -> Result<()>
    where
        F: FnMut((I, &T)) -> Result<()>,
    {
        // let pushed_len = self.pushed_len();

        // self.seek_if_needed(index)?;

        if !self.pushed.is_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let disk_len = I::from(Self::read_disk_len_(&self.file)?);

        while index < disk_len {
            f((index, self.get(index)?))?;
            index = index + 1;
        }

        // i = 0;
        // while i < pushed_len {
        //     f((I::from(i + disk_len), self.pushed.get(i).as_ref().unwrap()))?;
        //     i += 1;
        // }

        Ok(())
    }

    pub fn compute_inverse_more_to_less(&mut self, other: &mut StorableVec<T, I, SINGLE_THREAD>) -> Result<()>
    where
        I: StorableVecType,
        T: StorableVecIndex,
    {
        let index = self.last()?.cloned().unwrap_or_default();
        other.iter_from(index, |(v, i)| self.push_if_needed(*i, v))?;
        Ok(self.flush()?)
    }

    pub fn compute_inverse_less_to_more(
        &mut self,
        first_indexes: &mut StorableVec<T, I, SINGLE_THREAD>,
        last_indexes: &mut StorableVec<T, I, SINGLE_THREAD>,
    ) -> Result<()>
    where
        I: StorableVecType,
        T: StorableVecIndex,
    {
        first_indexes.iter_from(T::from(self.len()), |(value, first_index)| {
            let first_index = Self::i_to_usize(*first_index)?;
            let last_index = Self::i_to_usize(*last_indexes.get(value)?)?;
            (first_index..last_index).try_for_each(|index| self.push_if_needed(I::from(index), value))
        })?;
        Ok(self.flush()?)
    }

    pub fn compute_transform<A, F>(&mut self, other: &mut StorableVec<I, A, SINGLE_THREAD>, t: F) -> Result<()>
    where
        A: StorableVecType,
        F: Fn(&A) -> T,
    {
        other.iter_from(I::from(self.len()), |(i, a)| self.push_if_needed(i, t(a)))?;
        Ok(self.flush()?)
    }
}

impl<I, T> StorableVec<I, T, ASYNC_READ_ONLY>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    #[inline]
    pub fn get(&self, index: I) -> Result<Option<Value<'_, T>>> {
        self.get_(Self::i_to_usize(index)?)
    }
    #[inline]
    fn get_(&self, index: usize) -> Result<Option<Value<'_, T>>> {
        Ok(Some(Value::Owned(self.open_file_at_then_read(index)?.to_owned())))
    }

    // Add iter iter_from iter_range collect..
    // + add memory cap
}
