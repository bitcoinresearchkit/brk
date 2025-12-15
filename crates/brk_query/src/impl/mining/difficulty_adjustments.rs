use brk_error::Result;
use brk_types::{DifficultyAdjustmentEntry, TimePeriod};
use vecdb::VecIndex;

use super::epochs::iter_difficulty_epochs;
use crate::Query;

impl Query {
    pub fn difficulty_adjustments(
        &self,
        time_period: Option<TimePeriod>,
    ) -> Result<Vec<DifficultyAdjustmentEntry>> {
        let current_height = self.height();
        let end = current_height.to_usize();
        let start = match time_period {
            Some(tp) => end.saturating_sub(tp.block_count()),
            None => 0,
        };

        let mut entries = iter_difficulty_epochs(self.computer(), start, end);

        // Return in reverse chronological order (newest first)
        entries.reverse();
        Ok(entries)
    }
}
