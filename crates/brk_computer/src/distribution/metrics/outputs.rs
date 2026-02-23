use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredF64, StoredU64};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{ComputeIndexes, blocks, internal::ComputedFromHeightLast};

use super::ImportConfig;

/// Output metrics for a cohort.
#[derive(Traversable)]
pub struct OutputsMetrics<M: StorageMode = Rw> {
    pub utxo_count: ComputedFromHeightLast<StoredU64, M>,
    pub utxo_count_30d_change: ComputedFromHeightLast<StoredF64, M>,
}

impl OutputsMetrics {
    /// Import output metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            utxo_count: ComputedFromHeightLast::forced_import(
                cfg.db,
                &cfg.name("utxo_count"),
                cfg.version,
                cfg.indexes,
            )?,
            utxo_count_30d_change: ComputedFromHeightLast::forced_import(
                cfg.db,
                &cfg.name("utxo_count_30d_change"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub(crate) fn min_len(&self) -> usize {
        self.utxo_count.height.len()
    }

    /// Push utxo count to height-indexed vector.
    pub(crate) fn truncate_push(&mut self, height: Height, utxo_count: u64) -> Result<()> {
        self.utxo_count
            .height
            .truncate_push(height, StoredU64::from(utxo_count))?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![&mut self.utxo_count.height as &mut dyn AnyStoredVec].into_par_iter()
    }

    /// Compute aggregate values from separate cohorts.
    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.utxo_count.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.utxo_count.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    /// Compute derived metrics.
    pub(crate) fn compute_rest(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.utxo_count_30d_change.height.compute_rolling_change(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.utxo_count.height,
            exit,
        )?;

        Ok(())
    }
}
