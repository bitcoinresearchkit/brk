use brk_types::{Cents, Dollars, Height, Sats, TimePeriod, Timestamp};
use vecdb::{ReadableVec, VecIndex};

use crate::Query;

/// Number of blocks per aggregation window, matching mempool.space's granularity.
fn block_window(period: TimePeriod) -> usize {
    match period {
        TimePeriod::Day | TimePeriod::ThreeDays | TimePeriod::Week => 1,
        TimePeriod::Month => 3,
        TimePeriod::ThreeMonths => 12,
        TimePeriod::SixMonths => 18,
        TimePeriod::Year | TimePeriod::TwoYears => 48,
        TimePeriod::ThreeYears => 72,
        TimePeriod::All => 144,
    }
}

/// Per-window average with metadata.
pub struct WindowAvg {
    pub avg_height: Height,
    pub timestamp: Timestamp,
    pub avg_value: Sats,
    pub usd: Dollars,
}

/// Block range and window size for a time period.
pub struct BlockWindow {
    pub start: usize,
    pub end: usize,
    pub window: usize,
}

impl BlockWindow {
    pub fn new(query: &Query, time_period: TimePeriod) -> Self {
        let current_height = query.height();
        let computer = query.computer();
        let lookback = &computer.blocks.lookback;

        // Use pre-computed timestamp-based lookback for accurate time boundaries.
        // 24h, 1w, 1m, 1y use in-memory CachedVec; others fall back to PcoVec.
        let start_height = match time_period {
            TimePeriod::Day => lookback._24h.collect_one(current_height),
            TimePeriod::ThreeDays => lookback._3d.collect_one(current_height),
            TimePeriod::Week => lookback._1w.collect_one(current_height),
            TimePeriod::Month => lookback._1m.collect_one(current_height),
            TimePeriod::ThreeMonths => lookback._3m.collect_one(current_height),
            TimePeriod::SixMonths => lookback._6m.collect_one(current_height),
            TimePeriod::Year => lookback._1y.collect_one(current_height),
            TimePeriod::TwoYears => lookback._2y.collect_one(current_height),
            TimePeriod::ThreeYears => lookback._3y.collect_one(current_height),
            TimePeriod::All => None,
        }
        .unwrap_or_default();

        Self {
            start: start_height.to_usize(),
            end: current_height.to_usize() + 1,
            window: block_window(time_period),
        }
    }

    /// Compute per-window averages from a cumulative sats vec.
    /// Batch-reads timestamps, prices, and the cumulative in one pass.
    pub fn cumulative_averages(
        &self,
        query: &Query,
        cumulative: &impl ReadableVec<Height, Sats>,
    ) -> Vec<WindowAvg> {
        let indexer = query.indexer();
        let computer = query.computer();

        // Batch read all needed data for the range
        let all_ts = indexer
            .vecs
            .blocks
            .timestamp
            .collect_range_at(self.start, self.end);
        let all_prices: Vec<Cents> = computer
            .prices
            .spot.cents.height
            .collect_range_at(self.start, self.end);
        let read_start = self.start.saturating_sub(1);
        let all_cum = cumulative.collect_range_at(read_start, self.end);
        let offset = if self.start > 0 { 1 } else { 0 };

        let mut results = Vec::with_capacity(self.count());
        let mut pos = 0;
        let total = all_ts.len();

        while pos < total {
            let window_end = (pos + self.window).min(total);
            let block_count = (window_end - pos) as u64;
            let mid = (pos + window_end) / 2;
            let cum_end = all_cum[window_end - 1 + offset];
            let cum_start = if pos + offset > 0 {
                all_cum[pos + offset - 1]
            } else {
                Sats::ZERO
            };
            let total_sats = cum_end - cum_start;
            if let Some(avg) = (*total_sats).checked_div(block_count) {
                results.push(WindowAvg {
                    avg_height: Height::from(self.start + mid),
                    timestamp: all_ts[mid],
                    avg_value: Sats::from(avg),
                    usd: Dollars::from(all_prices[mid]),
                });
            }
            pos = window_end;
        }

        results
    }

    /// Batch-read timestamps for the midpoint of each window.
    pub fn timestamps(&self, query: &Query) -> Vec<Timestamp> {
        let all_ts = query
            .indexer()
            .vecs
            .blocks
            .timestamp
            .collect_range_at(self.start, self.end);
        let mut timestamps = Vec::with_capacity(self.count());
        let mut pos = 0;
        while pos < all_ts.len() {
            let window_end = (pos + self.window).min(all_ts.len());
            timestamps.push(all_ts[(pos + window_end) / 2]);
            pos = window_end;
        }
        timestamps
    }

    /// Number of windows in this range.
    fn count(&self) -> usize {
        (self.end - self.start).div_ceil(self.window)
    }

    /// Iterate windows, yielding (avg_height, window_start, window_end) for each.
    pub fn iter(&self) -> impl Iterator<Item = (Height, usize, usize)> + '_ {
        let mut pos = self.start;
        std::iter::from_fn(move || {
            if pos >= self.end {
                return None;
            }
            let window_end = (pos + self.window).min(self.end);
            let avg_height = Height::from((pos + window_end) / 2);
            let start = pos;
            pos = window_end;
            Some((avg_height, start, window_end))
        })
    }
}
