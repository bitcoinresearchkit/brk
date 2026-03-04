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
use brk_types::{Cents, Height, Indexes, Version};
use vecdb::{AnyStoredVec, Exit};

use crate::{blocks, distribution::state::CohortState, prices};

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

    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()>;

    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Cents,
        state: &mut CohortState,
    ) -> Result<()>;

    fn collect_all_vecs_mut(&mut self) -> Vec<&mut dyn AnyStoredVec>;

    fn min_stateful_height_len(&self) -> usize {
        self.supply()
            .min_len()
            .min(self.outputs().min_len())
            .min(self.activity().min_len())
            .min(self.realized_base().min_stateful_height_len())
            .min(self.unrealized_base().min_stateful_height_len())
            .min(self.cost_basis_base().min_stateful_height_len())
    }

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
        starting_indexes: &Indexes,
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
            .map(|o| &o.unrealized_base().net_sentiment.cents.height)
            .collect();

        self.unrealized_base_mut()
            .net_sentiment
            .cents
            .height
            .compute_weighted_average_of_others(starting_indexes.height, &weights, &values, exit)?;

        Ok(())
    }

    /// Compute net_sentiment.height as capital-weighted average from heterogeneous sources.
    fn compute_net_sentiment_from_others_dyn(
        &mut self,
        starting_indexes: &Indexes,
        others: &[&dyn CohortMetricsBase],
        exit: &Exit,
    ) -> Result<()> {
        let weights: Vec<_> = others
            .iter()
            .map(|o| &o.realized_base().realized_cap.height)
            .collect();
        let values: Vec<_> = others
            .iter()
            .map(|o| &o.unrealized_base().net_sentiment.cents.height)
            .collect();

        self.unrealized_base_mut()
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
            .compute_rest_part1(blocks, starting_indexes, exit)?;

        self.realized_base_mut()
            .sent_in_profit
            .compute(prices, starting_indexes.height, exit)?;
        self.realized_base_mut()
            .sent_in_loss
            .compute(prices, starting_indexes.height, exit)?;
        self.realized_base_mut()
            .compute_rest_part1(starting_indexes, exit)?;

        self.unrealized_base_mut()
            .compute_rest(prices, starting_indexes, exit)?;

        Ok(())
    }

    /// Compute net_sentiment.height for separate cohorts (greed - pain).
    fn compute_net_sentiment_height(
        &mut self,
        starting_indexes: &Indexes,
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
        starting_indexes: &Indexes,
        others: &[&dyn CohortMetricsBase],
        exit: &Exit,
    ) -> Result<()>
    where
        Self: Sized,
    {
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
        aggregate!(realized_base_mut, realized_base);
        aggregate!(unrealized_base_mut, unrealized_base);
        aggregate!(cost_basis_base_mut, cost_basis_base);
        Ok(())
    }
}
