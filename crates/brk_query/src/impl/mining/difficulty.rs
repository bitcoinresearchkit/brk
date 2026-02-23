use std::time::{SystemTime, UNIX_EPOCH};

use brk_error::Result;
use brk_types::{DifficultyAdjustment, DifficultyEpoch, Height};
use vecdb::ReadableVec;

use crate::Query;

/// Blocks per difficulty epoch (2 weeks target)
const BLOCKS_PER_EPOCH: u32 = 2016;

/// Target block time in seconds (10 minutes)
const TARGET_BLOCK_TIME: u64 = 600;

impl Query {
    pub fn difficulty_adjustment(&self) -> Result<DifficultyAdjustment> {
        let indexer = self.indexer();
        let computer = self.computer();
        let current_height = self.height();
        let current_height_u32: u32 = current_height.into();

        // Get current epoch
        let current_epoch = computer
            .indexes
            .height
            .difficultyepoch
            .collect_one(current_height)
            .unwrap();
        let current_epoch_usize: usize = current_epoch.into();

        // Get epoch start height
        let epoch_start_height = computer
            .indexes
            .difficultyepoch
            .first_height
            .collect_one(current_epoch)
            .unwrap();
        let epoch_start_u32: u32 = epoch_start_height.into();

        // Calculate epoch progress
        let next_retarget_height = epoch_start_u32 + BLOCKS_PER_EPOCH;
        let blocks_into_epoch = current_height_u32 - epoch_start_u32;
        let remaining_blocks = next_retarget_height - current_height_u32;
        let progress_percent = (blocks_into_epoch as f64 / BLOCKS_PER_EPOCH as f64) * 100.0;

        // Get timestamps using difficultyepoch_to_timestamp for epoch start
        let epoch_start_timestamp = computer
            .blocks
            .time
            .timestamp
            .difficultyepoch
            .collect_one(current_epoch)
            .unwrap();
        let current_timestamp = indexer
            .vecs
            .blocks
            .timestamp
            .collect_one(current_height)
            .unwrap();

        // Calculate average block time in current epoch
        let elapsed_time = (*current_timestamp - *epoch_start_timestamp) as u64;
        let time_avg = if blocks_into_epoch > 0 {
            elapsed_time / blocks_into_epoch as u64
        } else {
            TARGET_BLOCK_TIME
        };

        // Estimate remaining time and retarget date
        let remaining_time = remaining_blocks as u64 * time_avg;
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(*current_timestamp as u64);
        let estimated_retarget_date = now + remaining_time;

        // Calculate expected vs actual time for difficulty change estimate
        let expected_time = blocks_into_epoch as u64 * TARGET_BLOCK_TIME;
        let difficulty_change = if elapsed_time > 0 && blocks_into_epoch > 0 {
            ((expected_time as f64 / elapsed_time as f64) - 1.0) * 100.0
        } else {
            0.0
        };

        // Time offset from expected schedule
        let time_offset = expected_time as i64 - elapsed_time as i64;

        // Calculate previous retarget using stored difficulty values
        let previous_retarget = if current_epoch_usize > 0 {
            let prev_epoch = DifficultyEpoch::from(current_epoch_usize - 1);
            let prev_epoch_start = computer
                .indexes
                .difficultyepoch
                .first_height
                .collect_one(prev_epoch)
                .unwrap();

            let prev_difficulty = indexer
                .vecs
                .blocks
                .difficulty
                .collect_one(prev_epoch_start)
                .unwrap();
            let curr_difficulty = indexer
                .vecs
                .blocks
                .difficulty
                .collect_one(epoch_start_height)
                .unwrap();

            if *prev_difficulty > 0.0 {
                ((*curr_difficulty / *prev_difficulty) - 1.0) * 100.0
            } else {
                0.0
            }
        } else {
            0.0
        };

        Ok(DifficultyAdjustment {
            progress_percent,
            difficulty_change,
            estimated_retarget_date,
            remaining_blocks,
            remaining_time,
            previous_retarget,
            next_retarget_height: Height::from(next_retarget_height),
            time_avg,
            adjusted_time_avg: time_avg,
            time_offset,
        })
    }
}
