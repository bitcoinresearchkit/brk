use brk_error::Result;
use brk_types::{BlockSizeEntry, BlockSizesWeights, BlockWeightEntry, TimePeriod};
use vecdb::{ReadableOptionVec, VecIndex};

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

        // Rolling 24h average, sampled at day1 boundaries
        let sizes_vec = &computer
            .blocks
            .size
            .size
            .rolling
            .distribution
            .average
            ._24h
            .day1;
        let weights_vec = &computer
            .blocks
            .weight
            .weight
            .rolling
            .distribution
            .average
            ._24h
            .day1;

        let entries: Vec<_> = iter.collect(|di, ts, h| {
            let size: Option<u64> = sizes_vec.collect_one_flat(di).map(|s| *s);
            let weight: Option<u64> = weights_vec.collect_one_flat(di).map(|w| *w);
            Some((u32::from(h), (*ts), size, weight))
        });

        let sizes = entries
            .iter()
            .filter_map(|(h, ts, size, _)| {
                size.map(|s| BlockSizeEntry {
                    avg_height: *h,
                    timestamp: *ts,
                    avg_size: s,
                })
            })
            .collect();

        let weights = entries
            .iter()
            .filter_map(|(h, ts, _, weight)| {
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
