use brk_error::Result;
use brk_types::{Day1, DifficultyEntry, HashrateEntry, HashrateSummary, Height, TimePeriod};
use vecdb::{ReadableVec, VecIndex};

use super::epochs::iter_difficulty_epochs;
use crate::Query;

impl Query {
    pub fn hashrate(&self, time_period: Option<TimePeriod>) -> Result<HashrateSummary> {
        let indexer = self.indexer();
        let computer = self.computer();
        let current_height = self.height();

        // Get current difficulty
        let current_difficulty = *indexer.vecs.blocks.difficulty.collect_one(current_height).unwrap();

        // Get current hashrate
        let current_day1 = computer
            .indexes
            .height
            .day1
            .collect_one(current_height)
            .unwrap();

        let current_hashrate = *computer
            .mining
            .hashrate
            .hash_rate
            .day1
            .collect_one(current_day1)
            .unwrap() as u128;

        // Calculate start height based on time period
        let end = current_height.to_usize();
        let start = match time_period {
            Some(tp) => end.saturating_sub(tp.block_count()),
            None => 0,
        };

        // Get hashrate entries using iterators for efficiency
        let start_day1 = computer
            .indexes
            .height
            .day1
            .collect_one(Height::from(start))
            .unwrap();
        let end_day1 = current_day1;

        // Sample at regular intervals to avoid too many data points
        let total_days = end_day1
            .to_usize()
            .saturating_sub(start_day1.to_usize())
            + 1;
        let step = (total_days / 200).max(1); // Max ~200 data points

        let hashrate_vec = &computer.mining.hashrate.hash_rate.day1;
        let timestamp_vec = &computer.blocks.time.timestamp.day1;

        let mut hashrates = Vec::with_capacity(total_days / step + 1);
        let mut di = start_day1.to_usize();
        while di <= end_day1.to_usize() {
            let day1 = Day1::from(di);
            if let (Some(hr), Some(timestamp)) =
                (hashrate_vec.collect_one(day1), timestamp_vec.collect_one(day1))
            {
                hashrates.push(HashrateEntry {
                    timestamp,
                    avg_hashrate: (*hr) as u128,
                });
            }
            di += step;
        }

        // Get difficulty adjustments within the period
        let difficulty: Vec<DifficultyEntry> = iter_difficulty_epochs(computer, start, end)
            .into_iter()
            .map(|e| DifficultyEntry {
                timestamp: e.timestamp,
                difficulty: e.difficulty,
                height: e.height,
            })
            .collect();

        Ok(HashrateSummary {
            hashrates,
            difficulty,
            current_hashrate,
            current_difficulty,
        })
    }
}
