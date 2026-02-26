use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    Database, EagerVec, ImportableVec, PcoVec, Ro, Rw, StorageMode, StoredVec, VecIndex, Version,
};

use crate::internal::ComputedVecValue;

/// Minimum value in an aggregation period
#[derive(Deref, DerefMut, Traversable)]
pub struct MinVec<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw>(
    pub M::Stored<EagerVec<PcoVec<I, T>>>,
);

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> MinVec<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(EagerVec::forced_import(
            db,
            &format!("{name}_min"),
            version,
        )?))
    }

    pub fn read_only_clone(&self) -> MinVec<I, T, Ro> {
        MinVec(StoredVec::read_only_clone(&self.0))
    }
}
