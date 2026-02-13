//! Sweep round-value tolerance to find optimal rounding threshold.
//!
//! Tests different tolerance percentages (0%, 0.01%, 0.1%, 1%, etc.) for
//! detecting round BTC amounts, combined with several digit filter masks.
//!
//! Phase 1: single pass over indexer, store per-output relative errors.
//! Phase 2: sweep tolerance × mask combos across CPU cores.
//!
//! Run with: cargo run -p brk_oracle --example sweep_tolerance --release

use std::path::PathBuf;
use std::time::Instant;

use brk_indexer::Indexer;
use brk_oracle::{Config, NUM_BINS, Oracle, PRICES, START_HEIGHT, cents_to_bin, sats_to_bin};
use brk_types::{OutputType, Sats, TxIndex, TxOutIndex};
use vecdb::{AnyVec, VecIndex, VecIterator};

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

/// Returns the relative error of `sats` from its nearest round value (d × 10^n).
/// e.g. 10_050 → leading=1, round_val=10_000, rel_err = 50/10000 = 0.005
fn relative_roundness(sats: u64) -> f64 {
    let log = (sats as f64).log10();
    let magnitude = 10.0_f64.powf(log.floor());
    let leading = (sats as f64 / magnitude).round();
    let round_val = leading * magnitude;
    (sats as f64 - round_val).abs() / round_val
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

/// Per-output data: bin index, leading digit, relative error from round value.
struct RoundOutput {
    bin: u16,
    digit: u8,
    rel_err: f32, // f32 is plenty of precision, saves memory
}

struct BlockData {
    full_hist: Box<[u32; NUM_BINS]>,
    round_outputs: Vec<RoundOutput>,
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

    // Phase 1: precompute per-block data.
    // Store all potentially-round outputs with their relative error so we can
    // filter at different tolerance thresholds in Phase 2.
    eprintln!("Phase 1: precomputing block data...");

    let total_txs = indexer.vecs.transactions.height.len();
    let total_outputs = indexer.vecs.outputs.value.len();

    let mut first_txindex_iter = indexer.vecs.transactions.first_txindex.into_iter();
    let mut first_txoutindex_iter = indexer.vecs.transactions.first_txoutindex.into_iter();
    let mut out_first_iter = indexer.vecs.outputs.first_txoutindex.into_iter();
    let mut value_iter = indexer.vecs.outputs.value.into_iter();
    let mut outputtype_iter = indexer.vecs.outputs.outputtype.into_iter();

    let ref_config = Config::default();
    let total_blocks = total_heights - sweep_start;
    let mut blocks: Vec<BlockData> = Vec::with_capacity(total_blocks);

    // Use the widest tolerance we'll test (5%) to decide what to store.
    // Outputs beyond 5% relative error will never be filtered at any tolerance.
    let max_tolerance: f64 = 0.05;

    for h in START_HEIGHT..total_heights {
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

        if h < sweep_start {
            continue;
        }

        let mut full_hist = Box::new([0u32; NUM_BINS]);
        let mut round_outputs = Vec::new();

        for i in out_start..out_end {
            let sats: Sats = value_iter.get_at_unwrap(i);
            let output_type: OutputType = outputtype_iter.get_at_unwrap(i);
            if ref_config.excluded_output_types.contains(&output_type) {
                continue;
            }
            if *sats < ref_config.min_sats {
                continue;
            }
            if let Some(bin) = sats_to_bin(sats) {
                full_hist[bin] += 1;
                let d = leading_digit(*sats);
                if (1..=9).contains(&d) {
                    let rel_err = relative_roundness(*sats);
                    if rel_err <= max_tolerance {
                        round_outputs.push(RoundOutput {
                            bin: bin as u16,
                            digit: d,
                            rel_err: rel_err as f32,
                        });
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
    let mem_rounds: usize = blocks
        .iter()
        .map(|b| b.round_outputs.len() * std::mem::size_of::<RoundOutput>())
        .sum();
    eprintln!(
        "\r  {} blocks precomputed ({:.1} GB hists + {:.0} MB rounds) in {:.1}s",
        blocks.len(),
        mem_hists as f64 / 1e9,
        mem_rounds as f64 / 1e6,
        t0.elapsed().as_secs_f64()
    );

    // Phase 2: sweep tolerance × mask combos.
    // Tolerances as fractions (not percentages).
    let tolerances: &[(f64, &str)] = &[
        (0.0, "0%"),
        (0.0001, "0.01%"),
        (0.0005, "0.05%"),
        (0.001, "0.1%"),
        (0.002, "0.2%"),
        (0.005, "0.5%"),
        (0.01, "1%"),
        (0.02, "2%"),
        (0.05, "5%"),
    ];

    //                              987654321
    let masks: &[(u16, &str)] = &[
        (0b0_0000_0000, "none"),
        (0b0_0001_0111, "{1,2,3,5}"),
        (0b0_0001_1111, "{1,2,3,4,5}"),
        (0b0_0011_0111, "{1,2,3,5,6}"),
        (0b0_0111_0111, "{1,2,3,5,6,7}"),
        (0b1_1111_1111, "{1-9}"),
    ];

    let num_configs = tolerances.len() * masks.len();
    let num_threads = std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(8);
    eprintln!(
        "Phase 2: sweeping {} configs ({} tolerances × {} masks) across {} threads...",
        num_configs,
        tolerances.len(),
        masks.len(),
        num_threads
    );

    let t1 = Instant::now();
    let blocks = &blocks;
    let tolerances_ref = tolerances;
    let masks_ref = masks;

    let all_results: Vec<(usize, usize, Stats)> = std::thread::scope(|s| {
        let configs_per_thread = num_configs.div_ceil(num_threads);

        let handles: Vec<_> = (0..num_threads)
            .map(|t| {
                s.spawn(move || {
                    let cfg_start = t * configs_per_thread;
                    let cfg_end = ((t + 1) * configs_per_thread).min(num_configs);
                    if cfg_start >= cfg_end {
                        return vec![];
                    }
                    let mut results = Vec::with_capacity(cfg_end - cfg_start);

                    for cfg_idx in cfg_start..cfg_end {
                        let ti = cfg_idx / masks_ref.len();
                        let mi = cfg_idx % masks_ref.len();
                        let (tolerance, _) = tolerances_ref[ti];
                        let (mask, _) = masks_ref[mi];

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

                            // Remove outputs matching this tolerance + mask.
                            let tol_f32 = tolerance as f32;
                            for ro in &bd.round_outputs {
                                if mask & (1 << (ro.digit - 1)) != 0
                                    && ro.rel_err <= tol_f32
                                {
                                    hist[ro.bin as usize] -= 1;
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

                        results.push((ti, mi, stats));
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

    // Print results grouped by tolerance.
    println!();
    println!(
        "{:>8} {:>16} {:>8} {:>10} {:>10} {:>6} {:>6} {:>6} {:>8}",
        "Tol", "Digits", "Blocks", "RMSE%", "Max%", ">5%", ">10%", ">20%", "Bias"
    );
    println!("{}", "-".repeat(88));

    for (ti, &(_, tol_label)) in tolerances.iter().enumerate() {
        for (mi, &(_, mask_label)) in masks.iter().enumerate() {
            let (_, _, stats) = all_results
                .iter()
                .find(|(t, m, _)| *t == ti && *m == mi)
                .unwrap();
            println!(
                "{:>8} {:>16} {:>8} {:>8.3}% {:>8.1}% {:>6} {:>6} {:>6} {:>+8.2}",
                tol_label,
                mask_label,
                stats.total_blocks,
                stats.rmse_pct(),
                stats.max_pct(),
                stats.gt_5pct,
                stats.gt_10pct,
                stats.gt_20pct,
                stats.bias()
            );
        }
        println!();
    }

    // Find overall best config by RMSE.
    let best = all_results
        .iter()
        .min_by(|a, b| a.2.rmse_pct().partial_cmp(&b.2.rmse_pct()).unwrap())
        .unwrap();
    let (bti, bmi, bs) = best;
    println!(
        "Best: tolerance={}, digits={} → RMSE {:.3}%, Max {:.1}%, >5%: {}, >10%: {}, >20%: {}",
        tolerances[*bti].1,
        masks[*bmi].1,
        bs.rmse_pct(),
        bs.max_pct(),
        bs.gt_5pct,
        bs.gt_10pct,
        bs.gt_20pct,
    );

    // Show current config for reference.
    let current = all_results
        .iter()
        .find(|(t, m, _)| {
            tolerances[*t].0 == 0.001 && masks[*m].0 == 0b0_0011_0111
        })
        .unwrap();
    let (_, _, cs) = current;
    println!(
        "Current: tolerance=0.1%, digits={{1,2,3,5,6}} → RMSE {:.3}%, Max {:.1}%, >5%: {}, >10%: {}, >20%: {}",
        cs.rmse_pct(),
        cs.max_pct(),
        cs.gt_5pct,
        cs.gt_10pct,
        cs.gt_20pct,
    );

    println!("\nTotal time: {:.1}s", t0.elapsed().as_secs_f64());
}
