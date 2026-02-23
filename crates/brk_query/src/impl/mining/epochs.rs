use brk_computer::Computer;
use brk_types::{DifficultyAdjustmentEntry, DifficultyEpoch, Height};
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
        .difficultyepoch
        .collect_one(Height::from(start_height))
        .unwrap_or_default();
    let end_epoch = computer
        .indexes
        .height
        .difficultyepoch
        .collect_one(Height::from(end_height))
        .unwrap_or_default();

    let epoch_to_height = &computer.indexes.difficultyepoch.first_height;
    let epoch_to_timestamp = &computer.blocks.time.timestamp.difficultyepoch;
    let epoch_to_difficulty = &computer.blocks.difficulty.raw.difficultyepoch;

    let mut results = Vec::with_capacity(end_epoch.to_usize() - start_epoch.to_usize() + 1);
    let mut prev_difficulty: Option<f64> = None;

    for epoch_usize in start_epoch.to_usize()..=end_epoch.to_usize() {
        let epoch = DifficultyEpoch::from(epoch_usize);
        let epoch_height = epoch_to_height.collect_one(epoch).unwrap_or_default();

        // Skip epochs before our start height but track difficulty
        if epoch_height.to_usize() < start_height {
            prev_difficulty = epoch_to_difficulty.collect_one(epoch).map(|d| *d);
            continue;
        }

        let epoch_timestamp = epoch_to_timestamp.collect_one(epoch).unwrap_or_default();
        let epoch_difficulty = *epoch_to_difficulty.collect_one(epoch).unwrap_or_default();

        let change_percent = match prev_difficulty {
            Some(prev) if prev > 0.0 => ((epoch_difficulty / prev) - 1.0) * 100.0,
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
