//! ComputedTxDistribution - computes TxIndex data to height Distribution + dateindex MinMaxAverage + lazy aggregations.
//!
//! Note: Percentiles are computed at height level only. DateIndex and coarser
//! periods only have average+min+max since computing percentiles across all
//! transactions per day would be expensive.

use brk_error::Result;
use brk_indexer::Indexer;

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, TxIndex, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{CollectableVec, Database, Exit, IterableCloneableVec};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        ComputedVecValue, DerivedDateDistribution, Distribution, LazyDistribution, MinMaxAverage,
        NumericValue,
    },
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct ComputedTxDistribution<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: Distribution<Height, T>,
    pub difficultyepoch: LazyDistribution<DifficultyEpoch, T, Height, DifficultyEpoch>,
    pub dateindex: MinMaxAverage<DateIndex, T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: DerivedDateDistribution<T>,
}

const VERSION: Version = Version::ZERO;

impl<T> ComputedTxDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let height = Distribution::forced_import(db, name, version + VERSION)?;
        let dateindex = MinMaxAverage::forced_import(db, name, version + VERSION)?;
        let v = version + VERSION;

        let difficultyepoch =
            LazyDistribution::<DifficultyEpoch, T, Height, DifficultyEpoch>::from_distribution(
                name,
                v,
                height.average.0.boxed_clone(),
                height.minmax.min.0.boxed_clone(),
                height.minmax.max.0.boxed_clone(),
                indexes
                    .block
                    .difficultyepoch_to_difficultyepoch
                    .boxed_clone(),
            );

        let dates = DerivedDateDistribution::from_sources(
            name,
            v,
            dateindex.average.0.boxed_clone(),
            dateindex.minmax.min.0.boxed_clone(),
            dateindex.minmax.max.0.boxed_clone(),
            indexes,
        );

        Ok(Self {
            height,
            difficultyepoch,
            dateindex,
            dates,
        })
    }

    pub fn derive_from(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        txindex_source: &impl CollectableVec<TxIndex, T>,
        exit: &Exit,
    ) -> Result<()> {
        self.height.compute(
            starting_indexes.height,
            txindex_source,
            &indexer.vecs.tx.height_to_first_txindex,
            &indexes.block.height_to_txindex_count,
            exit,
        )?;

        self.dateindex.compute(
            starting_indexes.dateindex,
            &self.height.average.0,
            &indexes.time.dateindex_to_first_height,
            &indexes.time.dateindex_to_height_count,
            exit,
        )?;

        Ok(())
    }
}
