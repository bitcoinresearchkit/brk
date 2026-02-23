//! ComputedFromHeight using SumCum aggregation.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    Database, EagerVec, Exit, ImportableVec, PcoVec, ReadableCloneableVec, Rw, StorageMode,
};

use crate::{ComputeIndexes, indexes};

use crate::internal::{ComputedHeightDerivedSumCum, ComputedVecValue, NumericValue};

#[derive(Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedFromHeightSumCum<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    #[traversable(rename = "sum")]
    pub height: M::Stored<EagerVec<PcoVec<Height, T>>>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub rest: Box<ComputedHeightDerivedSumCum<T, M>>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedFromHeightSumCum<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, v)?;

        let rest =
            ComputedHeightDerivedSumCum::forced_import(db, name, height.read_only_boxed_clone(), v, indexes)?;

        Ok(Self { height, rest: Box::new(rest) })
    }

    /// Compute height_cumulative from self.height.
    pub(crate) fn compute_cumulative(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.rest.derive_from(starting_indexes, &self.height, exit)
    }

    pub(crate) fn compute(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: impl FnMut(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    ) -> Result<()> {
        compute(&mut self.height)?;
        self.compute_cumulative(starting_indexes, exit)
    }
}
