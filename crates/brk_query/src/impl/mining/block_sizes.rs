use brk_error::Result;
use brk_types::{BlockSizeEntry, BlockSizesWeights, BlockWeightEntry, TimePeriod};
use vecdb::{IterableVec, VecIndex};

use super::dateindex_iter::DateIndexIter;
use crate::Query;

impl Query {
    pub fn block_sizes_weights(&self, time_period: TimePeriod) -> Result<BlockSizesWeights> {
        let computer = self.computer();
        let current_height = self.height();
        let start = current_height
            .to_usize()
            .saturating_sub(time_period.block_count());

        let iter = DateIndexIter::new(computer, start, current_height.to_usize());

        let mut sizes_vec = computer
            .blocks
            .size
            .size
            .dateindex
            .distribution
            .average
            .0
            .iter();
        let mut weights_vec = computer
            .blocks
            .weight
            .weight
            .dateindex
            .distribution
            .average
            .0
            .iter();

        let entries: Vec<_> = iter.collect(|di, ts, h| {
            let size = sizes_vec.get(di).map(|s| *s);
            let weight = weights_vec.get(di).map(|w| *w);
            Some((h.into(), (*ts), size, weight))
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
