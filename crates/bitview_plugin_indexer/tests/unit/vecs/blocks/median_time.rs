use brk_types::{Height, Version};
use vecdb::{AnyStoredVec, Database, Stamp};

use super::*;

fn expected(timestamps: &[Timestamp]) -> Vec<Timestamp> {
    (0..timestamps.len())
        .map(|height| {
            let mut values = timestamps[height.saturating_sub(10)..=height].to_vec();
            values.sort_unstable();
            values[values.len() / 2]
        })
        .collect()
}

#[test]
fn batches_reorgs_and_reopen_match_window_medians() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut blocks = BlocksVecs::forced_import(&db, Version::ONE).unwrap();
    let mut timestamps = Vec::new();
    // Nonmonotonic, repeated timestamps, early even windows, and page crossings.
    for end in [1, 2, 6, 10, 11, 12, 25, 10_000] {
        for height in timestamps.len()..end {
            let value = Timestamp::from(((height * 7919) % 101) as u32);
            timestamps.push(value);
            blocks.timestamp.inner.push(value);
        }
        blocks.compute_median_times().unwrap();
        assert_eq!(blocks.median_time.collect(), expected(&timestamps));
        let len = blocks.median_time.len();
        blocks.compute_median_times().unwrap();
        assert_eq!(blocks.median_time.len(), len);
    }
    let stamp = Stamp::from(9999_u64);
    blocks.timestamp.inner.stamped_write(stamp).unwrap();
    blocks.median_time.stamped_write(stamp).unwrap();
    db.flush().unwrap();
    drop(blocks);
    let mut blocks = BlocksVecs::forced_import(&db, Version::ONE).unwrap();
    assert_eq!(blocks.median_time.collect(), expected(&timestamps));

    // Replace a suffix, then return to precisely the same height and stamp.
    blocks
        .truncate(Height::new(9990), Stamp::from(9989_u64))
        .unwrap();
    timestamps.truncate(9990);
    for value in 200_u32..210 {
        let value = Timestamp::from(value);
        timestamps.push(value);
        blocks.timestamp.inner.push(value);
    }
    blocks.compute_median_times().unwrap();
    assert_eq!(blocks.median_time.collect(), expected(&timestamps));
}

#[test]
fn import_backfills_missing_or_mismatched_checkpoint() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut blocks = BlocksVecs::forced_import(&db, Version::ONE).unwrap();
    let timestamps = (0..10_000)
        .map(|height| Timestamp::from(((height * 7919) % 101) as u32))
        .collect::<Vec<_>>();
    for &value in &timestamps {
        blocks.timestamp.inner.push(value);
    }
    let stamp = Stamp::from(9999_u64);
    blocks.timestamp.inner.stamped_write(stamp).unwrap();
    db.flush().unwrap();
    drop(blocks);

    // Simulates an existing database predating the median-time vector.
    let mut blocks = BlocksVecs::forced_import(&db, Version::ONE).unwrap();
    assert_eq!(blocks.median_time.collect(), expected(&timestamps));
    assert_eq!(blocks.median_time.stamp(), stamp);

    // Equal length is insufficient when the vector belongs to another checkpoint.
    blocks.median_time.clear().unwrap();
    for _ in &timestamps {
        blocks.median_time.push(Timestamp::ZERO);
    }
    blocks
        .median_time
        .stamped_write(Stamp::from(10_u64))
        .unwrap();
    db.flush().unwrap();
    drop(blocks);
    let blocks = BlocksVecs::forced_import(&db, Version::ONE).unwrap();
    assert_eq!(blocks.median_time.collect(), expected(&timestamps));
    assert_eq!(blocks.median_time.stamp(), stamp);
}
