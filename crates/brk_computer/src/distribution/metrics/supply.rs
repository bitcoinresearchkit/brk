use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, Version};

use crate::{ComputeIndexes, blocks, prices};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::internal::{
    HalveDollars, HalveSats, HalveSatsToBitcoin,
    LazyValueFromHeightLast, ValueChangeFromHeight, ValueFromHeightLast,
};

use super::ImportConfig;

/// Supply metrics for a cohort.
#[derive(Traversable)]
pub struct SupplyMetrics<M: StorageMode = Rw> {
    pub total: ValueFromHeightLast<M>,
    pub halved: LazyValueFromHeightLast,
    /// 30-day change in supply (net position change) - sats, btc, usd
    pub _30d_change: ValueChangeFromHeight<M>,
}

impl SupplyMetrics {
    /// Import supply metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply = ValueFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("supply"),
            cfg.version,
            cfg.indexes,
        )?;

        let supply_halved = LazyValueFromHeightLast::from_block_source::<
            HalveSats,
            HalveSatsToBitcoin,
            HalveDollars,
        >(&cfg.name("supply_halved"), &supply, cfg.version);

        let _30d_change = ValueChangeFromHeight::forced_import(
            cfg.db,
            &cfg.name("_30d_change"),
            cfg.version,
            cfg.indexes,
        )?;

        Ok(Self {
            total: supply,
            halved: supply_halved,
            _30d_change,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub(crate) fn min_len(&self) -> usize {
        self.total.sats.height.len()
    }

    /// Push supply state values to height-indexed vectors.
    pub(crate) fn truncate_push(&mut self, height: Height, supply: Sats) -> Result<()> {
        self.total.sats.height.truncate_push(height, supply)?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![
            &mut self.total.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.total.base.usd.height as &mut dyn AnyStoredVec,
        ]
        .into_par_iter()
    }

    /// Eagerly compute USD height values from sats × price.
    pub(crate) fn compute(
        &mut self,
        prices: &prices::Vecs,
        max_from: Height,
        exit: &Exit,
    ) -> Result<()> {
        self.total.compute(prices, max_from, exit)
    }

    /// Validate computed versions against base version.
    pub(crate) fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        // Validation logic for computed vecs
        Ok(())
    }

    /// Compute aggregate values from separate cohorts.
    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.total.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.total.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    /// Compute derived vecs from existing height data.
    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self._30d_change.compute_rolling(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.total.sats.height,
            &self.total.usd.height,
            exit,
        )
    }
}
