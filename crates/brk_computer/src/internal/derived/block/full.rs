//! ComputedDerivedBlockFull - height_cumulative + dateindex storage + difficultyepoch + lazy time periods.

use brk_error::Result;

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{Database, Exit, IterableBoxedVec, IterableCloneableVec, IterableVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ComputedVecValue, CumulativeVec, LazyPeriodsFull, Full, LazyFull, NumericValue,
        compute_cumulative_extend,
    },
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedDerivedBlockFull<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    #[traversable(rename = "cumulative")]
    pub height_cumulative: CumulativeVec<Height, T>,
    pub dateindex: Full<DateIndex, T>,
    #[deref]
    #[deref_mut]
    pub dates: LazyPeriodsFull<T>,
    pub difficultyepoch: LazyFull<DifficultyEpoch, T, Height, DifficultyEpoch>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedDerivedBlockFull<T>
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
        let v = version + VERSION;
        let height_cumulative = CumulativeVec::forced_import(db, name, v)?;
        let dateindex = Full::forced_import(db, name, v)?;

        Ok(Self {
            dates: LazyPeriodsFull::from_sources(
                name,
                v,
                dateindex.distribution.average.0.boxed_clone(),
                dateindex.distribution.minmax.min.0.boxed_clone(),
                dateindex.distribution.minmax.max.0.boxed_clone(),
                dateindex.sum_cum.sum.0.boxed_clone(),
                dateindex.sum_cum.cumulative.0.boxed_clone(),
                indexes,
            ),
            difficultyepoch: LazyFull::from_stats_aggregate(
                name,
                v,
                height_source.boxed_clone(),
                height_source.boxed_clone(),
                height_source.boxed_clone(),
                height_source.boxed_clone(),
                height_cumulative.0.boxed_clone(),
                indexes.difficultyepoch.identity.boxed_clone(),
            ),
            height_cumulative,
            dateindex,
        })
    }

    pub fn derive_from(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        height_source: &impl IterableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()> {
        // Compute height_cumulative from external source
        self.compute_height_cumulative(starting_indexes.height, height_source, exit)?;

        // Compute dateindex aggregations
        self.dateindex.compute(
            starting_indexes.dateindex,
            height_source,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )?;

        Ok(())
    }

    fn compute_height_cumulative(
        &mut self,
        max_from: Height,
        height_source: &impl IterableVec<Height, T>,
        exit: &Exit,
    ) -> Result<()> {
        compute_cumulative_extend(max_from, height_source, &mut self.height_cumulative.0, exit)
    }
}
