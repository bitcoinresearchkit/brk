use brk_error::Result;
use brk_types::{DateIndex, Dollars, Height, Version};
use vecdb::{Exit, IterableVec};

use crate::{ComputeIndexes, indexes, price};

/// Dynamic dispatch trait for cohort vectors.
///
/// This trait enables heterogeneous cohort processing via trait objects.
pub trait DynCohortVecs: Send + Sync {
    /// Get minimum length across height-indexed vectors written in block loop.
    fn min_stateful_height_len(&self) -> usize;

    /// Get minimum length across dateindex-indexed vectors written in block loop.
    fn min_stateful_dateindex_len(&self) -> usize;

    /// Reset the starting height for state tracking.
    fn reset_state_starting_height(&mut self);

    /// Import state from checkpoint at or before the given height.
    fn import_state(&mut self, starting_height: Height) -> Result<Height>;

    /// Validate that computed vectors have correct versions.
    fn validate_computed_versions(&mut self, base_version: Version) -> Result<()>;

    /// Push state to height-indexed vectors (truncating if needed).
    fn truncate_push(&mut self, height: Height) -> Result<()>;

    /// Compute and push unrealized profit/loss states.
    fn compute_then_truncate_push_unrealized_states(
        &mut self,
        height: Height,
        height_price: Option<Dollars>,
        dateindex: Option<DateIndex>,
        date_price: Option<Option<Dollars>>,
    ) -> Result<()>;

    /// First phase of post-processing computations.
    #[allow(clippy::too_many_arguments)]
    fn compute_rest_part1(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        exit: &Exit,
    ) -> Result<()>;
}

/// Static dispatch trait for cohort vectors with additional methods.
pub trait CohortVecs: DynCohortVecs {
    /// Compute aggregate cohort from component cohorts.
    fn compute_from_stateful(
        &mut self,
        starting_indexes: &ComputeIndexes,
        others: &[&Self],
        exit: &Exit,
    ) -> Result<()>;

    /// Second phase of post-processing computations.
    #[allow(clippy::too_many_arguments)]
    fn compute_rest_part2(
        &mut self,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
        starting_indexes: &ComputeIndexes,
        height_to_market_cap: Option<&impl IterableVec<Height, Dollars>>,
        dateindex_to_market_cap: Option<&impl IterableVec<DateIndex, Dollars>>,
        exit: &Exit,
    ) -> Result<()>;
}
