//! Sweep round-value digit filter to find optimal configuration.
//!
//! Tests all 512 subsets of leading digits {1,...,9} to find which
//! digits to filter out for best oracle accuracy.
//!
//! Phase 1: single pass over indexer, precompute per-block histograms.
//! Phase 2: run 512 configs in parallel across CPU cores.
//!
//! Run with: cargo run -p brk_oracle --example sweep_digits --release

use std::path::PathBuf;
use std::time::Instant;

use brk_indexer::Indexer;
use brk_oracle::{Config, NUM_BINS, Oracle, PRICES, START_HEIGHT, cents_to_bin, sats_to_bin};
use brk_types::{OutputType, Sats, TxIndex, TxOutIndex};
use vecdb::{AnyVec, ReadableVec, VecIndex};

const BINS_5PCT: f64 = 4.24;
const BINS_10PCT: f64 = 8.28;
const BINS_20PCT: f64 = 15.84;

fn bins_to_pct(bins: f64) -> f64 {
    (10.0_f64.powf(bins / 200.0) - 1.0) * 100.0
}

fn seed_bin(start_height: usize) -> f64 {
    let price: f64 = PRICES
        .lines()
        .nth(start_height - 1)
        .expect("prices.txt too short")
        .parse()
        .expect("Failed to parse seed price");
    cents_to_bin(price * 100.0)
}

fn leading_digit(sats: u64) -> u8 {
    let log = (sats as f64).log10();
    let magnitude = 10.0_f64.powf(log.floor());
    let d = (sats as f64 / magnitude).round() as u8;
    if d >= 10 { 1 } else { d }
}

fn is_round(sats: u64) -> bool {
    let log = (sats as f64).log10();
    let magnitude = 10.0_f64.powf(log.floor());
    let leading = (sats as f64 / magnitude).round();
    let round_val = leading * magnitude;
    (sats as f64 - round_val).abs() <= round_val * 0.001
}

fn mask_label(mask: u16) -> String {
    let digits: String = (1..=9u8)
        .filter(|&d| mask & (1 << (d - 1)) != 0)
        .map(|d| char::from_digit(d as u32, 10).unwrap())
        .collect();
    if digits.is_empty() {
        "none".to_string()
    } else {
        digits
    }
}

struct Stats {
    total_sq_err: f64,
    total_bias: f64,
    max_err: f64,
    total_blocks: u64,
    gt_5pct: u64,
    gt_10pct: u64,
    gt_20pct: u64,
}

impl Stats {
    fn new() -> Self {
        Self {
            total_sq_err: 0.0,
            total_bias: 0.0,
            max_err: 0.0,
            total_blocks: 0,
            gt_5pct: 0,
            gt_10pct: 0,
            gt_20pct: 0,
        }
    }

    fn update(&mut self, err: f64) {
        self.total_sq_err += err * err;
        self.total_bias += err;
        self.total_blocks += 1;
        let abs_err = err.abs();
        if abs_err > self.max_err {
            self.max_err = abs_err;
        }
        if abs_err > BINS_5PCT {
            self.gt_5pct += 1;
        }
        if abs_err > BINS_10PCT {
            self.gt_10pct += 1;
        }
        if abs_err > BINS_20PCT {
            self.gt_20pct += 1;
        }
    }

    fn rmse_pct(&self) -> f64 {
        bins_to_pct((self.total_sq_err / self.total_blocks as f64).sqrt())
    }

    fn max_pct(&self) -> f64 {
        bins_to_pct(self.max_err)
    }

    fn bias(&self) -> f64 {
        self.total_bias / self.total_blocks as f64
    }
}

struct BlockData {
    full_hist: Box<[u32; NUM_BINS]>,
    /// (bin_index, leading_digit) for outputs that are round values.
    round_outputs: Vec<(u16, u8)>,
    high_bin: f64,
    low_bin: f64,
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

    let sweep_start: usize = 575_000;

    // Phase 1: precompute per-block data in a single pass over the indexer.
    eprintln!("Phase 1: precomputing block data...");

    let total_txs = indexer.vecs.transactions.height.len();
    let total_outputs = indexer.vecs.outputs.value.len();

    let first_txindex: Vec<TxIndex> = indexer.vecs.transactions.first_txindex.collect();
    let out_first: Vec<TxOutIndex> = indexer.vecs.outputs.first_txoutindex.collect();

    let ref_config = Config::default();
    let total_blocks = total_heights - sweep_start;
    let mut blocks: Vec<BlockData> = Vec::with_capacity(total_blocks);

    for h in START_HEIGHT..total_heights {
        let ft = first_txindex[h];
        let next_ft = first_txindex.get(h + 1).copied().unwrap_or(TxIndex::from(total_txs));

        let out_start = if ft.to_usize() + 1 < next_ft.to_usize() {
            indexer.vecs.transactions.first_txoutindex.collect_one(ft + 1).unwrap().to_usize()
        } else {
            out_first.get(h + 1).copied().unwrap_or(TxOutIndex::from(total_outputs)).to_usize()
        };
        let out_end = out_first.get(h + 1).copied().unwrap_or(TxOutIndex::from(total_outputs)).to_usize();

        if h < sweep_start {
            continue;
        }

        let values: Vec<Sats> = indexer.vecs.outputs.value.collect_range_at(out_start, out_end);
        let output_types: Vec<OutputType> = indexer.vecs.outputs.outputtype.collect_range_at(out_start, out_end);

        let mut full_hist = Box::new([0u32; NUM_BINS]);
        let mut round_outputs = Vec::new();

        for (sats, output_type) in values.into_iter().zip(output_types) {
            if ref_config.excluded_output_types.contains(&output_type) {
                continue;
            }
            if *sats < ref_config.min_sats {
                continue;
            }
            if let Some(bin) = sats_to_bin(sats) {
                full_hist[bin] += 1;
                if is_round(*sats) {
                    let d = leading_digit(*sats);
                    if (1..=9).contains(&d) {
                        round_outputs.push((bin as u16, d));
                    }
                }
            }
        }

        let (high_bin, low_bin) = if h < height_bands.len() {
            height_bands[h]
        } else {
            (0.0, 0.0)
        };

        blocks.push(BlockData {
            full_hist,
            round_outputs,
            high_bin,
            low_bin,
        });

        if (h - sweep_start).is_multiple_of(50_000) {
            eprint!(
                "\r  {}/{} ({:.0}%)",
                h - sweep_start,
                total_blocks,
                (h - sweep_start) as f64 / total_blocks as f64 * 100.0
            );
        }
    }

    let mem_hists = blocks.len() * std::mem::size_of::<[u32; NUM_BINS]>();
    let mem_rounds: usize = blocks.iter().map(|b| b.round_outputs.len() * 3).sum();
    eprintln!(
        "\r  {} blocks precomputed ({:.1} GB hists + {:.0} MB rounds) in {:.1}s",
        blocks.len(),
        mem_hists as f64 / 1e9,
        mem_rounds as f64 / 1e6,
        t0.elapsed().as_secs_f64()
    );

    // Phase 2: sweep digit masks in parallel.
    // Always filter digit 1 (powers of 10), sweep digits 2-9.
    let base_mask: u16 = 1 << 0; // digit 1 always on
    let num_masks: usize = 256; // 2^8 subsets of {2,...,9}
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(8);
    eprintln!(
        "Phase 2: sweeping {} masks across {} threads...",
        num_masks, num_threads
    );

    let t1 = Instant::now();
    let blocks = &blocks; // shared reference for threads

    let all_results: Vec<(u16, Stats)> = std::thread::scope(|s| {
        let masks_per_thread = num_masks.div_ceil(num_threads);

        let handles: Vec<_> = (0..num_threads)
            .map(|t| {
                s.spawn(move || {
                    let mask_start = t * masks_per_thread;
                    let mask_end = ((t + 1) * masks_per_thread).min(num_masks);
                    let mut results = Vec::with_capacity(mask_end - mask_start);

                    for idx in mask_start..mask_end {
                        // Shift idx bits into positions 1-8 (digits 2-9) and add base_mask (digit 1).
                        let mask = base_mask | ((idx as u16) << 1);
                        let mut oracle = Oracle::new(
                            seed_bin(sweep_start),
                            Config {
                                exclude_common_round_values: false,
                                ..Default::default()
                            },
                        );
                        let mut stats = Stats::new();

                        for bd in blocks.iter() {
                            let mut hist = *bd.full_hist;
                            for &(bin, digit) in &bd.round_outputs {
                                if mask & (1 << (digit - 1)) != 0 {
                                    hist[bin as usize] -= 1;
                                }
                            }

                            let ref_bin = oracle.process_histogram(&hist);

                            if bd.high_bin > 0.0 && bd.low_bin > 0.0 {
                                let err = if ref_bin < bd.high_bin {
                                    ref_bin - bd.high_bin
                                } else if ref_bin > bd.low_bin {
                                    ref_bin - bd.low_bin
                                } else {
                                    0.0
                                };
                                stats.update(err);
                            }
                        }

                        results.push((mask, stats));
                    }

                    results
                })
            })
            .collect();

        handles
            .into_iter()
            .flat_map(|h| h.join().unwrap())
            .collect()
    });

    eprintln!("  Done in {:.1}s.", t1.elapsed().as_secs_f64());

    // Sort by RMSE.
    let mut results: Vec<&(u16, Stats)> = all_results.iter().collect();
    results.sort_by(|a, b| a.1.rmse_pct().partial_cmp(&b.1.rmse_pct()).unwrap());

    // Print top 20.
    println!();
    println!("Top 20 (by RMSE):");
    println!(
        "{:>4} {:>12} {:>10} {:>10} {:>6} {:>6} {:>6} {:>8}",
        "#", "Digits", "RMSE%", "Max%", ">5%", ">10%", ">20%", "Bias"
    );
    println!("{}", "-".repeat(70));
    for (rank, (mask, s)) in results.iter().take(20).enumerate() {
        println!(
            "{:>4} {:>12} {:>8.3}% {:>8.1}% {:>6} {:>6} {:>6} {:>+8.2}",
            rank + 1,
            mask_label(*mask),
            s.rmse_pct(),
            s.max_pct(),
            s.gt_5pct,
            s.gt_10pct,
            s.gt_20pct,
            s.bias()
        );
    }

    // Print bottom 5.
    println!();
    println!("Bottom 5 (worst):");
    println!(
        "{:>4} {:>12} {:>10} {:>10} {:>6} {:>6} {:>6} {:>8}",
        "#", "Digits", "RMSE%", "Max%", ">5%", ">10%", ">20%", "Bias"
    );
    println!("{}", "-".repeat(70));
    for (mask, s) in results.iter().rev().take(5) {
        println!(
            "{:>4} {:>12} {:>8.3}% {:>8.1}% {:>6} {:>6} {:>6} {:>+8.2}",
            "",
            mask_label(*mask),
            s.rmse_pct(),
            s.max_pct(),
            s.gt_5pct,
            s.gt_10pct,
            s.gt_20pct,
            s.bias()
        );
    }

    // Print current config {1,2,3,5} for reference.
    let current_mask: u16 = (1 << 0) | (1 << 1) | (1 << 2) | (1 << 4); // digits 1,2,3,5
    let current_stats = all_results
        .iter()
        .find(|(m, _)| *m == current_mask)
        .map(|(_, s)| s)
        .unwrap();
    let current_rank = results
        .iter()
        .position(|(m, _)| *m == current_mask)
        .unwrap();
    println!();
    println!(
        "Current {{1,2,3,5}} = rank {}/{}: RMSE {:.3}%, Max {:.1}%, >5%: {}, >10%: {}, >20%: {}",
        current_rank + 1,
        num_masks,
        current_stats.rmse_pct(),
        current_stats.max_pct(),
        current_stats.gt_5pct,
        current_stats.gt_10pct,
        current_stats.gt_20pct,
    );

    println!("\nTotal time: {:.1}s", t0.elapsed().as_secs_f64());
}
