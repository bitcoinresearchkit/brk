use brk_cohort::AGE_BOUNDARIES;
use brk_types::{ONE_HOUR_IN_SEC, Timestamp};

use crate::distribution::state::BlockState;

use super::groups::UTXOCohorts;

impl UTXOCohorts {
    /// Handle age transitions when processing a new block.
    ///
    /// UTXOs age with each block. When they cross hour boundaries,
    /// they move between age-based cohorts (e.g., from "0-1h" to "1h-1d").
    ///
    /// Complexity: O(k * log n) where:
    /// - k = 20 boundaries to check
    /// - n = total blocks in chain_state
    /// - Linear scan for end_idx is faster than binary search since typically 0-2 blocks cross each boundary
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

        // Get age_range cohort states (indexed 0..21)
        // Cohort i covers hours [BOUNDARIES[i-1], BOUNDARIES[i])
        // Cohort 0 covers [0, 1) hours
        // Cohort 20 covers [15*365*24, infinity) hours
        let mut age_cohorts: Vec<_> = self.0.age_range.iter_mut().map(|v| &mut v.state).collect();

        // For each boundary (in hours), find blocks that just crossed it
        for (boundary_idx, &boundary_hours) in AGE_BOUNDARIES.iter().enumerate() {
            let boundary_seconds = (boundary_hours as u32) * ONE_HOUR_IN_SEC;

            // Blocks crossing boundary B have timestamps in (prev - B*HOUR, curr - B*HOUR]
            // prev_hours < B and curr_hours >= B
            // means: block was younger than B hours, now is B hours or older
            let upper_timestamp = (*timestamp).saturating_sub(boundary_seconds);
            let lower_timestamp = (*prev_timestamp).saturating_sub(boundary_seconds);

            // Skip if the range is empty (would happen if boundary > chain age)
            if upper_timestamp <= lower_timestamp {
                continue;
            }

            // Binary search to find start, then linear scan for end (typically 0-2 blocks)
            let start_idx = chain_state.partition_point(|b| *b.timestamp <= lower_timestamp);
            let end_idx = chain_state[start_idx..]
                .iter()
                .position(|b| *b.timestamp > upper_timestamp)
                .map_or(chain_state.len(), |pos| start_idx + pos);

            // Move supply from younger cohort to older cohort
            for block_state in &chain_state[start_idx..end_idx] {
                if let Some(state) = age_cohorts[boundary_idx].as_mut() {
                    state.decrement(&block_state.supply, block_state.price);
                }
                if let Some(state) = age_cohorts[boundary_idx + 1].as_mut() {
                    state.increment(&block_state.supply, block_state.price);
                }
            }
        }
    }
}
