use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, StoredU64};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Exit, GenericStoredVec};

use crate::{ComputeIndexes, indexes, internal::ComputedFromHeightLast};

use super::ImportConfig;

/// Output metrics for a cohort.
#[derive(Clone, Traversable)]
pub struct OutputsMetrics {
    pub utxo_count: ComputedFromHeightLast<StoredU64>,
}

impl OutputsMetrics {
    /// Import output metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            utxo_count: ComputedFromHeightLast::forced_import(
                cfg.db,
                &cfg.name("utxo_count"),
                cfg.version,
                cfg.indexes,
            )?,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub fn min_len(&self) -> usize {
        self.utxo_count.height.len()
    }

    /// Push utxo count to height-indexed vector.
    pub fn truncate_push(&mut self, height: Height, utxo_count: u64) -> Result<()> {
        self.utxo_count
            .height
            .truncate_push(height, StoredU64::from(utxo_count))?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![&mut self.utxo_count.height as &mut dyn AnyStoredVec].into_par_iter()
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
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

    /// Compute derived metrics (dateindex from height).
    pub fn compute_rest(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.utxo_count.compute_rest(indexes, starting_indexes, exit)
    }
}
