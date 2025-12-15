use brk_error::Result;
use brk_types::{DateIndex, DifficultyEntry, HashrateEntry, HashrateSummary, Height, TimePeriod};
use vecdb::{GenericStoredVec, IterableVec, VecIndex};

use super::epochs::iter_difficulty_epochs;
use crate::Query;

impl Query {
    pub fn hashrate(&self, time_period: Option<TimePeriod>) -> Result<HashrateSummary> {
        let indexer = self.indexer();
        let computer = self.computer();
        let current_height = self.height();

        // Get current difficulty
        let current_difficulty = *indexer
            .vecs
            .block
            .height_to_difficulty
            .read_once(current_height)?;

        // Get current hashrate
        let current_dateindex = computer
            .indexes
            .height_to_dateindex
            .read_once(current_height)?;
        let current_hashrate = *computer
            .chain
            .indexes_to_hash_rate
            .dateindex
            .unwrap_last()
            .read_once(current_dateindex)? as u128;

        // Calculate start height based on time period
        let end = current_height.to_usize();
        let start = match time_period {
            Some(tp) => end.saturating_sub(tp.block_count()),
            None => 0,
        };

        // Get hashrate entries using iterators for efficiency
        let start_dateindex = computer
            .indexes
            .height_to_dateindex
            .read_once(Height::from(start))?;
        let end_dateindex = current_dateindex;

        // Sample at regular intervals to avoid too many data points
        let total_days = end_dateindex
            .to_usize()
            .saturating_sub(start_dateindex.to_usize())
            + 1;
        let step = (total_days / 200).max(1); // Max ~200 data points

        // Create iterators for the loop
        let mut hashrate_iter = computer
            .chain
            .indexes_to_hash_rate
            .dateindex
            .unwrap_last()
            .iter();
        let mut timestamp_iter = computer
            .chain
            .timeindexes_to_timestamp
            .dateindex_extra
            .unwrap_first()
            .iter();

        let mut hashrates = Vec::with_capacity(total_days / step + 1);
        let mut di = start_dateindex.to_usize();
        while di <= end_dateindex.to_usize() {
            let dateindex = DateIndex::from(di);
            if let (Some(hr), Some(timestamp)) =
                (hashrate_iter.get(dateindex), timestamp_iter.get(dateindex))
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
