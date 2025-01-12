use std::{
    cmp::Ordering,
    collections::{btree_map::Entry, BTreeMap},
    fmt::Debug,
    fs::{File, OpenOptions},
    io::{self, Write},
    marker::PhantomData,
    mem,
    ops::Range,
    path::Path,
};

use color_eyre::eyre::{eyre, ContextCompat};
use memmap2::{Mmap, MmapOptions};

#[derive(Debug)]
pub struct Vecdisk<I, T> {
    file: File,
    mmaps: BTreeMap<usize, Mmap>,
    // or ?
    // mmaps: [Arc<parking_lot::Rwlock<Option<Mmap>>>],
    // arr len: (file.metadata()?.len() as f64 / PAGE_SIZE as f64).ceil()
    // start read lock, if none, upgrade to write lock, set mmap, downgrade, read
    disk_len: usize,
    cache: Vec<T>,
    phantom: PhantomData<I>,
}

/// In bytes
const MAX_PAGE_SIZE: usize = 4096;

impl<I, T> Vecdisk<I, T>
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

        Ok(Self {
            disk_len: Self::byte_index_to_index(file.metadata()?.len() as usize),
            file,
            mmaps: BTreeMap::new(),
            cache: vec![],
            phantom: PhantomData,
        })
    }

    fn open_file(path: &Path) -> Result<File, io::Error> {
        OpenOptions::new()
            .read(true)
            .create(true)
            .truncate(false)
            .append(true)
            .open(path)
    }

    #[allow(unused)]
    #[inline]
    pub fn open_for(&mut self, index: I) -> color_eyre::Result<()> {
        self._open_for(index.into())
    }
    fn _open_for(&mut self, index: usize) -> color_eyre::Result<()> {
        if index >= self.disk_len {
            return Ok(());
        }

        let mmap_index = Self::index_to_mmap_index(index);

        if let Entry::Vacant(v) = self.mmaps.entry(mmap_index) {
            v.insert(unsafe {
                MmapOptions::new()
                    .len(MAX_PAGE_SIZE)
                    .offset((mmap_index * Self::PAGE_SIZE) as u64)
                    .map(&self.file)?
            });
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn open_first(&mut self) -> color_eyre::Result<()> {
        self._open_for(0)
    }

    #[allow(unused)]
    pub fn open_last(&mut self) -> color_eyre::Result<()> {
        self._open_for(self.len().checked_sub(1).unwrap_or_default())
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
    fn _get(&self, index: usize) -> color_eyre::Result<Option<&T>> {
        if self.disk_len == 0 {
            Ok(None)
        } else if index > self.disk_len - 1 {
            Ok(self.cache.get(index - self.disk_len))
        } else {
            let mmap_index = Self::index_to_mmap_index(index);
            let mmap = self.mmaps.get(&mmap_index).context("Expect mmap to be open")?;

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

    pub fn open_then_get(&mut self, index: I) -> color_eyre::Result<Option<&T>> {
        self._open_then_get(index.into())
    }
    pub fn _open_then_get(&mut self, index: usize) -> color_eyre::Result<Option<&T>> {
        self._open_for(index)?;
        self._get(index)
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
        let len = self.len();
        let index = index.into();
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

    #[allow(unused)]
    #[inline]
    pub fn mmaps_opened(&self) -> usize {
        self.mmaps.len()
    }

    #[allow(unused)]
    #[inline]
    pub fn get_mmap(&self, mmap_index: usize) -> Option<&Mmap> {
        self.mmaps.get(&mmap_index)
    }
}

pub trait AnyVecdisk {
    fn len(&self) -> usize;
    fn flush(&mut self) -> color_eyre::Result<()>;
}

impl<I, T> AnyVecdisk for Vecdisk<I, T>
where
    I: Into<usize>,
    T: Sized + Debug,
{
    fn len(&self) -> usize {
        self.disk_len + self.cache.len()
    }

    fn flush(&mut self) -> color_eyre::Result<()> {
        self.mmaps.clear();

        self.disk_len += self.cache.len();

        let mut bytes: Vec<u8> = vec![];

        mem::take(&mut self.cache).into_iter().for_each(|v| {
            let data: *const T = &v;
            let data: *const u8 = data as *const u8;
            let slice = unsafe { std::slice::from_raw_parts(data, Self::SIZE) };
            bytes.extend_from_slice(slice)
        });

        self.file.write_all(&bytes)?;

        Ok(())
    }
}
