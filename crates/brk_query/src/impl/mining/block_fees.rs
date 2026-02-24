use brk_error::Result;
use brk_types::{BlockFeesEntry, Height, Sats, TimePeriod};
use vecdb::{ReadableVec, VecIndex};

use super::day1_iter::Day1Iter;
use crate::Query;

impl Query {
    pub fn block_fees(&self, time_period: TimePeriod) -> Result<Vec<BlockFeesEntry>> {
        let computer = self.computer();
        let current_height = self.height();
        let start = current_height
            .to_usize()
            .saturating_sub(time_period.block_count());

        let iter = Day1Iter::new(computer, start, current_height.to_usize());

        let cumulative = &computer.transactions.fees.fee.sum_cum.cumulative;
        let first_height = &computer.indexes.day1.first_height;

        Ok(iter.collect(|di, ts, h| {
            let h_start = first_height.collect_one(di)?;
            let h_end = first_height
                .collect_one(di + 1_usize)
                .unwrap_or(Height::from(current_height.to_usize() + 1));
            let block_count = h_end.to_usize() - h_start.to_usize();
            if block_count == 0 {
                return None;
            }

            let cum_end = cumulative.collect_one_at(h_end.to_usize() - 1)?;
            let cum_start = if h_start.to_usize() > 0 {
                cumulative.collect_one_at(h_start.to_usize() - 1).unwrap_or(Sats::ZERO)
            } else {
                Sats::ZERO
            };
            let daily_sum = cum_end - cum_start;
            let avg_fees = Sats::from(*daily_sum / block_count as u64);

            Some(BlockFeesEntry {
                avg_height: h,
                timestamp: ts,
                avg_fees,
            })
        }))
    }
}
