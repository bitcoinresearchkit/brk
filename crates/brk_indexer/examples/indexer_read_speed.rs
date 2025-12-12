use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Sats;
use std::{fs, path::Path, time::Instant};

fn run_benchmark(indexer: &Indexer) -> (Sats, std::time::Duration, usize) {
    let start = Instant::now();
    let mut sum = Sats::ZERO;
    let mut count = 0;

    for value in indexer.vecs.txout.txoutindex_to_value.clean_iter().unwrap() {
        // for value in indexer.vecs.txoutindex_to_value.values() {
        sum += value;
        count += 1;
    }

    let duration = start.elapsed();
    (sum, duration, count)
}

fn main() -> Result<()> {
    brk_logger::init(Some(Path::new(".log")))?;

    let outputs_dir = Path::new(&std::env::var("HOME").unwrap()).join(".brk");
    fs::create_dir_all(&outputs_dir)?;

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║  BRK INDEXER SEQUENTIAL READ BENCHMARK                ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!("Loading indexer from: {}", outputs_dir.display());
    let indexer = Indexer::forced_import(&outputs_dir)?;
    println!("✅ Indexer loaded\n");

    // Warmup run
    println!("🔥 Warmup run...");
    let (sum, duration, count) = run_benchmark(&indexer);
    println!("   Sum: {} sats", sum);
    println!("   Time: {:.3}s", duration.as_secs_f64());
    println!("   Count: {} values\n", count);

    // Benchmark runs
    let num_runs = 4;
    let mut throughputs: Vec<f64> = Vec::new();

    println!("📊 Running {} benchmark iterations...\n", num_runs);

    for run in 1..=num_runs {
        let (_, duration, count) = run_benchmark(&indexer);
        let estimated_bytes = count * 8;
        let throughput = estimated_bytes as f64 / duration.as_secs_f64() / (1024.0 * 1024.0);
        let values_per_sec = count as f64 / duration.as_secs_f64() / 1_000_000.0;

        throughputs.push(throughput);

        println!(
            "Run {:2}/{num_runs}: {:.3}s - {:>6.0} MB/s - {:>6.1}M values/s",
            run,
            duration.as_secs_f64(),
            throughput,
            values_per_sec
        );
    }

    // Statistics
    let mean = throughputs.iter().sum::<f64>() / throughputs.len() as f64;
    let min = throughputs.iter().cloned().fold(f64::INFINITY, f64::min);
    let max = throughputs
        .iter()
        .cloned()
        .fold(f64::NEG_INFINITY, f64::max);
    let variance =
        throughputs.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / throughputs.len() as f64;
    let std_dev = variance.sqrt();
    let cv = (std_dev / mean) * 100.0;

    println!("\n╔════════════════════════════════════════════════════════╗");
    println!("║  📊 SUMMARY STATISTICS                                ║");
    println!("╚════════════════════════════════════════════════════════╝\n");

    println!(
        "Values:      {} ({:.2} MB)",
        count,
        (count * 8) as f64 / (1024.0 * 1024.0)
    );
    println!("Mean:        {:.0} MB/s", mean);
    println!("Min:         {:.0} MB/s", min);
    println!("Max:         {:.0} MB/s", max);
    println!("StdDev:      {:.1} MB/s", std_dev);
    println!("Variance:    {:.1}%", cv);

    if cv < 2.0 {
        println!("✅ Excellent consistency");
    } else if cv < 5.0 {
        println!("✅ Good consistency");
    } else {
        println!("⚠️  High variance");
    }

    println!();
    Ok(())
}
