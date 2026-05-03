use brk_error::Result;
use brk_types::{BlockFeesEntry, Cents, Dollars, Sats, TimePeriod};

use super::block_window::BlockWindow;
use crate::Query;

impl Query {
    pub fn block_fees(&self, time_period: TimePeriod) -> Result<Vec<BlockFeesEntry>> {
        let bw = BlockWindow::new(self, time_period);
        let fees: Vec<Sats> = bw.read(&self.computer().mining.rewards.fees.block.sats);
        let prices: Vec<Cents> = bw.read(&self.computer().prices.spot.cents.height);

        Ok(bw
            .buckets
            .iter()
            .map(|b| BlockFeesEntry {
                avg_height: b.avg_height,
                timestamp: b.avg_timestamp,
                avg_fees: b.mean_rounded(&fees),
                usd: Dollars::from(b.mean_rounded(&prices)),
            })
            .collect())
    }
}
