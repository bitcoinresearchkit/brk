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
        let end = self.height().to_usize();
        // Match mempool.space's wall-clock `time > NOW() - INTERVAL ${period}` cutoff
        // by walking back through real block timestamps, not estimating via block count.
        let start = match time_period {
            Some(tp) => self.start_height(tp).to_usize(),
            None => 0,
        };

        let mut entries = iter_difficulty_epochs(self.computer(), start, end);

        // Return in reverse chronological order (newest first)
        entries.reverse();
        Ok(entries)
    }
}
