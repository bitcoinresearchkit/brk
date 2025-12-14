use brk_error::Result;
use brk_types::{BlockFeeRatesEntry, FeeRatePercentiles, TimePeriod};
use vecdb::{IterableVec, VecIndex};

use super::dateindex_iter::DateIndexIter;
use crate::Query;

pub fn get_block_fee_rates(
    time_period: TimePeriod,
    query: &Query,
) -> Result<Vec<BlockFeeRatesEntry>> {
    let computer = query.computer();
    let current_height = query.get_height();
    let start = current_height
        .to_usize()
        .saturating_sub(time_period.block_count());

    let iter = DateIndexIter::new(computer, start, current_height.to_usize());

    let vecs = &computer.chain.indexes_to_fee_rate.dateindex;
    let mut min = vecs.unwrap_min().iter();
    let mut pct10 = vecs.unwrap_pct10().iter();
    let mut pct25 = vecs.unwrap_pct25().iter();
    let mut median = vecs.unwrap_median().iter();
    let mut pct75 = vecs.unwrap_pct75().iter();
    let mut pct90 = vecs.unwrap_pct90().iter();
    let mut max = vecs.unwrap_max().iter();

    Ok(iter.collect(|di, ts, h| {
        Some(BlockFeeRatesEntry {
            avg_height: h.into(),
            timestamp: *ts as u32,
            percentiles: FeeRatePercentiles::new(
                min.get(di).unwrap_or_default(),
                pct10.get(di).unwrap_or_default(),
                pct25.get(di).unwrap_or_default(),
                median.get(di).unwrap_or_default(),
                pct75.get(di).unwrap_or_default(),
                pct90.get(di).unwrap_or_default(),
                max.get(di).unwrap_or_default(),
            ),
        })
    }))
}
