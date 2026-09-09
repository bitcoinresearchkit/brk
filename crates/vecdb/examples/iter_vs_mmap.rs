use std::{
    hint,
    io::{self, Write},
    time::Instant,
};

use tempfile::tempdir;
use vecdb::{AnyStoredVec, BytesVec, Database, ImportableVec, ReadableVec, Version, WritableVec};

const VALUE_COUNT: usize = 10_000_000_000; // 10B u64s = 80GB
const BATCH_SIZE: usize = 100_000_000;
const RANGE_SIZE: usize = 1_000_000;
const REPEATS: usize = 100;
const SEED: u64 = 42;

fn main() {
    let dir = tempdir().unwrap();
    let db = Database::open(dir.path()).unwrap();
    let mut vec: BytesVec<usize, u64> = BytesVec::import(&db, "bench", Version::TWO).unwrap();

    // --- Write 80GB ---
    println!(
        "Writing {VALUE_COUNT} values ({} GB)...",
        VALUE_COUNT * 8 / 1_000_000_000
    );
    let write_start = Instant::now();
    let mut written = 0usize;
    while written < VALUE_COUNT {
        let batch_end = (written + BATCH_SIZE).min(VALUE_COUNT);
        for i in written..batch_end {
            vec.push(i as u64);
        }
        vec.write().unwrap();
        written = batch_end;
        let elapsed = write_start.elapsed();
        let pct = written as f64 / VALUE_COUNT as f64 * 100.0;
        let gb = written * 8 / 1_000_000_000;
        let gbs = gb as f64 / elapsed.as_secs_f64();
        eprint!("\r  {pct:.0}% - {gb} GB - {gbs:.1} GB/s  ");
    }
    db.flush().unwrap();
    eprintln!();
    println!("Write done in {:?}\n", write_start.elapsed());

    // === Full sequential read (80GB) ===
    println!("=== Full sequential read (80 GB) ===\n");

    {
        print!("  fold_range   ...  ");
        flush();
        let start = Instant::now();
        let sum = vec.fold_range(0, VALUE_COUNT, 0u64, |acc, v: u64| acc.wrapping_add(v));
        hint::black_box(sum);
        let elapsed = start.elapsed();
        println!("{elapsed:?} ({:.2} GB/s)", 80.0 / elapsed.as_secs_f64());
    }

    // === Range reads (fixed + random-start) x 100 ===
    let fixed_from = VALUE_COUNT / 2;
    let mut rng_state: u64 = SEED;
    let random_starts: Vec<usize> = (0..REPEATS)
        .map(|_| {
            rng_state ^= rng_state << 13;
            rng_state ^= rng_state >> 7;
            rng_state ^= rng_state << 17;
            (rng_state as usize) % (VALUE_COUNT - RANGE_SIZE)
        })
        .collect();

    println!("\n=== {REPEATS}x (fixed 1M + random-start 1M) ===\n");

    {
        print!("  fold_range   ...  ");
        flush();
        let start = Instant::now();
        let mut sum = 0u64;
        (0..REPEATS).for_each(|i| {
            sum = vec.fold_range(fixed_from, fixed_from + RANGE_SIZE, sum, |acc, v: u64| {
                acc.wrapping_add(v)
            });
            let from = random_starts[i];
            sum = vec.fold_range(from, from + RANGE_SIZE, sum, |acc, v: u64| {
                acc.wrapping_add(v)
            });
        });
        hint::black_box(sum);
        let elapsed = start.elapsed();
        println!("{elapsed:?} ({:?}/iter)", elapsed / REPEATS as u32);
    }
}

fn flush() {
    Write::flush(&mut io::stdout()).ok();
}
