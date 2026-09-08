//! Calculation algorithms shared by Bitview plugins.
//!
//! Metric vector ownership and view composition live in bitview_vecs.
mod age_band;
mod block_walker;
mod cumulative_sats;
mod drawdown;
mod statistics;
mod traits;
mod weighted;

pub use age_band::{AgeBand, MINIMUM_DURATION_DAYS};
pub use block_walker::{BlockAggregate, CoinbasePolicy, walk_blocks};
pub use cumulative_sats::compute_cumulative_sats_from_indexes;
pub use drawdown::ComputeDrawdown;
pub use statistics::{
    ComputeRollingMedianFromStarts, ExactOrderStats, FenwickNode, FenwickTree,
    compute_rolling_distribution_from_starts,
};
pub use traits::{ComputedVecValue, FixedRatio, NumericValue};
pub use weighted::{WeightedCohortContribution, WeightedCohortState, WeightedRatio};
