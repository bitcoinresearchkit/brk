use std::{
    collections::BTreeMap,
    iter::Sum,
    ops::{Deref, Div},
};

use brk_error::{Error, Result};
use brk_types::{Height, TimePeriod, Timestamp};
use vecdb::{ReadableVec, VecValue};

use crate::Query;

/// Time-bucket divisor in seconds: blocks are grouped by `timestamp / div`.
/// `div = 1` puts each block in its own bucket; coarser values down-sample
/// long windows so the response stays bounded.
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
    /// Use for float-backed types like `FeeRate`. Soundness: `offsets.len() >= 1`
    /// is guaranteed by `BlockWindow::new` (only non-empty groups become buckets),
    /// and indexing `values[i]` is in range when `values` was obtained via
    /// `BlockWindow::read` (which validates `values.len() >= window.len`).
    pub fn mean<T>(&self, values: &[T]) -> T
    where
        T: Copy + Sum + Div<usize, Output = T>,
    {
        self.offsets.iter().map(|&i| values[i]).sum::<T>() / self.offsets.len()
    }

    /// Round-half-up arithmetic mean for u64-backed integer types: returns
    /// `T::from((sum + n/2) / n)`. Use when truncating integer division would
    /// bias rolling averages downward. Soundness: `offsets.len() >= 1` is
    /// guaranteed by `BlockWindow::new`, and `values[i]` is in range when
    /// `values` was obtained via `BlockWindow::read`.
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
    /// Number of blocks observed in `[start, end)` at construction. Equals
    /// `timestamps.len()` after the prefetch; may be less than `end - start`
    /// when the timestamp vec lags under per-vec stamp race. Every value vec
    /// passed to `read` must yield at least this many elements.
    pub len: usize,
}

impl BlockWindow {
    /// Build a time-bucketed window over `[start_height(period), tip + 1)`.
    /// Prefetches `blocks.timestamp` once, groups block indices by
    /// `ts / div(period)` (chronological), and stores per-bucket offsets
    /// into the prefetched slice. Downstream metric reads (`BlockWindow::read`)
    /// reuse the same `[start, end)` so each bucket's offsets index directly
    /// into the value vec without a second walk.
    pub fn new(query: &Query, period: TimePeriod) -> Result<Self> {
        let start = query.start_height(period)?;
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

        let len = timestamps.len();

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

        Ok(Self {
            start,
            end,
            buckets,
            len,
        })
    }

    /// Read a height-keyed vec over this window's `[start, end)` range.
    /// Errors if the vec returns fewer elements than the window observed at
    /// construction (per-vec stamp lag): bucket offsets reach up to `len - 1`
    /// and would otherwise panic in `BlockBucket::mean(&values)`.
    pub fn read<V, T>(&self, vec: &V) -> Result<Vec<T>>
    where
        V: ReadableVec<Height, T>,
        T: VecValue,
    {
        let values = vec.collect_range(self.start, self.end);
        if values.len() < self.len {
            return Err(Error::Internal(
                "BlockWindow::read: value vec shorter than window (per-vec stamp lag)",
            ));
        }
        Ok(values)
    }
}
