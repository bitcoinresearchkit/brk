use brk_cohort::{Filter, Filtered};
use brk_error::Result;
use brk_types::{Cents, Height, Indexes, Version};
use vecdb::{Exit, ReadableVec};

use crate::{blocks, distribution::cohorts::traits::DynCohortVecs, distribution::metrics::TypeCohortMetrics, prices};

use super::UTXOCohortVecs;

impl Filtered for UTXOCohortVecs<TypeCohortMetrics> {
    fn filter(&self) -> &Filter {
        &self.metrics.filter
    }
}

impl DynCohortVecs for UTXOCohortVecs<TypeCohortMetrics> {
    fn min_stateful_height_len(&self) -> usize {
        self.metrics.min_stateful_height_len()
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
        height: Height,
        height_price: Cents,
        _is_day_boundary: bool,
    ) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.apply_pending();
            let unrealized_state = state.compute_unrealized_state(height_price);
            self.metrics
                .unrealized
                .truncate_push(height, &unrealized_state)?;
            self.metrics
                .supply
                .truncate_push_profitability(height, &unrealized_state)?;
        }
        Ok(())
    }

    fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.metrics
            .compute_rest_part1(blocks, prices, starting_indexes, exit)
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
