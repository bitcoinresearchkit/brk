macro_rules! impl_import_state {
    () => {
        fn import_state(&mut self, starting_height: Height) -> Result<Height> {
            if let Some(state) = self.state.as_mut() {
                if let Some(mut prev_height) = starting_height.decremented() {
                    prev_height = state.import_at_or_before(prev_height)?;

                    state.supply.value = self
                        .metrics
                        .supply
                        .total
                        .sats
                        .height
                        .collect_one(prev_height)
                        .unwrap();
                    state.supply.utxo_count = *self
                        .metrics
                        .outputs
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
    };
}

mod core;
mod minimal;
mod r#type;

use brk_cohort::{Filter, Filtered};
use brk_error::Result;
use brk_traversable::Traversable;
use brk_types::{Cents, Height, Indexes, Version};
use vecdb::{Exit, ReadableVec};

use crate::{
    blocks,
    distribution::{
        cohorts::traits::DynCohortVecs,
        metrics::{CohortMetricsBase, CohortMetricsState},
        state::UTXOCohortState,
    },
    prices,
};

#[derive(Traversable)]
pub struct UTXOCohortVecs<M: CohortMetricsState> {
    #[traversable(skip)]
    state_starting_height: Option<Height>,

    #[traversable(skip)]
    pub state: Option<Box<UTXOCohortState<M::Realized, M::CostBasis>>>,

    #[traversable(flatten)]
    pub metrics: M,
}

impl<M: CohortMetricsState> UTXOCohortVecs<M> {
    pub(crate) fn new(state: Option<Box<UTXOCohortState<M::Realized, M::CostBasis>>>, metrics: M) -> Self {
        Self {
            state_starting_height: None,
            state,
            metrics,
        }
    }

    fn reset_state_impl(&mut self) {
        self.state_starting_height = Some(Height::ZERO);
        if let Some(state) = self.state.as_mut() {
            state.reset();
        }
    }

    fn write_state_impl(&mut self, height: Height, cleanup: bool) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.write(height, cleanup)?;
        }
        Ok(())
    }

    fn reset_cost_basis_impl(&mut self) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            state.reset_cost_basis_data_if_needed()?;
        }
        Ok(())
    }

    fn reset_iteration_impl(&mut self) {
        if let Some(state) = self.state.as_mut() {
            state.reset_single_iteration_values();
        }
    }
}

// --- Blanket impl for CohortMetricsBase types (always use full RealizedState) ---

impl<M: CohortMetricsBase + Traversable> Filtered for UTXOCohortVecs<M> {
    fn filter(&self) -> &Filter {
        self.metrics.filter()
    }
}

impl<M: CohortMetricsBase + Traversable> DynCohortVecs for UTXOCohortVecs<M> {
    fn min_stateful_len(&self) -> usize {
        self.metrics.min_stateful_len()
    }

    fn reset_state_starting_height(&mut self) {
        self.reset_state_impl();
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
        _is_day_boundary: bool,
    ) -> Result<()> {
        if let Some(state) = self.state.as_mut() {
            self.metrics.compute_and_push_unrealized(
                height,
                height_price,
                state,
            )?;
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
            .compute_rest_part1(blocks, prices, starting_indexes, exit)?;
        Ok(())
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
