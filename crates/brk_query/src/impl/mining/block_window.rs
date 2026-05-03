use std::{
    collections::BTreeMap,
    iter::Sum,
    ops::{Deref, Div},
};

use brk_types::{Height, TimePeriod, Timestamp};
use vecdb::{ReadableVec, VecValue};

use crate::Query;

/// Mempool.space's `GROUP BY UNIX_TIMESTAMP(blockTimestamp) DIV ${div}` divisor in seconds.
/// `div = 1` puts each block in its own bucket.
fn time_div(period: TimePeriod) -> u32 {
    match period {
        TimePeriod::Day | TimePeriod::ThreeDays => 1,
        TimePeriod::Week => 300,
        TimePeriod::Month => 1800,
        TimePeriod::ThreeMonths => 7200,
        TimePeriod::SixMonths => 10800,
        TimePeriod::Year | TimePeriod::TwoYears => 28800,
        TimePeriod::ThreeYears => 43200,
        TimePeriod::All => 86400,
    }
}

/// Round-half-up integer division, matching MySQL's `CAST(AVG(...) AS INT)`.
const fn round_half_up(sum: u64, n: u64) -> u64 {
    (sum + n / 2) / n
}

/// One time-bucket of blocks in a `BlockWindow`.
pub struct BlockBucket {
    pub avg_height: Height,
    pub avg_timestamp: Timestamp,
    /// Offsets into the parent `BlockWindow`'s prefetched `[start, end)` slice.
    offsets: Vec<usize>,
}

impl BlockBucket {
    /// Float arithmetic mean of `values[offset]` across this bucket's blocks.
    /// Use for float-backed types like `FeeRate`.
    pub fn mean<T>(&self, values: &[T]) -> T
    where
        T: Copy + Sum + Div<usize, Output = T>,
    {
        self.offsets.iter().map(|&i| values[i]).sum::<T>() / self.offsets.len()
    }

    /// Round-half-up arithmetic mean for u64-backed integer types, matching
    /// mempool.space's `CAST(AVG(...) AS INT)`.
    pub fn mean_rounded<T>(&self, values: &[T]) -> T
    where
        T: Copy + Deref<Target = u64> + From<u64>,
    {
        let n = self.offsets.len() as u64;
        let sum: u64 = self.offsets.iter().map(|&i| *values[i]).sum();
        T::from(round_half_up(sum, n))
    }
}

/// Mempool-compatible time-bucketed block window. Groups blocks by
/// `block.timestamp / div` and exposes arithmetic means per bucket.
pub struct BlockWindow {
    pub start: Height,
    pub end: Height,
    pub buckets: Vec<BlockBucket>,
}

impl BlockWindow {
    pub fn new(query: &Query, period: TimePeriod) -> Self {
        let start = query.start_height(period);
        let end = query.height() + 1usize;
        let div = time_div(period);

        let timestamps: Vec<Timestamp> = query
            .indexer()
            .vecs
            .blocks
            .timestamp
            .collect_range(start, end);

        let mut groups: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        for (i, ts) in timestamps.iter().enumerate() {
            groups.entry(**ts / div).or_default().push(i);
        }

        let buckets = groups
            .into_values()
            .map(|offsets| {
                let n = offsets.len() as u64;
                let sum_h: u64 = offsets.iter().map(|&i| u64::from(start + i)).sum();
                let sum_ts: u64 = offsets.iter().map(|&i| u64::from(timestamps[i])).sum();
                BlockBucket {
                    avg_height: Height::from(round_half_up(sum_h, n)),
                    avg_timestamp: Timestamp::from(round_half_up(sum_ts, n) as u32),
                    offsets,
                }
            })
            .collect();

        Self {
            start,
            end,
            buckets,
        }
    }

    /// Read a height-keyed vec over this window's `[start, end)` range.
    pub fn read<V, T>(&self, vec: &V) -> Vec<T>
    where
        V: ReadableVec<Height, T>,
        T: VecValue,
    {
        vec.collect_range(self.start, self.end)
    }
}
