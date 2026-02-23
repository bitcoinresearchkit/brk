//! Diagnostic: sweep oracle start heights and clamp-top-N strategies.
//!
//! Run with: cargo run -p brk_oracle --example noise --release

use std::path::PathBuf;
use std::time::Instant;

use brk_indexer::Indexer;
use brk_oracle::{Config, NUM_BINS, Oracle, PRICES, cents_to_bin, sats_to_bin};
use brk_types::{Sats, TxIndex, TxOutIndex};
use vecdb::{AnyVec, ReadableVec, VecIndex};

const BINS_5PCT: f64 = 4.24;
const BINS_10PCT: f64 = 8.28;
const BINS_20PCT: f64 = 15.84;
const BPD: f64 = 200.0;

fn bins_to_pct(bins: f64) -> f64 {
    (10.0_f64.powf(bins / BPD) - 1.0) * 100.0
}

fn price_seed_bin(start_height: usize) -> f64 {
    let price: f64 = PRICES
        .lines()
        .nth(start_height - 1)
        .expect("prices.txt too short")
        .parse()
        .expect("Failed to parse seed price");
    cents_to_bin(price * 100.0)
}

/// Clamp the top N bins in `src` down to the (N+1)th highest value, writing into `dst`.
fn clamp_top_n(src: &[u32; NUM_BINS], dst: &mut [u32; NUM_BINS], n: usize) {
    // Find the (n+1)th largest value.
    // Collect non-zero counts, sort descending, take the (n+1)th.
    let mut top: Vec<u32> = src.iter().copied().filter(|&v| v > 0).collect();
    top.sort_unstable_by(|a, b| b.cmp(a));
    let clamp_to = if top.len() > n { top[n] } else { 0 };

    for (i, &v) in src.iter().enumerate() {
        dst[i] = v.min(clamp_to.max(v.min(clamp_to)));
    }
}

fn main() {
    let t0 = Instant::now();

    let data_dir = std::env::var("BRK_DIR")
        .map(PathBuf::from)
        .unwrap_or_else(|_| {
            let home = std::env::var("HOME").unwrap();
            PathBuf::from(home).join(".brk")
        });

    let indexer = Indexer::forced_import(&data_dir).expect("Failed to load indexer");
    let total_heights = indexer.vecs.blocks.timestamp.len();

    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let height_ohlc: Vec<[f64; 4]> = serde_json::from_str(
        &std::fs::read_to_string(format!("{manifest_dir}/examples/height_price_ohlc.json"))
            .expect("Failed to read height_price_ohlc.json"),
    )
    .expect("Failed to parse height OHLC");

    let height_bands: Vec<(f64, f64)> = height_ohlc
        .iter()
        .map(|ohlc| {
            let high = ohlc[1];
            let low = ohlc[2];
            if high > 0.0 && low > 0.0 {
                (cents_to_bin(high * 100.0), cents_to_bin(low * 100.0))
            } else {
                (0.0, 0.0)
            }
        })
        .collect();

    // Start heights: 630k, 600k, 575k, then 570k down to 500k by 5k.
    let mut start_heights: Vec<usize> = vec![630_000, 600_000, 575_000];
    let mut h = 570_000;
    while h >= 500_000 {
        start_heights.push(h);
        h -= 5_000;
    }
    let lowest = *start_heights.iter().min().unwrap();

    // Clamp-top-N values to test: 0 (no clamp), 2, 3, 5, 10.
    let clamp_values: Vec<usize> = vec![0, 2, 3, 5, 10];

    // Build per-block RAW histograms from the lowest start height.
    eprintln!("Building histograms from height {}...", lowest);

    let total_txs = indexer.vecs.transactions.height.len();
    let total_outputs = indexer.vecs.outputs.value.len();

    let first_txoutindex_reader = indexer.vecs.transactions.first_txoutindex.reader();
    let value_reader = indexer.vecs.outputs.value.reader();
    let outputtype_reader = indexer.vecs.outputs.outputtype.reader();

    let config = Config::default();
    let total_blocks = total_heights - lowest;

    struct BlockData {
        hist: Box<[u32; NUM_BINS]>,
        high_bin: f64,
        low_bin: f64,
    }

    let mut blocks: Vec<BlockData> = Vec::with_capacity(total_blocks);

    for h in lowest..total_heights {
        let first_txindex: TxIndex = indexer
            .vecs
            .transactions
            .first_txindex
            .collect_one_at(h)
            .unwrap();
        let next_first_txindex: TxIndex = indexer
            .vecs
            .transactions
            .first_txindex
            .collect_one_at(h + 1)
            .unwrap_or(TxIndex::from(total_txs));

        let out_start = if first_txindex.to_usize() + 1 < next_first_txindex.to_usize() {
            first_txoutindex_reader
                .get(first_txindex.to_usize() + 1)
                .to_usize()
        } else {
            indexer
                .vecs
                .outputs
                .first_txoutindex
                .collect_one_at(h + 1)
                .unwrap_or(TxOutIndex::from(total_outputs))
                .to_usize()
        };
        let out_end: usize = indexer
            .vecs
            .outputs
            .first_txoutindex
            .collect_one_at(h + 1)
            .unwrap_or(TxOutIndex::from(total_outputs))
            .to_usize();

        let mut hist = Box::new([0u32; NUM_BINS]);
        for i in out_start..out_end {
            let sats: Sats = value_reader.get(i);
            let output_type = outputtype_reader.get(i);
            if config.excluded_output_types.contains(&output_type) {
                continue;
            }
            if *sats < config.min_sats {
                continue;
            }
            if config.exclude_common_round_values && sats.is_common_round_value() {
                continue;
            }
            if let Some(bin) = sats_to_bin(sats) {
                hist[bin] += 1;
            }
        }

        let (high_bin, low_bin) = if h < height_bands.len() {
            height_bands[h]
        } else {
            (0.0, 0.0)
        };

        blocks.push(BlockData {
            hist,
            high_bin,
            low_bin,
        });

        if (h - lowest).is_multiple_of(50_000) {
            eprint!(
                "\r  {}/{} ({:.0}%)",
                h - lowest,
                total_blocks,
                (h - lowest) as f64 / total_blocks as f64 * 100.0
            );
        }
    }

    eprintln!(
        "\r  {} blocks built in {:.1}s",
        blocks.len(),
        t0.elapsed().as_secs_f64()
    );

    // For each clamp value, run all start heights.
    for &clamp_n in &clamp_values {
        println!();
        let label = if clamp_n == 0 {
            "no clamp".to_string()
        } else {
            format!("clamp top {}", clamp_n)
        };
        println!("=== {} ===", label);
        println!(
            "{:>8} {:>8} {:>8} {:>8} {:>6} {:>6} {:>6} {:>8}",
            "Start", "Blocks", "RMSE%", "Worst%", ">5%", ">10%", ">20%", "Worst@"
        );
        println!("{}", "-".repeat(72));

        for &start_height in &start_heights {
            let mut oracle = Oracle::new(price_seed_bin(start_height), config.clone());
            let block_offset = start_height - lowest;

            let mut worst_err: f64 = 0.0;
            let mut worst_height: usize = 0;
            let mut gt_5: u64 = 0;
            let mut gt_10: u64 = 0;
            let mut gt_20: u64 = 0;
            let mut total_sq_err: f64 = 0.0;
            let mut total_measured: u64 = 0;

            let mut clamped_hist = [0u32; NUM_BINS];
            for (i, bd) in blocks[block_offset..].iter().enumerate() {
                if clamp_n > 0 {
                    clamp_top_n(&bd.hist, &mut clamped_hist, clamp_n);
                    oracle.process_histogram(&clamped_hist);
                } else {
                    oracle.process_histogram(&bd.hist);
                }

                let height = start_height + i;
                let ref_bin = oracle.ref_bin();

                if bd.high_bin <= 0.0 || bd.low_bin <= 0.0 {
                    continue;
                }

                let err = if ref_bin < bd.high_bin {
                    ref_bin - bd.high_bin
                } else if ref_bin > bd.low_bin {
                    ref_bin - bd.low_bin
                } else {
                    0.0
                };

                total_measured += 1;
                total_sq_err += err * err;
                let abs_err = err.abs();
                if abs_err > BINS_5PCT {
                    gt_5 += 1;
                }
                if abs_err > BINS_10PCT {
                    gt_10 += 1;
                }
                if abs_err > BINS_20PCT {
                    gt_20 += 1;
                }
                if abs_err > worst_err {
                    worst_err = abs_err;
                    worst_height = height;
                }
            }

            let rmse = if total_measured > 0 {
                bins_to_pct((total_sq_err / total_measured as f64).sqrt())
            } else {
                0.0
            };

            println!(
                "{:>8} {:>8} {:>7.3}% {:>7.1}% {:>6} {:>6} {:>6}   {}",
                format!("{}k", start_height / 1000),
                total_measured,
                rmse,
                bins_to_pct(worst_err),
                gt_5,
                gt_10,
                gt_20,
                worst_height,
            );
        }
    }

    println!("\nTotal time: {:.1}s", t0.elapsed().as_secs_f64());
}
