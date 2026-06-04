use brk_cohort::{Filter, Filtered};
use brk_error::Result;
use brk_indexer::Lengths;
use brk_types::{Cents, Height, Version};
use vecdb::{Exit, ReadableVec};

use crate::{
    distribution::{cohorts::traits::DynCohortVecs, metrics::MinimalCohortMetrics},
    price,
};

use super::UTXOCohortVecs;

impl Filtered for UTXOCohortVecs<MinimalCohortMetrics> {
    fn filter(&self) -> &Filter {
        &self.metrics.filter
    }
}

impl DynCohortVecs for UTXOCohortVecs<MinimalCohortMetrics> {
    fn min_stateful_len(&self) -> usize {
        self.metrics.min_stateful_len()
    }

    fn reset_state_starting_height(&mut self) {
        self.reset_state_impl();
    }

    impl_import_state!();

    fn validate_computed_versions(&mut self, _base_version: Version) -> Result<()> {
        Ok(())
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

    fn push_unrealized_state(&mut self, _height_price: Cents) {}

    fn compute_rest_part1(
        &mut self,
        prices: &price::Vecs,
        starting_lengths: &Lengths,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_rest_part1(prices, starting_lengths, exit)
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
