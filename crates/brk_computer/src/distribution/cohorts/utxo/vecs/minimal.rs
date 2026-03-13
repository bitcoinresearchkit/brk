use brk_cohort::{Filter, Filtered};
use brk_error::Result;
use brk_types::{Cents, Height, Indexes, Version};
use vecdb::{Exit, ReadableVec};

use crate::{
    distribution::{cohorts::traits::DynCohortVecs, metrics::MinimalCohortMetrics},
    prices,
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

    fn truncate_push(&mut self, height: Height) -> Result<()> {
        if self.state_starting_height.is_some_and(|h| h > height) {
            return Ok(());
        }

        if let Some(state) = self.state.as_ref() {
            self.metrics.supply.truncate_push(height, state)?;
            self.metrics.outputs.truncate_push(height, state)?;
            self.metrics.realized.truncate_push(height, state)?;
        }

        Ok(())
    }

    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        _height: Height,
        _height_price: Cents,
        _is_day_boundary: bool,
    ) -> Result<()> {
        Ok(())
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
