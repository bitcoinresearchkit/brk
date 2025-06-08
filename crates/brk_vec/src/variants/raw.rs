use std::{
    fs,
    marker::PhantomData,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
    time::Duration,
};

use arc_swap::{ArcSwap, Guard};
use brk_core::{Error, Result, Value, Version};
use memmap2::Mmap;
use rayon::prelude::*;

use crate::{
    AnyCollectableVec, AnyIterableVec, AnyVec, BaseVecIterator, BoxedVecIterator, CollectableVec,
    GenericStoredVec, StoredIndex, StoredType, UnsafeSlice,
};

#[derive(Debug)]
pub struct RawVec<I, T> {
    version: Version,
    pathbuf: PathBuf,
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
    pub fn forced_import(path: &Path, version: Version) -> Result<Self> {
        let res = Self::import(path, version);
        match res {
            Err(Error::WrongEndian) | Err(Error::DifferentVersion { .. }) => {
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

        let file = Self::open_file_(Self::path_vec_(path).as_path())?;
        let mmap = Arc::new(ArcSwap::new(Self::new_mmap(file)?));

        Ok(Self {
            mmap,
            version,
            pathbuf: path.to_owned(),
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
}

impl<I, T> GenericStoredVec<I, T> for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn read_(&self, index: usize, mmap: &Mmap) -> Result<Option<T>> {
        let index = index * Self::SIZE_OF_T;
        let slice = &mmap[index..(index + Self::SIZE_OF_T)];
        T::try_read_from_bytes(slice)
            .map(|v| Some(v))
            .map_err(Error::from)
    }

    #[inline]
    fn mmap(&self) -> &ArcSwap<Mmap> {
        &self.mmap
    }

    #[inline]
    fn stored_len(&self) -> usize {
        self.mmap.load().len() / Self::SIZE_OF_T
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
    fn path(&self) -> &Path {
        self.pathbuf.as_path()
    }

    fn flush(&mut self) -> Result<()> {
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

        self.file_write_all(&bytes)?;

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

        let len = index * Self::SIZE_OF_T;

        self.file_set_len(len as u64)?;

        Ok(())
    }
}

impl<I, T> AnyVec for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn version(&self) -> Version {
        self.version
    }

    #[inline]
    fn name(&self) -> String {
        self.name_()
    }

    #[inline]
    fn len(&self) -> usize {
        self.len_()
    }

    #[inline]
    fn modified_time(&self) -> Result<Duration> {
        self.modified_time_()
    }

    #[inline]
    fn index_type_to_string(&self) -> String {
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
            version: self.version,
            pathbuf: self.pathbuf.clone(),
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
    fn path(&self) -> &Path {
        self.vec.path()
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
