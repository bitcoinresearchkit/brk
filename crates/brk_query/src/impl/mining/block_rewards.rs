use brk_error::Result;
use brk_types::{BlockRewardsEntry, Height, Sats, TimePeriod};
use vecdb::{ReadableVec, VecIndex};

use crate::Query;

impl Query {
    pub fn block_rewards(&self, time_period: TimePeriod) -> Result<Vec<BlockRewardsEntry>> {
        let computer = self.computer();
        let indexer = self.indexer();
        let current_height = self.height().to_usize();
        let start = current_height.saturating_sub(time_period.block_count());

        let coinbase_vec = &computer.mining.rewards.coinbase.block.sats;
        let timestamp_vec = &indexer.vecs.blocks.timestamp;

        match time_period {
            // Per-block, exact rewards
            TimePeriod::Day | TimePeriod::ThreeDays => {
                let rewards: Vec<Sats> =
                    coinbase_vec.collect_range_at(start, current_height + 1);
                let timestamps: Vec<brk_types::Timestamp> =
                    timestamp_vec.collect_range_at(start, current_height + 1);

                Ok(rewards
                    .iter()
                    .zip(timestamps.iter())
                    .enumerate()
                    .map(|(i, (reward, ts))| BlockRewardsEntry {
                        avg_height: (start + i) as u32,
                        timestamp: **ts,
                        avg_rewards: **reward,
                    })
                    .collect())
            }
            // Daily averages, sampled to ~200 points
            _ => {
                let first_height_vec = &computer.indexes.day1.first_height;
                let day1_vec = &computer.indexes.height.day1;

                let start_di = day1_vec
                    .collect_one(Height::from(start))
                    .unwrap_or_default();
                let end_di = day1_vec
                    .collect_one(Height::from(current_height))
                    .unwrap_or_default();

                let total_days =
                    end_di.to_usize().saturating_sub(start_di.to_usize()) + 1;
                let step = (total_days / 200).max(1);

                let mut entries = Vec::with_capacity(total_days / step + 1);
                let mut di = start_di.to_usize();

                while di <= end_di.to_usize() {
                    let day = brk_types::Day1::from(di);
                    let next_day = brk_types::Day1::from(di + 1);

                    if let Some(first_h) = first_height_vec.collect_one(day) {
                        let next_h = first_height_vec
                            .collect_one(next_day)
                            .unwrap_or(Height::from(current_height + 1));

                        let block_count = next_h.to_usize() - first_h.to_usize();
                        if block_count > 0 {
                            let sum = coinbase_vec.fold_range(
                                first_h,
                                next_h,
                                Sats::ZERO,
                                |acc, v| acc + v,
                            );
                            let avg = *sum / block_count as u64;

                            if let Some(ts) = timestamp_vec.collect_one(first_h) {
                                entries.push(BlockRewardsEntry {
                                    avg_height: first_h.to_usize() as u32,
                                    timestamp: *ts,
                                    avg_rewards: avg,
                                });
                            }
                        }
                    }

                    di += step;
                }

                Ok(entries)
            }
        }
    }
}
