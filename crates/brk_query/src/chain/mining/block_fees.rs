use brk_error::Result;
use brk_types::{BlockFeesEntry, TimePeriod};
use vecdb::{IterableVec, VecIndex};

use super::dateindex_iter::DateIndexIter;
use crate::Query;

pub fn get_block_fees(time_period: TimePeriod, query: &Query) -> Result<Vec<BlockFeesEntry>> {
    let computer = query.computer();
    let current_height = query.get_height();
    let start = current_height
        .to_usize()
        .saturating_sub(time_period.block_count());

    let iter = DateIndexIter::new(computer, start, current_height.to_usize());

    let mut fees = computer
        .chain
        .indexes_to_fee
        .sats
        .dateindex
        .unwrap_average()
        .iter();

    Ok(iter.collect(|di, ts, h| {
        fees.get(di).map(|fee| BlockFeesEntry {
            avg_height: h.into(),
            timestamp: *ts as u32,
            avg_fees: u64::from(*fee),
        })
    }))
}
