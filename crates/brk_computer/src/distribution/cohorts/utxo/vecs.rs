use brk_cohort::{Filter, Filtered};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Version};
use vecdb::{Exit, ReadableVec};

use crate::{ComputeIndexes, blocks, distribution::state::UTXOCohortState, prices};

use crate::distribution::metrics::CohortMetricsBase;

use super::super::traits::DynCohortVecs;

/// UTXO cohort with metrics and optional runtime state.
///
/// Generic over the metrics type to support different cohort configurations
/// (e.g. AllCohortMetrics, ExtendedCohortMetrics, BasicCohortMetrics, etc.)
#[derive(Traversable)]
pub struct UTXOCohortVecs<Metrics> {
    /// Starting height when state was imported
    #[traversable(skip)]
    state_starting_height: Option<Height>,

    /// Runtime state for block-by-block processing (separate cohorts only)
    #[traversable(skip)]
    pub state: Option<Box<UTXOCohortState>>,

    /// Metric vectors
    #[traversable(flatten)]
    pub metrics: Metrics,
}

impl<Metrics> UTXOCohortVecs<Metrics> {
    /// Create a new UTXOCohortVecs with state and metrics.
    pub(crate) fn new(state: Option<Box<UTXOCohortState>>, metrics: Metrics) -> Self {
        Self {
            state_starting_height: None,
            state,
            metrics,
        }
    }

}

impl<Metrics: CohortMetricsBase + Traversable> Filtered for UTXOCohortVecs<Metrics> {
    fn filter(&self) -> &Filter {
        self.metrics.filter()
    }
}

impl<Metrics: CohortMetricsBase + Traversable> DynCohortVecs for UTXOCohortVecs<Metrics> {
    fn min_stateful_height_len(&self) -> usize {
        self.metrics.min_stateful_height_len()
    }

    fn reset_state_starting_height(&mut self) {
        self.state_starting_height = Some(Height::ZERO);
        if let Some(state) = self.state.as_mut() {
            state.reset();
        }
    }

    fn import_state(&mut self, starting_height: Height) -> Result<Height> {
        if let Some(state) = self.state.as_mut() {
            if let Some(mut prev_height) = starting_height.decremented() {
                prev_height = state.import_at_or_before(prev_height)?;

                state.supply.value = self
                    .metrics
                    .supply()
                    .total
                    .sats
                    .height
                    .collect_one(prev_height)
                    .unwrap();
                state.supply.utxo_count = *self
                    .metrics
                    .outputs()
                    .utxo_count
                    .height
                    .collect_one(prev_height)
                    .unwrap();

                state.restore_realized_cap();

                let result = prev_height.incremented();
                self.state_starting_height = Some(result);
                Ok(result)
            } else {
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

        if let Some(state) = self.state.as_ref() {
            self.metrics.truncate_push(height, state)?;
        }

        Ok(())
    }

    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Cents,
    ) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            self.metrics
                .compute_then_truncate_push_unrealized_states(height, height_price, state)?;
        }
        Ok(())
    }

    fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_rest_part1(blocks, prices, starting_indexes, exit)
    }

    fn compute_net_sentiment_height(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_net_sentiment_height(starting_indexes, exit)
    }

    fn write_state(&mut self, height: Height, cleanup: bool) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.write(height, cleanup)?;
        }
        Ok(())
    }

    fn reset_cost_basis_data_if_needed(&mut self) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.reset_cost_basis_data_if_needed()?;
        }
        Ok(())
    }

    fn reset_single_iteration_values(&mut self) {
        if let Some(state) = self.state.as_mut() {
            state.reset_single_iteration_values();
        }
    }
}
