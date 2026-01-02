use std::path::Path;

use brk_cohort::{CohortContext, Filter, Filtered, StateLevel};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Bitcoin, DateIndex, Dollars, Height, Version};
use rayon::prelude::*;
use vecdb::{AnyStoredVec, Database, Exit, IterableVec};

use crate::{ComputeIndexes, indexes, price, distribution::state::UTXOCohortState};

use crate::distribution::metrics::{CohortMetrics, ImportConfig, SupplyMetrics};

use super::super::traits::{CohortVecs, DynCohortVecs};

/// UTXO cohort with metrics and optional runtime state.
#[derive(Clone, Traversable)]
pub struct UTXOCohortVecs {
    /// Starting height when state was imported
    state_starting_height: Option<Height>,

    /// Runtime state for block-by-block processing (separate cohorts only)
    #[traversable(skip)]
    pub state: Option<UTXOCohortState>,

    /// Metric vectors
    #[traversable(flatten)]
    pub metrics: CohortMetrics,
}

impl UTXOCohortVecs {
    /// Import UTXO cohort from database.
    ///
    /// `all_supply` is the supply metrics from the "all" cohort, used as global
    /// sources for `*_rel_to_market_cap` ratios. Pass `None` for the "all" cohort itself.
    #[allow(clippy::too_many_arguments)]
    pub fn forced_import(
        db: &Database,
        filter: Filter,
        name: &str,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        states_path: &Path,
        state_level: StateLevel,
        all_supply: Option<&SupplyMetrics>,
    ) -> Result<Self> {
        let compute_dollars = price.is_some();
        let full_name = CohortContext::Utxo.full_name(&filter, name);

        let cfg = ImportConfig {
            db,
            filter,
            full_name: &full_name,
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

            metrics: CohortMetrics::forced_import(&cfg, all_supply)?,
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

    /// Reset state starting height to zero and reset state values.
    pub fn reset_state_starting_height(&mut self) {
        self.state_starting_height = Some(Height::ZERO);
        if let Some(state) = self.state.as_mut() {
            state.reset();
        }
    }

    /// Returns a parallel iterator over all vecs for parallel writing.
    pub fn par_iter_vecs_mut(&mut self) -> impl ParallelIterator<Item = &mut dyn AnyStoredVec> {
        self.metrics.par_iter_mut()
    }

    /// Commit state to disk (separate from vec writes for parallelization).
    pub fn write_state(&mut self, height: Height, cleanup: bool) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.write(height, cleanup)?;
        }
        Ok(())
    }
}

impl Filtered for UTXOCohortVecs {
    fn filter(&self) -> &Filter {
        &self.metrics.filter
    }
}

impl DynCohortVecs for UTXOCohortVecs {
    fn min_stateful_height_len(&self) -> usize {
        self.metrics.min_stateful_height_len()
    }

    fn min_stateful_dateindex_len(&self) -> usize {
        self.metrics.min_stateful_dateindex_len()
    }

    fn reset_state_starting_height(&mut self) {
        self.state_starting_height = Some(Height::ZERO);
        if let Some(state) = self.state.as_mut() {
            state.reset();
        }
    }

    fn import_state(&mut self, starting_height: Height) -> Result<Height> {
        use vecdb::GenericStoredVec;

        // Import state from runtime state if present
        if let Some(state) = self.state.as_mut() {
            // State files are saved AT height H, so to resume at H+1 we need to import at H
            // Decrement first, then increment result to match expected starting_height
            if let Some(mut prev_height) = starting_height.decremented() {
                // Import price_to_amount state file (may adjust prev_height to actual file found)
                prev_height = state.import_at_or_before(prev_height)?;

                // Restore supply state from height-indexed vectors
                state.supply.value = self
                    .metrics
                    .supply
                    .height_to_supply
                    .read_once(prev_height)?;
                state.supply.utxo_count = *self
                    .metrics
                    .supply
                    .height_to_utxo_count
                    .read_once(prev_height)?;

                // Restore realized cap if present
                if let Some(realized_metrics) = self.metrics.realized.as_mut()
                    && let Some(realized_state) = state.realized.as_mut()
                {
                    realized_state.cap = realized_metrics
                        .height_to_realized_cap
                        .read_once(prev_height)?;
                }

                let result = prev_height.incremented();
                self.state_starting_height = Some(result);
                Ok(result)
            } else {
                // starting_height is 0, nothing to import
                self.state_starting_height = Some(Height::ZERO);
                Ok(Height::ZERO)
            }
        } else {
            self.state_starting_height = Some(starting_height);
            Ok(starting_height)
        }
    }

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.metrics.validate_computed_versions(base_version)
    }

    fn truncate_push(&mut self, height: Height) -> Result<()> {
        if self.state_starting_height.is_some_and(|h| h > height) {
            return Ok(());
        }

        // Push from state to metrics
        if let Some(state) = self.state.as_ref() {
            self.metrics.truncate_push(height, state)?;
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
        if let Some(state) = self.state.as_mut() {
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

    fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_rest_part1(indexes, price, starting_indexes, exit)
    }
}

impl CohortVecs for UTXOCohortVecs {
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
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
        starting_indexes: &ComputeIndexes,
        height_to_supply: &impl IterableVec<Height, Bitcoin>,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics.compute_rest_part2(
            indexes,
            price,
            starting_indexes,
            height_to_supply,
            height_to_market_cap,
            dateindex_to_market_cap,
            exit,
        )
    }
}
