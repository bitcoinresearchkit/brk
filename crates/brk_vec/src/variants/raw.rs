use std::{
    fs,
    marker::PhantomData,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};

use arc_swap::{ArcSwap, Guard};
use memmap2::Mmap;
use rayon::prelude::*;

use crate::{
    DynamicVec, Error, GenericVec, Result, StoredIndex, StoredType, UnsafeSlice, Value, Version,
};

#[derive(Debug)]
pub struct RawVec<I, T> {
    version: Version,
    pathbuf: PathBuf,
    // Consider  Arc<ArcSwap<Option<Mmap>>> for dataraces when reorg ?
    mmap: Arc<ArcSwap<Mmap>>,
    guard: Option<Guard<Arc<Mmap>>>,
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
        let guard = Some(mmap.load());

        Ok(Self {
            mmap,
            guard,
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

impl<I, T> DynamicVec for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    type I = I;
    type T = T;

    #[inline]
    fn get_stored_(&self, index: usize, mmap: &Mmap) -> Result<Option<T>> {
        let index = index * Self::SIZE_OF_T;
        let slice = &mmap[index..(index + Self::SIZE_OF_T)];
        Self::T::try_read_from_bytes(slice)
            .map(|v| Some(v))
            .map_err(Error::from)
    }

    #[inline]
    fn mmap(&self) -> &ArcSwap<Mmap> {
        &self.mmap
    }

    #[inline]
    fn guard(&self) -> &Option<Guard<Arc<Mmap>>> {
        &self.guard
    }
    #[inline]
    fn mut_guard(&mut self) -> &mut Option<Guard<Arc<Mmap>>> {
        &mut self.guard
    }

    #[inline]
    fn stored_len(&self) -> usize {
        if let Some(guard) = self.guard() {
            guard.len() / Self::SIZE_OF_T
        } else {
            self.mmap.load().len() / Self::SIZE_OF_T
        }
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
}

impl<I, T> GenericVec<I, T> for RawVec<I, T>
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

    #[inline]
    fn version(&self) -> Version {
        self.version
    }
}

impl<I, T> Clone for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn clone(&self) -> Self {
        Self {
            version: self.version,
            pathbuf: self.pathbuf.clone(),
            // Consider  Arc<ArcSwap<Option<Mmap>>> for dataraces when reorg ?
            mmap: self.mmap.clone(),
            guard: None,
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

impl<I, T> RawVecIterator<'_, I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    const SIZE_OF_T: usize = size_of::<T>();

    #[inline]
    pub fn set(&mut self, i: I) -> &mut Self {
        self.index = i.unwrap_to_usize();
        self
    }

    #[inline]
    pub fn set_(&mut self, i: usize) {
        self.index = i;
    }

    #[inline]
    pub fn get(&mut self, i: I) -> Option<(I, Value<'_, T>)> {
        self.set(i).next()
    }

    #[inline]
    pub fn get_(&mut self, i: usize) -> Option<(I, Value<'_, T>)> {
        self.set_(i);
        self.next()
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
        let vec = self.vec;
        let i = self.index;

        let stored_len = mmap.len() / Self::SIZE_OF_T;

        let result = if i >= stored_len {
            let j = i - stored_len;
            if j >= vec.pushed_len() {
                return None;
            }
            vec.pushed().get(j).map(|v| (I::from(i), Value::Ref(v)))
        } else {
            vec.get_stored_(i, mmap)
                .unwrap()
                .map(|v| (I::from(i), Value::Owned(v)))
        };

        self.index += 1;
        result
    }

    #[inline]
    fn last(mut self) -> Option<Self::Item>
    where
        Self: Sized,
    {
        let len = self.vec.len();
        if len == 0 {
            return None;
        }
        self.get_(len - 1)
            .map(|(i, v)| (i, Value::Owned(v.into_inner())))
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
