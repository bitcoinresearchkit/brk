use brk_error::Result;
use brk_types::{BlockRewardsEntry, TimePeriod};

use super::block_window::BlockWindow;
use crate::Query;

impl Query {
    pub fn block_rewards(&self, time_period: TimePeriod) -> Result<Vec<BlockRewardsEntry>> {
        let bw = BlockWindow::new(self, time_period);
        let cumulative = &self
            .computer()
            .mining
            .rewards
            .coinbase
            .cumulative
            .sats
            .height;
        Ok(bw
            .cumulative_averages(self, cumulative)
            .into_iter()
            .map(|w| BlockRewardsEntry {
                avg_height: w.avg_height,
                timestamp: w.timestamp,
                avg_rewards: w.avg_value,
                usd: w.usd,
            })
            .collect())
    }
}
