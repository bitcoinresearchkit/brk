use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use tempfile::tempdir;
#[cfg(feature = "pco")]
use vecdb::PcoVec;
use vecdb::{
    AnyStoredVec, Budgeted, BytesVec, CacheBudget, Database, ImportOptions, ImportableVec,
    ReadableVec, Version, WritableVec,
};

static BUDGET: CacheBudget = CacheBudget::new(32 * 1024 * 1024);

fn median(mut read: impl FnMut(), iterations: u32) -> Duration {
    let mut samples = Vec::with_capacity(15);
    for _ in 0..15 {
        let start = Instant::now();
        for _ in 0..iterations {
            read();
        }
        samples.push(start.elapsed() / iterations);
    }
    samples.sort_unstable();
    samples[7]
}

#[test]
#[ignore = "synthetic local storage benchmark"]
fn bounded_hash_ranges() {
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut values = BytesVec::<usize, [u8; 32], Budgeted>::import_with(
        ImportOptions::new(&db, "hashes", Version::ONE).with_cache_budget(&BUDGET),
    )
    .unwrap();
    for i in 0..1_000_000 {
        values.push([i as u8; 32]);
    }
    values.write().unwrap();
    for count in [1, 10] {
        let from = 1_000_000 - count;
        let expected: Vec<_> = (from..1_000_000).map(|i| [i as u8; 32]).collect();
        assert_eq!(values.collect_range_at(from, 1_000_000), expected);
        let warm = median(
            || {
                black_box(values.collect_range_at(black_box(from), 1_000_000));
            },
            10_000,
        );
        let reader = median(
            || {
                let reader = values.reader();
                for i in from..1_000_000 {
                    black_box(reader.try_get_at(i).unwrap());
                }
            },
            10_000,
        );
        eprintln!("hashes count={count}: warm range {warm:?}, direct reader {reader:?}");
    }
}

#[test]
#[ignore = "synthetic local storage benchmark"]
#[cfg(feature = "pco")]
fn compressed_source_cache() {
    const N: usize = 1_000_000;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut cached = PcoVec::<usize, u64, Budgeted>::import_with(
        ImportOptions::new(&db, "cached", Version::ONE).with_cache_budget(&BUDGET),
    )
    .unwrap();
    let mut plain = PcoVec::<usize, u64>::import(&db, "plain", Version::ONE).unwrap();
    for i in 0..N {
        let price = 1_000_000 + (i as u64 * 13) % 100_000;
        cached.push(price);
        plain.push(price);
    }
    cached.write().unwrap();
    plain.write().unwrap();
    for count in [1, 15, 512, 4_096, N] {
        let from = N - count;
        BUDGET.clear();
        let start = Instant::now();
        let expected = cached.collect_range_at(from, N);
        let cold = start.elapsed();
        assert_eq!(expected, plain.collect_range_at(from, N));
        let iterations = if count == N { 10 } else { 1_000 };
        let warm = median(
            || {
                black_box(cached.collect_range_at(black_box(from), N));
            },
            iterations,
        );
        let uncached = median(
            || {
                black_box(plain.collect_range_at(black_box(from), N));
            },
            iterations,
        );
        eprintln!(
            "pco range count={count}: first read {cold:?}, warm {warm:?}, uncached {uncached:?}, retained={}",
            BUDGET.used()
        );
    }
    let expected = plain.fold(0u64, u64::wrapping_add);
    assert_eq!(cached.fold(0u64, u64::wrapping_add), expected);
    let warm_fold = median(
        || {
            black_box(cached.fold(0u64, u64::wrapping_add));
        },
        20,
    );
    let direct_fold = median(
        || {
            black_box(plain.fold(0u64, u64::wrapping_add));
        },
        20,
    );
    let point = median(
        || {
            black_box(cached.collect_one_at(black_box(N - 1)));
        },
        10_000,
    );
    eprintln!(
        "pco million-value fold: warm {warm_fold:?}, uncached {direct_fold:?}; warm point {point:?}"
    );
}

#[test]
#[ignore = "synthetic local storage benchmark"]
fn incremental_range_fills() {
    const N: usize = 16_384;
    let directory = tempdir().unwrap();
    let db = Database::open(directory.path()).unwrap();
    let mut values = BytesVec::<usize, u64, Budgeted>::import_with(
        ImportOptions::new(&db, "values", Version::ONE).with_cache_budget(&BUDGET),
    )
    .unwrap();
    for index in 0..N {
        values.push(index as u64);
    }
    values.write().unwrap();
    for reverse in [false, true] {
        let fill = median(
            || {
                BUDGET.clear();
                for offset in 0..N {
                    let index = if reverse { N - 1 - offset } else { offset };
                    black_box(values.collect_one_at(black_box(index)));
                }
            },
            1,
        );
        let warm = median(
            || {
                black_box(values.fold(0u64, u64::wrapping_add));
            },
            100,
        );
        assert_eq!(values.collect(), (0..N as u64).collect::<Vec<_>>());
        eprintln!(
            "raw point fills reverse={reverse}: refill {fill:?}, warm fold {warm:?}, retained={}",
            BUDGET.used()
        );
    }
    let fill = median(
        || {
            BUDGET.clear();
            for from in (0..N).step_by(128) {
                black_box(values.collect_range_at(from, (from + 512).min(N)));
            }
        },
        1,
    );
    assert_eq!(values.collect(), (0..N as u64).collect::<Vec<_>>());
    eprintln!(
        "raw overlapping ranges: refill {fill:?}, retained={}",
        BUDGET.used()
    );
}
