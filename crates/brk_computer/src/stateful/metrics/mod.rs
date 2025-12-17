//! Metric vectors organized by category.
//!
//! Instead of a single 80+ field struct, metrics are grouped into logical categories:
//! - `supply`: Supply and UTXO count metrics (always computed)
//! - `activity`: Transaction activity metrics (always computed)
//! - `realized`: Realized cap, profit/loss, SOPR (requires price)
//! - `unrealized`: Unrealized profit/loss (requires price)
//! - `price`: Price paid metrics and percentiles (requires price)
//! - `relative`: Ratios relative to market cap, etc. (requires price)

mod activity;
mod config;
mod price_paid;
mod realized;
mod relative;
mod supply;
mod unrealized;

pub use activity::ActivityMetrics;
pub use config::ImportConfig;
pub use price_paid::PricePaidMetrics;
pub use realized::RealizedMetrics;
pub use relative::RelativeMetrics;
pub use supply::SupplyMetrics;
pub use unrealized::UnrealizedMetrics;

use brk_error::Result;
use brk_grouper::Filter;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, Version};
use vecdb::{Exit, IterableVec};

use crate::{Indexes, indexes, price, stateful::cohorts::CohortState};

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
    pub fn forced_import(cfg: &ImportConfig) -> Result<Self> {
        let compute_dollars = cfg.compute_dollars();

        Ok(Self {
            filter: cfg.filter.clone(),
            supply: SupplyMetrics::forced_import(cfg)?,
            activity: ActivityMetrics::forced_import(cfg)?,
            realized: compute_dollars
                .then(|| RealizedMetrics::forced_import(cfg))
                .transpose()?,
            unrealized: compute_dollars
                .then(|| UnrealizedMetrics::forced_import(cfg))
                .transpose()?,
            price_paid: compute_dollars
                .then(|| PricePaidMetrics::forced_import(cfg))
                .transpose()?,
            relative: compute_dollars
                .then(|| RelativeMetrics::forced_import(cfg))
                .transpose()?,
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

    /// Flush height-indexed vectors to disk.
    pub fn safe_flush(&mut self, exit: &Exit) -> Result<()> {
        self.supply.safe_flush(exit)?;
        self.activity.safe_flush(exit)?;

        if let Some(realized) = self.realized.as_mut() {
            realized.safe_flush(exit)?;
        }

        if let Some(unrealized) = self.unrealized.as_mut() {
            unrealized.safe_flush(exit)?;
        }

        if let Some(price_paid) = self.price_paid.as_mut() {
            price_paid.safe_flush(exit)?;
        }

        Ok(())
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
    pub fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
        state: &CohortState,
    ) -> Result<()> {
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

            price_paid.truncate_push_percentiles(height, state)?;
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
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply
            .compute_rest_part1(indexes, price, starting_indexes, exit)?;
        self.activity
            .compute_rest_part1(indexes, price, starting_indexes, exit)?;

        if let Some(realized) = self.realized.as_mut() {
            realized.compute_rest_part1(indexes, price, starting_indexes, exit)?;
        }

        Ok(())
    }

    /// Second phase of computed metrics (ratios, relative values).
    #[allow(clippy::too_many_arguments)]
    pub fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        dateindex_to_supply: &impl IterableVec<DateIndex, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        height_to_realized_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_realized_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.supply.compute_rest_part2(
            indexes,
            price,
            starting_indexes,
            height_to_supply,
            dateindex_to_supply,
            height_to_market_cap,
            dateindex_to_market_cap,
            exit,
        )?;

        if let Some(relative) = self.relative.as_mut() {
            relative.compute_rest_part2(
                indexes,
                starting_indexes,
                height_to_supply,
                dateindex_to_supply,
                height_to_market_cap,
                dateindex_to_market_cap,
                height_to_realized_cap,
                dateindex_to_realized_cap,
                &self.supply,
                self.unrealized.as_ref(),
                self.realized.as_ref(),
                exit,
            )?;
        }

        Ok(())
    }
}
