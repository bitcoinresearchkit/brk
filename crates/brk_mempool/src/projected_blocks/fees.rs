use brk_types::{FeeRate, RecommendedFees};

use super::stats::BlockStats;

/// Minimum fee rate for estimation (sat/vB).
const MIN_FEE_RATE: f64 = 1.0;

/// Compute recommended fees from block stats (mempool.space style).
pub fn compute_recommended_fees(stats: &[BlockStats]) -> RecommendedFees {
    RecommendedFees {
        fastest_fee: median_fee_for_block(stats, 0),
        half_hour_fee: median_fee_for_block(stats, 2),
        hour_fee: median_fee_for_block(stats, 5),
        economy_fee: median_fee_for_block(stats, 7),
        minimum_fee: FeeRate::from(MIN_FEE_RATE),
    }
}

/// Get the median fee rate for block N.
fn median_fee_for_block(stats: &[BlockStats], block_index: usize) -> FeeRate {
    stats
        .get(block_index)
        .map(|s| s.median_fee_rate())
        .unwrap_or_else(|| FeeRate::from(MIN_FEE_RATE))
}
