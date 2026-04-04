use brk_error::Result;
use brk_types::{BlockFeeRatesEntry, FeeRate, FeeRatePercentiles, TimePeriod};
use vecdb::ReadableVec;

use super::block_window::BlockWindow;
use crate::Query;

impl Query {
    pub fn block_fee_rates(&self, time_period: TimePeriod) -> Result<Vec<BlockFeeRatesEntry>> {
        let bw = BlockWindow::new(self, time_period);
        let computer = self.computer();
        let frd = &computer.transactions.fees.effective_fee_rate.distribution.block;

        let min = frd.min.height.collect_range_at(bw.start, bw.end);
        let pct10 = frd.pct10.height.collect_range_at(bw.start, bw.end);
        let pct25 = frd.pct25.height.collect_range_at(bw.start, bw.end);
        let median = frd.median.height.collect_range_at(bw.start, bw.end);
        let pct75 = frd.pct75.height.collect_range_at(bw.start, bw.end);
        let pct90 = frd.pct90.height.collect_range_at(bw.start, bw.end);
        let max = frd.max.height.collect_range_at(bw.start, bw.end);

        let timestamps = bw.timestamps(self);

        let mut results = Vec::with_capacity(timestamps.len());
        let mut pos = 0;
        let total = min.len();

        for ts in &timestamps {
            let window_end = (pos + bw.window).min(total);
            let count = window_end - pos;
            if count > 0 {
                let mid = (pos + window_end) / 2;
                let avg = |vals: &[FeeRate]| -> FeeRate {
                    let sum: f64 = vals[pos..window_end].iter().map(|f| f64::from(*f)).sum();
                    FeeRate::new(sum / count as f64)
                };

                results.push(BlockFeeRatesEntry {
                    avg_height: brk_types::Height::from(bw.start + mid),
                    timestamp: *ts,
                    percentiles: FeeRatePercentiles::new(
                        avg(&min),
                        avg(&pct10),
                        avg(&pct25),
                        avg(&median),
                        avg(&pct75),
                        avg(&pct90),
                        avg(&max),
                    ),
                });
            }
            pos = window_end;
        }

        Ok(results)
    }
}
