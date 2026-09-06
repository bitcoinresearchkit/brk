use vecdb::ReadableVec;

use super::*;

#[test]
fn truncate_cached_invalidates_same_length_cache() {
    let dir = tempfile::tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let inner = PcoVec::<Height, Timestamp>::forced_import(&db, "timestamp", Version::ONE).unwrap();
    let mut timestamps = CachedVec::wrap(inner);

    for timestamp in [10_u32, 20, 30] {
        timestamps.inner.push(Timestamp::from(timestamp));
    }
    assert_eq!(timestamps.collect(), [10_u32, 20, 30].map(Timestamp::from));

    timestamps
        .truncate_if_needed_with_stamp(Height::from(1_usize), Stamp::from(0_u64))
        .unwrap();
    timestamps.inner.push(Timestamp::from(200_u32));
    timestamps.inner.push(Timestamp::from(300_u32));

    assert_eq!(
        timestamps.collect(),
        [10_u32, 200, 300].map(Timestamp::from)
    );
}
