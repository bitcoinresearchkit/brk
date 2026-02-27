//! TxDerivedDistribution - per-block + rolling window distribution stats from tx-level data.
//!
//! Computes true distribution stats (average, min, max, median, percentiles) by reading
//! actual tx values for each scope: current block, last 1h, last 24h.

use brk_error::Result;
use brk_indexer::Indexer;

use brk_traversable::Traversable;
use brk_types::{Height, TxIndex};
use schemars::JsonSchema;
use vecdb::{Database, Exit, ReadableVec, Rw, StorageMode, Version};

use crate::{
    ComputeIndexes, indexes,
    internal::{
        BlockRollingDistribution, BlockWindowStarts, ComputedVecValue, Distribution, NumericValue,
    },
};

#[derive(Traversable)]
pub struct TxDerivedDistribution<T, M: StorageMode = Rw>
where
    T: ComputedVecValue + PartialOrd + JsonSchema,
{
    pub block: Distribution<Height, T, M>,
    #[traversable(flatten)]
    pub rolling: BlockRollingDistribution<T, M>,
}

impl<T> TxDerivedDistribution<T>
where
    T: NumericValue + JsonSchema,
{
    pub(crate) fn forced_import(db: &Database, name: &str, version: Version) -> Result<Self> {
        let block = Distribution::forced_import(db, name, version)?;
        let rolling = BlockRollingDistribution::forced_import(db, name, version)?;

        Ok(Self { block, rolling })
    }

    pub(crate) fn derive_from(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        block_windows: &BlockWindowStarts<'_>,
        txindex_source: &impl ReadableVec<TxIndex, T>,
        exit: &Exit,
    ) -> Result<()>
    where
        T: Copy + Ord + From<f64> + Default,
        f64: From<T>,
    {
        self.derive_from_with_skip(
            indexer,
            indexes,
            starting_indexes,
            block_windows,
            txindex_source,
            exit,
            0,
        )
    }

    /// Derive from source, skipping first N transactions per block from per-block stats.
    ///
    /// Use `skip_count: 1` to exclude coinbase transactions from fee/feerate stats.
    /// Rolling window distributions do NOT skip (negligible impact over many blocks).
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn derive_from_with_skip(
        &mut self,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        block_windows: &BlockWindowStarts<'_>,
        txindex_source: &impl ReadableVec<TxIndex, T>,
        exit: &Exit,
        skip_count: usize,
    ) -> Result<()>
    where
        T: Copy + Ord + From<f64> + Default,
        f64: From<T>,
    {
        // Per-block distribution (supports skip for coinbase exclusion)
        self.block.compute_with_skip(
            starting_indexes.height,
            txindex_source,
            &indexer.vecs.transactions.first_txindex,
            &indexes.height.txindex_count,
            exit,
            skip_count,
        )?;

        // 1h rolling: true distribution from all txs in last hour
        self.rolling._1h.compute_from_window(
            starting_indexes.height,
            txindex_source,
            &indexer.vecs.transactions.first_txindex,
            &indexes.height.txindex_count,
            block_windows._1h,
            exit,
        )?;

        // 24h rolling: true distribution from all txs in last 24 hours
        self.rolling._24h.compute_from_window(
            starting_indexes.height,
            txindex_source,
            &indexer.vecs.transactions.first_txindex,
            &indexes.height.txindex_count,
            block_windows._24h,
            exit,
        )?;

        Ok(())
    }
}
