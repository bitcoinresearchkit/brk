//! TxDerivedDistribution - computes TxIndex data to height Distribution + dateindex MinMaxAverage + lazy aggregations.
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
        ComputedVecValue, Distribution, LazyDateDerivedSpread, LazySpread, MinMaxAverage,
        NumericValue,
    },
};

#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct TxDerivedDistribution<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: Distribution<Height, T>,
    pub difficultyepoch: LazySpread<DifficultyEpoch, T, Height, DifficultyEpoch>,
    pub dateindex: MinMaxAverage<DateIndex, T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: LazyDateDerivedSpread<T>,
}

const VERSION: Version = Version::ZERO;

impl<T> TxDerivedDistribution<T>
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
            LazySpread::<DifficultyEpoch, T, Height, DifficultyEpoch>::from_distribution(
                name,
                v,
                height.boxed_average(),
                height.boxed_min(),
                height.boxed_max(),
                indexes.difficultyepoch.identity.boxed_clone(),
            );

        let dates = LazyDateDerivedSpread::from_sources(
            name,
            v,
            dateindex.boxed_average(),
            dateindex.boxed_min(),
            dateindex.boxed_max(),
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
        self.derive_from_with_skip(indexer, indexes, starting_indexes, txindex_source, exit, 0)
    }

    /// Derive from source, skipping first N transactions per block from all calculations.
    ///
    /// Use `skip_count: 1` to exclude coinbase transactions from fee/feerate stats.
    pub fn derive_from_with_skip(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        txindex_source: &impl CollectableVec<TxIndex, T>,
        exit: &Exit,
        skip_count: usize,
    ) -> Result<()> {
        self.height.compute_with_skip(
            starting_indexes.height,
            txindex_source,
            &indexer.vecs.transactions.first_txindex,
            &indexes.height.txindex_count,
            exit,
            skip_count,
        )?;

        self.dateindex.compute(
            starting_indexes.dateindex,
            &self.height.average().0,
            &indexes.dateindex.first_height,
            &indexes.dateindex.height_count,
            exit,
        )?;

        Ok(())
    }
}
