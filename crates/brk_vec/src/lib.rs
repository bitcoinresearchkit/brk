#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::{
    cmp::Ordering,
    fmt::Debug,
    fs::{self, File, OpenOptions},
    io::{self, Seek, SeekFrom, Write},
    marker::PhantomData,
    mem,
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub use memmap2;
use memmap2::Mmap;
use rayon::prelude::*;
pub use zerocopy;

mod enums;
mod structs;
mod traits;

pub use enums::*;
pub use structs::*;
pub use traits::*;
use zstd::DEFAULT_COMPRESSION_LEVEL;

const ONE_KIB: usize = 1024;
const MAX_PAGE_SIZE: usize = 16 * ONE_KIB;
const ONE_MIB: usize = ONE_KIB * ONE_KIB;
const MAX_CACHE_SIZE: usize = 100 * ONE_MIB;

type SmallCache<T> = Option<(usize, Box<[T]>)>;

///
/// A very small, fast, efficient and simple storable Vec
///
/// Reads (imports of Mmap) are lazy
///
/// Stores only raw data without any overhead, and doesn't even have a header
///
/// The file isn't portable for speed reasons (TODO: but could be ?)
///
/// If you don't call `.flush()` it just acts as a normal Vec
///
#[derive(Debug)]
pub struct StorableVec<I, T> {
    version: Version,
    pathbuf: PathBuf,
    stored_len: Length,
    compressed: Compressed,

    // Compressed
    decoded_pages: Option<Vec<OnceLock<Box<[T]>>>>,
    decoded_page: SmallCache<T>,
    pages: CompressedPagesMetadata,

    // Raw
    // raw_pages: Vec<OnceLock<Box<memmap2::Mmap>>>,
    // raw_page: memmap2::Mmap,
    // file: File,
    // file_position: u64,
    // buf: Vec<u8>,
    pushed: Vec<T>,
    phantom: PhantomData<I>,
}

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

    /// Same as import but will reset the folder under certain errors, so be careful !
    pub fn forced_import(path: &Path, version: Version, compressed: Compressed) -> Result<Self> {
        let res = Self::import(path, version, compressed);
        match res {
            Err(Error::WrongEndian)
            | Err(Error::DifferentCompressionMode)
            | Err(Error::DifferentVersion {
                found: _,
                expected: _,
            }) => {
                fs::remove_dir_all(path)?;
                Self::import(path, version, compressed)
            }
            _ => res,
        }
    }

    pub fn import(path: &Path, version: Version, compressed: Compressed) -> Result<Self> {
        fs::create_dir_all(path)?;

        let version_path = Self::path_version_(path);
        version.validate(version_path.as_ref())?;
        version.write(version_path.as_ref())?;

        let compressed_path = Self::path_compressed_(path);
        compressed.validate(compressed_path.as_ref())?;
        compressed.write(compressed_path.as_ref())?;

        let stored_len = Length::try_from(Self::path_length_(path).as_path())?;

        let pages = CompressedPagesMetadata::read(Self::path_pages_(path).as_path())?;

        Ok(Self {
            version,
            compressed,
            pathbuf: path.to_owned(),
            stored_len,
            decoded_pages: None,
            pushed: vec![],
            pages,
            decoded_page: None,
            phantom: PhantomData,
        })
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

    #[inline(always)]
    fn mmap(&self, page: &CompressedPageMetadata) -> io::Result<Mmap> {
        let len = page.bytes_len as usize;
        let offset = page.start;
        let file = self.open_file()?;

        Ok(unsafe {
            memmap2::MmapOptions::new()
                .len(len)
                .offset(offset)
                .map(&file)?
        })
    }

    fn decode(&self, page_index: usize) -> Result<Box<[T]>> {
        if self.pages.len() <= page_index {
            return Err(Error::ExpectVecToHaveIndex);
        }

        let page = self.pages.get(page_index).unwrap();

        let mmap = self.mmap(page)?;

        let decoded = zstd::decode_all(&mmap[..]);

        if decoded.is_err() {
            dbg!((page, page_index, &mmap[..], &mmap.len(), &decoded));
        }

        Ok(decoded?
            .chunks(Self::SIZE_OF_T)
            .map(|slice| T::try_read_from_bytes(slice).unwrap())
            .collect::<Vec<_>>()
            .into_boxed_slice())
    }

    pub fn open_then_read(&self, index: I) -> Result<Option<T>> {
        self.open_then_read_(Self::i_to_usize(index)?)
    }
    fn open_then_read_(&self, index: usize) -> Result<Option<T>> {
        Ok(self
            .decode(Self::index_to_page_index(index))?
            .get(Self::index_to_decoded_index(index))
            .cloned())
    }

    pub fn init_big_cache(&mut self) -> io::Result<()> {
        self.decoded_pages.replace(vec![]);
        self.reset_big_cache()
    }

    fn reset_big_cache(&mut self) -> io::Result<()> {
        if self.decoded_pages.is_none() {
            return Ok(());
        }

        let big_cache = self.decoded_pages.as_mut().unwrap();

        big_cache.par_iter_mut().for_each(|lock| {
            lock.take();
        });

        let len = (*self.stored_len as f64 / Self::PER_PAGE as f64).ceil() as usize;
        let len = Self::CACHE_LENGTH.min(len);

        if big_cache.len() != len {
            big_cache.resize_with(len, Default::default);
        }

        Ok(())
    }

    fn reset_caches(&mut self) -> io::Result<()> {
        self.decoded_page.take();
        self.reset_big_cache()
    }

    #[inline(always)]
    fn index_to_page_index(index: usize) -> usize {
        index / Self::PER_PAGE
    }

    #[inline(always)]
    fn index_to_decoded_index(index: usize) -> usize {
        index % Self::PER_PAGE
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

        if let Some(big_cache) = self
            .decoded_pages
            .as_ref()
            .and_then(|v| if v.is_empty() { None } else { Some(v) })
        {
            let page_index = Self::index_to_page_index(index);
            let last_index = *self.stored_len - 1;
            let max_page_index = last_index / Self::PER_PAGE;

            let min_page_index = (max_page_index + 1) - big_cache.len();

            if page_index >= min_page_index {
                return Ok(big_cache
                    .get(page_index - min_page_index)
                    .ok_or(Error::MmapsVecIsTooSmall)?
                    .get_or_init(|| self.decode(page_index).unwrap())
                    .get(Self::index_to_decoded_index(index))
                    .map(|v| Value::Ref(v)));
            }
        }

        Ok(self.open_then_read_(index)?.map(|v| Value::Owned(v)))
    }

    #[inline]
    pub fn read(&mut self, index: I) -> Result<Option<&T>> {
        self.read_(Self::i_to_usize(index)?)
    }
    #[inline]
    pub fn read_(&mut self, index: usize) -> Result<Option<&T>> {
        match self.index_to_pushed_index(index) {
            Ok(index) => {
                if let Some(index) = index {
                    return Ok(self.pushed.get(index));
                }
            }
            Err(Error::IndexTooHigh) => return Ok(None),
            Err(Error::IndexTooLow) => {}
            Err(error) => return Err(error),
        }

        let page_index = Self::index_to_page_index(index);

        if self.decoded_page.as_ref().is_none_or(|b| b.0 != page_index) {
            self.decoded_page
                .replace((page_index, self.decode(page_index)?));
        }

        Ok(self
            .decoded_page
            .as_ref()
            .unwrap()
            .1
            .get(Self::index_to_decoded_index(index)))
    }

    pub fn read_last(&mut self) -> Result<Option<&T>> {
        let len = self.len();
        if len == 0 {
            return Ok(None);
        }
        self.read_(len - 1)
    }

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
        if !self.pushed.is_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let stored_len = I::from(*self.stored_len);

        while index < stored_len {
            let v = self.read(index)?.unwrap();
            f((index, v))?;
            index = index + 1;
        }

        Ok(())
    }

    pub fn iter_from_cloned<F>(&mut self, mut index: I, mut f: F) -> Result<()>
    where
        F: FnMut((I, T, &mut Self)) -> Result<()>,
    {
        if !self.pushed.is_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let stored_len = I::from(*self.stored_len);

        while index < stored_len {
            let v = self.read(index)?.unwrap().clone();
            f((index, v, self))?;
            index = index + 1;
        }

        Ok(())
    }

    pub fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<T>> {
        if !self.pushed.is_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let len = *self.stored_len;

        let from = from.map_or(0, |from| {
            if from >= 0 {
                from as usize
            } else {
                (len as i64 + from) as usize
            }
        });

        let to = to.map_or(len - 1, |to| {
            if to >= 0 {
                to as usize
            } else {
                ((len - 1) as i64 + to) as usize
            }
        });

        if from > to {
            return Err(Error::RangeFromAfterTo);
        }

        let mut small_cache: SmallCache<T> = None;

        let values = (from..=to)
            .flat_map(|index| {
                let page_index = Self::index_to_page_index(index);

                if small_cache.as_ref().is_none_or(|b| b.0 != page_index) {
                    small_cache.replace((page_index, self.decode(page_index).unwrap()));
                }

                small_cache
                    .as_ref()
                    .unwrap()
                    .1
                    .get(Self::index_to_decoded_index(index))
                    .cloned()
            })
            .collect::<Vec<_>>();

        Ok(values)
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
    pub fn len(&self) -> usize {
        *self.stored_len + self.pushed_len()
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

        let mut file = self.open_file()?;

        let (starting_page_index, values) = if *self.stored_len % Self::PER_PAGE != 0 {
            if self.pages.is_empty() {
                unreachable!()
            }

            let last_page_index = self.pages.len() - 1;

            let values = if let Some(values) = self.decoded_pages.as_mut().and_then(|big_cache| {
                big_cache
                    .last_mut()
                    .and_then(|lock| lock.take())
                    .map(|b| b.into_vec())
            }) {
                values
            } else if self
                .decoded_page
                .as_ref()
                .is_some_and(|(page_index, _)| *page_index == last_page_index)
            {
                self.decoded_page.take().unwrap().1.into_vec()
            } else {
                self.decode(last_page_index)
                    .inspect_err(|_| {
                        dbg!(last_page_index, &self.pages);
                    })
                    .unwrap()
                    .into_vec()
            };

            let file_len = self.pages.pop().unwrap().start;

            Self::file_set_len(&mut file, file_len)?;

            (last_page_index, values)
        } else {
            (self.pages.len(), vec![])
        };

        self.stored_len += self.pushed_len();

        let compressed = values
            .into_par_iter()
            .chain(mem::take(&mut self.pushed).into_par_iter())
            .chunks(Self::PER_PAGE)
            .map(|chunk| (Self::compress_chunk(&chunk), chunk.len()))
            .collect::<Vec<_>>();

        compressed
            .iter()
            .enumerate()
            .for_each(|(i, (compressed_bytes, values_len))| {
                let page_index = starting_page_index + i;

                let start = if page_index != 0 {
                    let prev = self.pages.get(page_index - 1).unwrap();
                    prev.start + prev.bytes_len as u64
                } else {
                    0
                };

                let bytes_len = compressed_bytes.len() as u32;
                let values_len = *values_len as u32;

                let page = CompressedPageMetadata::new(start, bytes_len, values_len);

                self.pages.push(page_index, page);
            });

        let compressed = compressed
            .into_iter()
            .flat_map(|(v, _)| v)
            .collect::<Box<_>>();

        self.pages.write()?;
        file.write_all(&compressed)?;
        self.reset_caches()?;

        self.write_length()?;

        Ok(())
    }

    fn compress_chunk(chunk: &[T]) -> Box<[u8]> {
        if chunk.len() > Self::PER_PAGE {
            panic!();
        }

        let mut bytes: Vec<u8> = vec![0; chunk.len() * Self::SIZE_OF_T];

        let unsafe_bytes = UnsafeSlice::new(&mut bytes);

        chunk
            .into_par_iter()
            .enumerate()
            .for_each(|(i, v)| unsafe_bytes.copy_slice(i * Self::SIZE_OF_T, v.as_bytes()));

        zstd::encode_all(bytes.as_slice(), DEFAULT_COMPRESSION_LEVEL)
            .unwrap()
            .into_boxed_slice()
    }

    pub fn truncate_if_needed(&mut self, index: I) -> Result<()> {
        let index = Self::i_to_usize(index)?;

        if index >= *self.stored_len {
            return Ok(());
        }

        if index == 0 {
            self.reset_file()?;
            return Ok(());
        }

        let page_index = Self::index_to_page_index(index);

        let values = self.decode(page_index)?;
        let mut page = self.pages.truncate(page_index).unwrap();

        let mut file = self.open_file()?;
        Self::file_set_len(&mut file, page.start)?;

        let decoded_index = Self::index_to_decoded_index(index);

        if decoded_index != 0 {
            let chunk = &values[..decoded_index];

            let compressed = Self::compress_chunk(chunk);

            page.values_len = chunk.len() as u32;
            page.bytes_len = compressed.len() as u32;

            file.write_all(&compressed)?;

            self.pages.push(page_index, page);
        }

        self.pages.write()?;

        *self.stored_len = index;
        self.write_length()?;

        self.reset_caches()?;

        Ok(())
    }

    pub fn reset_file(&mut self) -> Result<()> {
        let mut file = self.open_file()?;
        Self::file_set_len(&mut file, 0)?;
        *self.stored_len = 0;
        self.reset_caches()?;
        Ok(())
    }

    fn file_set_len(file: &mut File, len: u64) -> io::Result<()> {
        file.set_len(len)?;
        file.seek(SeekFrom::End(0))?;
        Ok(())
    }

    #[inline]
    pub fn i_to_usize(index: I) -> Result<usize> {
        index.try_into().map_err(|_| Error::FailedKeyTryIntoUsize)
    }

    #[inline]
    fn index_to_pushed_index(&self, index: usize) -> Result<Option<usize>> {
        let file_len = *self.stored_len;

        if index >= file_len {
            let index = index - file_len;
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
        self.path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_owned()
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
        path.join("vec.zstd")
    }

    fn write_length(&self) -> io::Result<()> {
        self.stored_len.write(&self.path_length())
    }
    #[inline]
    fn path_length(&self) -> PathBuf {
        Self::path_length_(&self.pathbuf)
    }
    #[inline]
    fn path_length_(path: &Path) -> PathBuf {
        path.join("length")
    }

    #[inline]
    fn path_pages_(path: &Path) -> PathBuf {
        path.join("pages")
    }

    #[inline]
    fn path_version_(path: &Path) -> PathBuf {
        path.join("version")
    }

    #[inline]
    fn path_compressed_(path: &Path) -> PathBuf {
        path.join("compressed")
    }

    pub fn index_type_to_string(&self) -> &str {
        std::any::type_name::<I>()
    }

    pub fn version(&self) -> Version {
        self.version
    }
}

impl<I, T> Clone for StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn clone(&self) -> Self {
        Self::import(&self.pathbuf, self.version, self.compressed).unwrap()
    }
}
