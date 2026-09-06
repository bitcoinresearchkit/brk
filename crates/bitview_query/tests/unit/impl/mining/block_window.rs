use brk_types::Cents;
use std::collections::BTreeMap;

use super::*;

#[test]
fn windows_reject_reversed_or_partial_timestamp_sources() {
    let timestamps = [Timestamp::from(100u32), Timestamp::from(101u32)];
    assert!(BlockWindow::from_timestamps(1u32.into(), 0u32.into(), TimePeriod::All, &[]).is_err());
    for len in [0, 1] {
        assert!(
            BlockWindow::from_timestamps(
                0u32.into(),
                2u32.into(),
                TimePeriod::All,
                &timestamps[..len]
            )
            .is_err()
        );
    }
    assert!(
        BlockWindow::from_timestamps(0u32.into(), 1u32.into(), TimePeriod::All, &timestamps)
            .is_err()
    );
    assert!(
        BlockWindow::from_timestamps(0u32.into(), 2u32.into(), TimePeriod::All, &timestamps)
            .is_ok()
    );
}

#[test]
fn value_reads_require_the_complete_persisted_window() {
    use brk_types::{Sats, Version};
    use vecdb::{AnyStoredVec, Database, EagerVec, ImportableVec, PcoVec, WritableVec};

    let directory = tempfile::tempdir().unwrap();
    let database = Database::open(directory.path()).unwrap();
    let mut values: EagerVec<PcoVec<Height, Sats>> =
        EagerVec::forced_import(&database, "values", Version::ONE).unwrap();
    let window = BlockWindow::from_timestamps(
        0u32.into(),
        2u32.into(),
        TimePeriod::All,
        &[100u32.into(), 101u32.into()],
    )
    .unwrap();
    assert!(window.read(&values).is_err());
    values.push(Sats::from(1u64));
    values.write().unwrap();
    assert!(window.read(&values).is_err());
    values.push(Sats::from(2u64));
    values.write().unwrap();
    assert_eq!(
        window.read(&values).unwrap(),
        vec![Sats::from(1u64), Sats::from(2u64)]
    );
}

#[test]
fn bucket_means_preserve_clock_regressions_and_large_integer_values() {
    let timestamps = [
        Timestamp::from(300u32),
        Timestamp::from(100u32),
        Timestamp::from(300u32),
    ];
    let window =
        BlockWindow::from_timestamps(10u32.into(), 13u32.into(), TimePeriod::Day, &timestamps)
            .unwrap();
    assert_eq!(window.buckets.len(), 2);
    assert_eq!(window.buckets[0].avg_timestamp, timestamps[1]);
    assert_eq!(window.buckets[1].avg_timestamp, timestamps[0]);
    assert_eq!(u32::from(window.buckets[1].avg_height), 11);
    let values = [
        Cents::from(u64::MAX - 1),
        Cents::from(1u64),
        Cents::from(u64::MAX - 2),
    ];
    assert_eq!(u64::from(window.buckets[0].mean_rounded(&values)), 1);
    assert_eq!(
        u64::from(window.buckets[1].mean_rounded(&values)),
        u64::MAX - 1
    );
    assert_eq!(
        window.buckets[1].mean_price(&values),
        brk_types::Dollars::from(Cents::MAX_FINITE)
    );
    let unavailable = [Cents::NAN; 3];
    assert!(f64::from(window.buckets[1].mean_price(&unavailable)).is_nan());
}

#[test]
fn hashed_buckets_match_ordered_groups_for_all_periods() {
    let timestamps: Vec<_> = (0..10_000u32)
        .map(|height| {
            Timestamp::from(1_231_006_505 + height * 600 - if height % 11 == 0 { 1200 } else { 0 })
        })
        .chain([Timestamp::from(u32::MAX), Timestamp::from(0u32)])
        .collect();
    for period in [
        TimePeriod::Day,
        TimePeriod::ThreeDays,
        TimePeriod::Week,
        TimePeriod::Month,
        TimePeriod::ThreeMonths,
        TimePeriod::SixMonths,
        TimePeriod::Year,
        TimePeriod::TwoYears,
        TimePeriod::ThreeYears,
        TimePeriod::All,
    ] {
        let div = time_div(period);
        let mut expected: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        for (i, timestamp) in timestamps.iter().enumerate() {
            expected.entry(**timestamp / div).or_default().push(i);
        }
        assert_eq!(
            bucket_offsets(&timestamps, div),
            expected.into_iter().collect::<Vec<_>>()
        );
        assert!(bucket_offsets(&[], div).is_empty());
    }
}

#[test]
#[ignore = "million-row mining bucket grouping; excludes vector I/O and HTTP"]
fn benchmark_bucket_grouping() {
    use std::{hint::black_box, time::Instant};

    let timestamps: Vec<_> = (0..1_000_000u32)
        .map(|height| {
            Timestamp::from(1_231_006_505 + height * 600 - if height % 11 == 0 { 1200 } else { 0 })
        })
        .collect();
    let ordered = || {
        let mut groups: BTreeMap<u32, Vec<usize>> = BTreeMap::new();
        for (i, ts) in timestamps.iter().enumerate() {
            groups.entry(**ts / 86400).or_default().push(i);
        }
        groups.into_iter().collect::<Vec<_>>()
    };
    let hashed = || bucket_offsets(&timestamps, 86400);
    assert_eq!(ordered(), hashed());
    let mut times = [Vec::new(), Vec::new()];
    for round in 0..8 {
        for variant in [round % 2, 1 - round % 2] {
            let started = Instant::now();
            black_box(if variant == 0 { ordered() } else { hashed() });
            if round >= 2 {
                times[variant].push(started.elapsed());
            }
        }
    }
    for samples in &mut times {
        samples.sort_unstable();
    }
    eprintln!(
        "million-row daily buckets: ordered {:?}, hash then sort {:?}",
        times[0][3], times[1][3]
    );
}
