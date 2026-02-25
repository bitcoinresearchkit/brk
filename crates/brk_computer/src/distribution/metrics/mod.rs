mod activity;
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
use brk_types::{Cents, Height, Version};
use vecdb::{AnyStoredVec, Exit};

use crate::{ComputeIndexes, blocks, distribution::state::CohortState, prices};

/// Trait defining the interface for cohort metrics containers.
///
/// Provides typed accessor methods for base sub-metric components, default
/// implementations for shared operations that only use base fields, and
/// required methods for operations that vary by extension level.
pub trait CohortMetricsBase: Send + Sync {
    fn filter(&self) -> &Filter;
    fn supply(&self) -> &SupplyMetrics;
    fn supply_mut(&mut self) -> &mut SupplyMetrics;
    fn outputs(&self) -> &OutputsMetrics;
    fn outputs_mut(&mut self) -> &mut OutputsMetrics;
    fn activity(&self) -> &ActivityMetrics;
    fn activity_mut(&mut self) -> &mut ActivityMetrics;
    fn realized_base(&self) -> &RealizedBase;
    fn realized_base_mut(&mut self) -> &mut RealizedBase;
    fn unrealized_base(&self) -> &UnrealizedBase;
    fn unrealized_base_mut(&mut self) -> &mut UnrealizedBase;
    fn cost_basis_base(&self) -> &CostBasisBase;
    fn cost_basis_base_mut(&mut self) -> &mut CostBasisBase;

    // === Required methods (vary by extension level) ===

    /// Validate computed versions against base version.
    /// Extended types also validate cost_basis extended versions.
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()>;

    /// Compute and push unrealized states.
    /// Extended types also push cost_basis percentiles.
    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Cents,
        state: &mut CohortState,
    ) -> Result<()>;

    /// Collect all stored vecs for parallel writing.
    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec>;

    // === Default methods (shared across all cohort metric types, use base fields only) ===

    /// Get minimum length across height-indexed vectors written in block loop.
    fn min_stateful_height_len(&self) -> usize {
        self.supply()
            .min_len()
            .min(self.outputs().min_len())
            .min(self.activity().min_len())
            .min(self.realized_base().min_stateful_height_len())
            .min(self.unrealized_base().min_stateful_height_len())
            .min(self.cost_basis_base().min_stateful_height_len())
    }

    /// Push state values to height-indexed vectors.
    fn truncate_push(&mut self, height: Height, state: &CohortState) -> Result<()> {
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
        self.realized_base_mut()
            .truncate_push(height, &state.realized)?;
        Ok(())
    }

    /// Compute net_sentiment.height as capital-weighted average of component cohorts (same type).
    fn compute_net_sentiment_from_others(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()>
    where
        Self: Sized,
    {
        let weights: Vec<_> = others
            .iter()
            .map(|o| &o.realized_base().realized_cap.height)
            .collect();
        let values: Vec<_> = others
            .iter()
            .map(|o| &o.unrealized_base().net_sentiment.height)
            .collect();

        self.unrealized_base_mut()
            .net_sentiment
            .height
            .compute_weighted_average_of_others(starting_indexes.height, &weights, &values, exit)?;

        Ok(())
    }

    /// Compute net_sentiment.height as capital-weighted average from heterogeneous sources.
    fn compute_net_sentiment_from_others_dyn(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&dyn CohortMetricsBase],
        exit: &Exit,
    ) -> Result<()> {
        let weights: Vec<_> = others
            .iter()
            .map(|o| &o.realized_base().realized_cap.height)
            .collect();
        let values: Vec<_> = others
            .iter()
            .map(|o| &o.unrealized_base().net_sentiment.height)
            .collect();

        self.unrealized_base_mut()
            .net_sentiment
            .height
            .compute_weighted_average_of_others(starting_indexes.height, &weights, &values, exit)?;

        Ok(())
    }

    /// First phase of computed metrics (indexes from height).
    fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.supply_mut()
            .compute_rest_part1(blocks, starting_indexes, exit)?;
        self.outputs_mut()
            .compute_rest(blocks, starting_indexes, exit)?;
        self.activity_mut()
            .compute_rest_part1(blocks, starting_indexes, exit)?;

        self.realized_base_mut()
            .compute_rest_part1(starting_indexes, exit)?;

        self.unrealized_base_mut()
            .compute_rest(prices, starting_indexes, exit)?;

        Ok(())
    }

    /// Compute net_sentiment.height for separate cohorts (greed - pain).
    fn compute_net_sentiment_height(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()> {
        self.unrealized_base_mut()
            .compute_net_sentiment_height(starting_indexes, exit)?;
        Ok(())
    }

    /// Compute aggregate base metrics from heterogeneous source cohorts.
    /// Uses only base fields (supply, outputs, activity, realized_base, unrealized_base, cost_basis_base).
    fn compute_base_from_others(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&dyn CohortMetricsBase],
        exit: &Exit,
    ) -> Result<()>
    where
        Self: Sized,
    {
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
            &others.iter().map(|v| v.activity()).collect::<Vec<_>>(),
            exit,
        )?;
        self.realized_base_mut().compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.realized_base()).collect::<Vec<_>>(),
            exit,
        )?;
        self.unrealized_base_mut().compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.unrealized_base()).collect::<Vec<_>>(),
            exit,
        )?;
        self.cost_basis_base_mut().compute_from_stateful(
            starting_indexes,
            &others.iter().map(|v| v.cost_basis_base()).collect::<Vec<_>>(),
            exit,
        )?;
        Ok(())
    }
}
