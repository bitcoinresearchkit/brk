use std::{fmt::Debug, path::Path};

use derive_deref::{Deref, DerefMut};
use storable_vec::{StorableVecIndex, StorableVecType, Version};

#[derive(Debug, Deref, DerefMut)]
pub struct StorableVec<I, T, const MODE: u8>(storable_vec::StorableVec<I, T, MODE>);

impl<I, T, const MODE: u8> StorableVec<I, T, MODE>
where
    I: StorableVecIndex,
    T: StorableVecType,
{
    pub fn import(path: &Path, version: Version) -> storable_vec::Result<Self> {
        Ok(Self(storable_vec::StorableVec::forced_import(path, version)?))
    }
}
