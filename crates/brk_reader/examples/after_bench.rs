//! End-to-end benchmark: `Reader::after` (rayon-parallel + reorder thread)
//! versus `Reader::after_canonical` (1 reader + N parser threads + canonical
//! hash filter).
//!
//! Two phases:
//!
//! 1. **Tail scenarios** — pick an anchor `N` blocks below the chain tip
//!    and run each implementation `REPEATS` times. Exercises the tail
//!    (≤10) and forward (>10) code paths under realistic catchup ranges.
//! 2. **Full reindex** — anchor=`None` (genesis to tip), one run per
//!    config. Exercises every blk file once and shows steady-state
//!    throughput on the densest possible workload.
//!
//! Run with:
//!   cargo run --release -p brk_reader --example after_bench
//!
//! Requires a running bitcoind with a cookie file at the default path.

use std::time::{Duration, Instant};

use brk_error::Result;
use brk_reader::{Reader, Receiver};
use brk_rpc::{Auth, Client};
use brk_types::{BlockHash, Height, ReadBlock};

const SCENARIOS: &[usize] = &[5, 10, 100, 1_000, 10_000];
const REPEATS: usize = 3;

fn main() -> Result<()> {
    let bitcoin_dir = Client::default_bitcoin_path();
    let client = Client::new(
        Client::default_url(),
        Auth::CookieFile(bitcoin_dir.join(".cookie")),
    )?;
    let reader = Reader::new(bitcoin_dir.join("blocks"), &client);

    let tip = client.get_last_height()?;
    println!("Tip: {tip}");
    println!();
    println!(
        "{:>10}  {:>16}  {:>12}  {:>12}  {:>10}",
        "blocks", "impl", "best", "avg", "blk/s"
    );
    println!("{}", "-".repeat(68));

    for &n in SCENARIOS {
        let anchor_height = Height::from(tip.saturating_sub(n as u32));
        let anchor_hash = client.get_block_hash(*anchor_height as u64)?;
        let anchor = Some(BlockHash::from(anchor_hash));

        let after = bench(REPEATS, || reader.after(anchor.clone()))?;
        print_row(n, "after", &after);

        let canonical_1 = bench(REPEATS, || reader.after_canonical(anchor.clone()))?;
        print_row(n, "canonical[p=1]", &canonical_1);

        let canonical_4 =
            bench(REPEATS, || reader.after_canonical_with(anchor.clone(), 4))?;
        print_row(n, "canonical[p=4]", &canonical_4);

        let canonical_16 =
            bench(REPEATS, || reader.after_canonical_with(anchor.clone(), 16))?;
        print_row(n, "canonical[p=16]", &canonical_16);

        sanity_check(n, &after, &canonical_1);
        sanity_check(n, &after, &canonical_4);
        sanity_check(n, &after, &canonical_16);
        println!();
    }

    println!();
    println!("Full reindex (genesis → tip), one run per config:");
    println!(
        "{:>10}  {:>16}  {:>12}  {:>10}",
        "blocks", "impl", "elapsed", "blk/s"
    );
    println!("{}", "-".repeat(54));

    let after_full = run_once(|| reader.after(None))?;
    print_full_row("after", &after_full);
    let p1_full = run_once(|| reader.after_canonical(None))?;
    print_full_row("canonical[p=1]", &p1_full);
    sanity_check_full(&after_full, &p1_full);
    let p4_full = run_once(|| reader.after_canonical_with(None, 4))?;
    print_full_row("canonical[p=4]", &p4_full);
    sanity_check_full(&after_full, &p4_full);
    let p16_full = run_once(|| reader.after_canonical_with(None, 16))?;
    print_full_row("canonical[p=16]", &p16_full);
    sanity_check_full(&after_full, &p16_full);

    Ok(())
}

struct RunStats {
    best: Duration,
    avg: Duration,
    count: usize,
}

fn bench<F>(repeats: usize, mut f: F) -> Result<RunStats>
where
    F: FnMut() -> Result<Receiver<ReadBlock>>,
{
    let mut best = Duration::MAX;
    let mut total = Duration::ZERO;
    let mut count = 0;

    for _ in 0..repeats {
        let start = Instant::now();
        let recv = f()?;
        let mut n = 0;
        for block in recv.iter() {
            std::hint::black_box(block.height());
            n += 1;
        }
        let elapsed = start.elapsed();
        if elapsed < best {
            best = elapsed;
        }
        total += elapsed;
        count = n;
    }

    Ok(RunStats {
        best,
        avg: total / repeats as u32,
        count,
    })
}

fn print_row(requested: usize, label: &str, s: &RunStats) {
    let blk_per_s = if s.best.is_zero() {
        0.0
    } else {
        s.count as f64 / s.best.as_secs_f64()
    };
    println!(
        "{:>10}  {:>16}  {:>12?}  {:>12?}  {:>10.0}",
        requested, label, s.best, s.avg, blk_per_s
    );
}

fn sanity_check(requested: usize, after: &RunStats, canonical: &RunStats) {
    if after.count != canonical.count {
        println!(
            "  ⚠ block count mismatch: after={} canonical={}",
            after.count, canonical.count
        );
    } else if after.count != requested {
        println!(
            "  (note: got {} blocks, requested {}; tip may have advanced)",
            after.count, requested
        );
    }
}

struct FullRun {
    elapsed: Duration,
    count: usize,
}

fn run_once<F>(mut f: F) -> Result<FullRun>
where
    F: FnMut() -> Result<Receiver<ReadBlock>>,
{
    let start = Instant::now();
    let recv = f()?;
    let mut count = 0;
    for block in recv.iter() {
        std::hint::black_box(block.height());
        count += 1;
    }
    Ok(FullRun {
        elapsed: start.elapsed(),
        count,
    })
}

fn print_full_row(label: &str, run: &FullRun) {
    let blk_per_s = if run.elapsed.is_zero() {
        0.0
    } else {
        run.count as f64 / run.elapsed.as_secs_f64()
    };
    println!(
        "{:>10}  {:>16}  {:>12?}  {:>10.0}",
        run.count, label, run.elapsed, blk_per_s
    );
}

fn sanity_check_full(after: &FullRun, canonical: &FullRun) {
    if after.count != canonical.count {
        println!(
            "  ⚠ block count mismatch vs after: {} vs {}",
            after.count, canonical.count
        );
    }
}
