use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    Database, EagerVec, ImportableVec, PcoVec, Ro, Rw, StorageMode, StoredVec, VecIndex, Version,
};

use crate::internal::ComputedVecValue;

/// Sum of values in an aggregation period
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct SumVec<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw>(
    pub M::Stored<EagerVec<PcoVec<I, T>>>,
);

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> SumVec<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(EagerVec::forced_import(
            db,
            &format!("{name}_sum"),
            version,
        )?))
    }

    #[inline]
    pub(crate) fn inner(&self) -> &EagerVec<PcoVec<I, T>> {
        &self.0
    }

    pub fn read_only_clone(&self) -> SumVec<I, T, Ro> {
        SumVec(StoredVec::read_only_clone(&self.0))
    }
}
