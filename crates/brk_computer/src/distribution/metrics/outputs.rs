use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, StoredI64, StoredU64, Version};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::{blocks, internal::{ComputedPerBlock, RollingDelta1m}};

use super::ImportConfig;

/// Output metrics for a cohort.
#[derive(Traversable)]
pub struct OutputsMetrics<M: StorageMode = Rw> {
    pub utxo_count: ComputedPerBlock<StoredU64, M>,
    pub utxo_count_delta: RollingDelta1m<StoredU64, StoredI64, M>,
}

impl OutputsMetrics {
    /// Import output metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        Ok(Self {
            utxo_count: cfg.import("utxo_count", Version::ZERO)?,
            utxo_count_delta: cfg.import("utxo_count_delta", Version::ONE)?,
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

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![&mut self.utxo_count.height as &mut dyn AnyStoredVec]
    }

    /// Compute aggregate values from separate cohorts.
    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
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
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.utxo_count_delta.compute(
            starting_indexes.height,
            &blocks.lookback.height_1m_ago,
            &self.utxo_count.height,
            exit,
        )?;

        Ok(())
    }
}
