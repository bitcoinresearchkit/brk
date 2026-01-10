use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, ImportableVec, IterableBoxedVec, IterableCloneableVec, PcoVec, VecIndex, Version};

use crate::internal::ComputedVecValue;

/// Sum of values in an aggregation period
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct SumVec<I: VecIndex, T: ComputedVecValue + JsonSchema>(
    pub EagerVec<PcoVec<I, T>>,
);

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> SumVec<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(EagerVec::forced_import(db, &format!("{name}_sum"), version)?))
    }

    /// Import with raw name (no suffix) for backwards compat
    pub fn forced_import_raw(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(EagerVec::forced_import(db, name, version)?))
    }

    #[inline]
    pub fn inner(&self) -> &EagerVec<PcoVec<I, T>> {
        &self.0
    }

    pub fn boxed_clone(&self) -> IterableBoxedVec<I, T> {
        self.0.boxed_clone()
    }
}
