mod activity;
mod config;
mod price;
mod realized;
mod supply;
mod unrealized;

pub use activity::ActivityMetrics;
pub use config::ImportConfig;
pub use price::{PricePaidMetrics, RelativeMetrics};
pub use realized::RealizedMetrics;
pub use supply::SupplyMetrics;
pub use unrealized::UnrealizedMetrics;

use brk_error::Result;
use brk_grouper::Filter;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Exit, IterableVec};

use crate::{Indexes, indexes, price as price_vecs, stateful::state::CohortState};

/// All metrics for a cohort, organized by category.
#[derive(Clone, Traversable)]
pub struct CohortMetrics {
    #[traversable(skip)]
    pub filter: Filter,

    /// Supply and UTXO count (always computed)
    pub supply: SupplyMetrics,

    /// Transaction activity (always computed)
    pub activity: ActivityMetrics,

    /// Realized cap and profit/loss (requires price data)
    pub realized: Option<RealizedMetrics>,

    /// Unrealized profit/loss (requires price data)
    pub unrealized: Option<UnrealizedMetrics>,

    /// Price paid metrics (requires price data)
    pub price_paid: Option<PricePaidMetrics>,

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

        let unrealized = compute_dollars
            .then(|| UnrealizedMetrics::forced_import(cfg))
            .transpose()?;

        let relative = unrealized
            .as_ref()
            .map(|u| RelativeMetrics::forced_import(cfg, u, &supply, all_supply))
            .transpose()?;

        Ok(Self {
            filter: cfg.filter.clone(),
            supply,
            activity: ActivityMetrics::forced_import(cfg)?,
            realized: compute_dollars
                .then(|| RealizedMetrics::forced_import(cfg))
                .transpose()?,
            price_paid: compute_dollars
                .then(|| PricePaidMetrics::forced_import(cfg))
                .transpose()?,
            relative,
            unrealized,
        })
    }

    /// Get minimum length across height-indexed vectors.
    pub fn min_len(&self) -> usize {
        self.supply.min_len().min(self.activity.min_len())
    }

    /// Push state values to height-indexed vectors.
    pub fn truncate_push(&mut self, height: Height, state: &CohortState) -> Result<()> {
        self.supply.truncate_push(height, &state.supply)?;
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

    /// Write height-indexed vectors to disk.
    pub fn write(&mut self) -> Result<()> {
        self.supply.write()?;
        self.activity.write()?;

        if let Some(realized) = self.realized.as_mut() {
            realized.write()?;
        }

        if let Some(unrealized) = self.unrealized.as_mut() {
            unrealized.write()?;
        }

        if let Some(price_paid) = self.price_paid.as_mut() {
            price_paid.write()?;
        }

        Ok(())
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        let mut vecs: Vec<&mut dyn AnyStoredVec> = Vec::new();

        vecs.extend(self.supply.par_iter_mut().collect::<Vec<_>>());
        vecs.extend(self.activity.par_iter_mut().collect::<Vec<_>>());

        if let Some(realized) = self.realized.as_mut() {
            vecs.extend(realized.par_iter_mut().collect::<Vec<_>>());
        }

        if let Some(unrealized) = self.unrealized.as_mut() {
            vecs.extend(unrealized.par_iter_mut().collect::<Vec<_>>());
        }

        if let Some(price_paid) = self.price_paid.as_mut() {
            vecs.extend(price_paid.par_iter_mut().collect::<Vec<_>>());
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

        if let Some(price_paid) = self.price_paid.as_mut() {
            price_paid.validate_computed_versions(base_version)?;
        }

        Ok(())
    }

    /// Compute and push unrealized states.
    /// Percentiles are only computed at date boundaries (when dateindex is Some).
    pub fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
        state: &mut CohortState,
    ) -> Result<()> {
        // Apply pending updates before reading
        state.apply_pending();

        if let (Some(unrealized), Some(price_paid), Some(height_price)) = (
            self.unrealized.as_mut(),
            self.price_paid.as_mut(),
            height_price,
        ) {
            price_paid.truncate_push_minmax(height, state)?;

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
                price_paid.truncate_push_percentiles(dateindex, state)?;
            }
        }

        Ok(())
    }

    /// Compute aggregate cohort values from separate cohorts.
    pub fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &v.supply).collect::<Vec<_>>(),
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

        if let Some(price_paid) = self.price_paid.as_mut() {
            price_paid.compute_from_stateful(
                starting_indexes,
                &others
                    .iter()
                    .filter_map(|v| v.price_paid.as_ref())
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
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply
            .compute_rest_part1(indexes, price, starting_indexes, exit)?;
        self.activity
            .compute_rest_part1(indexes, price, starting_indexes, exit)?;

        if let Some(realized) = self.realized.as_mut() {
            realized.compute_rest_part1(indexes, starting_indexes, exit)?;
        }

        if let Some(unrealized) = self.unrealized.as_mut() {
            unrealized.compute_rest_part1(price, starting_indexes, exit)?;
        }

        if let Some(price_paid) = self.price_paid.as_mut() {
            price_paid.compute_rest_part1(indexes, starting_indexes, exit)?;
        }

        Ok(())
    }

    /// Second phase of computed metrics (ratios, relative values).
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price_vecs::Vecs>,
        starting_indexes: &Indexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        if let Some(realized) = self.realized.as_mut() {
            realized.compute_rest_part2(
                indexes,
                price,
                starting_indexes,
                height_to_supply,
                height_to_market_cap,
                dateindex_to_market_cap,
                exit,
            )?;
        }

        Ok(())
    }
}
