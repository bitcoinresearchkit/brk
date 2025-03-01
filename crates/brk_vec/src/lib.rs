#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("main.rs")]
#![doc = "```"]

use std::{
    cmp::Ordering,
    error,
    fmt::Debug,
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    marker::PhantomData,
    mem,
    ops::{Add, Range, Sub},
    path::{Path, PathBuf},
    sync::OnceLock,
};

use brk_exit::Exit;
pub use memmap2;
use rayon::prelude::*;
pub use zerocopy;

mod enums;
mod structs;
mod traits;

pub use enums::*;
pub use structs::*;
pub use traits::*;

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
    version: Version,
    pathbuf: PathBuf,
    file: File,
    /// **Number of values NOT number of bytes**
    file_len: usize,
    file_position: u64,
    buf: Vec<u8>,
    mmaps: Vec<OnceLock<Box<memmap2::Mmap>>>, // Boxed Mmap to reduce the size of the Lock (from 24 to 16)
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
const ONE_MB: usize = 1024 * 1024;
// const MAX_CACHE_SIZE: usize = usize::MAX;
const MAX_CACHE_SIZE: usize = 100 * ONE_MB;
const FLUSH_EVERY: usize = 10_000;

impl<I, T> StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
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

        let version_path = Self::path_version_(path);
        version.validate(version_path.as_ref())?;
        version.write(version_path.as_ref())?;

        let file = Self::open_file_(&Self::path_vec_(path))?;

        let mut slf = Self {
            version,
            pathbuf: path.to_owned(),
            file_position: 0,
            file_len: Self::read_disk_len_(&file)?,
            file,
            buf: Self::create_buffer(),
            mmaps: vec![],
            pushed: vec![],
            // updated: BTreeMap::new(),
            // inserted: BTreeMap::new(),
            // removed: BTreeSet::new(),
            phantom: PhantomData,
            // min: AtomicUsize::new(usize::MAX),
            // opened_mmaps: AtomicUsize::new(0),
        };

        slf.reset_file_metadata()?;

        Ok(slf)
    }

    #[inline]
    fn create_buffer() -> Vec<u8> {
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

    pub fn open_then_read(&self, index: I) -> Result<T> {
        self.open_then_read_(Self::i_to_usize(index)?)
    }
    fn open_then_read_(&self, index: usize) -> Result<T> {
        let mut file = self.open_file()?;
        Self::seek_(&mut file, Self::index_to_byte_index(index))?;
        let mut buf = Self::create_buffer();
        Self::read_exact(&mut file, &mut buf).map(|v| v.to_owned())
    }

    fn read_disk_len(&self) -> io::Result<usize> {
        Self::read_disk_len_(&self.file)
    }
    fn read_disk_len_(file: &File) -> io::Result<usize> {
        Ok(Self::byte_index_to_index(file.metadata()?.len() as usize))
    }

    fn reset_file_metadata(&mut self) -> io::Result<()> {
        self.file_len = self.read_disk_len()?;
        self.file_position = self.file.seek(SeekFrom::Start(0))?;
        Ok(())
    }

    pub fn reset_mmaps(&mut self) -> io::Result<()> {
        self.mmaps.par_iter_mut().for_each(|lock| {
            lock.take();
        });

        let len = (self.file_len as f64 / Self::PER_PAGE as f64).ceil() as usize;
        let len = Self::CACHE_LENGTH.min(len);

        if self.mmaps.len() != len {
            self.mmaps.resize_with(len, Default::default);
        }

        Ok(())
    }

    #[inline]
    fn seek(&mut self, byte_index: u64) -> io::Result<u64> {
        self.file.seek(SeekFrom::Start(byte_index))
    }
    #[inline]
    fn seek_(file: &mut File, byte_index: u64) -> io::Result<u64> {
        file.seek(SeekFrom::Start(byte_index))
    }

    fn read_exact<'a>(file: &'a mut File, buf: &'a mut [u8]) -> Result<&'a T> {
        file.read_exact(buf)?;
        let v = T::try_ref_from_bytes(&buf[..])?;
        Ok(v)
    }

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
        let min_page_index = (max_page_index + 1) - self.mmaps.len();

        // let min_open_page = self.min.load(AtomicOrdering::SeqCst);

        // if self.min.load(AtomicOrdering::SeqCst) {
        //     self.min.set(value)
        // }

        if !self.mmaps.is_empty() && page_index >= min_page_index {
            let mmap = &**self
                .mmaps
                .get(page_index - min_page_index)
                .ok_or(Error::MmapsVecIsTooSmall)?
                .get_or_init(|| {
                    Box::new(unsafe {
                        memmap2::MmapOptions::new()
                            .len(Self::PAGE_SIZE)
                            .offset((page_index * Self::PAGE_SIZE) as u64)
                            .map(&self.file)
                            .unwrap()
                    })
                });

            let range = Self::index_to_byte_range(index);
            let slice = &mmap[range];
            return Ok(Some(Value::Ref(T::try_ref_from_bytes(slice)?)));
        }

        Ok(self.open_then_read_(index).map_or(None, |v| Some(Value::Owned(v))))
    }

    #[inline]
    pub fn read(&mut self, index: I) -> Result<Option<&T>> {
        self.read_(Self::i_to_usize(index)?)
    }
    #[inline]
    pub fn read_(&mut self, index: usize) -> Result<Option<&T>> {
        let byte_index = Self::index_to_byte_index(index);
        if self.file_position != byte_index {
            self.file_position = self.seek(Self::index_to_byte_index(index))?;
        }
        match Self::read_exact(&mut self.file, &mut self.buf) {
            Ok(value) => {
                self.file_position += Self::SIZE_OF_T as u64;
                Ok(Some(value))
            }
            Err(e) => Err(e),
        }
    }

    fn read_last(&mut self) -> Result<Option<&T>> {
        let len = self.len();
        if len == 0 {
            return Ok(None);
        }
        self.read_(len - 1)
    }

    pub fn iter<F>(&self, f: F) -> Result<()>
    where
        F: FnMut((I, &T)) -> Result<()>,
    {
        self.iter_from(I::default(), f)
    }

    pub fn iter_from<F>(&self, mut index: I, mut f: F) -> Result<()>
    where
        F: FnMut((I, &T)) -> Result<()>,
    {
        let mut file = self.open_file()?;

        let disk_len = I::from(Self::read_disk_len_(&file)?);
        let pushed_len = I::from(self.pushed_len());

        Self::seek_(&mut file, Self::index_to_byte_index(Self::i_to_usize(index)?))?;

        let mut buf = Self::create_buffer();

        while index < disk_len {
            f((index, Self::read_exact(&mut file, &mut buf)?))?;
            index = index + 1;
        }

        let disk_len = Self::i_to_usize(disk_len)?;
        let mut i = I::default();
        while i < pushed_len {
            f((i + disk_len, self.pushed.get(Self::i_to_usize(i)?).as_ref().unwrap()))?;
            i = i + 1;
        }

        Ok(())
    }

    pub fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<T>> {
        if !self.pushed.is_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let mut file = self.open_file()?;

        let len = Self::read_disk_len_(&file)?;

        let from = from.map_or(0, |from| {
            if from >= 0 {
                from as usize
            } else {
                (len as i64 + from) as usize
            }
        });

        let to = to.map_or(len, |to| {
            if to >= 0 {
                to as usize
            } else {
                (len as i64 + to) as usize
            }
        });

        if from >= to {
            return Err(Error::RangeFromAfterTo);
        }

        Self::seek_(&mut file, Self::index_to_byte_index(from))?;

        let mut buf = Self::create_buffer();

        Ok((from..to)
            .map(|_| Self::read_exact(&mut file, &mut buf).map(|v| v.to_owned()).unwrap())
            .collect::<Vec<_>>())
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.pushed.push(value)
    }

    #[inline]
    pub fn push_if_needed(&mut self, index: I, value: T) -> Result<()> {
        match self.len().cmp(&Self::i_to_usize(index)?) {
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
    }

    #[inline]
    fn push_and_flush_if_needed(&mut self, index: I, value: T, exit: &Exit) -> Result<()> {
        match self.len().cmp(&Self::i_to_usize(index)?) {
            Ordering::Less => {
                return Err(Error::IndexTooHigh);
            }
            ord => {
                if ord == Ordering::Greater {
                    self.safe_truncate_if_needed(index, exit)?;
                }
                self.pushed.push(value);
            }
        }

        if self.pushed_len() >= FLUSH_EVERY {
            Ok(self.safe_flush(exit)?)
        } else {
            Ok(())
        }
    }

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

    pub fn flush(&mut self) -> io::Result<()> {
        if self.pushed.is_empty() {
            return Ok(());
        }

        let mut bytes: Vec<u8> = vec![0; self.pushed_len() * Self::SIZE_OF_T];

        let unsafe_bytes = UnsafeSlice::new(&mut bytes);

        mem::take(&mut self.pushed)
            .into_par_iter()
            .enumerate()
            .for_each(|(i, v)| unsafe_bytes.copy_slice(i * Self::SIZE_OF_T, v.as_bytes()));

        self.file.write_all(&bytes)?;

        self.reset_file_metadata()?;

        Ok(())
    }

    pub fn safe_flush(&mut self, exit: &Exit) -> io::Result<()> {
        if exit.triggered() {
            return Ok(());
        }
        exit.block();
        self.flush()?;
        exit.release();
        Ok(())
    }

    pub fn reset_file(&mut self) -> Result<()> {
        self.truncate_if_needed(I::from(0))?;
        Ok(())
    }

    fn validate_computed_version_or_reset_file(&mut self, version: Version) -> Result<()> {
        let path = self.path_computed_version();
        if version.validate(path.as_ref()).is_err() {
            self.reset_file()?;
        }
        version.write(path.as_ref())?;
        Ok(())
    }

    pub fn truncate_if_needed(&mut self, index: I) -> Result<Option<T>> {
        let index = Self::i_to_usize(index)?;

        if index >= self.file_len {
            return Ok(None);
        }

        let value_at_index = self.open_then_read_(index).ok();

        self.file.set_len(Self::index_to_byte_index(index))?;

        self.reset_file_metadata()?;

        Ok(value_at_index)
    }
    pub fn safe_truncate_if_needed(&mut self, index: I, exit: &Exit) -> Result<()> {
        if exit.triggered() {
            return Ok(());
        }
        exit.block();
        self.truncate_if_needed(index)?;
        exit.release();
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

    pub fn file_name(&self) -> String {
        self.path().file_name().unwrap().to_str().unwrap().to_owned()
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

    #[inline]
    fn path_computed_version(&self) -> PathBuf {
        self.path().join("computed_version")
    }

    pub fn index_type_to_string(&self) -> &str {
        std::any::type_name::<I>()
    }

    pub fn compute_transform<A, F>(
        &mut self,
        max_from: I,
        other: &mut StorableVec<I, A>,
        t: F,
        exit: &Exit,
    ) -> Result<()>
    where
        A: StoredType,
        F: Fn(&A) -> T,
    {
        self.validate_computed_version_or_reset_file(Version::from(0) + self.version + other.version)?;

        let index = max_from.min(I::from(self.len()));
        other.iter_from(index, |(i, a)| self.push_and_flush_if_needed(i, t(a), exit))?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_inverse_more_to_less(
        &mut self,
        max_from: T,
        other: &mut StorableVec<T, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType + StoredIndex,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(Version::from(0) + self.version + other.version)?;

        let index = max_from.min(self.read_last()?.cloned().unwrap_or_default());
        other.iter_from(index, |(v, i)| self.push_and_flush_if_needed(*i, v, exit))?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_inverse_less_to_more(
        &mut self,
        max_from: T,
        first_indexes: &mut StorableVec<T, I>,
        last_indexes: &mut StorableVec<T, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType,
        T: StoredIndex,
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version + first_indexes.version + last_indexes.version,
        )?;

        let index = max_from.min(T::from(self.len()));
        first_indexes.iter_from(index, |(value, first_index)| {
            let first_index = Self::i_to_usize(*first_index)?;
            let last_index = Self::i_to_usize(*last_indexes.read(value)?.unwrap())?;
            (first_index..last_index).try_for_each(|index| self.push_and_flush_if_needed(I::from(index), value, exit))
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_last_index_from_first(
        &mut self,
        max_from: I,
        first_indexes: &mut StorableVec<I, T>,
        final_len: usize,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Copy + From<usize> + Sub<T, Output = T> + StoredIndex,
    {
        self.validate_computed_version_or_reset_file(Version::from(0) + self.version + first_indexes.version)?;

        let index = max_from.min(I::from(self.len()));
        let one = T::from(1);
        let mut prev_index: Option<I> = None;
        first_indexes.iter_from(index, |(i, v)| {
            if let Some(prev_index) = prev_index {
                self.push_and_flush_if_needed(prev_index, *v - one, exit)?;
            }
            prev_index.replace(i);
            Ok(())
        })?;
        if let Some(prev_index) = prev_index {
            self.push_and_flush_if_needed(prev_index, T::from(final_len) - one, exit)?;
        }

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_count_from_indexes<T2>(
        &mut self,
        max_from: I,
        first_indexes: &mut StorableVec<I, T2>,
        last_indexes: &mut StorableVec<I, T2>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<T2>,
        T2: StoredType + Copy + Add<usize, Output = T2> + Sub<T2, Output = T2> + TryInto<T>,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version + first_indexes.version + last_indexes.version,
        )?;

        let index = max_from.min(I::from(self.len()));
        first_indexes.iter_from(index, |(i, first_index)| {
            let last_index = last_indexes.read(i)?.unwrap();
            let count = *last_index + 1_usize - *first_index;
            self.push_and_flush_if_needed(i, count.into(), exit)
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_is_first_ordered<A>(
        &mut self,
        max_from: I,
        self_to_other: &mut StorableVec<I, A>,
        other_to_self: &mut StorableVec<A, I>,
        exit: &Exit,
    ) -> Result<()>
    where
        I: StoredType,
        T: From<bool>,
        A: StoredIndex + StoredType,
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version + self_to_other.version + other_to_self.version,
        )?;

        let index = max_from.min(I::from(self.len()));
        self_to_other.iter_from(index, |(i, other)| {
            self.push_and_flush_if_needed(i, T::from(other_to_self.read(*other)?.unwrap() == &i), exit)
        })?;

        Ok(self.safe_flush(exit)?)
    }

    pub fn compute_sum_from_indexes<T2, F>(
        &mut self,
        max_from: I,
        first_indexes: &mut StorableVec<I, T2>,
        last_indexes: &mut StorableVec<I, T2>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: From<T2>,
        T2: StoredType + Copy + Add<usize, Output = T2> + Sub<T2, Output = T2> + TryInto<T>,
        <T2 as TryInto<T>>::Error: error::Error + 'static,
        F: Fn(&T2) -> T,
    {
        self.validate_computed_version_or_reset_file(
            Version::from(0) + self.version + first_indexes.version + last_indexes.version,
        )?;

        let index = max_from.min(I::from(self.len()));
        first_indexes.iter_from(index, |(index, first_index)| {
            let last_index = last_indexes.read(index)?.unwrap();
            let count = *last_index + 1_usize - *first_index;
            self.push_and_flush_if_needed(index, count.into(), exit)
        })?;

        Ok(self.safe_flush(exit)?)
    }
}

impl<I, T> Clone for StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn clone(&self) -> Self {
        let path = &self.pathbuf;
        let path_version = Self::path_version_(path);
        let version = Version::try_from(path_version.as_path()).unwrap();
        Self::import(path, version).unwrap()
    }
}
