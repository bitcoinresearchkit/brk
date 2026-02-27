use brk_error::Result;
use brk_types::{BlockRewardsEntry, TimePeriod};
use vecdb::{ReadableOptionVec, VecIndex};

use super::day1_iter::Day1Iter;
use crate::Query;

impl Query {
    pub fn block_rewards(&self, time_period: TimePeriod) -> Result<Vec<BlockRewardsEntry>> {
        let computer = self.computer();
        let current_height = self.height();
        let start = current_height
            .to_usize()
            .saturating_sub(time_period.block_count());

        let iter = Day1Iter::new(computer, start, current_height.to_usize());

        let rewards_vec = &computer
            .mining
            .rewards
            .coinbase
            .rolling
            ._24h
            .distribution
            .average
            .sats
            .day1;

        Ok(iter.collect(|di, ts, h| {
            rewards_vec.collect_one_flat(di).map(|reward| BlockRewardsEntry {
                avg_height: h.into(),
                timestamp: *ts,
                avg_rewards: *reward,
            })
        }))
    }
}
