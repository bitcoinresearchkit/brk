use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    Database, EagerVec, ImportableVec, PcoVec, Ro, Rw, StorageMode, StoredVec, VecIndex, Version,
};

use crate::internal::ComputedVecValue;

/// Cumulative sum across aggregation periods
#[derive(Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct CumulativeVec<I: VecIndex, T: ComputedVecValue + JsonSchema, M: StorageMode = Rw>(
    pub M::Stored<EagerVec<PcoVec<I, T>>>,
);

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> CumulativeVec<I, T> {
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(EagerVec::forced_import(
            db,
            &format!("{name}_cumulative"),
            version,
        )?))
    }

    #[inline]
    pub(crate) fn inner(&self) -> &EagerVec<PcoVec<I, T>> {
        &self.0
    }

    pub fn read_only_clone(&self) -> CumulativeVec<I, T, Ro> {
        CumulativeVec(StoredVec::read_only_clone(&self.0))
    }
}
