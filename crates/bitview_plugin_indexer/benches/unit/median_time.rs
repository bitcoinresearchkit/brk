use std::{hint::black_box, time::Instant};

use brk_types::Version;
use tempfile::tempdir;
use vecdb::{AnyStoredVec, CacheBudget, Database, Stamp};

use super::*;

#[test]
#[ignore = "manual median-time source read benchmark"]
fn benchmark_median_time_reads() {
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    static CACHE: CacheBudget = CacheBudget::new(64 * 1024 * 1024);
    let mut blocks = BlocksVecs::forced_import(&CACHE, &db, Version::ONE).unwrap();
    let end = 1_000_000;
    for height in 0..end {
        blocks.timestamp.push(Timestamp::from(
            1_231_006_505 + (height * 600) as u32 + ((height * 7919) % 1800) as u32,
        ));
    }
    let started = Instant::now();
    blocks.compute_median_times().unwrap();
    eprintln!("median-time backfill {end} rows: {:?}", started.elapsed());
    let stamp = Stamp::from((end - 1) as u64);
    blocks.timestamp.stamped_write(stamp).unwrap();
    blocks.median_time.stamped_write(stamp).unwrap();
    db.flush().unwrap();

    for warm in [false, true] {
        CACHE.clear();
        if warm {
            blocks.timestamp.collect();
        }
        // Historical compressed pages as well as the raw partial tip page.
        for end in [900_000, end] {
            for count in [1, 10, 15] {
                let begin = end - count;
                let read = |stored: bool| {
                    let from = if stored { begin } else { begin - 10 };
                    let timestamps = blocks.timestamp.collect_range_at(from, end);
                    let medians = stored.then(|| blocks.median_time.collect_range_at(begin, end));
                    let mut rows = Vec::with_capacity(count);
                    for height in (begin..end).rev() {
                        let median = if stored {
                            medians.as_ref().unwrap()[height - begin]
                        } else {
                            let mut values =
                                timestamps[height - 10 - from..=height - from].to_vec();
                            values.sort_unstable();
                            values[5]
                        };
                        rows.push((timestamps[height - from], median));
                    }
                    rows
                };
                assert_eq!(read(false), read(true));
                let mut samples = [Vec::new(), Vec::new()];
                for round in 0..14 {
                    for variant in [round % 2, 1 - round % 2] {
                        let started = Instant::now();
                        for _ in 0..1000 {
                            black_box(read(black_box(variant == 1)));
                        }
                        if round >= 4 {
                            samples[variant].push(started.elapsed() / 1000);
                        }
                    }
                }
                for sample in &mut samples {
                    sample.sort_unstable();
                }
                eprintln!(
                    "median-time reads warm={warm} end={end} count={count}: old {:?}, stored {:?}",
                    samples[0][5], samples[1][5]
                );
            }
        }
    }
}
