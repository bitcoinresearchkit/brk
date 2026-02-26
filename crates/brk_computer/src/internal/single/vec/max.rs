use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    Database, EagerVec, ImportableVec, PcoVec, Ro, Rw, StorageMode, StoredVec, VecIndex, Version,
};

use crate::internal::ComputedVecValue;

/// Maximum value in an aggregation period
#[derive(Deref, DerefMut, Traversable)]
pub struct MaxVec<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw>(
    pub M::Stored<EagerVec<PcoVec<I, T>>>,
);

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> MaxVec<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(EagerVec::forced_import(
            db,
            &format!("{name}_max"),
            version,
        )?))
    }

    pub fn read_only_clone(&self) -> MaxVec<I, T, Ro> {
        MaxVec(StoredVec::read_only_clone(&self.0))
    }
}
