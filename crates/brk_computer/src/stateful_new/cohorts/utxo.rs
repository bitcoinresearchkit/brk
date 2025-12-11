//! UTXO cohort vectors with metrics and state.

use std::path::Path;

use brk_error::Result;
use brk_grouper::{CohortContext, Filter, Filtered, StateLevel};
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, Sats, Version};
use vecdb::{Database, Exit, IterableVec};

use crate::{
    Indexes, PriceToAmount, UTXOCohortState,
    grouped::{PERCENTILES, PERCENTILES_LEN},
    indexes, price,
    stateful_new::{CohortVecs, DynCohortVecs},
};

use super::super::metrics::{CohortMetrics, ImportConfig};

/// UTXO cohort with metrics and optional runtime state.
#[derive(Clone, Traversable)]
pub struct UTXOCohortVecs {
    /// Starting height when state was imported
    state_starting_height: Option<Height>,

    /// Runtime state for block-by-block processing
    #[traversable(skip)]
    pub state: Option<UTXOCohortState>,

    /// For aggregate cohorts that only need price_to_amount for percentiles
    #[traversable(skip)]
    pub price_to_amount: Option<PriceToAmount>,

    /// Metric vectors
    #[traversable(flatten)]
    pub metrics: CohortMetrics,
}

impl UTXOCohortVecs {
    /// Import UTXO cohort from database.
    pub fn forced_import(
        db: &Database,
        filter: Filter,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: &Path,
        state_level: StateLevel,
    ) -> Result<Self> {
        let compute_dollars = price.is_some();
        let full_name = filter.to_full_name(CohortContext::Utxo);

        let cfg = ImportConfig {
            db,
            filter,
            context: CohortContext::Utxo,
            version,
            indexes,
            price,
        };

        Ok(Self {
            state_starting_height: None,

            state: if state_level.is_full() {
                Some(UTXOCohortState::new(
                    states_path,
                    &full_name,
                    compute_dollars,
                ))
            } else {
                None
            },

            price_to_amount: if state_level.is_price_only() && compute_dollars {
                Some(PriceToAmount::create(states_path, &full_name))
            } else {
                None
            },

            metrics: CohortMetrics::forced_import(&cfg)?,
        })
    }

    /// Get the starting height when state was imported.
    pub fn state_starting_height(&self) -> Option<Height> {
        self.state_starting_height
    }

    /// Set the state starting height.
    pub fn set_state_starting_height(&mut self, height: Height) {
        self.state_starting_height = Some(height);
    }

    /// Reset state starting height to zero.
    pub fn reset_state_starting_height(&mut self) {
        self.state_starting_height = Some(Height::ZERO);
    }

    /// Compute percentile prices from standalone price_to_amount.
    /// Returns NaN array if price_to_amount is None or empty.
    pub fn compute_percentile_prices_from_standalone(
        &self,
        supply: Sats,
    ) -> [Dollars; PERCENTILES_LEN] {
        let mut result = [Dollars::NAN; PERCENTILES_LEN];

        let price_to_amount = match self.price_to_amount.as_ref() {
            Some(p) => p,
            None => return result,
        };

        if price_to_amount.is_empty() || supply == Sats::ZERO {
            return result;
        }

        let total = supply;
        let targets = PERCENTILES.map(|p| total * p as u64 / 100);

        let mut accumulated = Sats::ZERO;
        let mut pct_idx = 0;

        for (&price, &sats) in price_to_amount.iter() {
            accumulated += sats;

            while pct_idx < PERCENTILES_LEN && accumulated >= targets[pct_idx] {
                result[pct_idx] = price;
                pct_idx += 1;
            }

            if pct_idx >= PERCENTILES_LEN {
                break;
            }
        }

        result
    }
}

impl Filtered for UTXOCohortVecs {
    fn filter(&self) -> &Filter {
        &self.metrics.filter
    }
}

impl DynCohortVecs for UTXOCohortVecs {
    fn min_height_vecs_len(&self) -> usize {
        self.metrics.min_len()
    }

    fn reset_state_starting_height(&mut self) {
        self.state_starting_height = Some(Height::ZERO);
    }

    fn import_state(&mut self, starting_height: Height) -> Result<Height> {
        // Import state from runtime state if present
        if let Some(state) = self.state.as_mut() {
            let imported = state.import_at_or_before(starting_height)?;
            self.state_starting_height = Some(imported);
            Ok(imported)
        } else {
            self.state_starting_height = Some(starting_height);
            Ok(starting_height)
        }
    }

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.metrics.validate_computed_versions(base_version)
    }

    fn truncate_push(&mut self, height: Height) -> Result<()> {
        if self.state_starting_height.map_or(false, |h| h > height) {
            return Ok(());
        }

        // Push from state to metrics
        if let Some(state) = self.state.as_ref() {
            self.metrics.truncate_push(height, &state)?;
        }

        Ok(())
    }

    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
    ) -> Result<()> {
        if let Some(state) = self.state.as_ref() {
            self.metrics.compute_then_truncate_push_unrealized_states(
                height,
                height_price,
                dateindex,
                date_price,
                state,
            )?;
        }
        Ok(())
    }

    fn safe_flush_stateful_vecs(&mut self, height: Height, exit: &Exit) -> Result<()> {
        self.metrics.safe_flush(exit)?;

        if let Some(state) = self.state.as_mut() {
            state.commit(height)?;
        }

        Ok(())
    }

    fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_rest_part1(indexes, price, starting_indexes, exit)
    }
}

impl CohortVecs for UTXOCohortVecs {
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()> {
        self.metrics.compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| &v.metrics).collect::<Vec<_>>(),
            exit,
        )
    }

    fn compute_rest_part2(
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
        self.metrics.compute_rest_part2(
            indexes,
            price,
            starting_indexes,
            height_to_supply,
            dateindex_to_supply,
            height_to_market_cap,
            dateindex_to_market_cap,
            height_to_realized_cap,
            dateindex_to_realized_cap,
            exit,
        )
    }
}
