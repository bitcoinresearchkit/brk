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
        fn filter(&self) -> &brk_cohort::Filter { &self.filter }
        fn supply(&self) -> &$crate::distribution::metrics::SupplyFull { &self.supply }
        fn supply_mut(&mut self) -> &mut $crate::distribution::metrics::SupplyFull { &mut self.supply }
        fn outputs(&self) -> &$crate::distribution::metrics::OutputsFull { &self.outputs }
        fn outputs_mut(&mut self) -> &mut $crate::distribution::metrics::OutputsFull { &mut self.outputs }
        fn activity(&self) -> &Self::ActivityVecs { &self.activity }
        fn activity_mut(&mut self) -> &mut Self::ActivityVecs { &mut self.activity }
        fn realized(&self) -> &Self::RealizedVecs { &self.realized }
        fn realized_mut(&mut self) -> &mut Self::RealizedVecs { &mut self.realized }
        fn unrealized(&self) -> &Self::UnrealizedVecs { &self.unrealized }
        fn unrealized_mut(&mut self) -> &mut Self::UnrealizedVecs { &mut self.unrealized }
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

pub use activity::{ActivityCore, ActivityFull, ActivityLike};
pub use cohort::{
    AllCohortMetrics, BasicCohortMetrics, CoreCohortMetrics,
    ExtendedAdjustedCohortMetrics, ExtendedCohortMetrics, MinimalCohortMetrics, TypeCohortMetrics,
};
pub use config::ImportConfig;
pub use cost_basis::CostBasis;
pub use profitability::ProfitabilityMetrics;
pub use outputs::{OutputsBase, OutputsFull};
pub use realized::{
    AdjustedSopr, RealizedCore, RealizedFull, RealizedFullAccum, RealizedLike,
    RealizedMinimal,
};
pub use relative::{
    RelativeForAll, RelativeToAll, RelativeWithExtended,
};
pub use supply::{SupplyBase, SupplyFull};
pub use unrealized::{UnrealizedBase, UnrealizedBasic, UnrealizedCore, UnrealizedFull, UnrealizedLike};

use brk_cohort::Filter;
use brk_error::Result;
use brk_types::{Cents, Height, Indexes, Version};
use vecdb::{AnyStoredVec, Exit, StorageMode};

use crate::{blocks, distribution::state::{CohortState, CoreRealizedState, MinimalRealizedState, RealizedOps, RealizedState}, prices};

pub trait CohortMetricsState {
    type Realized: RealizedOps;
}

impl<M: StorageMode> CohortMetricsState for TypeCohortMetrics<M> {
    type Realized = MinimalRealizedState;
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
    type ActivityVecs: ActivityLike;
    type RealizedVecs: RealizedLike;
    type UnrealizedVecs: UnrealizedLike;

    fn filter(&self) -> &Filter;
    fn supply(&self) -> &SupplyFull;
    fn supply_mut(&mut self) -> &mut SupplyFull;
    fn outputs(&self) -> &OutputsFull;
    fn outputs_mut(&mut self) -> &mut OutputsFull;
    fn activity(&self) -> &Self::ActivityVecs;
    fn activity_mut(&mut self) -> &mut Self::ActivityVecs;
    fn realized(&self) -> &Self::RealizedVecs;
    fn realized_mut(&mut self) -> &mut Self::RealizedVecs;
    fn unrealized(&self) -> &Self::UnrealizedVecs;
    fn unrealized_mut(&mut self) -> &mut Self::UnrealizedVecs;

    /// Convenience: access activity as `&ActivityCore` (via `ActivityLike::as_core`).
    fn activity_core(&self) -> &ActivityCore { self.activity().as_core() }
    fn activity_core_mut(&mut self) -> &mut ActivityCore { self.activity_mut().as_core_mut() }

    /// Convenience: access realized as `&RealizedCore` (via `RealizedLike::as_core`).
    fn realized_core(&self) -> &RealizedCore { self.realized().as_core() }
    fn realized_core_mut(&mut self) -> &mut RealizedCore { self.realized_mut().as_core_mut() }

    /// Convenience: access unrealized as `&UnrealizedBase` (via `UnrealizedLike::as_base`).
    fn unrealized_base(&self) -> &UnrealizedBase { self.unrealized().as_base() }
    fn unrealized_base_mut(&mut self) -> &mut UnrealizedBase { self.unrealized_mut().as_base_mut() }

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()> {
        self.supply_mut().validate_computed_versions(base_version)?;
        self.activity_mut().validate_computed_versions(base_version)?;
        Ok(())
    }

    /// Apply pending state, compute and push unrealized state.
    fn compute_and_push_unrealized(
        &mut self,
        height: Height,
        height_price: Cents,
        state: &mut CohortState<RealizedState>,
    ) -> Result<()> {
        state.apply_pending();
        let unrealized_state = state.compute_unrealized_state(height_price);
        self.unrealized_mut()
            .truncate_push(height, &unrealized_state)?;
        Ok(())
    }

    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec>;

    fn min_stateful_height_len(&self) -> usize {
        self.supply()
            .min_len()
            .min(self.outputs().min_len())
            .min(self.activity().min_len())
            .min(self.realized().min_stateful_height_len())
            .min(self.unrealized().min_stateful_height_len())
    }

    fn truncate_push(&mut self, height: Height, state: &CohortState<RealizedState>) -> Result<()> {
        self.supply_mut().truncate_push(height, state)?;
        self.outputs_mut().truncate_push(height, state)?;
        self.activity_mut().truncate_push(height, state)?;
        self.realized_mut().truncate_push(height, state)?;
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
            .compute_rest_part1(blocks, starting_indexes, exit)?;

        self.realized_mut()
            .compute_rest_part1(blocks, starting_indexes, exit)?;

        self.unrealized_mut()
            .compute_rest(blocks, prices, starting_indexes, exit)?;

        self.unrealized_mut()
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
            &others.iter().map(|v| v.unrealized_base()).collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }
}
