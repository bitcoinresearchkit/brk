use std::{
    fs::{self, File},
    io,
    marker::PhantomData,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};

use arc_swap::{ArcSwap, Guard};
use brk_core::{Error, Result, Value, Version};
use memmap2::Mmap;
use rayon::prelude::*;

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator, BoxedVecIterator, CollectableVec,
    Format, GenericStoredVec, HEADER_OFFSET, Header, StoredIndex, StoredType, UnsafeSlice,
};

const VERSION: Version = Version::ONE;

#[derive(Debug)]
pub struct RawVec<I, T> {
    header: Header,
    parent: PathBuf,
    name: String,
    // file: Option<File>,
    // Consider  Arc<ArcSwap<Option<Mmap>>> for dataraces when reorg ?
    mmap: Arc<ArcSwap<Mmap>>,
    pushed: Vec<T>,
    phantom: PhantomData<I>,
}

impl<I, T> RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    /// Same as import but will reset the folder under certain errors, so be careful !
    pub fn forced_import(parent: &Path, name: &str, mut version: Version) -> Result<Self> {
        version = version + VERSION;
        let res = Self::import(parent, name, version);
        match res {
            Err(Error::DifferentCompressionMode)
            | Err(Error::WrongEndian)
            | Err(Error::WrongLength)
            | Err(Error::DifferentVersion { .. }) => {
                let path = Self::path_(parent, name);
                fs::remove_file(path)?;
                Self::import(parent, name, version)
            }
            _ => res,
        }
    }

    pub fn import(parent: &Path, name: &str, version: Version) -> Result<Self> {
        let path = Self::path_(parent, name);
        let (mmap, header) = match Self::open_file_(&path) {
            Ok(mut file) => {
                if file.metadata()?.len() == 0 {
                    let header = Header::create_and_write(&mut file, version, Format::Raw)?;
                    let mmap = Self::new_mmap(&file)?;
                    (mmap, header)
                } else {
                    let mmap = Self::new_mmap(&file)?;
                    // dbg!(&mmap[..]);
                    let header = Header::import_and_verify(&mmap, version, Format::Raw)?;
                    // dbg!((&header, name, I::to_string()));
                    (mmap, header)
                }
            }
            Err(e) => match e.kind() {
                io::ErrorKind::NotFound => {
                    fs::create_dir_all(Self::folder_(parent, name))?;
                    let mut file = Self::open_file_(&path)?;
                    let header = Header::create_and_write(&mut file, version, Format::Raw)?;
                    let mmap = Self::new_mmap(&file)?;
                    (mmap, header)
                }
                _ => return Err(e.into()),
            },
        };

        let mmap = Arc::new(ArcSwap::new(mmap));

        Ok(Self {
            mmap,
            header,
            // file: Some(file),
            name: name.to_string(),
            parent: parent.to_owned(),
            pushed: vec![],
            phantom: PhantomData,
        })
    }

    #[inline]
    pub fn iter(&self) -> RawVecIterator<'_, I, T> {
        self.into_iter()
    }

    #[inline]
    pub fn iter_at(&self, i: I) -> RawVecIterator<'_, I, T> {
        self.iter_at_(i.unwrap_to_usize())
    }

    #[inline]
    pub fn iter_at_(&self, i: usize) -> RawVecIterator<'_, I, T> {
        let mut iter = self.into_iter();
        iter.set_(i);
        iter
    }

    pub fn write_header_if_needed(&mut self) -> io::Result<Option<File>> {
        if self.header.modified() {
            let mut file = self.open_file()?;
            self.header.write(&mut file)?;
            Ok(Some(file))
        } else {
            Ok(None)
        }
    }
}

impl<I, T> GenericStoredVec<I, T> for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn read_(&self, index: usize, mmap: &Mmap) -> Result<Option<T>> {
        let index = index * Self::SIZE_OF_T + HEADER_OFFSET;
        let slice = &mmap[index..(index + Self::SIZE_OF_T)];
        T::try_read_from_bytes(slice)
            .map(|v| Some(v))
            .map_err(Error::from)
    }

    fn header(&self) -> &Header {
        &self.header
    }

    fn mut_header(&mut self) -> &mut Header {
        &mut self.header
    }

    #[inline]
    fn mmap(&self) -> &ArcSwap<Mmap> {
        &self.mmap
    }

    #[inline]
    fn stored_len(&self) -> usize {
        self.stored_len_(&self.mmap.load())
    }
    #[inline]
    fn stored_len_(&self, mmap: &Mmap) -> usize {
        (mmap.len() - HEADER_OFFSET) / Self::SIZE_OF_T
    }

    #[inline]
    fn pushed(&self) -> &[T] {
        self.pushed.as_slice()
    }
    #[inline]
    fn mut_pushed(&mut self) -> &mut Vec<T> {
        &mut self.pushed
    }

    #[inline]
    fn parent(&self) -> &Path {
        &self.parent
    }

    fn flush(&mut self) -> Result<()> {
        let file_opt = self.write_header_if_needed()?;

        let pushed_len = self.pushed_len();

        if pushed_len == 0 {
            return Ok(());
        }

        let bytes = {
            let pushed = &mut self.pushed;

            let mut bytes: Vec<u8> = vec![0; pushed.len() * Self::SIZE_OF_T];

            let unsafe_bytes = UnsafeSlice::new(&mut bytes);

            mem::take(pushed)
                .into_par_iter()
                .enumerate()
                .for_each(|(i, v)| unsafe_bytes.copy_slice(i * Self::SIZE_OF_T, v.as_bytes()));

            bytes
        };

        let mut file = file_opt.unwrap_or(self.open_file()?);

        self.file_write_all(&mut file, &bytes)?;

        Ok(())
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

        let len = index * Self::SIZE_OF_T + HEADER_OFFSET;

        let mut file = self.open_file()?;
        self.file_set_len(&mut file, len as u64)?;

        Ok(())
    }

    fn reset(&mut self) -> Result<()> {
        self.reset_()
    }
}

impl<I, T> AnyVec for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn version(&self) -> Version {
        self.header.vec_version()
    }

    #[inline]
    fn name(&self) -> &str {
        &self.name
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

impl<I, T> Clone for RawVec<I, T> {
    fn clone(&self) -> Self {
        Self {
            header: self.header.clone(),
            parent: self.parent.clone(),
            name: self.name.clone(),
            mmap: self.mmap.clone(),
            pushed: vec![],
            phantom: PhantomData,
        }
    }
}

#[derive(Debug)]
pub struct RawVecIterator<'a, I, T> {
    vec: &'a RawVec<I, T>,
    guard: Guard<Arc<Mmap>>,
    index: usize,
}

impl<I, T> BaseVecIterator for RawVecIterator<'_, I, T>
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

impl<'a, I, T> Iterator for RawVecIterator<'a, I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Value<'a, T>);

    fn next(&mut self) -> Option<Self::Item> {
        let mmap = &self.guard;
        let index = self.index;

        let opt = self
            .vec
            .get_or_read_(index, mmap)
            .unwrap()
            .map(|v| (I::from(index), v));

        if opt.is_some() {
            self.index += 1;
        }

        opt
    }
}

impl<'a, I, T> IntoIterator for &'a RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type Item = (I, Value<'a, T>);
    type IntoIter = RawVecIterator<'a, I, T>;

    fn into_iter(self) -> Self::IntoIter {
        RawVecIterator {
            vec: self,
            guard: self.mmap.load(),
            index: 0,
        }
    }
}

impl<I, T> AnyIterableVec<I, T> for RawVec<I, T>
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

impl<I, T> AnyCollectableVec for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn collect_range_serde_json(
        &self,
        from: Option<i64>,
        to: Option<i64>,
    ) -> Result<Vec<serde_json::Value>> {
        CollectableVec::collect_range_serde_json(self, from, to)
    }
}
