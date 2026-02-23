use brk_error::Result;
use brk_types::{BlockSizeEntry, BlockSizesWeights, BlockWeightEntry, TimePeriod};
use vecdb::{ReadableVec, VecIndex};

use super::day1_iter::Day1Iter;
use crate::Query;

impl Query {
    pub fn block_sizes_weights(&self, time_period: TimePeriod) -> Result<BlockSizesWeights> {
        let computer = self.computer();
        let current_height = self.height();
        let start = current_height
            .to_usize()
            .saturating_sub(time_period.block_count());

        let iter = Day1Iter::new(computer, start, current_height.to_usize());

        let sizes_vec = &computer
            .blocks
            .size
            .size
            .day1
            .average;
        let weights_vec = &computer
            .blocks
            .weight
            .weight
            .day1
            .average;

        let entries: Vec<_> = iter.collect(|di, ts, h| {
            let size = sizes_vec.collect_one(di).map(|s| *s);
            let weight = weights_vec.collect_one(di).map(|w| *w);
            Some((h.into(), (*ts), size, weight))
        });

        let sizes = entries
            .iter()
            .filter_map(|(h, ts, size, _): &(u32, _, _, _)| {
                size.map(|s| BlockSizeEntry {
                    avg_height: *h,
                    timestamp: *ts,
                    avg_size: s,
                })
            })
            .collect();

        let weights = entries
            .iter()
            .filter_map(|(h, ts, _, weight): &(u32, _, _, _)| {
                weight.map(|w| BlockWeightEntry {
                    avg_height: *h,
                    timestamp: *ts,
                    avg_weight: w,
                })
            })
            .collect();

        Ok(BlockSizesWeights { sizes, weights })
    }
}
