use brk_types::{FeeRate, RecommendedFees};

use super::stats::BlockStats;

/// Output rounding granularity in sat/vB. mempool.space's
/// `/api/v1/fees/recommended` uses `1.0`, their `/precise`
/// variant uses `0.001`. bitview always emits precise.
const MIN_INCREMENT: FeeRate = FeeRate::new(0.001);
/// `getPreciseRecommendedFee` adds this to `fastestFee` and
/// half of it to `halfHourFee`, then floors them. Compensates
/// for sub-1-sat/vB fees mined by hashrate that ignores the
/// relay floor.
const PRIORITY_FACTOR: FeeRate = FeeRate::new(0.5);
const MIN_FASTEST_FEE: FeeRate = FeeRate::new(1.0);
const MIN_HALF_HOUR_FEE: FeeRate = FeeRate::new(0.5);

pub struct Fees;

impl Fees {
    /// Literal port of mempool.space's `getPreciseRecommendedFee`
    /// (backend/src/api/fee-api.ts). `min_fee` is bitcoind's live
    /// `mempoolminfee` in sat/vB and acts as a floor for every tier
    /// while the mempool is purging by fee.
    pub fn compute(stats: &[BlockStats], min_fee: FeeRate) -> RecommendedFees {
        let minimum_fee = min_fee.ceil_to(MIN_INCREMENT).max(MIN_INCREMENT);

        let first = Self::block_fee(stats, 0, None, minimum_fee);
        let second = Self::block_fee(stats, 1, Some(first), minimum_fee);
        let third = Self::block_fee(stats, 2, Some(second), minimum_fee);

        let economy = third.clamp(minimum_fee, minimum_fee * 2.0);
        let hour = minimum_fee.max(third).max(economy);
        let half_hour = minimum_fee.max(second).max(hour);
        let fastest = minimum_fee.max(first).max(half_hour);

        let fastest = (fastest + PRIORITY_FACTOR).max(MIN_FASTEST_FEE);
        let half_hour = (half_hour + PRIORITY_FACTOR / 2.0).max(MIN_HALF_HOUR_FEE);

        RecommendedFees {
            fastest_fee: fastest.round_milli(),
            half_hour_fee: half_hour.round_milli(),
            hour_fee: hour.round_milli(),
            economy_fee: economy.round_milli(),
            minimum_fee: minimum_fee.round_milli(),
        }
    }

    /// Optimized median for the i-th projected block, or `min_fee` if
    /// the block doesn't exist. `prev` is the prior tier's optimized
    /// fee, used to smooth toward continuity.
    fn block_fee(
        stats: &[BlockStats],
        i: usize,
        prev: Option<FeeRate>,
        min_fee: FeeRate,
    ) -> FeeRate {
        stats.get(i).map_or(min_fee, |b| {
            Self::optimize_median_fee(b, stats.get(i + 1), prev, min_fee)
        })
    }

    /// Pick the fee for one projected block, smoothing toward the
    /// previous tier and discounting partially-full final blocks.
    fn optimize_median_fee(
        block: &BlockStats,
        next_block: Option<&BlockStats>,
        previous_fee: Option<FeeRate>,
        min_fee: FeeRate,
    ) -> FeeRate {
        let median = block.median_fee_rate();
        let use_fee = previous_fee.map_or(median, |prev| FeeRate::mean(median, prev));
        let vsize = u64::from(block.total_vsize);
        if vsize <= 500_000 || median < min_fee {
            return min_fee;
        }
        if vsize <= 950_000 && next_block.is_none() {
            let multiplier = (vsize - 500_000) as f64 / 500_000.0;
            return (use_fee * multiplier).round_to(MIN_INCREMENT).max(min_fee);
        }
        use_fee.ceil_to(MIN_INCREMENT).max(min_fee)
    }
}
