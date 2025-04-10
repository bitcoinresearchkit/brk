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

use crate::{DynamicVec, Error, GenericVec, Result, StoredIndex, StoredType, UnsafeSlice, Version};

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
    fn cached_get_stored_(&mut self, index: usize, mmap: &Mmap) -> Result<Option<T>> {
        self.get_stored_(index, mmap)
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
}

impl<I, T> GenericVec<I, T> for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn iter_from<F>(&mut self, index: I, mut f: F) -> Result<()>
    where
        F: FnMut((I, T, &mut dyn DynamicVec<I = Self::I, T = Self::T>)) -> Result<()>,
    {
        if !self.is_pushed_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let start = index.to_usize()?;

        let stored_len = self.stored_len();
        if start >= stored_len {
            return Ok(());
        }

        let guard = self.mmap.load();

        (start..stored_len).try_for_each(|index| {
            let v = self.get_stored_(index, &guard)?.unwrap();
            f((I::from(index), v, self as &mut dyn DynamicVec<I = I, T = T>))
        })
    }

    fn collect_range(&self, from: Option<usize>, to: Option<usize>) -> Result<Vec<T>> {
        if !self.is_pushed_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let stored_len = self.stored_len();

        let from = from.unwrap_or_default();
        let to = to.map_or(stored_len, |i| i.min(stored_len));

        if from >= stored_len {
            return Ok(vec![]);
        }

        let mmap = self.mmap.load();

        (from..to)
            .map(|index| self.get_stored_(index, &mmap).map(|opt| opt.unwrap()))
            .collect::<Result<Vec<_>>>()
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
    fn path(&self) -> &Path {
        self.pathbuf.as_path()
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
