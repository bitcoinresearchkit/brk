use brk_error::Result;
use brk_types::{BlockRewardsEntry, TimePeriod};
use vecdb::{IterableVec, VecIndex};

use super::dateindex_iter::DateIndexIter;
use crate::Query;

impl Query {
    pub fn block_rewards(&self, time_period: TimePeriod) -> Result<Vec<BlockRewardsEntry>> {
        let computer = self.computer();
        let current_height = self.height();
        let start = current_height
            .to_usize()
            .saturating_sub(time_period.block_count());

        let iter = DateIndexIter::new(computer, start, current_height.to_usize());

        // coinbase = subsidy + fees
        let mut rewards = computer
            .blocks
            .rewards
            .indexes_to_coinbase
            .sats
            .dateindex
            .unwrap_average()
            .iter();

        Ok(iter.collect(|di, ts, h| {
            rewards.get(di).map(|reward| BlockRewardsEntry {
                avg_height: h.into(),
                timestamp: *ts,
                avg_rewards: *reward,
            })
        }))
    }
}
