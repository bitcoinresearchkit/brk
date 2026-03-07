use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Indexes, Sats, SatsSigned, Version};

use crate::{blocks, prices};
use vecdb::{AnyStoredVec, AnyVec, Exit, Rw, StorageMode, WritableVec};

use crate::internal::{
    HalveCents, HalveDollars, HalveSats, HalveSatsToBitcoin, LazyValueFromHeight,
    RollingDelta1m, ValueFromHeight,
};

use super::ImportConfig;

/// Supply metrics for a cohort.
#[derive(Traversable)]
pub struct SupplyMetrics<M: StorageMode = Rw> {
    pub total: ValueFromHeight<M>,
    pub halved: LazyValueFromHeight,
    pub delta: RollingDelta1m<Sats, SatsSigned, M>,
}

impl SupplyMetrics {
    /// Import supply metrics from database.
    pub(crate) fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let supply = cfg.import("supply", Version::ZERO)?;

        let supply_halved = LazyValueFromHeight::from_block_source::<
            HalveSats,
            HalveSatsToBitcoin,
            HalveCents,
            HalveDollars,
        >(&cfg.name("supply_halved"), &supply, cfg.version);

        let delta = cfg.import("supply_delta", Version::ONE)?;

        Ok(Self {
            total: supply,
            halved: supply_halved,
            delta,
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

    pub(crate) fn collect_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec> {
        vec![
            &mut self.total.base.sats.height as &mut dyn AnyStoredVec,
            &mut self.total.base.cents.height as &mut dyn AnyStoredVec,
        ]
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
        starting_indexes: &Indexes,
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
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.delta.compute(
            starting_indexes.height,
            &blocks.count.height_1m_ago,
            &self.total.sats.height,
            exit,
        )
    }
}
