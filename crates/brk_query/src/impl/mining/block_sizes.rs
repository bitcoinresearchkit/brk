use brk_error::Result;
use brk_types::{BlockSizeEntry, BlockSizesWeights, BlockWeightEntry, TimePeriod, Weight};
use vecdb::ReadableVec;

use super::block_window::BlockWindow;
use crate::Query;

impl Query {
    pub fn block_sizes_weights(&self, time_period: TimePeriod) -> Result<BlockSizesWeights> {
        let computer = self.computer();
        let bw = BlockWindow::new(self, time_period);
        let timestamps = bw.timestamps(self);

        // Batch read per-block rolling 24h medians for the range
        let all_sizes = computer
            .blocks
            .size
            .size
            .rolling
            .distribution
            .median
            ._24h
            .height
            .collect_range_at(bw.start, bw.end);
        let all_weights = computer
            .blocks
            .weight
            .weight
            .rolling
            .distribution
            .median
            ._24h
            .height
            .collect_range_at(bw.start, bw.end);

        // Sample at window midpoints
        let mut sizes = Vec::with_capacity(timestamps.len());
        let mut weights = Vec::with_capacity(timestamps.len());

        for ((avg_height, start, _end), ts) in bw.iter().zip(&timestamps) {
            let mid = start - bw.start + (bw.window / 2).min(all_sizes.len().saturating_sub(1));
            if let Some(&size) = all_sizes.get(mid) {
                sizes.push(BlockSizeEntry {
                    avg_height,
                    timestamp: *ts,
                    avg_size: *size,
                });
            }
            if let Some(&weight) = all_weights.get(mid) {
                weights.push(BlockWeightEntry {
                    avg_height,
                    timestamp: *ts,
                    avg_weight: Weight::from(*weight),
                });
            }
        }

        Ok(BlockSizesWeights { sizes, weights })
    }
}
