use brk_computer::Computer;
use brk_types::{DifficultyAdjustmentEntry, DifficultyEpoch, Height};
use vecdb::{GenericStoredVec, IterableVec, VecIndex};

/// Iterate over difficulty epochs within a height range.
pub fn iter_difficulty_epochs(
    computer: &Computer,
    start_height: usize,
    end_height: usize,
) -> Vec<DifficultyAdjustmentEntry> {
    let start_epoch = computer
        .indexes
        .block
        .height_to_difficultyepoch
        .read_once(Height::from(start_height))
        .unwrap_or_default();
    let end_epoch = computer
        .indexes
        .block
        .height_to_difficultyepoch
        .read_once(Height::from(end_height))
        .unwrap_or_default();

    let mut epoch_to_height_iter = computer
        .indexes
        .block
        .difficultyepoch_to_first_height
        .iter();
    let mut epoch_to_timestamp_iter = computer.blocks.time.difficultyepoch_to_timestamp.iter();
    let mut epoch_to_difficulty_iter = computer
        .blocks
        .mining
        .indexes_to_difficulty
        .difficultyepoch
        .iter();

    let mut results = Vec::with_capacity(end_epoch.to_usize() - start_epoch.to_usize() + 1);
    let mut prev_difficulty: Option<f64> = None;

    for epoch_usize in start_epoch.to_usize()..=end_epoch.to_usize() {
        let epoch = DifficultyEpoch::from(epoch_usize);
        let epoch_height = epoch_to_height_iter.get(epoch).unwrap_or_default();

        // Skip epochs before our start height but track difficulty
        if epoch_height.to_usize() < start_height {
            prev_difficulty = epoch_to_difficulty_iter.get(epoch).map(|d| *d);
            continue;
        }

        let epoch_timestamp = epoch_to_timestamp_iter.get(epoch).unwrap_or_default();
        let epoch_difficulty = *epoch_to_difficulty_iter.get(epoch).unwrap_or_default();

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
