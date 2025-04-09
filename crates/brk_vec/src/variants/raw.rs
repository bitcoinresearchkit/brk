use std::{
    fs,
    marker::PhantomData,
    mem,
    path::{Path, PathBuf},
    sync::Arc,
};

use arc_swap::{ArcSwap, Guard};
use axum::Json;
use memmap2::Mmap;
use rayon::prelude::*;

use crate::{AnyVec, Error, Result, StoredIndex, StoredType, UnsafeSlice, Value, Version};

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

impl<I, T> AnyVec<I, T> for RawVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    #[inline]
    fn get_(&mut self, index: usize) -> Result<Option<Value<T>>> {
        match self.index_to_pushed_index(index) {
            Ok(index) => {
                if let Some(index) = index {
                    return Ok(self.pushed().get(index).map(|v| Value::Ref(v)));
                }
            }
            Err(Error::IndexTooHigh) => return Ok(None),
            Err(Error::IndexTooLow) => {}
            Err(error) => return Err(error),
        }

        let v = if let Some(guard) = self.guard.as_ref() {
            Self::guard_to_value(guard, index)
        } else {
            Self::guard_to_value(&self.new_guard(), index)
        };

        Ok(Some(Value::Owned(v)))
    }

    fn iter_from<F>(&mut self, index: I, mut f: F) -> Result<()>
    where
        F: FnMut((I, T, &mut Self)) -> Result<()>,
    {
        if !self.is_pushed_empty() {
            return Err(Error::UnsupportedUnflushedState);
        }

        let guard = self.mmap.load();

        let start = index.to_usize()? * Self::SIZE_OF_T;

        guard[start..]
            .chunks(Self::SIZE_OF_T)
            .enumerate()
            .try_for_each(|(i, chunk)| {
                let v = T::try_read_from_bytes(chunk).unwrap();
                f((I::from(i), v, self))
            })?;

        Ok(())
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

    fn collect_range(&self, from: Option<i64>, to: Option<i64>) -> Result<Json<Vec<T>>> {
        let guard = self.mmap.load();

        let len = guard.len() / Self::SIZE_OF_T;

        if len == 0 {
            return Ok(Json(vec![]));
        }

        let from = from.map_or(0, |i| Self::fix_i64(i, len, true)) * Self::SIZE_OF_T;
        let to = to.map_or(len, |i| Self::fix_i64(i, len, false)) * Self::SIZE_OF_T;

        Ok(Json(
            guard[from * Self::SIZE_OF_T..to * Self::SIZE_OF_T]
                .chunks(Self::SIZE_OF_T)
                .map(|chunk| T::try_read_from_bytes(chunk).unwrap())
                .collect::<Vec<_>>(),
        ))
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

    #[inline]
    fn version(&self) -> Version {
        self.version
    }
}
