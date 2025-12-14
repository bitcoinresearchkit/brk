use brk_error::Result;
use brk_types::{DifficultyAdjustmentEntry, TimePeriod};
use vecdb::VecIndex;

use crate::Query;

use super::epochs::iter_difficulty_epochs;

/// Get historical difficulty adjustments.
pub fn get_difficulty_adjustments(
    time_period: Option<TimePeriod>,
    query: &Query,
) -> Result<Vec<DifficultyAdjustmentEntry>> {
    let current_height = query.get_height();
    let end = current_height.to_usize();
    let start = match time_period {
        Some(tp) => end.saturating_sub(tp.block_count()),
        None => 0,
    };

    let mut entries = iter_difficulty_epochs(query.computer(), start, end);

    // Return in reverse chronological order (newest first)
    entries.reverse();
    Ok(entries)
}
