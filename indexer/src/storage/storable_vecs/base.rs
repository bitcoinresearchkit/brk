use std::{
    fmt::Debug,
    io,
    ops::{Deref, DerefMut},
    path::{Path, PathBuf},
};

use storable_vec::{StoredIndex, StoredType, Version};

use super::Height;

#[derive(Debug)]
pub struct StorableVec<I, T, const MODE: u8> {
    height: Option<Height>,
    vec: storable_vec::StorableVec<I, T, MODE>,
}

impl<I, T, const MODE: u8> StorableVec<I, T, MODE>
where
    I: StoredIndex,
    T: StoredType,
{
    pub fn import(path: &Path, version: Version) -> storable_vec::Result<Self> {
        Ok(Self {
            height: Height::try_from(Self::path_height_(path).as_path()).ok(),
            vec: storable_vec::StorableVec::forced_import(path, version)?,
        })
    }

    pub fn flush(&mut self, height: Height) -> io::Result<()> {
        if self.needs(height) {
            height.write(&self.path_height())?;
        }
        self.vec.flush()
    }

    pub fn height(&self) -> storable_vec::Result<Height> {
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

impl<I, T, const MODE: u8> Deref for StorableVec<I, T, MODE> {
    type Target = storable_vec::StorableVec<I, T, MODE>;
    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}
impl<I, T, const MODE: u8> DerefMut for StorableVec<I, T, MODE> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

pub trait AnyStorableVec: Send + Sync {
    fn height(&self) -> storable_vec::Result<Height>;
    fn flush(&mut self, height: Height) -> io::Result<()>;
}

impl<I, T, const MODE: u8> AnyStorableVec for StorableVec<I, T, MODE>
where
    I: StoredIndex,
    T: StoredType,
{
    fn height(&self) -> storable_vec::Result<Height> {
        self.height()
    }

    fn flush(&mut self, height: Height) -> io::Result<()> {
        self.flush(height)
    }
}
