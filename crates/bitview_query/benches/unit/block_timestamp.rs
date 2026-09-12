use std::{cmp::Reverse, hint::black_box, time::Instant};

use brk_types::Version;
use tempfile::{TempDir, tempdir};
use vecdb::{
    AnyStoredVec, Budgeted, Database, ImportableVec, PcoVec, Stamp, StoredVec, WritableVec,
};

use super::*;

type Timestamps = <PcoVec<Height, Timestamp, Budgeted> as StoredVec>::ReadOnly;

// Exact pre-optimization control; never used by production lookup.
fn select_with_stored_median(
    len: usize,
    target: Timestamp,
    mut maximum: impl FnMut(usize) -> Result<Timestamp>,
    mut raw: impl FnMut(usize) -> Result<Timestamp>,
    mut median: impl FnMut(usize) -> Result<Timestamp>,
) -> Result<(usize, Timestamp)> {
    if len == 0 {
        return Err(Error::StateUpdating);
    }
    let crossing = lower_bound(len, target, true, &mut maximum)?;
    if crossing == 0 {
        return Err(Error::NotFound("No block at or before timestamp".into()));
    }
    let mut best = maximum(crossing - 1)?;
    let mut height = lower_bound(crossing, best, false, &mut maximum)?;
    if best == target {
        return Ok((height, best));
    }
    for h in crossing..len {
        let timestamp = raw(h)?;
        if timestamp <= target && timestamp > best {
            best = timestamp;
            height = h;
        }
        if best == target {
            break;
        }
        // Header consensus requires descendants to exceed previous MTP.
        // BIP113 reused that clock, rather than introducing the header rule.
        if median(h)? > target {
            break;
        }
    }
    Ok((height, best))
}

#[test]
fn a_valid_two_hour_spike_can_leave_a_long_scan_window() {
    let mut raw = vec![0u32];
    let mut maximum = vec![0u32];
    let mut median = vec![0u32];
    for h in 1usize..22_000 {
        let mut window = raw[h.saturating_sub(11)..h].to_vec();
        window.sort_unstable();
        let value = if h == 11 {
            7200
        } else {
            window[window.len() / 2] + 2
        };
        assert!(value > window[window.len() / 2]);
        raw.push(value);
        maximum.push(maximum[h - 1].max(value));
        let mut window = raw[h.saturating_sub(10)..=h].to_vec();
        window.sort_unstable();
        median.push(window[window.len() / 2]);
    }
    let mut scanned = 0;
    let result = select_timestamp(
        raw.len(),
        Timestamp::from(7199u32),
        |h| Ok(maximum[h].into()),
        |h| {
            scanned += 1;
            Ok(raw[h].into())
        },
    )
    .unwrap();
    assert_eq!(
        select_with_stored_median(
            raw.len(),
            Timestamp::from(7199u32),
            |h| Ok(maximum[h].into()),
            |h| Ok(raw[h].into()),
            |h| Ok(median[h].into())
        )
        .unwrap(),
        result
    );
    let expected = raw
        .iter()
        .enumerate()
        .filter(|(_, value)| **value <= 7199)
        .max_by_key(|(h, value)| (**value, Reverse(*h)))
        .unwrap();
    assert_eq!(result, (expected.0, Timestamp::from(*expected.1)));
    assert_eq!(scanned, 21_578);
}

fn fixture(len: usize, skewed: bool) -> (TempDir, Database, [Timestamps; 3]) {
    init_cache();
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut columns: [PcoVec<Height, Timestamp, Budgeted>; 3] = ["raw", "maximum", "median"]
        .map(|name| PcoVec::forced_import(&db, name, Version::ONE).unwrap());
    let mut maximum = Timestamp::ZERO;
    let mut window = [Timestamp::ZERO; 11];
    for h in 0..len {
        let value = if skewed && h > 0 {
            let mut previous = window;
            let count = h.min(11);
            previous[..count].sort_unstable();
            if h == 11 {
                Timestamp::from(1_231_006_505u32 + 7200)
            } else {
                Timestamp::from(*previous[count / 2] + 2)
            }
        } else {
            Timestamp::from(1_231_006_505 + (h * 600) as u32 + ((h * 7919) % 1200) as u32)
        };
        maximum = maximum.max(value);
        window[h % 11] = value;
        let mut sorted = window;
        let count = (h + 1).min(11);
        sorted[..count].sort_unstable();
        for (column, value) in columns.iter_mut().zip([value, maximum, sorted[count / 2]]) {
            column.push(value);
        }
    }
    for column in &mut columns {
        column.stamped_write(Stamp::from((len - 1) as u64)).unwrap();
    }
    db.flush().unwrap();
    let columns = columns.map(|column| column.read_only_clone());
    (dir, db, columns)
}

fn lookup(
    columns: &[Timestamps; 3],
    len: usize,
    target: Timestamp,
    stored_median: bool,
) -> ((usize, Timestamp), usize) {
    let mut scanned = 0;
    let mut read_max = |h| columns[1].collect_one_at(h).data();
    let mut read_raw = |h| {
        scanned += 1;
        columns[0].collect_one_at(h).data()
    };
    let selected = if stored_median {
        select_with_stored_median(len, target, &mut read_max, &mut read_raw, |h| {
            columns[2].collect_one_at(h).data()
        })
    } else {
        select_timestamp(len, target, &mut read_max, &mut read_raw)
    }
    .unwrap();
    assert_eq!(columns[0].collect_one_at(selected.0), Some(selected.1));
    (selected, scanned)
}

#[test]
fn persisted_timestamp_search_matches_sorted_predecessor() {
    let (_dir, _db, columns) = fixture(20_000, false);
    let mut oracle: Vec<_> = columns[0]
        .collect()
        .into_iter()
        .enumerate()
        .map(|(h, value)| (value, Reverse(h)))
        .collect();
    oracle.sort_unstable();
    for warm in [false, true] {
        init_cache().clear();
        if warm {
            columns[0].collect();
            columns[1].collect();
        }
        for index in (0..20_000).step_by(137) {
            for target in [
                oracle[index].0,
                Timestamp::from(*oracle[index].0 + 1),
                Timestamp::from(u32::MAX),
            ] {
                let i = oracle.partition_point(|(value, _)| *value <= target);
                let &(value, Reverse(height)) = &oracle[i - 1];
                for stored in [false, true] {
                    assert_eq!(lookup(&columns, 20_000, target, stored).0, (height, value));
                }
            }
        }
    }
}

#[test]
#[ignore = "persisted million-row timestamp lookup; excludes HTTP and OS cache eviction"]
fn benchmark_timestamp_selection_storage() {
    let len = 1_000_000;
    for skewed in [false, true] {
        let (_dir, _db, columns) = fixture(len, skewed);
        let targets = [
            Timestamp::from(u32::MAX),
            columns[0].collect_one_at(900_000).unwrap(),
            Timestamp::from(*columns[0].collect_one_at(900_000).unwrap() + 1),
            Timestamp::from(1_231_006_505u32 + 7199),
        ];
        for warm in [false, true] {
            init_cache().clear();
            if warm {
                let started = Instant::now();
                columns[0].collect();
                columns[1].collect();
                eprintln!(
                    "timestamp range fill {:?}; retained {} bytes",
                    started.elapsed(),
                    init_cache().used()
                );
            }
            for target in targets {
                let mut samples = [Vec::new(), Vec::new()];
                let mut scan = 0;
                for round in 0..12 {
                    for variant in [round % 2, 1 - round % 2] {
                        let started = Instant::now();
                        for _ in 0..100 {
                            let result = lookup(&columns, len, target, variant == 0);
                            scan = result.1;
                            black_box(result);
                        }
                        if round >= 2 {
                            samples[variant].push(started.elapsed() / 100);
                        }
                    }
                }
                for values in &mut samples {
                    values.sort_unstable();
                }
                eprintln!(
                    "timestamp skewed={skewed} warm={warm} target={target} stored/rolling={:?}/{:?} scanned={scan}",
                    samples[0][5], samples[1][5]
                );
            }
        }
    }
}
use crate::test_cache::init_cache;
