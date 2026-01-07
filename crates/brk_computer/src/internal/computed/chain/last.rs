//! ComputedChain for last-value aggregation.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{DifficultyEpoch, Height, Version};
use schemars::JsonSchema;
use vecdb::{Database, EagerVec, Exit, ImportableVec, IterableCloneableVec, PcoVec};

use crate::{ComputeIndexes, indexes};

use crate::internal::{ComputedVecValue, LazyLast, NumericValue};

#[derive(Clone, Traversable)]
#[traversable(merge)]
pub struct ComputedChainLast<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: EagerVec<PcoVec<Height, T>>,
    pub difficultyepoch: LazyLast<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedChainLast<T>
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

        let difficultyepoch = LazyLast::from_source(
            name,
            v,
            height.boxed_clone(),
            indexes
                .block
                .difficultyepoch_to_difficultyepoch
                .boxed_clone(),
        );

        Ok(Self {
            height,
            difficultyepoch,
        })
    }

    pub fn compute<F>(
        &mut self,
        _starting_indexes: &ComputeIndexes,
        _exit: &Exit,
        mut compute: F,
    ) -> Result<()>
    where
        F: FnMut(&mut EagerVec<PcoVec<Height, T>>) -> Result<()>,
    {
        compute(&mut self.height)?;
        Ok(())
    }
}
