//! Verify the production restart property: an oracle restored via
//! `from_checkpoint` (seeded from the previous block's stored cents price,
//! replayed over the last `window_size` blocks) produces bit-exact `ref_bin`
//! values matching a continuously-running oracle from the restart height
//! onward.
//!
//! Mirrors the production filter exactly (per-tx OP_RETURN drop + per-output
//! `default_eligible_bin`), so it exercises the same code path
//! `brk_computer::prices::compute::feed_blocks` uses at runtime.
//!
//! Run with: cargo run -p brk_oracle --example determinism --release

use std::path::PathBuf;

use brk_indexer::Indexer;
use brk_oracle::{
    Config, Histogram, Oracle, PRICES, START_HEIGHT, bin_to_cents, cents_to_bin,
    default_eligible_bin,
};
use brk_types::{OutputType, Sats, TxIndex, TxOutIndex};
use vecdb::{AnyVec, ReadableVec, VecIndex};

fn seed_bin_for_start_height() -> f64 {
    let price: f64 = PRICES
        .lines()
        .nth(START_HEIGHT - 1)
        .expect("prices.txt too short for START_HEIGHT")
        .parse()
        .expect("Failed to parse seed price");
    cents_to_bin(price * 100.0)
}

struct Block {
    values: Vec<Sats>,
    output_types: Vec<OutputType>,
    tx_starts: Vec<usize>,
    out_start: usize,
    out_end: usize,
}

fn build_histogram(block: &Block) -> Histogram {
    let mut hist = Histogram::zeros();
    for tx in 0..block.tx_starts.len() {
        let lo = block.tx_starts[tx] - block.out_start;
        let hi = block
            .tx_starts
            .get(tx + 1)
            .map(|s| s - block.out_start)
            .unwrap_or(block.out_end - block.out_start);
        if block.output_types[lo..hi].contains(&OutputType::OpReturn) {
            continue;
        }
        for i in lo..hi {
            if let Some(bin) = default_eligible_bin(block.values[i], block.output_types[i]) {
                hist.increment(bin as usize);
            }
        }
    }
    hist
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

    let restart_offset = 1000;
    let end_offset = restart_offset + window_size * 4;
    let end_height = (START_HEIGHT + end_offset).min(total_heights);
    let restart_at = START_HEIGHT + restart_offset;
    let warmup_start = restart_at - window_size;

    assert!(
        end_height > restart_at,
        "indexer has {total_heights} blocks; need at least {} to test restart at {restart_at}",
        restart_at + 1
    );

    println!(
        "Loading {} blocks ({START_HEIGHT}..{end_height})...",
        end_height - START_HEIGHT
    );
    let total_txs = indexer.vecs.transactions.txid.len();
    let total_outputs = indexer.vecs.outputs.value.len();
    let first_tx_index: Vec<TxIndex> = indexer.vecs.transactions.first_tx_index.collect();
    let out_first: Vec<TxOutIndex> = indexer.vecs.outputs.first_txout_index.collect();
    let mut txout_cursor = indexer.vecs.transactions.first_txout_index.cursor();

    let mut blocks: Vec<Block> = Vec::with_capacity(end_height - START_HEIGHT);
    for h in START_HEIGHT..end_height {
        let ft = first_tx_index[h];
        let next_ft = first_tx_index
            .get(h + 1)
            .copied()
            .unwrap_or(TxIndex::from(total_txs));
        let block_first_tx = ft.to_usize() + 1;
        let tx_count = next_ft.to_usize() - block_first_tx;
        let out_end = out_first
            .get(h + 1)
            .copied()
            .unwrap_or(TxOutIndex::from(total_outputs))
            .to_usize();

        txout_cursor.advance(block_first_tx - txout_cursor.position());
        let mut tx_starts: Vec<usize> = Vec::with_capacity(tx_count);
        for _ in 0..tx_count {
            tx_starts.push(txout_cursor.next().unwrap().to_usize());
        }
        let out_start = tx_starts.first().copied().unwrap_or(out_end);

        let values: Vec<Sats> = indexer
            .vecs
            .outputs
            .value
            .collect_range_at(out_start, out_end);
        let output_types: Vec<OutputType> = indexer
            .vecs
            .outputs
            .output_type
            .collect_range_at(out_start, out_end);

        blocks.push(Block {
            values,
            output_types,
            tx_starts,
            out_start,
            out_end,
        });
    }

    let mut continuous = Oracle::new(seed_bin_for_start_height(), config.clone());
    let continuous_bins: Vec<f64> = blocks
        .iter()
        .map(|b| continuous.process_histogram(&build_histogram(b)))
        .collect();
    println!("Continuous oracle: {} blocks processed", continuous_bins.len());

    let prev_bin = continuous_bins[restart_at - START_HEIGHT - 1];
    let seed_bin = cents_to_bin(bin_to_cents(prev_bin) as f64);
    println!(
        "Restart at {restart_at}: prev_bin={prev_bin:.4} -> cents -> seed_bin={seed_bin:.4} (delta {:.6})",
        seed_bin - prev_bin
    );

    let warmup_slice = &blocks[warmup_start - START_HEIGHT..restart_at - START_HEIGHT];
    let mut restored = Oracle::from_checkpoint(seed_bin, config.clone(), |o| {
        for b in warmup_slice {
            o.process_histogram(&build_histogram(b));
        }
    });

    let restored_bins: Vec<f64> = blocks[restart_at - START_HEIGHT..]
        .iter()
        .map(|b| restored.process_histogram(&build_histogram(b)))
        .collect();
    println!("Restored oracle: {} blocks processed", restored_bins.len());

    let mut mismatches: Vec<(usize, f64, f64)> = Vec::new();
    for (i, &r) in restored_bins.iter().enumerate() {
        let c = continuous_bins[restart_at - START_HEIGHT + i];
        if r != c {
            mismatches.push((restart_at + i, c, r));
        }
    }

    println!();
    if mismatches.is_empty() {
        println!(
            "All {} blocks from {restart_at} onward match exactly.",
            restored_bins.len()
        );
    } else {
        println!(
            "{} of {} blocks differ (showing up to 5):",
            mismatches.len(),
            restored_bins.len()
        );
        for (h, c, r) in mismatches.iter().take(5) {
            println!(
                "  h={h}: continuous={c:.6}, restored={r:.6}, delta={:.6}",
                r - c
            );
        }
    }

    assert_eq!(
        mismatches.len(),
        0,
        "restored oracle diverged from continuous oracle"
    );

    println!();
    println!("Assertion passed: from_checkpoint restart is bit-exact.");
}
