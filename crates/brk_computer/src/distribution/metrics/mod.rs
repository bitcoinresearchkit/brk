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
use brk_types::{Cents, Dollars, Height, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Exit, ReadableVec, Rw, StorageMode};

use crate::{ComputeIndexes, blocks, distribution::state::CohortState, prices};

/// All metrics for a cohort, organized by category.
#[derive(Traversable)]
pub struct CohortMetrics<M: StorageMode = Rw> {
    #[traversable(skip)]
    pub filter: Filter,

    /// Supply metrics (always computed)
    pub supply: Box<SupplyMetrics<M>>,

    /// Output metrics - UTXO count (always computed)
    pub outputs: Box<OutputsMetrics<M>>,

    /// Transaction activity (always computed)
    pub activity: Box<ActivityMetrics<M>>,

    /// Realized cap and profit/loss
    pub realized: Box<RealizedMetrics<M>>,

    /// Unrealized profit/loss
    pub unrealized: Box<UnrealizedMetrics<M>>,

    /// Cost basis metrics
    pub cost_basis: Box<CostBasisMetrics<M>>,

    /// Relative metrics (not all cohorts compute this)
    pub relative: Option<Box<RelativeMetrics>>,
}

impl CohortMetrics {
    /// Import all metrics from database.
    ///
    /// `all_supply` is the supply metrics from the "all" cohort, used as global
    /// sources for `*_rel_to_market_cap` and `*_rel_to_circulating_supply` ratios.
    /// Pass `None` for the "all" cohort itself.
    pub(crate) fn forced_import(cfg: &ImportConfig, all_supply: Option<&SupplyMetrics>) -> Result<Self> {
        let supply = SupplyMetrics::forced_import(cfg)?;
        let outputs = OutputsMetrics::forced_import(cfg)?;

        let unrealized = UnrealizedMetrics::forced_import(cfg)?;
        let realized = RealizedMetrics::forced_import(cfg)?;

        let relative = cfg
            .compute_relative()
            .then(|| {
                RelativeMetrics::forced_import(
                    cfg,
                    &unrealized,
                    &supply,
                    all_supply,
                    Some(&realized),
                )
            })
            .transpose()?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: Box::new(supply),
            outputs: Box::new(outputs),
            activity: Box::new(ActivityMetrics::forced_import(cfg)?),
            realized: Box::new(realized),
            cost_basis: Box::new(CostBasisMetrics::forced_import(cfg)?),
            relative: relative.map(Box::new),
            unrealized: Box::new(unrealized),
        })
    }

    /// Get minimum length across height-indexed vectors written in block loop.
    pub(crate) fn min_stateful_height_len(&self) -> usize {
        self.supply
            .min_len()
            .min(self.outputs.min_len())
            .min(self.activity.min_len())
            .min(self.realized.min_stateful_height_len())
            .min(self.unrealized.min_stateful_height_len())
            .min(self.cost_basis.min_stateful_height_len())
    }

    /// Push state values to height-indexed vectors.
    pub(crate) fn truncate_push(&mut self, height: Height, state: &CohortState) -> Result<()> {
        self.supply.truncate_push(height, state.supply.value)?;
        self.outputs
            .truncate_push(height, state.supply.utxo_count)?;
        self.activity.truncate_push(
            height,
            state.sent,
            state.satblocks_destroyed,
            state.satdays_destroyed,
        )?;

        self.realized.truncate_push(height, &state.realized)?;

        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub(crate) fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();

        vecs.extend(self.supply.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.outputs.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.activity.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.realized.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.unrealized.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.cost_basis.par_iter_mut().collect::<Vec<_>>());

        vecs.into_par_iter()
    }

    /// Validate computed versions against base version.
    pub(crate) fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.supply.validate_computed_versions(base_version)?;
        self.activity.validate_computed_versions(base_version)?;
        self.realized.validate_computed_versions(base_version)?;
        self.cost_basis.validate_computed_versions(base_version)?;

        Ok(())
    }

    /// Compute and push unrealized states and percentiles.
    pub(crate) fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Cents,
        state: &mut CohortState,
    ) -> Result<()> {
        // Apply pending updates before reading
        state.apply_pending();

        self.cost_basis.truncate_push_minmax(height, state)?;

        let (height_unrealized_state, _) = state.compute_unrealized_states(height_price, None);

        self.unrealized
            .truncate_push(height, &height_unrealized_state)?;

        let spot = height_price.to_dollars();
        self.cost_basis
            .truncate_push_percentiles(height, state, spot)?;

        Ok(())
    }

    /// Compute aggregate cohort values from separate cohorts.
    pub(crate) fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.supply).collect::<Vec<_>>(),
            exit,
        )?;
        self.outputs.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.outputs).collect::<Vec<_>>(),
            exit,
        )?;
        self.activity.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.activity).collect::<Vec<_>>(),
            exit,
        )?;

        self.realized.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.realized).collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.unrealized).collect::<Vec<_>>(),
            exit,
        )?;
        self.cost_basis.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &*v.cost_basis).collect::<Vec<_>>(),
            exit,
        )?;

        Ok(())
    }

    /// Compute net_sentiment.height as capital-weighted average of component cohorts.
    ///
    /// For aggregate cohorts, the simple greed-pain formula produces values outside
    /// the range of components due to asymmetric weighting. This computes net_sentiment
    /// as a proper weighted average using realized_cap as weight.
    ///
    /// Only computes height; day1 derivation is done separately via compute_net_sentiment_rest.
    pub(crate) fn compute_net_sentiment_from_others(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        let weights: Vec<_> = others
            .iter()
            .map(|o| &o.realized.realized_cap.height)
            .collect();
        let values: Vec<_> = others
            .iter()
            .map(|o| &o.unrealized.net_sentiment.height)
            .collect();

        self.unrealized
            .net_sentiment
            .height
            .compute_weighted_average_of_others(starting_indexes.height, &weights, &values, exit)?;

        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    pub(crate) fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute_rest_part1(blocks, starting_indexes, exit)?;
        self.outputs.compute_rest(blocks, starting_indexes, exit)?;
        self.activity.compute_rest_part1(blocks, starting_indexes, exit)?;

        self.realized.compute_rest_part1(starting_indexes, exit)?;

        self.unrealized
            .compute_rest(prices, starting_indexes, exit)?;

        Ok(())
    }

    /// Second phase of computed metrics (ratios, relative values).
    pub(crate) fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: Option<&impl ReadableVec<Height, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.realized.compute_rest_part2(
            blocks,
            prices,
            starting_indexes,
            &self.supply.total.btc.height,
            height_to_market_cap,
            exit,
        )?;

        Ok(())
    }

    /// Compute net_sentiment.height for separate cohorts (greed - pain).
    /// Called only for separate cohorts; aggregates compute via weighted average in compute_from_stateful.
    pub(crate) fn compute_net_sentiment_height(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.unrealized
            .compute_net_sentiment_height(starting_indexes, exit)?;
        Ok(())
    }
}
