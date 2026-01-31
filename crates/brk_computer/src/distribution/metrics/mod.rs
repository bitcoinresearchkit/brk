mod activity;
mod config;
mod cost_basis;
mod outputs;
mod realized;
mod relative;
mod supply;
mod unrealized;

pub use activity::*;
pub use config::*;
pub use cost_basis::*;
pub use outputs::*;
pub use realized::*;
pub use relative::*;
pub use supply::*;
pub use unrealized::*;

use brk_cohort::Filter;
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{CentsUnsigned, DateIndex, Dollars, Height, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Exit, IterableVec};

use crate::{ComputeIndexes, distribution::state::CohortState, indexes, price as price_vecs};

/// All metrics for a cohort, organized by category.
#[derive(Clone, Traversable)]
pub struct CohortMetrics {
    #[traversable(skip)]
    pub filter: Filter,

    /// Supply metrics (always computed)
    pub supply: SupplyMetrics,

    /// Output metrics - UTXO count (always computed)
    pub outputs: OutputsMetrics,

    /// Transaction activity (always computed)
    pub activity: ActivityMetrics,

    /// Realized cap and profit/loss (requires price data)
    pub realized: Option<RealizedMetrics>,

    /// Unrealized profit/loss (requires price data)
    pub unrealized: Option<UnrealizedMetrics>,

    /// Cost basis metrics (requires price data)
    pub cost_basis: Option<CostBasisMetrics>,

    /// Relative metrics (requires price data)
    pub relative: Option<RelativeMetrics>,
}

impl CohortMetrics {
    /// Import all metrics from database.
    ///
    /// `all_supply` is the supply metrics from the "all" cohort, used as global
    /// sources for `*_rel_to_market_cap` and `*_rel_to_circulating_supply` ratios.
    /// Pass `None` for the "all" cohort itself.
    pub fn forced_import(cfg: &ImportConfig, all_supply: Option<&SupplyMetrics>) -> Result<Self> {
        let compute_dollars = cfg.compute_dollars();

        let supply = SupplyMetrics::forced_import(cfg)?;
        let outputs = OutputsMetrics::forced_import(cfg)?;

        let unrealized = compute_dollars
            .then(|| UnrealizedMetrics::forced_import(cfg))
            .transpose()?;

        let realized = compute_dollars
            .then(|| RealizedMetrics::forced_import(cfg))
            .transpose()?;

        let relative = (cfg.compute_relative() && unrealized.is_some())
            .then(|| {
                RelativeMetrics::forced_import(
                    cfg,
                    unrealized.as_ref().unwrap(),
                    &supply,
                    all_supply,
                    realized.as_ref(),
                )
            })
            .transpose()?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply,
            outputs,
            activity: ActivityMetrics::forced_import(cfg)?,
            realized,
            cost_basis: compute_dollars
                .then(|| CostBasisMetrics::forced_import(cfg))
                .transpose()?,
            relative,
            unrealized,
        })
    }

    /// Get minimum length across height-indexed vectors written in block loop.
    pub fn min_stateful_height_len(&self) -> usize {
        let mut min = self
            .supply
            .min_len()
            .min(self.outputs.min_len())
            .min(self.activity.min_len());

        if let Some(realized) = &self.realized {
            min = min.min(realized.min_stateful_height_len());
        }
        if let Some(unrealized) = &self.unrealized {
            min = min.min(unrealized.min_stateful_height_len());
        }
        if let Some(cost_basis) = &self.cost_basis {
            min = min.min(cost_basis.min_stateful_height_len());
        }

        min
    }

    /// Get minimum length across dateindex-indexed vectors written in block loop.
    pub fn min_stateful_dateindex_len(&self) -> usize {
        let mut min = usize::MAX;

        if let Some(unrealized) = &self.unrealized {
            min = min.min(unrealized.min_stateful_dateindex_len());
        }
        if let Some(cost_basis) = &self.cost_basis {
            min = min.min(cost_basis.min_stateful_dateindex_len());
        }

        min
    }

    /// Push state values to height-indexed vectors.
    pub fn truncate_push(&mut self, height: Height, state: &CohortState) -> Result<()> {
        self.supply.truncate_push(height, state.supply.value)?;
        self.outputs
            .truncate_push(height, state.supply.utxo_count)?;
        self.activity.truncate_push(
            height,
            state.sent,
            state.satblocks_destroyed,
            state.satdays_destroyed,
        )?;

        if let (Some(realized), Some(realized_state)) =
            (self.realized.as_mut(), state.realized.as_ref())
        {
            realized.truncate_push(height, realized_state)?;
        }

        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();

        vecs.extend(self.supply.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.outputs.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.activity.par_iter_mut().collect::<Vec<_>>());

        if let Some(realized) = self.realized.as_mut() {
            vecs.extend(realized.par_iter_mut().collect::<Vec<_>>());
        }

        if let Some(unrealized) = self.unrealized.as_mut() {
            vecs.extend(unrealized.par_iter_mut().collect::<Vec<_>>());
        }

        if let Some(cost_basis) = self.cost_basis.as_mut() {
            vecs.extend(cost_basis.par_iter_mut().collect::<Vec<_>>());
        }

        vecs.into_par_iter()
    }

    /// Validate computed versions against base version.
    pub fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.supply.validate_computed_versions(base_version)?;
        self.activity.validate_computed_versions(base_version)?;

        if let Some(realized) = self.realized.as_mut() {
            realized.validate_computed_versions(base_version)?;
        }

        if let Some(cost_basis) = self.cost_basis.as_mut() {
            cost_basis.validate_computed_versions(base_version)?;
        }

        Ok(())
    }

    /// Compute and push unrealized states.
    /// Percentiles are only computed at date boundaries (when dateindex is Some).
    pub fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<CentsUnsigned>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<CentsUnsigned>>,
        state: &mut CohortState,
    ) -> Result<()> {
        // Apply pending updates before reading
        state.apply_pending();

        if let (Some(unrealized), Some(cost_basis), Some(height_price)) = (
            self.unrealized.as_mut(),
            self.cost_basis.as_mut(),
            height_price,
        ) {
            cost_basis.truncate_push_minmax(height, state)?;

            let (height_unrealized_state, date_unrealized_state) =
                state.compute_unrealized_states(height_price, date_price.unwrap());

            unrealized.truncate_push(
                height,
                dateindex,
                &height_unrealized_state,
                date_unrealized_state.as_ref(),
            )?;

            // Only compute expensive percentiles at date boundaries (~144x reduction)
            if let Some(dateindex) = dateindex {
                let spot = date_price
                    .unwrap()
                    .map(|c| c.to_dollars())
                    .unwrap_or(Dollars::NAN);
                cost_basis.truncate_push_percentiles(dateindex, state, spot)?;
            }
        }

        Ok(())
    }

    /// Compute aggregate cohort values from separate cohorts.
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &v.supply).collect::<Vec<_>>(),
            exit,
        )?;
        self.outputs.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &v.outputs).collect::<Vec<_>>(),
            exit,
        )?;
        self.activity.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &v.activity).collect::<Vec<_>>(),
            exit,
        )?;

        if let Some(realized) = self.realized.as_mut() {
            realized.compute_from_stateful(
                starting_indexes,
                &others
                    .iter()
                    .filter_map(|v| v.realized.as_ref())
                    .collect::<Vec<_>>(),
                exit,
            )?;
        }

        if let Some(unrealized) = self.unrealized.as_mut() {
            unrealized.compute_from_stateful(
                starting_indexes,
                &others
                    .iter()
                    .filter_map(|v| v.unrealized.as_ref())
                    .collect::<Vec<_>>(),
                exit,
            )?;
        }

        if let Some(cost_basis) = self.cost_basis.as_mut() {
            cost_basis.compute_from_stateful(
                starting_indexes,
                &others
                    .iter()
                    .filter_map(|v| v.cost_basis.as_ref())
                    .collect::<Vec<_>>(),
                exit,
            )?;
        }

        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price_vecs::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply
            .compute_rest_part1(indexes, starting_indexes, exit)?;
        self.outputs.compute_rest(indexes, starting_indexes, exit)?;
        self.activity
            .compute_rest_part1(indexes, starting_indexes, exit)?;

        if let Some(realized) = self.realized.as_mut() {
            realized.compute_rest_part1(indexes, starting_indexes, exit)?;
        }

        if let Some(unrealized) = self.unrealized.as_mut() {
            unrealized.compute_rest(indexes, price, starting_indexes, exit)?;
        }

        if let Some(cost_basis) = self.cost_basis.as_mut() {
            cost_basis.compute_rest_part1(indexes, starting_indexes, exit)?;
        }

        Ok(())
    }

    /// Second phase of computed metrics (ratios, relative values).
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price_vecs::Vecs>,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        if let Some(realized) = self.realized.as_mut() {
            realized.compute_rest_part2(
                indexes,
                price,
                starting_indexes,
                &self.supply.total.bitcoin.height,
                height_to_market_cap,
                dateindex_to_market_cap,
                exit,
            )?;
        }

        Ok(())
    }
}
