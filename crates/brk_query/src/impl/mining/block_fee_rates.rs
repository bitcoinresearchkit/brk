// TODO: INCOMPLETE - indexes_to_fee_rate.day1 doesn't have percentile fields
// because from_txindex.rs calls remove_percentiles() before creating day1.
// Need to either:
// 1. Use .height instead and convert height to day1 for iteration
// 2. Fix from_txindex.rs to preserve percentiles for day1
// 3. Create a separate day1 computation path with percentiles

#![allow(dead_code)]

use brk_error::Result;
use brk_types::{
    BlockFeeRatesEntry,
    // FeeRatePercentiles,
    TimePeriod,
};
// use vecdb::{IterableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn block_fee_rates(&self, _time_period: TimePeriod) -> Result<Vec<BlockFeeRatesEntry>> {
        // Disabled until percentile data is available at day1 level
        Ok(Vec::new())

        // Original implementation:
        // let computer = self.computer();
        // let current_height = self.height();
        // let start = current_height
        //     .to_usize()
        //     .saturating_sub(time_period.block_count());
        //
        // let iter = Day1Iter::new(computer, start, current_height.to_usize());
        //
        // let vecs = &computer.transactions.transaction.indexes_to_fee_rate.day1;
        // let mut min = vecs.unwrap_min().iter();
        // let mut pct10 = vecs.unwrap_pct10().iter();
        // let mut pct25 = vecs.unwrap_pct25().iter();
        // let mut median = vecs.unwrap_median().iter();
        // let mut pct75 = vecs.unwrap_pct75().iter();
        // let mut pct90 = vecs.unwrap_pct90().iter();
        // let mut max = vecs.unwrap_max().iter();
        //
        // Ok(iter.collect(|di, ts, h| {
        //     Some(BlockFeeRatesEntry {
        //         avg_height: h,
        //         timestamp: ts,
        //         percentiles: FeeRatePercentiles::new(
        //             min.get(di).unwrap_or_default(),
        //             pct10.get(di).unwrap_or_default(),
        //             pct25.get(di).unwrap_or_default(),
        //             median.get(di).unwrap_or_default(),
        //             pct75.get(di).unwrap_or_default(),
        //             pct90.get(di).unwrap_or_default(),
        //             max.get(di).unwrap_or_default(),
        //         ),
        //     })
        // }))
    }
}
