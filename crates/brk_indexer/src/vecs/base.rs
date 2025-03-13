use std::{
    fmt::Debug,
    io,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use brk_vec::{Compressed, StoredIndex, StoredType, Version};

use super::Height;

#[derive(Debug)]
pub struct StorableVec<I, T> {
    height: Option<Height>,
    vec: brk_vec::StorableVec<I, T>,
}

impl<I, T> StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    pub fn import(path: &Path, version: Version, compressed: Compressed) -> brk_vec::Result<Self> {
        let mut vec = brk_vec::StorableVec::forced_import(path, version, compressed)?;

        vec.init_big_cache()?;

        Ok(Self {
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
            vec,
        })
    }

    pub fn truncate_if_needed(&mut self, index: I, height: Height) -> brk_vec::Result<()> {
        if self.height.is_none_or(|self_height| self_height != height) {
            height.write(&self.path_height())?;
        }
        self.vec.truncate_if_needed(index)?;
        Ok(())
    }

    pub fn height(&self) -> brk_core::Result<Height> {
        Height::try_from(self.path_height().as_path())
    }
    fn path_height(&self) -> PathBuf {
        Self::path_height_(self.vec.path())
    }
    fn path_height_(path: &Path) -> PathBuf {
        path.join("height")
    }

    pub fn flush(&mut self, height: Height) -> io::Result<()> {
        height.write(&self.path_height())?;
        self.vec.flush()?;
        self.vec.init_big_cache()
    }
}

impl<I, T> Deref for StorableVec<I, T> {
    type Target = brk_vec::StorableVec<I, T>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl<I, T> DerefMut for StorableVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}
impl<I, T> Clone for StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn clone(&self) -> Self {
        Self {
            height: self.height,
            vec: self.vec.clone(),
        }
    }
}

pub trait AnyIndexedVec: Send + Sync {
    fn height(&self) -> brk_core::Result<Height>;
    fn flush(&mut self, height: Height) -> io::Result<()>;
}

impl<I, T> AnyIndexedVec for StorableVec<I, T>
where
    I: StoredIndex,
    T: StoredType,
{
    fn height(&self) -> brk_core::Result<Height> {
        self.height()
    }

    fn flush(&mut self, height: Height) -> io::Result<()> {
        self.flush(height)
    }
}
