use brk_error::Result;
use brk_types::{BlockFeesEntry, TimePeriod};

use super::block_window::BlockWindow;
use crate::Query;

impl Query {
    pub fn block_fees(&self, time_period: TimePeriod) -> Result<Vec<BlockFeesEntry>> {
        let bw = BlockWindow::new(self, time_period);
        let cumulative = &self.computer().mining.rewards.fees.cumulative.sats.height;
        Ok(bw
            .cumulative_averages(self, cumulative)
            .into_iter()
            .map(|w| BlockFeesEntry {
                avg_height: w.avg_height,
                timestamp: w.timestamp,
                avg_fees: w.avg_value,
                usd: w.usd,
            })
            .collect())
    }
}
