use brk_error::Result;
use brk_types::{Cents, Dollars, Height, Sats, Version};
use vecdb::{Exit, ReadableVec};

use crate::{ComputeIndexes, blocks, prices};

/// Dynamic dispatch trait for cohort vectors.
///
/// This trait enables heterogeneous cohort processing via trait objects.
pub trait DynCohortVecs: Send + Sync {
    /// Get minimum length across height-indexed vectors written in block loop.
    fn min_stateful_height_len(&self) -> usize;

    /// Reset the starting height for state tracking.
    fn reset_state_starting_height(&mut self);

    /// Import state from checkpoint at or before the given height.
    fn import_state(&mut self, starting_height: Height) -> Result<Height>;

    /// Validate that computed vectors have correct versions.
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()>;

    /// Push state to height-indexed vectors (truncating if needed).
    fn truncate_push(&mut self, height: Height) -> Result<()>;

    /// Compute and push unrealized profit/loss states and percentiles.
    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Cents,
    ) -> Result<()>;

    /// First phase of post-processing computations.
    fn compute_rest_part1(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()>;

    /// Compute net_sentiment.height for separate cohorts (greed - pain).
    fn compute_net_sentiment_height(
        &mut self,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()>;

    /// Write state checkpoint to disk.
    fn write_state(&mut self, height: Height, cleanup: bool) -> Result<()>;

    /// Reset cost basis data (called during fresh start).
    fn reset_cost_basis_data_if_needed(&mut self) -> Result<()>;

    /// Reset per-block iteration values.
    fn reset_single_iteration_values(&mut self);
}

/// Static dispatch trait for cohort vectors with additional methods.
///
/// Used by address cohorts where all cohorts share the same concrete type.
pub trait CohortVecs: DynCohortVecs {
    /// Compute aggregate cohort from component cohorts.
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()>;

    /// Second phase of post-processing computations.
    fn compute_rest_part2(
        &mut self,
        blocks: &blocks::Vecs,
        prices: &prices::Vecs,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: &impl ReadableVec<Height, Dollars>,
        all_supply_sats: &impl ReadableVec<Height, Sats>,
        exit: &Exit,
    ) -> Result<()>;
}
