use brk_error::Result;
use brk_types::{BlockRewardsEntry, Cents, Dollars, Sats, TimePeriod};

use super::block_window::BlockWindow;
use crate::Query;

impl Query {
    pub fn block_rewards(&self, time_period: TimePeriod) -> Result<Vec<BlockRewardsEntry>> {
        let bw = BlockWindow::new(self, time_period);
        let rewards: Vec<Sats> = bw.read(&self.computer().mining.rewards.coinbase.block.sats);
        let prices: Vec<Cents> = bw.read(&self.computer().prices.spot.cents.height);

        Ok(bw
            .buckets
            .iter()
            .map(|b| BlockRewardsEntry {
                avg_height: b.avg_height,
                timestamp: b.avg_timestamp,
                avg_rewards: b.mean_rounded(&rewards),
                usd: Dollars::from(b.mean_rounded(&prices)),
            })
            .collect())
    }
}
