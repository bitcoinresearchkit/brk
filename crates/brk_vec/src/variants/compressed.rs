use std::{
    fs::{self, File},
    mem,
    path::Path,
    sync::Arc,
};

use arc_swap::{ArcSwap, Guard};
use memmap2::Mmap;
use rayon::prelude::*;
use zstd::DEFAULT_COMPRESSION_LEVEL;

use crate::{
    CompressedPageMetadata, CompressedPagesMetadata, DynamicVec, Error, GenericVec, RawVec, Result,
    StoredIndex, StoredType, UnsafeSlice, Value, VecIterator, Version,
};

const ONE_KIB: usize = 1024;
const ONE_MIB: usize = ONE_KIB * ONE_KIB;
pub const MAX_CACHE_SIZE: usize = 100 * ONE_MIB;
pub const MAX_PAGE_SIZE: usize = 16 * ONE_KIB;

#[derive(Debug)]
pub struct CompressedVec<I, T> {
    inner: RawVec<I, T>,
    pages_meta: Arc<ArcSwap<CompressedPagesMetadata>>,
}

impl<I, T> CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub const PER_PAGE: usize = MAX_PAGE_SIZE / Self::SIZE_OF_T;
    pub const PAGE_SIZE: usize = Self::PER_PAGE * Self::SIZE_OF_T;
    pub const CACHE_LENGTH: usize = MAX_CACHE_SIZE / Self::PAGE_SIZE;

    /// Same as import but will reset the folder under certain errors, so be careful !
    pub fn forced_import(path: &Path, version: Version) -> Result<Self> {
        let res = Self::import(path, version);
        match res {
            Err(Error::WrongEndian)
            | Err(Error::DifferentVersion { .. })
            | Err(Error::DifferentCompressionMode) => {
                fs::remove_dir_all(path)?;
                Self::import(path, version)
            }
            _ => res,
        }
    }

    pub fn import(path: &Path, version: Version) -> Result<Self> {
        fs::create_dir_all(path)?;

        let vec_exists = fs::exists(Self::path_vec_(path)).is_ok_and(|b| b);
        let compressed_path = Self::path_compressed_(path);
        let compressed_exists = fs::exists(&compressed_path).is_ok_and(|b| b);

        if vec_exists && !compressed_exists {
            return Err(Error::DifferentCompressionMode);
        }

        if !vec_exists && !compressed_exists {
            File::create(&compressed_path)?;
        }

        Ok(Self {
            inner: RawVec::import(path, version)?,
            pages_meta: Arc::new(ArcSwap::new(Arc::new(CompressedPagesMetadata::read(path)?))),
        })
    }

    fn decode_page(&self, page_index: usize, mmap: &Mmap) -> Result<Vec<T>> {
        Self::decode_page_(self.stored_len(), page_index, mmap, &self.pages_meta.load())
    }

    fn decode_page_(
        stored_len: usize,
        page_index: usize,
        mmap: &Mmap,
        compressed_pages_meta: &CompressedPagesMetadata,
    ) -> Result<Vec<T>> {
        if Self::page_index_to_index(page_index) >= stored_len {
            return Err(Error::IndexTooHigh);
        } else if compressed_pages_meta.len() <= page_index {
            return Err(Error::ExpectVecToHaveIndex);
        }

        let page = compressed_pages_meta.get(page_index).unwrap();
        let len = page.bytes_len as usize;
        let offset = page.start as usize;

        Ok(zstd::decode_all(&mmap[offset..offset + len])
            .inspect_err(|_| {
                dbg!((len, offset, page_index, &mmap[..], &mmap.len()));
            })?
            .chunks(Self::SIZE_OF_T)
            .map(|slice| T::try_read_from_bytes(slice).unwrap())
            .collect::<Vec<_>>())
    }

    fn compress_page(chunk: &[T]) -> Vec<u8> {
        if chunk.len() > Self::PER_PAGE {
            panic!();
        }

        let mut bytes: Vec<u8> = vec![0; chunk.len() * Self::SIZE_OF_T];

        let unsafe_bytes = UnsafeSlice::new(&mut bytes);

        chunk
            .into_par_iter()
            .enumerate()
            .for_each(|(i, v)| unsafe_bytes.copy_slice(i * Self::SIZE_OF_T, v.as_bytes()));

        zstd::encode_all(bytes.as_slice(), DEFAULT_COMPRESSION_LEVEL).unwrap()
    }

    #[inline(always)]
    fn index_to_page_index(index: usize) -> usize {
        index / Self::PER_PAGE
    }

    #[inline(always)]
    fn page_index_to_index(page_index: usize) -> usize {
        page_index * Self::PER_PAGE
    }

    fn stored_len_(pages_meta: &Guard<Arc<CompressedPagesMetadata>>) -> usize {
        if let Some(last) = pages_meta.last() {
            (pages_meta.len() - 1) * Self::PER_PAGE + last.values_len as usize
        } else {
            0
        }
    }

    #[inline]
    pub fn iter(&self) -> CompressedVecIterator<'_, I, T> {
        self.into_iter()
    }

    #[inline]
    pub fn iter_at(&self, i: I) -> CompressedVecIterator<'_, I, T> {
        self.iter_at_(i.unwrap_to_usize())
    }

    #[inline]
    pub fn iter_at_(&self, i: usize) -> CompressedVecIterator<'_, I, T> {
        let mut iter = self.into_iter();
        iter.set_(i);
        iter
    }
}

impl<I, T> DynamicVec for CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type I = I;
    type T = T;

    #[inline]
    fn read_(&self, index: usize, mmap: &Mmap) -> Result<Option<T>> {
        let page_index = Self::index_to_page_index(index);
        let decoded_index = index % Self::PER_PAGE;

        Ok(self
            .decode_page(page_index, mmap)?
            .get(decoded_index)
            .cloned())
    }

    #[inline]
    fn mmap(&self) -> &ArcSwap<Mmap> {
        self.inner.mmap()
    }

    #[inline]
    fn stored_len(&self) -> usize {
        Self::stored_len_(&self.pages_meta.load())
    }

    #[inline]
    fn pushed(&self) -> &[T] {
        self.inner.pushed()
    }
    #[inline]
    fn mut_pushed(&mut self) -> &mut Vec<T> {
        self.inner.mut_pushed()
    }

    #[inline]
    fn path(&self) -> &Path {
        self.inner.path()
    }
}

impl<I, T> GenericVec<I, T> for CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn collect_range(&self, from: Option<usize>, to: Option<usize>) -> Result<Vec<T>> {
        let stored_len = self.stored_len();
        let from = from.unwrap_or_default();
        let to = to.map_or(stored_len, |i| i.min(stored_len));

        if from >= stored_len || from >= to {
            return Ok(vec![]);
        }

        Ok(self
            .iter_at_(from)
            .take(to - from)
            .map(|(_, v)| v.into_inner())
            .collect::<Vec<_>>())
    }

    fn flush(&mut self) -> Result<()> {
        let pushed_len = self.pushed_len();

        if pushed_len == 0 {
            return Ok(());
        }

        let stored_len = self.stored_len();

        let mut pages_meta = (**self.pages_meta.load()).clone();

        let mut starting_page_index = pages_meta.len();
        let mut values = vec![];
        let mut truncate_at = None;

        if self.stored_len() % Self::PER_PAGE != 0 {
            if pages_meta.is_empty() {
                unreachable!()
            }

            let last_page_index = pages_meta.len() - 1;

            values = Self::decode_page_(
                stored_len,
                last_page_index,
                &self.mmap().load(),
                &pages_meta,
            )
            .inspect_err(|_| {
                dbg!(last_page_index, &pages_meta);
            })
            .unwrap();

            truncate_at.replace(pages_meta.pop().unwrap().start);
            starting_page_index = last_page_index;
        }

        let compressed = values
            .into_par_iter()
            .chain(mem::take(self.mut_pushed()).into_par_iter())
            .chunks(Self::PER_PAGE)
            .map(|chunk| (Self::compress_page(chunk.as_ref()), chunk.len()))
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

        let buf = compressed
            .into_iter()
            .flat_map(|(v, _)| v)
            .collect::<Vec<_>>();

        pages_meta.write()?;

        if let Some(truncate_at) = truncate_at {
            self.file_set_len(truncate_at)?;
        }

        self.file_write_all(&buf)?;

        self.pages_meta.store(Arc::new(pages_meta));

        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        let mut pages_meta = (**self.pages_meta.load()).clone();
        pages_meta.truncate(0);
        pages_meta.write()?;
        self.pages_meta.store(Arc::new(pages_meta));
        self.file_truncate_and_write_all(0, &[])
    }

    fn truncate_if_needed(&mut self, index: I) -> Result<()> {
        let index = index.to_usize()?;

        if index >= self.stored_len() {
            return Ok(());
        }

        if index == 0 {
            self.reset()?;
            return Ok(());
        }

        let mut pages_meta = (**self.pages_meta.load()).clone();

        let page_index = Self::index_to_page_index(index);

        let values = self.decode_page(page_index, &self.mmap().load())?;
        let mut buf = vec![];

        let mut page = pages_meta.truncate(page_index).unwrap();

        let len = page.start;

        let decoded_index = index % Self::PER_PAGE;

        if decoded_index != 0 {
            let chunk = &values[..decoded_index];

            buf = Self::compress_page(chunk);

            page.values_len = chunk.len() as u32;
            page.bytes_len = buf.len() as u32;

            pages_meta.push(page_index, page);
        }

        pages_meta.write()?;

        self.pages_meta.store(Arc::new(pages_meta));

        self.file_truncate_and_write_all(len, &buf)?;

        Ok(())
    }

    #[inline]
    fn version(&self) -> Version {
        self.inner.version()
    }
}

impl<I, T> Clone for CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            pages_meta: self.pages_meta.clone(),
        }
    }
}

#[derive(Debug)]
pub struct CompressedVecIterator<'a, I, T> {
    vec: &'a CompressedVec<I, T>,
    guard: Guard<Arc<Mmap>>,
    decoded_page: Option<(usize, Vec<T>)>,
    // second_decoded_page?: Option<(usize, Vec<T>)>,
    pages_meta: Guard<Arc<CompressedPagesMetadata>>,
    stored_len: usize,
    index: usize,
}

impl<I, T> CompressedVecIterator<'_, I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    const SIZE_OF_T: usize = size_of::<T>();
    const PER_PAGE: usize = MAX_PAGE_SIZE / Self::SIZE_OF_T;
}

// impl<'a, I, T> VecIterator<'a> for CompressedVecIterator<'a, I, T>
// where
//     I: StoredIndex,
//     T: StoredType,
// {
//     type I = I;
//     type T = T;

//     #[inline]
//     fn mut_index(&mut self) -> &mut usize {
//         &mut self.index
//     }

//     #[inline]
//     fn len(&self) -> usize {
//         self.vec.len()
//     }
// }

impl<'a, I, T> Iterator for CompressedVecIterator<'a, I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Value<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        let mmap = &self.guard;
        let i = self.index;
        let stored_len = self.stored_len;

        let result = if i >= stored_len {
            let j = i - stored_len;
            if j >= self.vec.pushed_len() {
                return None;
            }
            self.vec
                .pushed()
                .get(j)
                .map(|v| (I::from(i), Value::Ref(v)))
        } else {
            let page_index = i / Self::PER_PAGE;

            if self.decoded_page.as_ref().is_none_or(|b| b.0 != page_index) {
                let values = CompressedVec::<I, T>::decode_page_(
                    stored_len,
                    page_index,
                    mmap,
                    &self.pages_meta,
                )
                .unwrap();
                self.decoded_page.replace((page_index, values));
            }

            self.decoded_page
                .as_ref()
                .unwrap()
                .1
                .get(i % Self::PER_PAGE)
                .map(|v| (I::from(i), Value::Owned(v.clone())))
        };

        self.index += 1;

        result
    }
}

impl<'a, I, T> IntoIterator for &'a CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Value<'a, T>);
    type IntoIter = CompressedVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        let pages_meta = self.pages_meta.load();
        let stored_len = CompressedVec::<I, T>::stored_len_(&pages_meta);
        CompressedVecIterator {
            vec: self,
            guard: self.mmap().load(),
            decoded_page: None,
            pages_meta,
            stored_len,
            index: 0,
        }
    }
}
