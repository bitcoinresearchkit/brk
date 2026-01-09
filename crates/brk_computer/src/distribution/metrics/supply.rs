use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Height, Sats, Version};

use crate::ComputeIndexes;
use rayon::prelude::*;
use vecdb::{AnyStoredVec, AnyVec, Exit, GenericStoredVec, IterableCloneableVec};

use crate::{
    indexes,
    internal::{
        HalfClosePriceTimesSats, HalveDollars, HalveSats, HalveSatsToBitcoin, LazyBlockValue,
        LazyValueDateLast, ValueBlockLast,
    },
    price,
};

use super::ImportConfig;

/// Supply metrics for a cohort.
#[derive(Clone, Traversable)]
pub struct SupplyMetrics {
    pub supply: ValueBlockLast,
    pub supply_half_value: LazyBlockValue,
    pub supply_half: LazyValueDateLast,
}

impl SupplyMetrics {
    /// Import supply metrics from database.
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let compute_dollars = cfg.compute_dollars();

        let supply = ValueBlockLast::forced_import(
            cfg.db,
            &cfg.name("supply"),
            cfg.version,
            cfg.indexes,
            compute_dollars,
        )?;

        let price_source = cfg
            .price
            .map(|p| p.usd.split.close.height.boxed_clone());

        // Create lazy supply_half from supply sources
        let supply_half_value =
            LazyBlockValue::from_sources::<HalveSats, HalveSatsToBitcoin, HalfClosePriceTimesSats>(
                &cfg.name("supply_half"),
                supply.sats.height.boxed_clone(),
                price_source,
                cfg.version,
            );

        let supply_half = LazyValueDateLast::from_block_source::<
            HalveSats,
            HalveSatsToBitcoin,
            HalveDollars,
        >(&cfg.name("supply_half"), &supply, cfg.version);

        Ok(Self {
            supply,
            supply_half_value,
            supply_half,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub fn min_len(&self) -> usize {
        self.supply.sats.height.len()
    }

    /// Push supply state values to height-indexed vectors.
    pub fn truncate_push(&mut self, height: Height, supply: Sats) -> Result<()> {
        self.supply.sats.height.truncate_push(height, supply)?;
        Ok(())
    }

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.supply.sats.height.write()?;
        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        vec![&mut self.supply.sats.height as &mut dyn AnyStoredVec].into_par_iter()
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
        self.supply.sats.height.compute_sum_of_others(
            starting_indexes.height,
            &others
                .iter()
                .map(|v| &v.supply.sats.height)
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }

    /// Compute derived vecs from existing height data.
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply
            .compute_rest(indexes, price, starting_indexes, exit)
    }
}
