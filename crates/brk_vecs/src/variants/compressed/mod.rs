use std::{
    borrow::Cow,
    collections::{BTreeMap, BTreeSet},
    fs, mem,
    sync::Arc,
};

use brk_core::{Error, Result, Version};
use memmap2::Mmap;
use parking_lot::{RwLock, RwLockReadGuard};
use rayon::prelude::*;
use zstd::DEFAULT_COMPRESSION_LEVEL;

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator, BoxedVecIterator, CollectableVec,
    File, GenericStoredVec, HEADER_OFFSET, Header, RawVec, Reader, StoredIndex, StoredType,
    UnsafeSlice,
};

mod compressed_page_meta;
mod compressed_pages_meta;

use compressed_page_meta::*;
use compressed_pages_meta::*;

const ONE_KIB: usize = 1024;
const ONE_MIB: usize = ONE_KIB * ONE_KIB;
pub const MAX_CACHE_SIZE: usize = 100 * ONE_MIB;
pub const MAX_PAGE_SIZE: usize = 64 * ONE_KIB;

const VERSION: Version = Version::TWO;

#[derive(Debug)]
pub struct CompressedVec<I, T> {
    inner: RawVec<I, T>,
    pages_meta: Arc<RwLock<CompressedPagesMetadata>>,
}

impl<I, T> CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub const PER_PAGE: usize = MAX_PAGE_SIZE / Self::SIZE_OF_T;
    pub const PAGE_SIZE: usize = Self::PER_PAGE * Self::SIZE_OF_T;
    pub const CACHE_LENGTH: usize = MAX_CACHE_SIZE / Self::PAGE_SIZE;

    /// Same as import but will reset the vec under certain errors, so be careful !
    pub fn forced_import(file: &Arc<File>, name: &str, mut version: Version) -> Result<Self> {
        version = version + VERSION;
        let res = Self::import(file, name, version);
        match res {
            Err(Error::DifferentCompressionMode)
            | Err(Error::WrongEndian)
            | Err(Error::WrongLength)
            | Err(Error::DifferentVersion { .. }) => {
                todo!();

                // let path = Self::path_(file, name);
                // fs::remove_file(path)?;
                // Self::import(file, name, version)
            }
            _ => res,
        }
    }

    #[allow(unreachable_code, unused_variables)]
    pub fn import(file: &Arc<File>, name: &str, version: Version) -> Result<Self> {
        // let mut inner = RawVec::import(file, name, version)?;

        todo!();

        // let pages_meta = {
        //     let path = inner
        //         .folder()
        //         .join(format!("{}-pages-meta", I::to_string()));
        //     if inner.is_empty() {
        //         let _ = fs::remove_file(&path);
        //     }
        //     CompressedPagesMetadata::read(&path)?
        // };

        // inner.set_stored_len(if let Some(last) = pages_meta.last() {
        //     (pages_meta.len() - 1) * Self::PER_PAGE + last.values_len as usize
        // } else {
        //     0
        // });

        // Ok(Self {
        //     inner,
        //     pages_meta: Arc::new(RwLock::new(pages_meta)),
        // })
    }

    fn decode_page(&self, page_index: usize, reader: &Reader) -> Result<Vec<T>> {
        Self::decode_page_(
            self.stored_len(),
            page_index,
            reader,
            &self.pages_meta.read(),
        )
    }

    fn decode_page_(
        stored_len: usize,
        page_index: usize,
        reader: &Reader,
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

        let slice = reader.read(offset as u64, (offset + len) as u64);

        Ok(zstd::decode_all(slice)
            .inspect_err(|_| {
                dbg!((len, offset, page_index, slice));
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

    #[inline]
    fn index_to_page_index(index: usize) -> usize {
        index / Self::PER_PAGE
    }

    #[inline]
    fn page_index_to_index(page_index: usize) -> usize {
        page_index * Self::PER_PAGE
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

impl<I, T> GenericStoredVec<I, T> for CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn file(&self) -> &File {
        self.inner.file()
    }

    fn region_index(&self) -> usize {
        self.inner.region_index()
    }

    #[inline]
    fn read_(&self, index: usize, reader: &Reader) -> Result<Option<T>> {
        let page_index = Self::index_to_page_index(index);
        let decoded_index = index % Self::PER_PAGE;

        Ok(self
            .decode_page(page_index, reader)?
            .get(decoded_index)
            .cloned())
    }

    fn header(&self) -> &Header {
        self.inner.header()
    }

    fn mut_header(&mut self) -> &mut Header {
        self.inner.mut_header()
    }

    #[inline]
    fn stored_len(&self) -> usize {
        self.inner.stored_len()
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
    fn holes(&self) -> &BTreeSet<usize> {
        self.inner.holes()
    }
    #[inline]
    fn mut_holes(&mut self) -> &mut BTreeSet<usize> {
        panic!("unsupported")
    }
    #[inline]
    fn updated(&self) -> &BTreeMap<usize, T> {
        self.inner.updated()
    }
    #[inline]
    fn mut_updated(&mut self) -> &mut BTreeMap<usize, T> {
        panic!("unsupported")
    }

    fn flush(&mut self) -> Result<()> {
        todo!();

        // let file_opt = self.inner.write_header_if_needed()?;

        // let pushed_len = self.pushed_len();

        // if pushed_len == 0 {
        //     return Ok(());
        // }

        // let stored_len = self.stored_len();

        // let mut file = file_opt.unwrap_or(self.open_file()?);

        // let mut pages_meta = self.pages_meta.read();

        // let mut starting_page_index = pages_meta.len();
        // let mut values = vec![];
        // let mut truncate_at = None;

        // if self.stored_len() % Self::PER_PAGE != 0 {
        //     if pages_meta.is_empty() {
        //         unreachable!()
        //     }

        //     let last_page_index = pages_meta.len() - 1;

        //     let mmap = unsafe { Mmap::map(&file)? };

        //     values = Self::decode_page_(stored_len, last_page_index, &mmap, &pages_meta)
        //         .inspect_err(|_| {
        //             dbg!(last_page_index, &pages_meta);
        //         })
        //         .unwrap();

        //     truncate_at.replace(pages_meta.pop().unwrap().start);
        //     starting_page_index = last_page_index;
        // }

        // let compressed = values
        //     .into_par_iter()
        //     .chain(mem::take(self.mut_pushed()).into_par_iter())
        //     .chunks(Self::PER_PAGE)
        //     .map(|chunk| (Self::compress_page(chunk.as_ref()), chunk.len()))
        //     .collect::<Vec<_>>();

        // compressed
        //     .iter()
        //     .enumerate()
        //     .for_each(|(i, (compressed_bytes, values_len))| {
        //         let page_index = starting_page_index + i;

        //         let start = if page_index != 0 {
        //             let prev = pages_meta.get(page_index - 1).unwrap();
        //             prev.start + prev.bytes_len as u64
        //         } else {
        //             0
        //         };
        //         let offsetted_start = start + HEADER_OFFSET as u64;

        //         let bytes_len = compressed_bytes.len() as u32;
        //         let values_len = *values_len as u32;

        //         let page = CompressedPageMetadata::new(offsetted_start, bytes_len, values_len);

        //         pages_meta.push(page_index, page);
        //     });

        // let buf = compressed
        //     .into_iter()
        //     .flat_map(|(v, _)| v)
        //     .collect::<Vec<_>>();

        // pages_meta.write()?;

        // if let Some(truncate_at) = truncate_at {
        //     self.file_set_len(&mut file, truncate_at)?;
        // }

        // self.file_write_all(&mut file, &buf)?;

        // self.pages_meta.store(Arc::new(pages_meta));

        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        // let mut pages_meta = (**self.pages_meta.load()).clone();
        // pages_meta.truncate(0);
        // pages_meta.write()?;
        // self.pages_meta.store(Arc::new(pages_meta));
        self.reset_()
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

        let mut pages_meta = self.pages_meta.write();

        let page_index = Self::index_to_page_index(index);

        let reader = self.create_static_reader();
        let values = self.decode_page(page_index, &reader)?;
        drop(reader);

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

        // self.file_truncate_and_write_all(&mut file, len, &buf)?;

        Ok(())
    }
}

impl<I, T> AnyVec for CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn version(&self) -> Version {
        self.inner.version()
    }

    #[inline]
    fn name(&self) -> &str {
        self.inner.name()
    }

    #[inline]
    fn len(&self) -> usize {
        self.len_()
    }

    #[inline]
    fn index_type_to_string(&self) -> &'static str {
        I::to_string()
    }

    #[inline]
    fn value_type_to_size_of(&self) -> usize {
        size_of::<T>()
    }
}

impl<I, T> Clone for CompressedVec<I, T> {
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
    reader: Reader<'a>,
    decoded_page: Option<(usize, Vec<T>)>,
    pages_meta: RwLockReadGuard<'a, CompressedPagesMetadata>,
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

impl<I, T> BaseVecIterator for CompressedVecIterator<'_, I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn mut_index(&mut self) -> &mut usize {
        &mut self.index
    }

    #[inline]
    fn len(&self) -> usize {
        self.vec.len()
    }

    #[inline]
    fn name(&self) -> &str {
        self.vec.name()
    }
}

impl<'a, I, T> Iterator for CompressedVecIterator<'a, I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Cow<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
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
                .map(|v| (I::from(i), Cow::Borrowed(v)))
        } else {
            let page_index = i / Self::PER_PAGE;

            if self.decoded_page.as_ref().is_none_or(|b| b.0 != page_index) {
                let values = CompressedVec::<I, T>::decode_page_(
                    stored_len,
                    page_index,
                    &self.reader,
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
                .map(|v| (I::from(i), Cow::Owned(v.clone())))
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
    type Item = (I, Cow<'a, T>);
    type IntoIter = CompressedVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        let pages_meta = self.pages_meta.read();
        let stored_len = self.stored_len();

        CompressedVecIterator {
            vec: self,
            reader: self.create_static_reader(),
            decoded_page: None,
            pages_meta,
            index: 0,
            stored_len,
        }
    }
}

impl<I, T> AnyIterableVec<I, T> for CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn boxed_iter<'a>(&'a self) -> BoxedVecIterator<'a, I, T>
    where
        T: 'a,
    {
        Box::new(self.into_iter())
    }
}

impl<I, T> AnyCollectableVec for CompressedVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn collect_range_serde_json(
        &self,
        from: Option<usize>,
        to: Option<usize>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
