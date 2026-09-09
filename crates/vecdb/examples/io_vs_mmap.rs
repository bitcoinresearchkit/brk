use std::{
    env, hint,
    io::{self, Write},
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicU64, Ordering},
    },
    thread,
    time::{Duration, Instant},
};

use tempfile::tempdir;
use vecdb::{
    AnyStoredVec, AnyVec, BytesVec, Database, ImportableVec, PcoVec, ReadableVec, Version,
    WritableVec,
};

const VALUE_COUNT: usize = 10_000_000_000; // 10B u64s = 80GB
const BATCH_SIZE: usize = 100_000_000;
const SEED: u64 = 42;

const RANGE_SIZES: &[usize] = &[1_000_000, 10_000_000, 50_000_000, 100_000_000, 500_000_000];
const THREAD_COUNTS: &[usize] = &[1, 2, 4, 8];

fn repetitions(range_size: usize) -> usize {
    match range_size {
        n if n < 10_000 => 10_000,
        n if n < 1_000_000 => 1_000,
        n if n < 10_000_000 => 100,
        _ => 10,
    }
}

fn xorshift(state: &mut u64) -> u64 {
    *state ^= *state << 13;
    *state ^= *state >> 7;
    *state ^= *state << 17;
    *state
}

fn random_starts(count: usize, max_start: usize) -> Vec<usize> {
    let mut rng = SEED;
    (0..count)
        .map(|_| xorshift(&mut rng) as usize % max_start.max(1))
        .collect()
}

// --- Sequential benchmark ---

fn bench_fold<V>(vec: &V, range_size: usize, starts: &[usize]) -> Duration
where
    V: ReadableVec<usize, u64>,
{
    let reps = starts.len();
    let mut sum = 0u64;
    let start = Instant::now();
    for &s in starts {
        sum = vec.fold_range(s, s + range_size, sum, |acc, v| acc.wrapping_add(v));
    }
    let elapsed = start.elapsed();
    hint::black_box(sum);
    elapsed / reps as u32
}

// --- Parallel benchmark ---

fn bench_par_fold<V>(vec: &V, range_size: usize, starts: &[usize], threads: usize) -> Duration
where
    V: ReadableVec<usize, u64> + Sync,
{
    let reps = starts.len();
    let chunk_size = reps.div_ceil(threads);
    let sum = Arc::new(AtomicU64::new(0));

    let start = Instant::now();
    thread::scope(|s| {
        for chunk in starts.chunks(chunk_size) {
            let sum = Arc::clone(&sum);
            s.spawn(move || {
                let mut local_sum = 0u64;
                for &st in chunk {
                    local_sum = vec
                        .fold_range(st, st + range_size, local_sum, |acc, v| acc.wrapping_add(v));
                }
                sum.fetch_add(local_sum, Ordering::Relaxed);
            });
        }
    });
    let elapsed = start.elapsed();
    hint::black_box(sum.load(Ordering::Relaxed));
    elapsed / reps as u32
}

// --- Output formatting ---

fn print_header() {
    println!("{:>12} {:>10} {:>14}", "range_size", "bytes", "fold/iter");
    println!("{}", "-".repeat(40));
}

fn print_par_header() {
    print!("{:>12} {:>10}", "range_size", "bytes");
    for &t in THREAD_COUNTS {
        print!(" {:>12}", format!("{t}T"));
    }
    println!();
    println!("{}", "-".repeat(12 + 10 + THREAD_COUNTS.len() * 13));
}

fn print_row(range_size: usize, per: Duration) {
    let bytes = range_size * 8;
    println!(
        "{:>12} {:>10} {:>14}",
        range_size,
        format_bytes(bytes),
        format_duration(per),
    );
}

fn print_par_row(range_size: usize, times: &[Duration]) {
    let bytes = range_size * 8;
    print!("{:>12} {:>10}", range_size, format_bytes(bytes));
    for d in times {
        print!(" {:>12}", format_duration(*d));
    }
    println!();
}

fn format_bytes(bytes: usize) -> String {
    if bytes >= 1_000_000_000 {
        format!("{:.1} GB", bytes as f64 / 1e9)
    } else if bytes >= 1_000_000 {
        format!("{:.1} MB", bytes as f64 / 1e6)
    } else if bytes >= 1_000 {
        format!("{:.1} KB", bytes as f64 / 1e3)
    } else {
        format!("{} B", bytes)
    }
}

fn format_duration(d: Duration) -> String {
    let ns = d.as_nanos();
    if ns >= 1_000_000_000 {
        format!("{:.2} s", d.as_secs_f64())
    } else if ns >= 1_000_000 {
        format!("{:.2} ms", ns as f64 / 1e6)
    } else if ns >= 1_000 {
        format!("{:.2} us", ns as f64 / 1e3)
    } else {
        format!("{} ns", ns)
    }
}

// --- Populate ---

fn populate_bytes(dir: &Path) {
    eprint!("Populating BytesVec with {VALUE_COUNT} u64s (80 GB)...");
    flush();
    let pop_start = Instant::now();
    let db = Database::open(dir).unwrap();
    let mut vec: BytesVec<usize, u64> = BytesVec::import(&db, "bench", Version::ONE).unwrap();
    let mut written = 0;
    while written < VALUE_COUNT {
        let end = (written + BATCH_SIZE).min(VALUE_COUNT);
        for i in written..end {
            vec.push(i as u64);
        }
        vec.write().unwrap();
        written = end;
        let elapsed = pop_start.elapsed();
        let gb = written * 8 / 1_000_000_000;
        let gbs = gb as f64 / elapsed.as_secs_f64();
        eprint!(
            "\r  {:.0}% - {gb} GB - {gbs:.1} GB/s  ",
            written as f64 / VALUE_COUNT as f64 * 100.0
        );
    }
    db.flush().unwrap();
    eprintln!("\n  done ({:?})", pop_start.elapsed());
}

fn populate_pco(dir: &Path) {
    eprint!("Populating PcoVec with {VALUE_COUNT} u64s...");
    flush();
    let pop_start = Instant::now();
    let db = Database::open(dir).unwrap();
    let mut vec: PcoVec<usize, u64> = PcoVec::import(&db, "bench", Version::ONE).unwrap();
    let mut written = 0;
    while written < VALUE_COUNT {
        let end = (written + BATCH_SIZE).min(VALUE_COUNT);
        for i in written..end {
            vec.push(i as u64);
        }
        vec.write().unwrap();
        written = end;
        let elapsed = pop_start.elapsed();
        let pct = written as f64 / VALUE_COUNT as f64 * 100.0;
        eprint!("\r  {pct:.0}% ({:?})  ", elapsed);
    }
    db.flush().unwrap();
    eprintln!("\n  done ({:?})", pop_start.elapsed());
}

// --- Bench per type ---

fn bench_type<V>(vec: &V, label: &str)
where
    V: ReadableVec<usize, u64> + AnyVec + Sync,
{
    println!(
        "\n=== {label} — {} values ({} GB) ===\n",
        VALUE_COUNT,
        VALUE_COUNT * 8 / 1_000_000_000
    );

    // Sequential
    println!("--- Sequential ---");
    print_header();
    for &range_size in RANGE_SIZES {
        let reps = repetitions(range_size);
        let max_start = VALUE_COUNT.saturating_sub(range_size);
        let starts = random_starts(reps, max_start);
        let per = bench_fold(vec, range_size, &starts);
        print_row(range_size, per);
    }

    // Parallel
    println!("\n--- Parallel ---");
    print_par_header();
    for &range_size in RANGE_SIZES {
        let reps = repetitions(range_size);
        let max_start = VALUE_COUNT.saturating_sub(range_size);
        let starts = random_starts(reps, max_start);

        let times: Vec<Duration> = THREAD_COUNTS
            .iter()
            .map(|&t| bench_par_fold(vec, range_size, &starts, t))
            .collect();
        print_par_row(range_size, &times);
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let mode = args.get(1).map(|s| s.as_str()).unwrap_or("both");

    match mode {
        "bytes" => {
            let dir = tempdir().unwrap();
            populate_bytes(dir.path());
            let db = Database::open(dir.path()).unwrap();
            let vec: BytesVec<usize, u64> = BytesVec::import(&db, "bench", Version::ONE).unwrap();
            bench_type(&vec, "BytesVec<usize, u64>");
        }
        "pco" => {
            let dir = tempdir().unwrap();
            populate_pco(dir.path());
            let db = Database::open(dir.path()).unwrap();
            let vec: PcoVec<usize, u64> = PcoVec::import(&db, "bench", Version::ONE).unwrap();
            bench_type(&vec, "PcoVec<usize, u64>");
        }
        _ => {
            let bytes_dir = tempdir().unwrap();
            populate_bytes(bytes_dir.path());
            {
                let db = Database::open(bytes_dir.path()).unwrap();
                let vec: BytesVec<usize, u64> =
                    BytesVec::import(&db, "bench", Version::ONE).unwrap();
                bench_type(&vec, "BytesVec<usize, u64>");
            }
            drop(bytes_dir);

            let pco_dir = tempdir().unwrap();
            populate_pco(pco_dir.path());
            {
                let db = Database::open(pco_dir.path()).unwrap();
                let vec: PcoVec<usize, u64> = PcoVec::import(&db, "bench", Version::ONE).unwrap();
                bench_type(&vec, "PcoVec<usize, u64>");
            }
            drop(pco_dir);
        }
    }
}

fn flush() {
    Write::flush(&mut io::stdout()).ok();
}
