use brk_cohort::{Filter, Filtered};
use brk_error::Result;
use brk_types::{Cents, Height, Indexes, Version};
use vecdb::{Exit, ReadableVec};

use crate::{
    distribution::{cohorts::traits::DynCohortVecs, metrics::CoreCohortMetrics},
    prices,
};

use super::UTXOCohortVecs;

impl Filtered for UTXOCohortVecs<CoreCohortMetrics> {
    fn filter(&self) -> &Filter {
        &self.metrics.filter
    }
}

impl DynCohortVecs for UTXOCohortVecs<CoreCohortMetrics> {
    fn min_stateful_len(&self) -> usize {
        self.metrics.min_stateful_len()
    }

    fn reset_state_starting_height(&mut self) {
        self.reset_state_impl();
    }

    impl_import_state!();

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.metrics.validate_computed_versions(base_version)
    }

    fn push_state(&mut self, height: Height) {
        if self.state_starting_height.is_some_and(|h| h > height) {
            return;
        }

        if let Some(state) = self.state.as_ref() {
            self.metrics.supply.push_state(state);
            self.metrics.outputs.push_state(state);
            self.metrics.activity.push_state(state);
            self.metrics.realized.push_state(state);
        }
    }

    fn push_unrealized_state(&mut self, height_price: Cents) {
        if let Some(state) = self.state.as_mut() {
            state.apply_pending();
            let unrealized_state = state.compute_unrealized_state(height_price);
            self.metrics.unrealized.push_state(&unrealized_state);
            self.metrics.supply.push_profitability(&unrealized_state);
        }
    }

    fn compute_rest_part1(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_rest_part1(prices, starting_indexes, exit)
    }

    fn write_state(&mut self, height: Height, cleanup: bool) -> Result<()> {
        self.write_state_impl(height, cleanup)
    }

    fn reset_cost_basis_data_if_needed(&mut self) -> Result<()> {
        self.reset_cost_basis_impl()
    }

    fn reset_single_iteration_values(&mut self) {
        self.reset_iteration_impl();
    }
}
