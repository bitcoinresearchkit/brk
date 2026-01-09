//! DerivedComputedBlockDistribution - dateindex storage + lazy time periods + difficultyepoch.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, IterableBoxedVec, IterableCloneableVec, IterableVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ComputedVecValue, DerivedDateDistribution, Distribution, LazyDistribution, NumericValue,
    },
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct DerivedComputedBlockDistribution<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub dateindex: Distribution<DateIndex, T>,
    #[deref]
    #[deref_mut]
    pub dates: DerivedDateDistribution<T>,
    pub difficultyepoch: LazyDistribution<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> DerivedComputedBlockDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        height_source: IterableBoxedVec<Height, T>,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let dateindex = Distribution::forced_import(db, name, version + VERSION)?;
        let v = version + VERSION;

        let dates = DerivedDateDistribution::from_sources(
            name,
            v,
            dateindex.average.0.boxed_clone(),
            dateindex.minmax.min.0.boxed_clone(),
            dateindex.minmax.max.0.boxed_clone(),
            indexes,
        );

        let difficultyepoch = LazyDistribution::from_distribution(
            name,
            v,
            height_source.boxed_clone(),
            height_source.boxed_clone(),
            height_source,
            indexes.difficultyepoch.identity.boxed_clone(),
        );

        Ok(Self {
            dateindex,
            dates,
            difficultyepoch,
        })
    }

    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        height_source: &impl IterableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()> {
        self.dateindex.compute(
            starting_indexes.dateindex,
            height_source,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )?;

        Ok(())
    }
}
