use std::{
    fmt::Debug,
    io,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use storable_vec::{StorableVecIndex, StorableVecType, Version};

use super::Height;

#[derive(Debug)]
pub struct StorableVec<I, T> {
    height: Option<Height>,
    vec: storable_vec::StorableVec<I, T>,
}

impl<I, T> StorableVec<I, T>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    pub fn import(path: &Path, version: Version) -> io::Result<Self> {
        Ok(Self {
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
            vec: storable_vec::StorableVec::import(path, version)?,
        })
    }

    pub fn flush(&mut self, height: Height) -> io::Result<()> {
        if self.needs(height) {
            height.write(&self.path_height())?;
        }
        self.vec.flush()
    }

    pub fn height(&self) -> color_eyre::Result<Height> {
        Height::try_from(self.path_height().as_path())
    }
    fn path_height(&self) -> PathBuf {
        Self::path_height_(self.vec.path())
    }
    fn path_height_(path: &Path) -> PathBuf {
        path.join("height")
    }

    pub fn needs(&self, height: Height) -> bool {
        self.height.is_none_or(|self_height| height > self_height)
    }
    #[allow(unused)]
    pub fn has(&self, height: Height) -> bool {
        !self.needs(height)
    }
}

impl<I, T> Deref for StorableVec<I, T> {
    type Target = storable_vec::StorableVec<I, T>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl<I, T> DerefMut for StorableVec<I, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

pub trait AnyStorableVec {
    fn height(&self) -> color_eyre::Result<Height>;
    fn flush(&mut self, height: Height) -> io::Result<()>;
    fn reset_cache(&mut self);
}

impl<I, T> AnyStorableVec for StorableVec<I, T>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    fn height(&self) -> color_eyre::Result<Height> {
        self.height()
    }

    fn reset_cache(&mut self) {
        self.vec.reset_cache()
    }

    fn flush(&mut self, height: Height) -> io::Result<()> {
        self.flush(height)
    }
}
