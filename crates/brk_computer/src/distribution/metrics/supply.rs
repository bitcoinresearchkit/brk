use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, Version};

use crate::ComputeIndexes;
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Exit, GenericStoredVec};

use crate::{
    indexes,
    internal::{
        HalfClosePriceTimesSats, HalveDollars, HalveSats, HalveSatsToBitcoin,
        LazyBinaryValueFromHeightLast, ValueChangeFromDate, ValueFromHeightLast,
    },
};

use super::ImportConfig;

/// Supply metrics for a cohort.
#[derive(Clone, Traversable)]
pub struct SupplyMetrics {
    pub total: ValueFromHeightLast,
    pub halved: LazyBinaryValueFromHeightLast,
    /// 30-day change in supply (net position change) - sats, btc, usd
    pub _30d_change: ValueChangeFromDate,
}

impl SupplyMetrics {
    /// Import supply metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply = ValueFromHeightLast::forced_import(
            cfg.db,
            &cfg.name("supply"),
            cfg.version,
            cfg.indexes,
            cfg.price,
        )?;

        let supply_halved = LazyBinaryValueFromHeightLast::from_block_source::<
            HalveSats,
            HalveSatsToBitcoin,
            HalfClosePriceTimesSats,
            HalveDollars,
        >(&cfg.name("supply_halved"), &supply, cfg.price, cfg.version);

        let _30d_change = ValueChangeFromDate::forced_import(
            cfg.db,
            &cfg.name("_30d_change"),
            cfg.version,
            cfg.compute_dollars(),
            cfg.indexes,
        )?;

        Ok(Self {
            total: supply,
            halved: supply_halved,
            _30d_change,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub fn min_len(&self) -> usize {
        self.total.sats.height.len()
    }

    /// Push supply state values to height-indexed vectors.
    pub fn truncate_push(&mut self, height: Height, supply: Sats) -> Result<()> {
        self.total.sats.height.truncate_push(height, supply)?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![&mut self.total.sats.height as &mut dyn AnyStoredVec].into_par_iter()
    }

    /// Validate computed versions against base version.
    pub fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        // Validation logic for computed vecs
        Ok(())
    }

    /// Compute aggregate values from separate cohorts.
    pub fn compute_from_stateful(
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
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.total.compute_rest(indexes, starting_indexes, exit)?;

        // 30-day change in supply
        self._30d_change.compute_change(
            starting_indexes.dateindex,
            &self.total.sats.dateindex.0,
            self.total.dollars.as_ref().map(|d| &d.dateindex.0),
            30,
            exit,
        )?;

        Ok(())
    }
}
