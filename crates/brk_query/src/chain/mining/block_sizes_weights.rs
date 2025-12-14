use brk_error::Result;
use brk_types::{BlockSizeEntry, BlockSizesWeights, BlockWeightEntry, TimePeriod};
use vecdb::{IterableVec, VecIndex};

use super::dateindex_iter::DateIndexIter;
use crate::Query;

pub fn get_block_sizes_weights(
    time_period: TimePeriod,
    query: &Query,
) -> Result<BlockSizesWeights> {
    let computer = query.computer();
    let current_height = query.get_height();
    let start = current_height
        .to_usize()
        .saturating_sub(time_period.block_count());

    let iter = DateIndexIter::new(computer, start, current_height.to_usize());

    let mut sizes_vec = computer
        .chain
        .indexes_to_block_size
        .dateindex
        .unwrap_average()
        .iter();
    let mut weights_vec = computer
        .chain
        .indexes_to_block_weight
        .dateindex
        .unwrap_average()
        .iter();

    let entries: Vec<_> = iter.collect(|di, ts, h| {
        let size = sizes_vec.get(di).map(|s| u64::from(*s));
        let weight = weights_vec.get(di).map(|w| u64::from(*w));
        Some((h.into(), *ts as u32, size, weight))
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
