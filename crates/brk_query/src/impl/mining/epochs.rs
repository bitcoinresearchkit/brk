use brk_computer::Computer;
use brk_types::{DifficultyAdjustmentEntry, Height};
use vecdb::{ReadableVec, Ro, VecIndex};

/// Iterate over difficulty epochs within a height range.
pub fn iter_difficulty_epochs(
    computer: &Computer<Ro>,
    start_height: usize,
    end_height: usize,
) -> Vec<DifficultyAdjustmentEntry> {
    let start_epoch = computer
        .indexes
        .height
        .epoch
        .collect_one(Height::from(start_height))
        .unwrap_or_default();
    let end_epoch = computer
        .indexes
        .height
        .epoch
        .collect_one(Height::from(end_height))
        .unwrap_or_default();

    let mut height_cursor = computer.indexes.epoch.first_height.cursor();
    let mut timestamp_cursor = computer.indexes.timestamp.epoch.cursor();
    let mut difficulty_cursor = computer.blocks.difficulty.value.epoch.cursor();

    let mut results = Vec::with_capacity(end_epoch.to_usize() - start_epoch.to_usize() + 1);
    let mut prev_difficulty: Option<f64> = None;

    for epoch_usize in start_epoch.to_usize()..=end_epoch.to_usize() {
        let epoch_height = height_cursor.get(epoch_usize).unwrap_or_default();

        // Skip epochs before our start height but track difficulty
        if epoch_height.to_usize() < start_height {
            prev_difficulty = difficulty_cursor.get(epoch_usize).map(|d| *d);
            continue;
        }

        let epoch_timestamp = timestamp_cursor.get(epoch_usize).unwrap_or_default();
        let epoch_difficulty = *difficulty_cursor.get(epoch_usize).unwrap_or_default();

        let change_percent = match prev_difficulty {
            Some(prev) if prev > 0.0 => epoch_difficulty / prev,
            _ => 0.0,
        };

        results.push(DifficultyAdjustmentEntry {
            timestamp: epoch_timestamp,
            height: epoch_height,
            difficulty: epoch_difficulty,
            change_percent,
        });

        prev_difficulty = Some(epoch_difficulty);
    }

    results
}
