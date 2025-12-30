use brk_error::Result;
use brk_types::{BlockFeesEntry, TimePeriod};
use vecdb::{IterableVec, VecIndex};

use super::dateindex_iter::DateIndexIter;
use crate::Query;

impl Query {
    pub fn block_fees(&self, time_period: TimePeriod) -> Result<Vec<BlockFeesEntry>> {
        let computer = self.computer();
        let current_height = self.height();
        let start = current_height
            .to_usize()
            .saturating_sub(time_period.block_count());

        let iter = DateIndexIter::new(computer, start, current_height.to_usize());

        let mut fees = computer
            .chain
            .transaction
            .indexes_to_fee
            .sats
            .dateindex
            .unwrap_average()
            .iter();

        Ok(iter.collect(|di, ts, h| {
            fees.get(di).map(|fee| BlockFeesEntry {
                avg_height: h,
                timestamp: ts,
                avg_fees: fee,
            })
        }))
    }
}
