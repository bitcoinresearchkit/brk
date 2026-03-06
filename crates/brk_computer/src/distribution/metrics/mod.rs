/// Aggregate a field by summing the same field across `others`.
macro_rules! sum_others {
    ($self_:ident, $si:ident, $others:ident, $exit:ident; $($field:tt).+) => {
        $self_.$($field).+.compute_sum_of_others(
            $si.height,
            &$others.iter().map(|v| &v.$($field).+).collect::<Vec<_>>(),
            $exit,
        )?
    };
}

mod activity;

/// DRY macro for `CohortMetricsBase` impl on cohort metric types.
///
/// All types share the same 13 accessor methods and common `collect_all_vecs_mut` shape.
/// Two variants handle the cost basis difference:
///
/// - `base_cost_basis`: `CostBasisBase` only (no percentiles, no cost_basis version check)
/// - `extended_cost_basis`: `CostBasisWithExtended` (percentiles + cost_basis version check)
/// - `deref_extended_cost_basis`: Deref wrapper delegating to `self.inner` (avoids DerefMut borrow conflicts)
macro_rules! impl_cohort_metrics_base {
    ($type:ident, base_cost_basis) => {
        impl $crate::distribution::metrics::CohortMetricsBase for $type {
            impl_cohort_metrics_base!(@accessors);

            fn validate_computed_versions(&mut self, base_version: brk_types::Version) -> brk_error::Result<()> {
                self.supply.validate_computed_versions(base_version)?;
                self.activity.validate_computed_versions(base_version)?;
                Ok(())
            }

            fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn vecdb::AnyStoredVec> {
                let mut vecs: Vec<&mut dyn vecdb::AnyStoredVec> = Vec::new();
                vecs.extend(self.supply.collect_vecs_mut());
                vecs.extend(self.outputs.collect_vecs_mut());
                vecs.extend(self.activity.collect_vecs_mut());
                vecs.extend(self.realized.collect_vecs_mut());
                vecs.extend(self.cost_basis.collect_vecs_mut());
                vecs.extend(self.unrealized.collect_vecs_mut());
                vecs
            }
        }
    };

    ($type:ident, extended_cost_basis) => {
        impl $crate::distribution::metrics::CohortMetricsBase for $type {
            impl_cohort_metrics_base!(@accessors);

            fn validate_computed_versions(&mut self, base_version: brk_types::Version) -> brk_error::Result<()> {
                self.supply.validate_computed_versions(base_version)?;
                self.activity.validate_computed_versions(base_version)?;
                self.cost_basis.validate_computed_versions(base_version)?;
                Ok(())
            }

            fn compute_then_truncate_push_unrealized_states(
                &mut self,
                height: brk_types::Height,
                height_price: brk_types::Cents,
                state: &mut $crate::distribution::state::CohortState<$crate::distribution::state::RealizedState>,
                is_day_boundary: bool,
            ) -> brk_error::Result<()> {
                self.compute_and_push_unrealized_base(height, height_price, state)?;
                self.cost_basis
                    .extended
                    .truncate_push_percentiles(height, state, is_day_boundary)?;
                Ok(())
            }

            fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn vecdb::AnyStoredVec> {
                let mut vecs: Vec<&mut dyn vecdb::AnyStoredVec> = Vec::new();
                vecs.extend(self.supply.collect_vecs_mut());
                vecs.extend(self.outputs.collect_vecs_mut());
                vecs.extend(self.activity.collect_vecs_mut());
                vecs.extend(self.realized.collect_vecs_mut());
                vecs.extend(self.cost_basis.collect_vecs_mut());
                vecs.extend(self.unrealized.collect_vecs_mut());
                vecs.push(&mut self.dormancy.height);
                vecs.push(&mut self.velocity.height);
                vecs
            }
        }
    };

    ($type:ident, deref_extended_cost_basis) => {
        impl $crate::distribution::metrics::CohortMetricsBase for $type {
            impl_cohort_metrics_base!(@deref_accessors);

            fn validate_computed_versions(&mut self, base_version: brk_types::Version) -> brk_error::Result<()> {
                self.inner.validate_computed_versions(base_version)
            }

            fn compute_then_truncate_push_unrealized_states(
                &mut self,
                height: brk_types::Height,
                height_price: brk_types::Cents,
                state: &mut $crate::distribution::state::CohortState<$crate::distribution::state::RealizedState>,
                is_day_boundary: bool,
            ) -> brk_error::Result<()> {
                self.inner.compute_then_truncate_push_unrealized_states(
                    height, height_price, state, is_day_boundary,
                )
            }

            fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn vecdb::AnyStoredVec> {
                self.inner.collect_all_vecs_mut()
            }
        }
    };

    (@accessors) => {
        fn filter(&self) -> &brk_cohort::Filter { &self.filter }
        fn supply(&self) -> &$crate::distribution::metrics::SupplyMetrics { &self.supply }
        fn supply_mut(&mut self) -> &mut $crate::distribution::metrics::SupplyMetrics { &mut self.supply }
        fn outputs(&self) -> &$crate::distribution::metrics::OutputsMetrics { &self.outputs }
        fn outputs_mut(&mut self) -> &mut $crate::distribution::metrics::OutputsMetrics { &mut self.outputs }
        fn activity(&self) -> &$crate::distribution::metrics::ActivityMetrics { &self.activity }
        fn activity_mut(&mut self) -> &mut $crate::distribution::metrics::ActivityMetrics { &mut self.activity }
        fn realized_full(&self) -> &$crate::distribution::metrics::RealizedFull { &self.realized }
        fn realized_full_mut(&mut self) -> &mut $crate::distribution::metrics::RealizedFull { &mut self.realized }
        fn unrealized_full(&self) -> &$crate::distribution::metrics::UnrealizedFull { &self.unrealized }
        fn unrealized_full_mut(&mut self) -> &mut $crate::distribution::metrics::UnrealizedFull { &mut self.unrealized }
        fn cost_basis_base(&self) -> &$crate::distribution::metrics::CostBasisBase { &self.cost_basis }
        fn cost_basis_base_mut(&mut self) -> &mut $crate::distribution::metrics::CostBasisBase { &mut self.cost_basis }
    };

    (@deref_accessors) => {
        fn filter(&self) -> &brk_cohort::Filter { self.inner.filter() }
        fn supply(&self) -> &$crate::distribution::metrics::SupplyMetrics { self.inner.supply() }
        fn supply_mut(&mut self) -> &mut $crate::distribution::metrics::SupplyMetrics { self.inner.supply_mut() }
        fn outputs(&self) -> &$crate::distribution::metrics::OutputsMetrics { self.inner.outputs() }
        fn outputs_mut(&mut self) -> &mut $crate::distribution::metrics::OutputsMetrics { self.inner.outputs_mut() }
        fn activity(&self) -> &$crate::distribution::metrics::ActivityMetrics { self.inner.activity() }
        fn activity_mut(&mut self) -> &mut $crate::distribution::metrics::ActivityMetrics { self.inner.activity_mut() }
        fn realized_full(&self) -> &$crate::distribution::metrics::RealizedFull { self.inner.realized_full() }
        fn realized_full_mut(&mut self) -> &mut $crate::distribution::metrics::RealizedFull { self.inner.realized_full_mut() }
        fn unrealized_full(&self) -> &$crate::distribution::metrics::UnrealizedFull { self.inner.unrealized_full() }
        fn unrealized_full_mut(&mut self) -> &mut $crate::distribution::metrics::UnrealizedFull { self.inner.unrealized_full_mut() }
        fn cost_basis_base(&self) -> &$crate::distribution::metrics::CostBasisBase { self.inner.cost_basis_base() }
        fn cost_basis_base_mut(&mut self) -> &mut $crate::distribution::metrics::CostBasisBase { self.inner.cost_basis_base_mut() }
    };
}

mod cohort;
mod config;
mod cost_basis;
mod outputs;
mod realized;
mod relative;
mod supply;
mod unrealized;

pub use activity::*;
pub use cohort::*;
pub use config::*;
pub use cost_basis::*;
pub use outputs::*;
pub use realized::*;
pub use relative::*;
pub use supply::*;
pub use unrealized::*;

use brk_cohort::Filter;
use brk_error::Result;
use brk_types::{Cents, Height, Indexes, Version};
use vecdb::{AnyStoredVec, Exit, StorageMode};

use crate::{blocks, distribution::state::{CohortState, CoreRealizedState, MinimalRealizedState, RealizedOps, RealizedState}, prices};

pub trait CohortMetricsState {
    type Realized: RealizedOps;
}

impl<M: StorageMode> CohortMetricsState for MinimalCohortMetrics<M> {
    type Realized = MinimalRealizedState;
}
impl<M: StorageMode> CohortMetricsState for CoreCohortMetrics<M> {
    type Realized = CoreRealizedState;
}
impl<M: StorageMode> CohortMetricsState for BasicCohortMetrics<M> {
    type Realized = RealizedState;
}
impl<M: StorageMode> CohortMetricsState for ExtendedCohortMetrics<M> {
    type Realized = RealizedState;
}
impl<M: StorageMode> CohortMetricsState for ExtendedAdjustedCohortMetrics<M> {
    type Realized = RealizedState;
}
impl<M: StorageMode> CohortMetricsState for AllCohortMetrics<M> {
    type Realized = RealizedState;
}

pub trait CohortMetricsBase: CohortMetricsState<Realized = RealizedState> + Send + Sync {
    fn filter(&self) -> &Filter;
    fn supply(&self) -> &SupplyMetrics;
    fn supply_mut(&mut self) -> &mut SupplyMetrics;
    fn outputs(&self) -> &OutputsMetrics;
    fn outputs_mut(&mut self) -> &mut OutputsMetrics;
    fn activity(&self) -> &ActivityMetrics;
    fn activity_mut(&mut self) -> &mut ActivityMetrics;
    fn realized_full(&self) -> &RealizedFull;
    fn realized_full_mut(&mut self) -> &mut RealizedFull;
    fn unrealized_full(&self) -> &UnrealizedFull;
    fn unrealized_full_mut(&mut self) -> &mut UnrealizedFull;
    fn cost_basis_base(&self) -> &CostBasisBase;
    fn cost_basis_base_mut(&mut self) -> &mut CostBasisBase;

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()>;

    /// Apply pending, push min/max cost basis, compute and push unrealized state.
    fn compute_and_push_unrealized_base(
        &mut self,
        height: Height,
        height_price: Cents,
        state: &mut CohortState<RealizedState>,
    ) -> Result<()> {
        state.apply_pending();
        self.cost_basis_base_mut()
            .truncate_push_minmax(height, state)?;
        let unrealized_state = state.compute_unrealized_state(height_price);
        self.unrealized_full_mut()
            .truncate_push(height, &unrealized_state)?;
        Ok(())
    }

    /// Compute and push unrealized states. Extended types override to also push percentiles.
    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Cents,
        state: &mut CohortState<RealizedState>,
        _is_day_boundary: bool,
    ) -> Result<()> {
        self.compute_and_push_unrealized_base(height, height_price, state)
    }

    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec>;

    fn min_stateful_height_len(&self) -> usize {
        self.supply()
            .min_len()
            .min(self.outputs().min_len())
            .min(self.activity().min_len())
            .min(self.realized_full().min_stateful_height_len())
            .min(self.unrealized_full().min_stateful_height_len())
            .min(self.cost_basis_base().min_stateful_height_len())
    }

    fn truncate_push(&mut self, height: Height, state: &CohortState<RealizedState>) -> Result<()> {
        self.supply_mut()
            .truncate_push(height, state.supply.value)?;
        self.outputs_mut()
            .truncate_push(height, state.supply.utxo_count)?;
        self.activity_mut().truncate_push(
            height,
            state.sent,
            state.satblocks_destroyed,
            state.satdays_destroyed,
        )?;
        self.realized_full_mut()
            .truncate_push(height, &state.realized)?;
        Ok(())
    }

    /// Compute net_sentiment.height as capital-weighted average of component cohorts.
    fn compute_net_sentiment_from_others<T: CohortMetricsBase>(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&T],
        exit: &Exit,
    ) -> Result<()> {
        let weights: Vec<_> = others
            .iter()
            .map(|o| &o.realized_full().realized_cap.height)
            .collect();
        let values: Vec<_> = others
            .iter()
            .map(|o| &o.unrealized_full().net_sentiment.cents.height)
            .collect();

        self.unrealized_full_mut()
            .net_sentiment
            .cents
            .height
            .compute_weighted_average_of_others(starting_indexes.height, &weights, &values, exit)?;

        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_mut()
            .compute(prices, starting_indexes.height, exit)?;
        self.supply_mut()
            .compute_rest_part1(blocks, starting_indexes, exit)?;
        self.outputs_mut()
            .compute_rest(blocks, starting_indexes, exit)?;
        self.activity_mut()
            .sent
            .compute(prices, starting_indexes.height, exit)?;
        self.activity_mut()
            .compute_rest_part1(blocks, prices, starting_indexes, exit)?;

        self.realized_full_mut()
            .sent_in_profit
            .compute(prices, starting_indexes.height, exit)?;
        self.realized_full_mut()
            .sent_in_loss
            .compute(prices, starting_indexes.height, exit)?;
        self.realized_full_mut()
            .compute_rest_part1(starting_indexes, exit)?;

        self.unrealized_full_mut()
            .compute_rest(prices, starting_indexes, exit)?;

        Ok(())
    }

    /// Compute net_sentiment.height for separate cohorts (greed - pain).
    fn compute_net_sentiment_height(
        &mut self,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.unrealized_full_mut()
            .compute_net_sentiment_height(starting_indexes, exit)?;
        Ok(())
    }

    /// Compute aggregate base metrics from source cohorts.
    fn compute_base_from_others<T: CohortMetricsBase>(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&T],
        exit: &Exit,
    ) -> Result<()> {
        macro_rules! aggregate {
            ($self_mut:ident, $accessor:ident) => {
                self.$self_mut().compute_from_stateful(
                    starting_indexes,
                    &others.iter().map(|v| v.$accessor()).collect::<Vec<_>>(),
                    exit,
                )?
            };
        }

        aggregate!(supply_mut, supply);
        aggregate!(outputs_mut, outputs);
        aggregate!(activity_mut, activity);
        aggregate!(realized_full_mut, realized_full);
        aggregate!(unrealized_full_mut, unrealized_full);
        aggregate!(cost_basis_base_mut, cost_basis_base);
        Ok(())
    }
}
