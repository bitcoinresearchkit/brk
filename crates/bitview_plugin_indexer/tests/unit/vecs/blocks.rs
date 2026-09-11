use tempfile::tempdir;
use vecdb::{Budgeted, ImportOptions, ReadableVec};

use super::*;

#[test]
fn truncate_cached_invalidates_same_length_cache() {
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let cache = Box::leak(Box::new(CacheBudget::new(1024)));
    let mut timestamps = PcoVec::<Height, Timestamp, Budgeted>::forced_import_with(
        ImportOptions::new(&db, "timestamp", Version::ONE).with_cache_budget(cache),
    )
    .unwrap();

    for timestamp in [10_u32, 20, 30] {
        timestamps.push(Timestamp::from(timestamp));
    }
    timestamps.write().unwrap();
    assert_eq!(timestamps.collect(), [10_u32, 20, 30].map(Timestamp::from));

    timestamps
        .truncate_if_needed_with_stamp(Height::from(1_usize), Stamp::from(0_u64))
        .unwrap();
    timestamps.push(Timestamp::from(200_u32));
    timestamps.push(Timestamp::from(300_u32));
    timestamps.write().unwrap();

    assert_eq!(
        timestamps.collect(),
        [10_u32, 200, 300].map(Timestamp::from)
    );
}
