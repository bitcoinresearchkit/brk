use brk_grouper::AGE_BOUNDARIES;
use brk_types::{ONE_DAY_IN_SEC, Timestamp};

use crate::stateful::states::BlockState;

use super::UTXOCohorts;

impl UTXOCohorts {
    /// Handle age transitions when processing a new block.
    ///
    /// UTXOs age with each block. When they cross day boundaries,
    /// they move between age-based cohorts (e.g., from "0-1d" to "1-7d").
    ///
    /// Complexity: O(k * (log n + m)) where:
    /// - k = 19 boundaries to check
    /// - n = total blocks in chain_state
    /// - m = blocks crossing each boundary (typically 0-2 per boundary per block)
    pub fn tick_tock_next_block(&mut self, chain_state: &[BlockState], timestamp: Timestamp) {
        if chain_state.is_empty() {
            return;
        }

        let prev_timestamp = chain_state.last().unwrap().timestamp;
        let elapsed = (*timestamp).saturating_sub(*prev_timestamp);

        // Skip if no time has passed
        if elapsed == 0 {
            return;
        }

        // Get age_range cohort states (indexed 0..20)
        // Cohort i covers days [BOUNDARIES[i-1], BOUNDARIES[i])
        // Cohort 0 covers [0, 1) days
        // Cohort 19 covers [15*365, infinity) days
        let mut age_cohorts: Vec<_> = self.0.age_range.iter_mut().map(|v| &mut v.state).collect();

        // For each boundary, find blocks that just crossed it
        for (boundary_idx, &boundary_days) in AGE_BOUNDARIES.iter().enumerate() {
            let boundary_seconds = (boundary_days as u32) * ONE_DAY_IN_SEC;

            // Blocks crossing boundary B have timestamps in (prev - B*DAY, curr - B*DAY]
            // prev_days < B and curr_days >= B
            // means: block was younger than B days, now is B days or older
            let upper_timestamp = (*timestamp).saturating_sub(boundary_seconds);
            let lower_timestamp = (*prev_timestamp).saturating_sub(boundary_seconds);

            // Skip if the range is empty (would happen if boundary > chain age)
            if upper_timestamp <= lower_timestamp {
                continue;
            }

            // Binary search to find blocks in the timestamp range (lower, upper]
            let start_idx = chain_state.partition_point(|b| *b.timestamp <= lower_timestamp);
            let end_idx = chain_state.partition_point(|b| *b.timestamp <= upper_timestamp);

            // Process blocks that crossed this boundary
            for block_state in &chain_state[start_idx..end_idx] {
                // Double-check the day boundary was actually crossed
                // (handles edge cases with day boundaries)
                let prev_days = prev_timestamp.difference_in_days_between(block_state.timestamp);
                let curr_days = timestamp.difference_in_days_between(block_state.timestamp);

                if prev_days >= boundary_days || curr_days < boundary_days {
                    continue;
                }

                // Block crossed from cohort[boundary_idx] to cohort[boundary_idx + 1]
                // Decrement from the "younger" cohort
                if let Some(state) = age_cohorts[boundary_idx].as_mut() {
                    state.decrement(&block_state.supply, block_state.price);
                }
                // Increment in the "older" cohort
                if let Some(state) = age_cohorts[boundary_idx + 1].as_mut() {
                    state.increment(&block_state.supply, block_state.price);
                }
            }
        }
    }
}
