use brk_error::Result;
use brk_traversable::Traversable;
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, ImportableVec, IterableBoxedVec, IterableCloneableVec, PcoVec, VecIndex, Version};

use crate::internal::ComputedVecValue;

/// Cumulative sum across aggregation periods
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(transparent)]
pub struct CumulativeVec<I: VecIndex, T: ComputedVecValue + JsonSchema>(
    pub EagerVec<PcoVec<I, T>>,
);

impl<I: VecIndex, T: ComputedVecValue + JsonSchema> CumulativeVec<I, T> {
    pub fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        Ok(Self(EagerVec::forced_import(db, &format!("{name}_cumulative"), version)?))
    }

    #[inline]
    pub fn inner(&self) -> &EagerVec<PcoVec<I, T>> {
        &self.0
    }

    pub fn boxed_clone(&self) -> IterableBoxedVec<I, T> {
        self.0.boxed_clone()
    }
}
