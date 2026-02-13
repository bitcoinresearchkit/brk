//! Verify oracle determinism: oracles started from different heights converge
//! to identical ref_bin values after the ring buffer fills.
//!
//! Creates a reference oracle at height 575k and test oracles every 1000 blocks
//! up to 630k. After window_size blocks, each test oracle should produce the
//! same ref_bin as the reference, proving the truncated EMA provides
//! start-point independence.
//!
//! Run with: cargo run -p brk_oracle --example determinism --release

use std::path::PathBuf;

use brk_indexer::Indexer;
use brk_oracle::{Config, NUM_BINS, Oracle, PRICES, START_HEIGHT, cents_to_bin, sats_to_bin};
use brk_types::{OutputType, Sats, TxIndex, TxOutIndex};
use vecdb::{AnyVec, VecIndex, VecIterator};

fn seed_bin(height: usize) -> f64 {
    let price: f64 = PRICES
        .lines()
        .nth(height - 1)
        .expect("prices.txt too short")
        .parse()
        .expect("Failed to parse seed price");
    cents_to_bin(price * 100.0)
}

struct TestRun {
    start_height: usize,
    oracle: Option<Oracle>,
    converged_at: Option<usize>,
    diverged_after: bool,
}

fn main() {
    let data_dir = std::env::var("BRK_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap();
            PathBuf::from(home).join(".brk")
        });

    let indexer = Indexer::forced_import(&data_dir).expect("Failed to load indexer");
    let total_heights = indexer.vecs.blocks.timestamp.len();

    let config = Config::default();
    let window_size = config.window_size;

    let total_txs = indexer.vecs.transactions.height.len();
    let total_outputs = indexer.vecs.outputs.value.len();

    let mut first_txindex_iter = indexer.vecs.transactions.first_txindex.into_iter();
    let mut first_txoutindex_iter = indexer.vecs.transactions.first_txoutindex.into_iter();
    let mut out_first_iter = indexer.vecs.outputs.first_txoutindex.into_iter();
    let mut value_iter = indexer.vecs.outputs.value.into_iter();
    let mut outputtype_iter = indexer.vecs.outputs.outputtype.into_iter();

    let ref_config = Config::default();

    // Reference oracle at 575k.
    let ref_start = START_HEIGHT;
    let mut ref_oracle = Oracle::new(seed_bin(ref_start), Config::default());

    // Test oracles every 1000 blocks from 576k to 630k.
    let mut runs: Vec<TestRun> = (576_000..=630_000)
        .step_by(1000)
        .map(|h| TestRun {
            start_height: h,
            oracle: None,
            converged_at: None,
            diverged_after: false,
        })
        .collect();

    let last_start = runs.last().map(|r| r.start_height).unwrap_or(ref_start);
    // Process enough blocks for all oracles to converge + verification margin.
    let end_height = (last_start + window_size + 100).min(total_heights);

    for h in START_HEIGHT..end_height {
        let first_txindex: TxIndex = first_txindex_iter.get_at_unwrap(h);
        let next_first_txindex = first_txindex_iter
            .get_at(h + 1)
            .unwrap_or(TxIndex::from(total_txs));

        let out_start = if first_txindex.to_usize() + 1 < next_first_txindex.to_usize() {
            first_txoutindex_iter
                .get_at_unwrap(first_txindex.to_usize() + 1)
                .to_usize()
        } else {
            out_first_iter
                .get_at(h + 1)
                .unwrap_or(TxOutIndex::from(total_outputs))
                .to_usize()
        };
        let out_end = out_first_iter
            .get_at(h + 1)
            .unwrap_or(TxOutIndex::from(total_outputs))
            .to_usize();

        let mut hist = [0u32; NUM_BINS];
        for i in out_start..out_end {
            let sats: Sats = value_iter.get_at_unwrap(i);
            let output_type: OutputType = outputtype_iter.get_at_unwrap(i);
            if ref_config.excluded_output_types.contains(&output_type) {
                continue;
            }
            if *sats < ref_config.min_sats
                || (ref_config.exclude_common_round_values && sats.is_common_round_value())
            {
                continue;
            }
            if let Some(bin) = sats_to_bin(sats) {
                hist[bin] += 1;
            }
        }

        let ref_bin = ref_oracle.process_histogram(&hist);

        for run in &mut runs {
            if h < run.start_height {
                continue;
            }
            if run.oracle.is_none() {
                run.oracle = Some(Oracle::new(seed_bin(run.start_height), Config::default()));
            }
            let test_bin = run.oracle.as_mut().unwrap().process_histogram(&hist);

            if run.converged_at.is_some() {
                if test_bin != ref_bin {
                    run.diverged_after = true;
                }
            } else if test_bin == ref_bin {
                run.converged_at = Some(h);
            }
        }
    }

    // Print results.
    println!();
    println!(
        "{:<12} {:>16} {:>8}",
        "Start", "Converged at", "Blocks"
    );
    println!("{}", "-".repeat(40));

    let mut max_blocks = 0usize;
    let mut failed = Vec::new();
    let mut diverged = Vec::new();

    for run in &runs {
        if let Some(converged) = run.converged_at {
            let blocks = converged - run.start_height;
            if blocks > max_blocks {
                max_blocks = blocks;
            }
            println!(
                "{:<12} {:>16} {:>8}",
                run.start_height, converged, blocks
            );
            if run.diverged_after {
                diverged.push(run.start_height);
            }
        } else {
            println!("{:<12} {:>16} {:>8}", run.start_height, "NEVER", "-");
            failed.push(run.start_height);
        }
    }

    println!();
    println!(
        "{}/{} converged, max {} blocks to converge (window_size={})",
        runs.len() - failed.len(),
        runs.len(),
        max_blocks,
        window_size,
    );

    if !diverged.is_empty() {
        println!("DIVERGED after convergence: {:?}", diverged);
    }
    if !failed.is_empty() {
        println!("NEVER converged: {:?}", failed);
    }

    // Assertions.
    assert!(
        failed.is_empty(),
        "{} oracles never converged: {:?}",
        failed.len(),
        failed
    );
    assert!(
        diverged.is_empty(),
        "{} oracles diverged after convergence: {:?}",
        diverged.len(),
        diverged
    );
    assert!(
        max_blocks <= window_size * 2,
        "Convergence took {} blocks, expected <= {} (2 * window_size)",
        max_blocks,
        window_size * 2
    );

    println!();
    println!("All assertions passed!");
}
