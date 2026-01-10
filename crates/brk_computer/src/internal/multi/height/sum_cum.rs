//! ComputedBlock using SumCum aggregation.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{
    AnyStoredVec, AnyVec, Database, EagerVec, Exit, GenericStoredVec, ImportableVec,
    IterableCloneableVec, IterableVec, PcoVec, VecIndex,
};

use crate::{indexes, ComputeIndexes};

use crate::internal::{ComputedVecValue, ComputedDerivedBlockSumCum, NumericValue};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedBlockSumCum<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    #[traversable(rename = "sum")]
    pub height: EagerVec<PcoVec<Height, T>>,
    #[deref]
    #[deref_mut]
    pub rest: ComputedDerivedBlockSumCum<T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedBlockSumCum<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v = version + VERSION;

        let height: EagerVec<PcoVec<Height, T>> = EagerVec::forced_import(db, name, v)?;

        let rest = ComputedDerivedBlockSumCum::forced_import(
            db,
            name,
            height.boxed_clone(),
            v,
            indexes,
        )?;

        Ok(Self { height, rest })
    }

    pub fn compute_all<F>(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    {
        compute(&mut self.height)?;
        self.compute_rest(indexes, starting_indexes, exit)
    }

    /// Compute rest from self.height (for stateful computation patterns).
    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.rest.derive_from(indexes, starting_indexes, &self.height, exit)
    }

    /// Derive from an external height source (e.g., a LazyVec).
    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        source: &impl IterableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()> {
        let target_len = source.len();
        let starting_height = starting_indexes.height.to_usize().min(self.height.len());

        self.height
            .validate_computed_version_or_reset(source.version())?;

        let mut source_iter = source.iter();
        for h_idx in starting_height..target_len {
            let height = Height::from(h_idx);
            let value = source_iter.get_unwrap(height);
            self.height.truncate_push(height, value)?;
        }
        self.height.write()?;

        self.rest.derive_from(indexes, starting_indexes, &self.height, exit)
    }
}
