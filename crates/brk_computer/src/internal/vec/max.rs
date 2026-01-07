use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, ImportableVec, PcoVec, VecIndex, Version};

use crate::internal::ComputedVecValue;

/// Maximum value in an aggregation period
#[derive(Clone, Deref, DerefMut, Traversable)]
pub struct MaxVec<I: VecIndex, T: ComputedVecValue + JsonSchema>(
    pub EagerVec<PcoVec<I, T>>,
);

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> MaxVec<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(EagerVec::forced_import(db, &format!("{name}_max"), version)?))
    }

    #[inline]
    pub fn inner(&self) -> &EagerVec<PcoVec<I, T>> {
        &self.0
    }
}
