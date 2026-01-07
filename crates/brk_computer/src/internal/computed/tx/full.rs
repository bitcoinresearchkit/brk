//! DerivedTxFull - aggregates from TxIndex to height Full + dateindex Stats + lazy date periods.

use brk_error::Result;
use brk_indexer::Indexer;

use brk_traversable::Traversable;
use brk_types::{DateIndex, DifficultyEpoch, Height, TxIndex, Version};
use derive_more::{Deref, DerefMut};
use schemars::JsonSchema;
use vecdb::{CollectableVec, Database, Exit, IterableCloneableVec};

use crate::{
    indexes, ComputeIndexes,
    internal::{ComputedVecValue, DerivedDateFull, Full, LazyFull, NumericValue, Stats},
};

/// Aggregates from TxIndex to height/dateindex with full stats.
#[derive(Clone, Deref, DerefMut, Traversable)]
#[traversable(merge)]
pub struct DerivedTxFull<T>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub height: Full<Height, T>,
    pub difficultyepoch: LazyFull<DifficultyEpoch, T, Height, DifficultyEpoch>,
    pub dateindex: Stats<DateIndex, T>,
    #[deref]
    #[deref_mut]
    #[traversable(flatten)]
    pub dates: DerivedDateFull<T>,
}

const VERSION: Version = Version::ZERO;

impl<T> DerivedTxFull<T>
where
    T: NumericValue + JsonSchema,
{
    pub fn forced_import(
        db: &Database,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let height = Full::forced_import(db, name, version + VERSION)?;
        let dateindex = Stats::forced_import(db, name, version + VERSION)?;
        let v = version + VERSION;

        let difficultyepoch =
            LazyFull::<DifficultyEpoch, T, Height, DifficultyEpoch>::from_stats_aggregate(
                name,
                v,
                height.distribution.average.0.boxed_clone(),
                height.distribution.minmax.min.0.boxed_clone(),
                height.distribution.minmax.max.0.boxed_clone(),
                height.sum_cum.sum.0.boxed_clone(),
                height.sum_cum.cumulative.0.boxed_clone(),
                indexes.block.difficultyepoch_to_difficultyepoch.boxed_clone(),
            );

        let dates = DerivedDateFull::from_sources(
            name,
            v,
            dateindex.average.0.boxed_clone(),
            dateindex.minmax.min.0.boxed_clone(),
            dateindex.minmax.max.0.boxed_clone(),
            dateindex.sum_cum.sum.0.boxed_clone(),
            dateindex.sum_cum.cumulative.0.boxed_clone(),
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
            &self.height.distribution.average.0,
            &indexes.time.dateindex_to_first_height,
            &indexes.time.dateindex_to_height_count,
            exit,
        )?;

        Ok(())
    }
}
