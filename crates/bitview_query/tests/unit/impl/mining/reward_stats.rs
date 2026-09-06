use std::time::{SystemTime, UNIX_EPOCH};

use brk_types::{Sats, Version};
use vecdb::{
    AnyStoredVec, Database, EagerVec, ImportableVec, PcoVec, ReadOnlyClone, ReadableVec,
    WritableVec,
};

use super::*;

#[test]
fn cumulative_delta_rejects_reversed_ranges_and_decreasing_values() {
    let directory = tempfile::tempdir().unwrap();
    let database = Database::open(directory.path()).unwrap();
    let mut values: EagerVec<PcoVec<Height, Sats>> =
        EagerVec::forced_import(&database, "values", Version::ONE).unwrap();
    values.push(Sats::from(10u64));
    values.push(Sats::from(5u64));
    values.write().unwrap();
    assert!(cumulative_delta(&values, 1u32.into(), 0u32.into()).is_err());
    assert!(cumulative_delta(&values, 1u32.into(), 1u32.into()).is_err());
}

#[test]
fn cumulative_delta_matches_range_sum_across_read_strategies() {
    let suffix = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path = std::env::temp_dir().join(format!(
        "bitview-reward-stats-{}-{suffix}",
        std::process::id()
    ));
    let db = Database::open(&path).unwrap();
    let mut cumulative: EagerVec<PcoVec<Height, Sats>> =
        EagerVec::forced_import(&db, "cumulative", Version::ONE).unwrap();
    let values = (0_usize..2_051)
        .map(|index| Sats::from(index + 1))
        .collect::<Vec<_>>();
    let mut total = Sats::ZERO;

    for value in &values {
        total += *value;
        cumulative.push(total);
    }
    cumulative.write().unwrap();

    let read_only = cumulative.read_only_clone();
    let page = read_only.cursor_chunk_size();
    for (start, end) in [
        (0, 0),
        (0, values.len() - 1),
        (values.len() - 1, values.len() - 1),
        (values.len() - page, values.len() - 1),
        (values.len() - page - 1, values.len() - 1),
        (32, 1_750),
    ] {
        let expected = values[start..=end].iter().copied().sum();
        let actual = cumulative_delta(&read_only, Height::from(start), Height::from(end)).unwrap();
        assert_eq!(actual, expected, "range {start}..={end}");
    }

    drop(read_only);
    drop(cumulative);
    drop(db);
    std::fs::remove_dir_all(path).unwrap();
}
