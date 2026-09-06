use std::{
    hint::black_box,
    time::{Duration, Instant},
};

use tempfile::tempdir;
#[cfg(feature = "pco")]
use vecdb::PcoVec;
use vecdb::{
    AnyStoredVec, BytesVec, CachedVec, Database, ImportableVec, ReadableVec, Version, WritableVec,
};

#[test]
#[ignore = "bounded 32-byte range reads; synthetic local storage, no server"]
fn bounded_hash_ranges() {
    for (history, count) in [
        (0usize, 10),
        (10, 1),
        (10, 10),
        (1_000_000, 1),
        (1_000_000, 10),
    ] {
        let directory = tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let mut values = BytesVec::<usize, [u8; 32]>::import(&db, "hashes", Version::ONE).unwrap();
        for index in 0..history {
            values.push([index as u8; 32]);
        }
        values.write().unwrap();
        let cached = CachedVec::wrap(values.read_only_clone());
        let begin = history.saturating_sub(count);
        let expected: Vec<_> = (begin..history).map(|index| [index as u8; 32]).collect();
        assert_eq!(cached.collect_range_at(begin, history), expected);
        assert_eq!(cached.inner.collect_range_at(begin, history), expected);
        let reader = cached.inner.reader();
        assert_eq!(
            (begin..history)
                .map(|i| reader.try_get_at(i).unwrap())
                .collect::<Vec<_>>(),
            expected
        );
        drop(reader);

        for cold in [false, true] {
            let mut times = [Vec::new(), Vec::new(), Vec::new()];
            let iterations = if cold { 10 } else { 10_000 };
            for round in 0..24 {
                for offset in 0..3 {
                    let variant = (round + offset) % 3;
                    let read = || {
                        if variant == 2 {
                            let reader = cached.inner.reader();
                            for i in (black_box(begin)..black_box(history)).rev() {
                                black_box(reader.try_get_at(i).unwrap());
                            }
                        } else {
                            let rows = if variant == 0 {
                                cached.collect_range_at(black_box(begin), black_box(history))
                            } else {
                                cached
                                    .inner
                                    .collect_range_at(black_box(begin), black_box(history))
                            };
                            for row in rows.iter().rev() {
                                black_box(*row);
                            }
                        }
                    };
                    let mut elapsed = Duration::ZERO;
                    if cold {
                        for _ in 0..iterations {
                            cached.invalidate();
                            let started = Instant::now();
                            read();
                            elapsed += started.elapsed();
                        }
                    } else {
                        let started = Instant::now();
                        for _ in 0..iterations {
                            read();
                        }
                        elapsed = started.elapsed();
                    }
                    if round >= 4 {
                        times[variant].push(elapsed / iterations);
                    }
                }
            }
            for samples in &mut times {
                samples.sort_unstable();
            }
            eprintln!(
                "history={history} count={count} cold={cold}: cached {:?}, bounded {:?}, reader {:?}",
                times[0][10], times[1][10], times[2][10]
            );
        }
    }
}

#[test]
#[ignore = "bounded compressed price reads; synthetic local storage, no server"]
#[cfg(feature = "pco")]
fn bounded_price_ranges() {
    // u64 pages contain 1024 values: cover a tail, a full page, and a
    // 15-value range crossing from a full page into the next tail.
    for history in [2usize, 1_000_000, 1_000_448, 1_000_453] {
        let directory = tempdir().unwrap();
        let db = Database::open(directory.path()).unwrap();
        let mut values = PcoVec::<usize, u64>::import(&db, "prices", Version::ONE).unwrap();
        let price = |index: usize| 1_000_000 + (index as u64 * 13) % 100_000;
        for index in 0..history {
            values.push(price(index));
        }
        values.write().unwrap();
        let cached = CachedVec::wrap(values.read_only_clone());
        let begin = history.saturating_sub(15);
        let expected: Vec<_> = (begin..history).map(price).collect();
        assert_eq!(cached.collect_range_at(begin, history), expected);
        assert_eq!(cached.inner.collect_range_at(begin, history), expected);
        assert_eq!(
            &cached.cached_snapshot().unwrap()[begin..history],
            expected.as_slice()
        );
        for cold in [false, true] {
            let mut times = [Vec::new(), Vec::new(), Vec::new()];
            let iterations = if cold { 10 } else { 1000 };
            for round in 0..24 {
                for offset in 0..3 {
                    let variant = (round + offset) % 3;
                    let mut elapsed = Duration::ZERO;
                    for _ in 0..iterations {
                        if cold {
                            cached.invalidate();
                        }
                        let started = Instant::now();
                        let rows = if variant == 0 {
                            cached.collect_range_at(black_box(begin), black_box(history))
                        } else if variant == 1 {
                            cached
                                .inner
                                .collect_range_at(black_box(begin), black_box(history))
                        } else {
                            cached.cached_snapshot().map_or_else(
                                || {
                                    cached
                                        .inner
                                        .collect_range_at(black_box(begin), black_box(history))
                                },
                                |values| values[black_box(begin)..black_box(history)].to_vec(),
                            )
                        };
                        black_box(rows);
                        elapsed += started.elapsed();
                        if cold && variant == 2 {
                            assert!(cached.cached_snapshot().is_none());
                        }
                    }
                    if round >= 4 {
                        times[variant].push(elapsed / iterations);
                    }
                }
            }
            for samples in &mut times {
                samples.sort_unstable();
            }
            eprintln!(
                "price history={history} count={} cold={cold}: cached {:?}, bounded {:?}, reuse-or-bounded {:?}",
                expected.len(),
                times[0][10],
                times[1][10],
                times[2][10]
            );
        }
    }
}
