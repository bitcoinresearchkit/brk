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

/// Accessor methods for `CohortMetricsBase` implementations.
///
/// All cohort metric types share the same field names (`filter`, `supply`, `outputs`,
/// `activity`, `realized`, `unrealized`). For wrapper types like
/// `ExtendedAdjustedCohortMetrics`, Rust's auto-deref resolves these through `Deref`.
macro_rules! impl_cohort_accessors {
    () => {
        fn filter(&self) -> &brk_cohort::Filter {
            &self.filter
        }
        fn supply(&self) -> &$crate::distribution::metrics::SupplyCore {
            &self.supply
        }
        fn supply_mut(&mut self) -> &mut $crate::distribution::metrics::SupplyCore {
            &mut self.supply
        }
        fn outputs(&self) -> &$crate::distribution::metrics::OutputsBase {
            &self.outputs
        }
        fn outputs_mut(&mut self) -> &mut $crate::distribution::metrics::OutputsBase {
            &mut self.outputs
        }
        fn activity(&self) -> &Self::ActivityVecs {
            &self.activity
        }
        fn activity_mut(&mut self) -> &mut Self::ActivityVecs {
            &mut self.activity
        }
        fn realized(&self) -> &Self::RealizedVecs {
            &self.realized
        }
        fn realized_mut(&mut self) -> &mut Self::RealizedVecs {
            &mut self.realized
        }
        fn unrealized(&self) -> &Self::UnrealizedVecs {
            &self.unrealized
        }
        fn unrealized_mut(&mut self) -> &mut Self::UnrealizedVecs {
            &mut self.unrealized
        }
        fn supply_and_unrealized_mut(&mut self) -> (&$crate::distribution::metrics::SupplyCore, &mut Self::UnrealizedVecs) {
            (&*self.supply, &mut self.unrealized)
        }
    };
}

/// Variant of `impl_cohort_accessors` for wrapper types that deref to an `inner` field.
/// Uses `self.inner.*` directly to enable split borrows that Rust cannot infer through `Deref`.
macro_rules! impl_cohort_accessors_inner {
    () => {
        fn filter(&self) -> &brk_cohort::Filter {
            &self.inner.filter
        }
        fn supply(&self) -> &$crate::distribution::metrics::SupplyCore {
            &self.inner.supply
        }
        fn supply_mut(&mut self) -> &mut $crate::distribution::metrics::SupplyCore {
            &mut self.inner.supply
        }
        fn outputs(&self) -> &$crate::distribution::metrics::OutputsBase {
            &self.inner.outputs
        }
        fn outputs_mut(&mut self) -> &mut $crate::distribution::metrics::OutputsBase {
            &mut self.inner.outputs
        }
        fn activity(&self) -> &Self::ActivityVecs {
            &self.inner.activity
        }
        fn activity_mut(&mut self) -> &mut Self::ActivityVecs {
            &mut self.inner.activity
        }
        fn realized(&self) -> &Self::RealizedVecs {
            &self.inner.realized
        }
        fn realized_mut(&mut self) -> &mut Self::RealizedVecs {
            &mut self.inner.realized
        }
        fn unrealized(&self) -> &Self::UnrealizedVecs {
            &self.inner.unrealized
        }
        fn unrealized_mut(&mut self) -> &mut Self::UnrealizedVecs {
            &mut self.inner.unrealized
        }
        fn supply_and_unrealized_mut(&mut self) -> (&$crate::distribution::metrics::SupplyCore, &mut Self::UnrealizedVecs) {
            (&*self.inner.supply, &mut self.inner.unrealized)
        }
    };
}

mod cohort;
mod config;
mod cost_basis;
mod outputs;
mod profitability;
mod realized;
mod relative;
mod supply;
mod unrealized;

pub use activity::{ActivityCore, ActivityFull, ActivityLike, ActivityMinimal};
pub use cohort::{
    AllCohortMetrics, BasicCohortMetrics, CoreCohortMetrics, ExtendedAdjustedCohortMetrics,
    ExtendedCohortMetrics, MinimalCohortMetrics, TypeCohortMetrics,
};
pub use config::ImportConfig;
pub use cost_basis::CostBasis;
pub use outputs::OutputsBase;
pub use profitability::ProfitabilityMetrics;
pub use realized::{
    AdjustedSopr, RealizedCore, RealizedFull, RealizedFullAccum, RealizedLike, RealizedMinimal,
};
pub use relative::{RelativeForAll, RelativeToAll, RelativeWithExtended};
pub use supply::{SupplyBase, SupplyCore};
pub use unrealized::{
    UnrealizedBase, UnrealizedBasic, UnrealizedCore, UnrealizedFull, UnrealizedLike,
    UnrealizedMinimal,
};

use brk_cohort::Filter;
use brk_error::Result;
use brk_types::{Cents, Indexes, Version};
use vecdb::{AnyStoredVec, Exit, StorageMode};

use crate::{
    distribution::state::{
        CohortState, CoreRealizedState, CostBasisData, CostBasisOps, CostBasisRaw,
        MinimalRealizedState, RealizedOps, RealizedState, WithCapital, WithoutCapital,
    },
    prices,
};

pub trait CohortMetricsState {
    type Realized: RealizedOps;
    type CostBasis: CostBasisOps;
}

impl<M: StorageMode> CohortMetricsState for TypeCohortMetrics<M> {
    type Realized = MinimalRealizedState;
    type CostBasis = CostBasisData<WithoutCapital>;
}
impl<M: StorageMode> CohortMetricsState for MinimalCohortMetrics<M> {
    type Realized = MinimalRealizedState;
    type CostBasis = CostBasisRaw;
}
impl<M: StorageMode> CohortMetricsState for CoreCohortMetrics<M> {
    type Realized = CoreRealizedState;
    type CostBasis = CostBasisData<WithoutCapital>;
}
impl<M: StorageMode> CohortMetricsState for BasicCohortMetrics<M> {
    type Realized = RealizedState;
    type CostBasis = CostBasisData<WithCapital>;
}
impl<M: StorageMode> CohortMetricsState for ExtendedCohortMetrics<M> {
    type Realized = RealizedState;
    type CostBasis = CostBasisData<WithCapital>;
}
impl<M: StorageMode> CohortMetricsState for ExtendedAdjustedCohortMetrics<M> {
    type Realized = RealizedState;
    type CostBasis = CostBasisData<WithCapital>;
}
impl<M: StorageMode> CohortMetricsState for AllCohortMetrics<M> {
    type Realized = RealizedState;
    type CostBasis = CostBasisData<WithCapital>;
}

pub trait CohortMetricsBase:
    CohortMetricsState<Realized = RealizedState, CostBasis = CostBasisData<WithCapital>> + Send + Sync
{
    type ActivityVecs: ActivityLike;
    type RealizedVecs: RealizedLike;
    type UnrealizedVecs: UnrealizedLike;

    fn filter(&self) -> &Filter;
    fn supply(&self) -> &SupplyCore;
    fn supply_mut(&mut self) -> &mut SupplyCore;
    fn outputs(&self) -> &OutputsBase;
    fn outputs_mut(&mut self) -> &mut OutputsBase;
    fn activity(&self) -> &Self::ActivityVecs;
    fn activity_mut(&mut self) -> &mut Self::ActivityVecs;
    fn realized(&self) -> &Self::RealizedVecs;
    fn realized_mut(&mut self) -> &mut Self::RealizedVecs;
    fn unrealized(&self) -> &Self::UnrealizedVecs;
    fn unrealized_mut(&mut self) -> &mut Self::UnrealizedVecs;
    fn supply_and_unrealized_mut(&mut self) -> (&SupplyCore, &mut Self::UnrealizedVecs);

    /// Convenience: access activity as `&ActivityCore` (via `ActivityLike::as_core`).
    fn activity_core(&self) -> &ActivityCore {
        self.activity().as_core()
    }
    fn activity_core_mut(&mut self) -> &mut ActivityCore {
        self.activity_mut().as_core_mut()
    }

    /// Convenience: access realized as `&RealizedCore` (via `RealizedLike::as_core`).
    fn realized_core(&self) -> &RealizedCore {
        self.realized().as_core()
    }
    fn realized_core_mut(&mut self) -> &mut RealizedCore {
        self.realized_mut().as_core_mut()
    }

    /// Convenience: access unrealized as `&UnrealizedBase` (via `UnrealizedLike::as_base`).
    fn unrealized_base(&self) -> &UnrealizedBase {
        self.unrealized().as_base()
    }
    fn unrealized_base_mut(&mut self) -> &mut UnrealizedBase {
        self.unrealized_mut().as_base_mut()
    }

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.supply_mut().validate_computed_versions(base_version)?;
        self.activity_mut()
            .validate_computed_versions(base_version)?;
        Ok(())
    }

    /// Apply pending state, compute and push unrealized state.
    fn compute_and_push_unrealized(
        &mut self,
        height_price: Cents,
        state: &mut CohortState<RealizedState, CostBasisData<WithCapital>>,
    ) {
        state.apply_pending();
        let unrealized_state = state.compute_unrealized_state(height_price);
        self.unrealized_mut().push_state(&unrealized_state);
        self.supply_mut().push_profitability(&unrealized_state);
    }

    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec>;

    fn min_stateful_len(&self) -> usize {
        self.supply()
            .min_len()
            .min(self.outputs().min_len())
            .min(self.activity().min_len())
            .min(self.realized().min_stateful_len())
            .min(self.unrealized().min_stateful_len())
    }

    fn push_state(
        &mut self,
        state: &CohortState<RealizedState, CostBasisData<WithCapital>>,
    ) {
        self.supply_mut().push_state(state);
        self.outputs_mut().push_state(state);
        self.activity_mut().push_state(state);
        self.realized_mut().push_state(state);
    }

    /// First phase of computed metrics (indexes from height).
    fn compute_rest_part1(
        &mut self,
        prices: &prices::Vecs,
        starting_indexes: &Indexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_mut()
            .compute(prices, starting_indexes.height, exit)?;
        self.activity_mut()
            .compute_rest_part1(prices, starting_indexes, exit)?;

        self.realized_mut()
            .compute_rest_part1(starting_indexes, exit)?;

        let (supply, unrealized) = self.supply_and_unrealized_mut();
        unrealized.compute_rest(
            prices,
            starting_indexes,
            &supply.in_profit.sats.height,
            &supply.in_loss.sats.height,
            exit,
        )?;

        Ok(())
    }

    /// Compute aggregate base metrics from source cohorts.
    fn compute_base_from_others<T: CohortMetricsBase>(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&T],
        exit: &Exit,
    ) -> Result<()> {
        self.supply_mut().compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.supply()).collect::<Vec<_>>(),
            exit,
        )?;
        self.outputs_mut().compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.outputs()).collect::<Vec<_>>(),
            exit,
        )?;
        self.activity_mut().compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.activity_core()).collect::<Vec<_>>(),
            exit,
        )?;
        self.realized_mut().compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.realized_core()).collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized_base_mut().compute_from_stateful(
            starting_indexes,
            &others
                .iter()
                .map(|v| v.unrealized_base())
                .collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }
}
