use brk_error::Result;
use brk_types::{BlockRewardsEntry, TimePeriod};
use vecdb::{IterableVec, VecIndex};

use super::dateindex_iter::DateIndexIter;
use crate::Query;

pub fn get_block_rewards(time_period: TimePeriod, query: &Query) -> Result<Vec<BlockRewardsEntry>> {
    let computer = query.computer();
    let current_height = query.get_height();
    let start = current_height
        .to_usize()
        .saturating_sub(time_period.block_count());

    let iter = DateIndexIter::new(computer, start, current_height.to_usize());

    // coinbase = subsidy + fees
    let mut rewards = computer
        .chain
        .indexes_to_coinbase
        .sats
        .dateindex
        .unwrap_average()
        .iter();

    Ok(iter.collect(|di, ts, h| {
        rewards.get(di).map(|reward| BlockRewardsEntry {
            avg_height: h.into(),
            timestamp: *ts as u32,
            avg_rewards: u64::from(*reward),
        })
    }))
}
