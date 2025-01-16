use std::{
    cmp::Ordering,
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{self, Write},
    marker::PhantomData,
    mem,
    ops::{Deref, DerefMut, Range},
    path::Path,
    sync::OnceLock,
};

use color_eyre::eyre::{eyre, ContextCompat};

use memmap2::{Mmap, MmapOptions};

///
/// A Push only vec stored on disk using Mmap
///
/// Reads (imports of Mmap) are lazy
///
/// Stores only raw data without any overhead, and doesn't even have a header (TODO: which it should, at least to Err if wrong endian)
///
/// The file isn't portable for speed reasons (TODO: but could be ?)
///
#[derive(Debug)]
pub struct StorableVec<I, T> {
    file: File,
    mmaps: VecLazyMmap,
    disk_len: usize,
    cache: Vec<T>,
    phantom: PhantomData<I>,
}

/// In bytes
const MAX_PAGE_SIZE: usize = 4096;

impl<I, T> StorableVec<I, T>
where
    I: Into<usize>,
    T: Sized + Debug,
{
    pub const SIZE: usize = size_of::<T>();

    pub const PER_PAGE: usize = MAX_PAGE_SIZE / Self::SIZE;
    /// In bytes
    pub const PAGE_SIZE: usize = Self::PER_PAGE * Self::SIZE;

    pub fn import(path: &Path) -> color_eyre::Result<Self> {
        let file = Self::open_file(path)?;

        let mut this = Self {
            disk_len: Self::byte_index_to_index(file.metadata()?.len() as usize),
            file,
            mmaps: VecLazyMmap::default(),
            cache: vec![],
            phantom: PhantomData,
        };

        this.reset_mmaps();

        Ok(this)
    }

    fn reset_mmaps(&mut self) {
        self.mmaps
            .reset((self.disk_len as f64 / Self::PER_PAGE as f64).ceil() as usize);
    }

    fn open_file(path: &Path) -> Result<File, io::Error> {
        OpenOptions::new()
            .read(true)
            .create(true)
            .truncate(false)
            .append(true)
            .open(path)
    }

    #[inline]
    fn index_to_mmap_index(index: usize) -> usize {
        Self::index_to_byte_index(index) / Self::PAGE_SIZE
    }

    #[inline]
    fn index_to_range(index: usize) -> Range<usize> {
        let index = Self::index_to_byte_index(index) % Self::PAGE_SIZE;
        index..(index + Self::SIZE)
    }

    #[inline]
    fn index_to_byte_index(index: usize) -> usize {
        index * Self::SIZE
    }

    #[inline]
    fn byte_index_to_index(byte_index: usize) -> usize {
        byte_index / Self::SIZE
    }

    #[allow(unused)]
    #[inline]
    pub fn get(&self, index: I) -> color_eyre::Result<Option<&T>> {
        self._get(index.into())
    }
    pub fn _get(&self, index: usize) -> color_eyre::Result<Option<&T>> {
        if self.disk_len == 0 || index > self.disk_len - 1 {
            Ok(self.cache.get(index - self.disk_len))
        } else {
            let mmap_index = Self::index_to_mmap_index(index);

            let mmap = self
                .mmaps
                .get(mmap_index)
                .context("Expect mmap to be open")?
                .get_or_load(
                    MAX_PAGE_SIZE,
                    (mmap_index * Self::PAGE_SIZE) as u64,
                    &self.file,
                );

            let range = Self::index_to_range(index);
            let src = &mmap[range];

            let (prefix, shorts, suffix) = unsafe { src.align_to::<T>() };

            if !prefix.is_empty() || shorts.len() != 1 || !suffix.is_empty() {
                dbg!(&src, &prefix, &shorts, &suffix);
                return Err(eyre!("align_to issue"));
            }

            Ok(Some(&shorts[0]))
        }
    }

    #[allow(unused)]
    pub fn first(&self) -> color_eyre::Result<Option<&T>> {
        self._get(0)
    }

    #[allow(unused)]
    pub fn last(&self) -> color_eyre::Result<Option<&T>> {
        let len = self.len();
        if len == 0 {
            return Ok(None);
        }
        self._get(len - 1)
    }

    pub fn push(&mut self, value: T) {
        self.cache.push(value)
    }

    pub fn push_if_needed(&mut self, index: I, value: T) -> color_eyre::Result<()> {
        self._push_if_needed(index.into(), value)
    }
    pub fn _push_if_needed(&mut self, index: usize, value: T) -> color_eyre::Result<()> {
        let len = self.len();
        match len.cmp(&index) {
            Ordering::Greater => Ok(()),
            Ordering::Equal => {
                self.push(value);
                Ok(())
            }
            Ordering::Less => {
                dbg!(std::any::type_name::<I>(), std::any::type_name::<T>());
                dbg!(len, index, value);
                Err(eyre!("Index is too high"))
            }
        }
    }

    pub fn len(&self) -> usize {
        self.disk_len + self.cache.len()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn flush(&mut self) -> io::Result<()> {
        self.disk_len += self.cache.len();
        self.reset_mmaps();

        let mut bytes: Vec<u8> = vec![];

        mem::take(&mut self.cache).into_iter().for_each(|v| {
            let data: *const T = &v;
            let data: *const u8 = data as *const u8;
            let slice = unsafe { std::slice::from_raw_parts(data, Self::SIZE) };
            bytes.extend_from_slice(slice)
        });

        self.file.write_all(&bytes)
    }
}

pub trait AnyStorableVec {
    fn len(&self) -> usize;
    fn is_empty(&self) -> bool;
    fn flush(&mut self) -> io::Result<()>;
}

impl<I, T> AnyStorableVec for StorableVec<I, T>
where
    I: Into<usize>,
    T: Sized + Debug,
{
    fn len(&self) -> usize {
        self.len()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }

    fn flush(&mut self) -> io::Result<()> {
        self.flush()
    }
}

#[derive(Debug, Default)]
struct VecLazyMmap(Vec<LazyMmap>);
impl VecLazyMmap {
    pub fn reset(&mut self, len: usize) {
        self.0.clear();
        self.0.resize_with(len, Default::default);
    }
}
impl Deref for VecLazyMmap {
    type Target = Vec<LazyMmap>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl DerefMut for VecLazyMmap {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

/// Box to reduce the size, would be 24 instead
#[derive(Debug, Default)]
struct LazyMmap(OnceLock<Box<Mmap>>);
impl LazyMmap {
    pub fn get_or_load(&self, len: usize, offset: u64, file: &File) -> &Mmap {
        self.0.get_or_init(|| {
            Box::new(unsafe {
                MmapOptions::new()
                    .len(len)
                    .offset(offset)
                    .map(file)
                    .unwrap()
            })
        })
    }
}
