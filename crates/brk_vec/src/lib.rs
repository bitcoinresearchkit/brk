#![doc = include_str!("../README.md")]
#![doc = "\n## Example\n\n```rust"]
#![doc = include_str!("../examples/main.rs")]
#![doc = "```"]

use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Seek, SeekFrom, Write},
    marker::PhantomData,
    mem,
    path::{Path, PathBuf},
    sync::OnceLock,
};

pub use memmap2;
use rayon::prelude::*;
pub use zerocopy;
use zstd::DEFAULT_COMPRESSION_LEVEL;

mod enums;
mod structs;
mod traits;

pub use enums::*;
pub use structs::*;
pub use traits::*;

const ONE_KIB: usize = 1024;
pub const MAX_PAGE_SIZE: usize = 16 * ONE_KIB;
const ONE_MIB: usize = ONE_KIB * ONE_KIB;
pub const MAX_CACHE_SIZE: usize = 100 * ONE_MIB;

#[allow(private_interfaces)]
#[derive(Debug)]
pub enum StorableVec<I, T> {
    Raw {
        base: Base<I, T>,
    },
    Compressed {
        base: Base<I, T>,
        pages_meta: CompressedPagesMetadata,
    },
}

impl<I, T> StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub const SIZE_OF_T: usize = size_of::<T>();
    pub const PER_PAGE: usize = MAX_PAGE_SIZE / Self::SIZE_OF_T;
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
        let base = Base::import(path, version, compressed)?;

        if *compressed {
            let pages_meta = CompressedPagesMetadata::read(Self::path_pages_meta_(path).as_path())?;

            Ok(Self::Compressed { base, pages_meta })
        } else {
            Ok(Self::Raw { base })
        }
    }

    #[inline]
    pub fn get(&mut self, index: I) -> Result<Option<&T>> {
        self.get_(index.to_usize()?)
    }
    #[inline]
    pub fn get_(&mut self, index: usize) -> Result<Option<&T>> {
        match self.index_to_pushed_index(index) {
            Ok(index) => {
                if let Some(index) = index {
                    return Ok(self.pushed().get(index));
                }
            }
            Err(Error::IndexTooHigh) => return Ok(None),
            Err(Error::IndexTooLow) => {}
            Err(error) => return Err(error),
        }

        let page_index = Self::index_to_page_index(index);

        if self.page().is_none_or(|b| b.0 != page_index) {
            let values = self.decode_page(page_index)?;
            self.mut_page().replace((page_index, values));
        }

        self.page().unwrap().1.get(index)
    }

    pub fn get_last(&mut self) -> Result<Option<&T>> {
        let len = self.len();
        if len == 0 {
            return Ok(None);
        }
        self.get_(len - 1)
    }

    pub fn read(&self, index: I) -> Result<Option<T>> {
        self.read_(index.to_usize()?)
    }
    pub fn read_(&self, index: usize) -> Result<Option<T>> {
        Ok(match self {
            Self::Raw { .. } => {
                let mut file = self.open_file()?;
                let byte_index = Self::index_to_byte_index(index);
                file.seek(SeekFrom::Start(byte_index))?;
                let mut buf = vec![0; Self::SIZE_OF_T];
                file.read_exact(&mut buf)?;
                T::try_ref_from_bytes(&buf[..]).ok().map(|v| v.to_owned())
            }
            Self::Compressed { .. } => self
                .decode_page(Self::index_to_page_index(index))?
                .get(index)?
                .cloned(),
        })
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
        if !self.is_pushed_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let stored_len = I::from(self.stored_len());

        while index < stored_len {
            let v = self.get(index)?.unwrap();
            f((index, v))?;
            index = index + 1;
        }

        Ok(())
    }

    pub fn iter_from_cloned<F>(&mut self, mut index: I, mut f: F) -> Result<()>
    where
        F: FnMut((I, T, &mut Self)) -> Result<()>,
    {
        if !self.is_pushed_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let stored_len = I::from(self.stored_len());

        while index < stored_len {
            let v = self.get(index)?.unwrap().clone();
            f((index, v, self))?;
            index = index + 1;
        }

        Ok(())
    }

    pub fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Vec<T>> {
        if !self.is_pushed_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let len = self.stored_len();

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

        let mut page: Option<(usize, Values<T>)> = None;

        let values = (from..=to)
            .flat_map(|index| {
                let page_index = Self::index_to_page_index(index);

                if page.as_ref().is_none_or(|b| b.0 != page_index) {
                    let values = self.decode_page(page_index).unwrap();
                    page.replace((page_index, values));
                }

                page.as_ref().unwrap().1.get(index).ok().flatten().cloned()
            })
            .collect::<Vec<_>>();

        Ok(values)
    }

    pub fn decode_page(&self, page_index: usize) -> Result<Values<T>> {
        Self::decode_page_(
            self.stored_len(),
            page_index,
            self.file(),
            match self {
                Self::Raw { .. } => None,
                Self::Compressed { pages_meta, .. } => Some(pages_meta),
            },
        )
    }

    fn decode_page_(
        stored_len: usize,
        page_index: usize,
        file: &File,
        compressed_pages_meta: Option<&CompressedPagesMetadata>,
    ) -> Result<Values<T>> {
        if Self::page_index_to_index(page_index) >= stored_len {
            return Err(Error::IndexTooHigh);
        }

        let (len, offset) = if let Some(pages_meta) = compressed_pages_meta {
            if pages_meta.len() <= page_index {
                return Err(Error::ExpectVecToHaveIndex);
            }
            let page = pages_meta.get(page_index).unwrap();
            (page.bytes_len as usize, page.start)
        } else {
            (Self::PAGE_SIZE, Self::page_index_to_byte_index(page_index))
        };

        let mmap = unsafe {
            memmap2::MmapOptions::new()
                .len(len)
                .offset(offset)
                .map(file)?
        };

        let compressed = compressed_pages_meta.is_some();

        if compressed {
            let decoded = zstd::decode_all(&mmap[..]);

            if decoded.is_err() {
                dbg!((len, offset, page_index, &mmap[..], &mmap.len(), &decoded));
            }

            Ok(Values::from(
                decoded?
                    .chunks(Self::SIZE_OF_T)
                    .map(|slice| T::try_read_from_bytes(slice).unwrap())
                    .collect::<Vec<_>>()
                    .into_boxed_slice(),
            ))
        } else {
            Ok(Values::from(mmap))
        }
    }

    #[inline]
    pub fn push(&mut self, value: T) {
        self.mut_base().pushed.push(value)
    }

    pub fn flush(&mut self) -> io::Result<()> {
        let pushed_len = self.pushed_len();

        if pushed_len == 0 {
            return Ok(());
        }

        let stored_len = self.stored_len();

        let bytes = match self {
            Self::Compressed { base, pages_meta } => {
                let (starting_page_index, values) = if *base.stored_len % Self::PER_PAGE != 0 {
                    if pages_meta.is_empty() {
                        unreachable!()
                    }

                    let last_page_index = pages_meta.len() - 1;

                    let values = if let Some(values) = base
                        .pages
                        .as_mut()
                        .and_then(|big_cache| big_cache.last_mut().and_then(|lock| lock.take()))
                    {
                        values
                    } else if base
                        .page
                        .as_ref()
                        .is_some_and(|(page_index, _)| *page_index == last_page_index)
                    {
                        base.page.take().unwrap().1
                    } else {
                        Self::decode_page_(
                            stored_len,
                            last_page_index,
                            &base.file,
                            Some(pages_meta),
                        )
                        .inspect_err(|_| {
                            dbg!(last_page_index, &pages_meta);
                        })
                        .unwrap()
                    };

                    let file_len = pages_meta.pop().unwrap().start;

                    file_set_len(&mut base.file, file_len)?;

                    (last_page_index, values)
                } else {
                    (pages_meta.len(), Values::default())
                };

                let compressed = Vec::from(values.as_arr())
                    .into_par_iter()
                    .chain(mem::take(&mut base.pushed).into_par_iter())
                    .chunks(Self::PER_PAGE)
                    .map(|chunk| (Self::compress_chunk(chunk.as_ref()), chunk.len()))
                    .collect::<Vec<_>>();

                compressed
                    .iter()
                    .enumerate()
                    .for_each(|(i, (compressed_bytes, values_len))| {
                        let page_index = starting_page_index + i;

                        let start = if page_index != 0 {
                            let prev = pages_meta.get(page_index - 1).unwrap();
                            prev.start + prev.bytes_len as u64
                        } else {
                            0
                        };

                        let bytes_len = compressed_bytes.len() as u32;
                        let values_len = *values_len as u32;

                        let page = CompressedPageMetadata::new(start, bytes_len, values_len);

                        pages_meta.push(page_index, page);
                    });

                pages_meta.write()?;

                compressed
                    .into_iter()
                    .flat_map(|(v, _)| v)
                    .collect::<Vec<_>>()
            }
            Self::Raw { base } => {
                let pushed = &mut base.pushed;

                let mut bytes: Vec<u8> = vec![0; pushed.len() * Self::SIZE_OF_T];

                let unsafe_bytes = UnsafeSlice::new(&mut bytes);

                mem::take(pushed)
                    .into_par_iter()
                    .enumerate()
                    .for_each(|(i, v)| unsafe_bytes.copy_slice(i * Self::SIZE_OF_T, v.as_bytes()));

                bytes
            }
        };

        self.mut_file().write_all(&bytes)?;

        self.reset_caches();

        self.increase_stored_len(pushed_len);

        self.write_stored_length()?;

        Ok(())
    }

    pub fn truncate_if_needed(&mut self, index: I) -> Result<()> {
        let index = index.to_usize()?;

        if index >= self.stored_len() {
            return Ok(());
        }

        if index == 0 {
            self.reset()?;
            return Ok(());
        }

        let page_index = Self::index_to_page_index(index);

        let values = match self {
            Self::Compressed { .. } => self.decode_page(page_index)?,
            Self::Raw { .. } => Values::default(),
        };

        let (len, bytes) = match self {
            Self::Compressed { pages_meta, .. } => {
                let mut page = pages_meta.truncate(page_index).unwrap();

                let len = page.start;

                let decoded_index = Self::index_to_decoded_index(index);

                let compressed = if decoded_index != 0 {
                    let chunk = &values.as_arr()[..decoded_index];

                    let compressed = Self::compress_chunk(chunk);

                    page.values_len = chunk.len() as u32;
                    page.bytes_len = compressed.len() as u32;

                    pages_meta.push(page_index, page);

                    compressed
                } else {
                    vec![].into_boxed_slice()
                };

                pages_meta.write()?;

                (len, compressed)
            }
            Self::Raw { .. } => {
                // let value_at_index = self.open_then_read_(index).ok();

                let len = Self::index_to_byte_index(index);

                (len, vec![].into_boxed_slice())
            }
        };

        let file = self.mut_file();

        file_set_len(file, len)?;

        if !bytes.is_empty() {
            file.write_all(&bytes)?;
        }

        self.set_stored_len(index);

        self.write_stored_length()?;

        self.reset_caches();

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

    pub fn enable_large_cache(&mut self) {
        self.mut_pages().replace(vec![]);
        self.reset_large_cache();
    }

    pub fn disable_large_cache(&mut self) {
        self.mut_base().pages.take();
    }

    fn reset_large_cache(&mut self) {
        let stored_len = self.stored_len();

        if let Some(pages) = self.mut_pages().as_mut() {
            pages.par_iter_mut().for_each(|lock| {
                lock.take();
            });

            let len = (stored_len as f64 / Self::PER_PAGE as f64).ceil() as usize;
            let len = Self::CACHE_LENGTH.min(len);

            if pages.len() != len {
                pages.resize_with(len, Default::default);
            }
        }
    }

    pub fn large_cache_len(&self) -> usize {
        self.pages().map_or(0, |v| v.len())
    }

    fn reset_small_cache(&mut self) {
        self.mut_base().page.take();
    }

    fn reset_caches(&mut self) {
        self.reset_small_cache();
        self.reset_large_cache();
    }

    pub fn reset(&mut self) -> Result<()> {
        self.mut_base().reset_file()?;
        self.reset_stored_len();
        self.reset_caches();
        Ok(())
    }

    #[inline]
    pub fn index_to_pushed_index(&self, index: usize) -> Result<Option<usize>> {
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
    fn index_to_byte_index(index: usize) -> u64 {
        (index * Self::SIZE_OF_T) as u64
    }

    #[inline(always)]
    fn index_to_page_index(index: usize) -> usize {
        index / Self::PER_PAGE
    }

    #[inline(always)]
    fn page_index_to_index(page_index: usize) -> usize {
        page_index * Self::PER_PAGE
    }

    #[inline(always)]
    fn page_index_to_byte_index(page_index: usize) -> u64 {
        (page_index * Self::PAGE_SIZE) as u64
    }

    #[inline(always)]
    fn index_to_decoded_index(index: usize) -> usize {
        index % Self::PER_PAGE
    }

    #[inline]
    fn path_pages_meta_(path: &Path) -> PathBuf {
        path.join("pages_meta")
    }

    #[inline]
    fn page(&self) -> Option<&(usize, Values<T>)> {
        self.base().page.as_ref()
    }

    #[inline]
    fn mut_page(&mut self) -> &mut Option<(usize, Values<T>)> {
        &mut self.mut_base().page
    }

    #[inline]
    pub fn pages(&self) -> Option<&Vec<OnceLock<Values<T>>>> {
        self.base().pages.as_ref()
    }

    #[inline]
    fn mut_pages(&mut self) -> &mut Option<Vec<OnceLock<Values<T>>>> {
        &mut self.mut_base().pages
    }

    #[inline]
    pub fn len(&self) -> usize {
        self.stored_len() + self.pushed_len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    #[inline]
    pub fn has(&self, index: I) -> Result<bool> {
        Ok(self.has_(index.to_usize()?))
    }
    #[inline]
    fn has_(&self, index: usize) -> bool {
        index < self.len()
    }

    #[inline]
    pub fn pushed(&self) -> &Vec<T> {
        &self.base().pushed
    }

    #[inline]
    pub fn pushed_len(&self) -> usize {
        self.pushed().len()
    }

    #[inline]
    fn is_pushed_empty(&self) -> bool {
        self.pushed_len() == 0
    }

    #[inline]
    pub fn stored_len(&self) -> usize {
        *self.base().stored_len
    }

    #[inline]
    fn set_stored_len(&mut self, len: usize) {
        *self.mut_base().stored_len = len;
    }

    fn increase_stored_len(&mut self, len: usize) {
        *self.mut_base().stored_len += len;
    }

    #[inline]
    fn reset_stored_len(&mut self) {
        self.set_stored_len(0);
    }

    fn write_stored_length(&self) -> io::Result<()> {
        self.base().write_stored_length()
    }

    #[inline]
    pub fn path(&self) -> &Path {
        &self.base().pathbuf
    }

    fn file(&self) -> &File {
        &self.base().file
    }

    fn mut_file(&mut self) -> &mut File {
        &mut self.mut_base().file
    }

    fn open_file(&self) -> io::Result<File> {
        self.base().open_file()
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
    pub fn version(&self) -> Version {
        self.base().version
    }

    #[inline]
    fn compressed(&self) -> Compressed {
        self.base().compressed
    }

    #[inline]
    fn base(&self) -> &Base<I, T> {
        match self {
            Self::Raw { base, .. } => base,
            Self::Compressed { base, .. } => base,
        }
    }

    #[inline]
    fn mut_base(&mut self) -> &mut Base<I, T> {
        match self {
            Self::Raw { base, .. } => base,
            Self::Compressed { base, .. } => base,
        }
    }

    pub fn index_type_to_string(&self) -> &str {
        std::any::type_name::<I>()
    }
}

#[derive(Debug)]
struct Base<I, T> {
    pub version: Version,
    pub pathbuf: PathBuf,
    pub stored_len: Length,
    pub compressed: Compressed,
    pub page: Option<(usize, Values<T>)>,
    pub pages: Option<Vec<OnceLock<Values<T>>>>,
    pub pushed: Vec<T>,
    pub file: File,
    pub phantom: PhantomData<I>,
}

impl<I, T> Base<I, T> {
    pub fn import(path: &Path, version: Version, compressed: Compressed) -> Result<Self> {
        fs::create_dir_all(path)?;

        let version_path = Self::path_version_(path);
        version.validate(version_path.as_ref())?;
        version.write(version_path.as_ref())?;

        let compressed_path = Self::path_compressed_(path);
        compressed.validate(compressed_path.as_ref())?;
        compressed.write(compressed_path.as_ref())?;

        let stored_len = Length::try_from(Self::path_length_(path).as_path())?;

        Ok(Self {
            version,
            compressed,
            pathbuf: path.to_owned(),
            file: Self::open_file_(Self::path_vec_(path).as_path())?,
            stored_len,
            page: None,
            pages: None,
            pushed: vec![],
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

    fn reset_file(&mut self) -> Result<()> {
        file_set_len(&mut self.file, 0)?;
        Ok(())
    }

    #[inline]
    fn path_vec(&self) -> PathBuf {
        Self::path_vec_(&self.pathbuf)
    }
    #[inline]
    fn path_vec_(path: &Path) -> PathBuf {
        path.join("vec")
    }

    fn write_stored_length(&self) -> io::Result<()> {
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
    fn path_version_(path: &Path) -> PathBuf {
        path.join("version")
    }

    #[inline]
    fn path_compressed_(path: &Path) -> PathBuf {
        path.join("compressed")
    }
}

impl<I, T> Clone for StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn clone(&self) -> Self {
        Self::import(self.path(), self.version(), self.compressed()).unwrap()
    }
}

fn file_set_len(file: &mut File, len: u64) -> io::Result<()> {
    file.set_len(len)?;
    file.seek(SeekFrom::End(0))?;
    Ok(())
}
