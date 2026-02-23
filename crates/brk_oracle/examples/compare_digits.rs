//! Compare specific digit filter configurations across multiple start heights.
//!
//! Run with: cargo run -p brk_oracle --example compare_digits --release

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

    // Configs to compare.
    //                              987654321
    let masks: &[(u16, &str)] = &[
        (0b0_0111_0111, "{1,2,3,5,6,7}"),
        (0b0_0011_0111, "{1,2,3,5,6}"),
        (0b0_0001_1111, "{1,2,3,4,5}"),
        (0b0_0001_0111, "{1,2,3,5}"),
    ];

    let start_heights: &[usize] = &[575_000, 600_000, 630_000];

    // (mask_idx, start_idx) -> (Oracle, Stats)
    let n = masks.len() * start_heights.len();
    let mut oracles: Vec<Option<Oracle>> = (0..n).map(|_| None).collect();
    let mut stats: Vec<Stats> = (0..n).map(|_| Stats::new()).collect();

    let idx = |m: usize, s: usize| -> usize { m * start_heights.len() + s };

    let total_txs = indexer.vecs.transactions.height.len();
    let total_outputs = indexer.vecs.outputs.value.len();

    let first_txindex: Vec<TxIndex> = indexer.vecs.transactions.first_txindex.collect();
    let out_first: Vec<TxOutIndex> = indexer.vecs.outputs.first_txoutindex.collect();

    let ref_config = Config::default();
    let earliest_start = *start_heights.iter().min().unwrap();

    for h in START_HEIGHT..total_heights {
        let ft = first_txindex[h];
        let next_ft = first_txindex.get(h + 1).copied().unwrap_or(TxIndex::from(total_txs));

        let out_start = if ft.to_usize() + 1 < next_ft.to_usize() {
            indexer.vecs.transactions.first_txoutindex.collect_one(ft + 1).unwrap().to_usize()
        } else {
            out_first.get(h + 1).copied().unwrap_or(TxOutIndex::from(total_outputs)).to_usize()
        };
        let out_end = out_first.get(h + 1).copied().unwrap_or(TxOutIndex::from(total_outputs)).to_usize();

        if h < earliest_start {
            continue;
        }

        let values: Vec<Sats> = indexer.vecs.outputs.value.collect_range_at(out_start, out_end);
        let output_types: Vec<OutputType> = indexer.vecs.outputs.outputtype.collect_range_at(out_start, out_end);

        // Build full histogram and per-digit histograms.
        let mut full_hist = [0u32; NUM_BINS];
        let mut digit_hist = [[0u32; NUM_BINS]; 9];

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
                        digit_hist[(d - 1) as usize][bin] += 1;
                    }
                }
            }
        }

        // Feed each (mask, start_height) combo.
        for (mi, &(mask, _)) in masks.iter().enumerate() {
            // Build filtered histogram for this mask.
            let mut hist = full_hist;
            (0..9usize).for_each(|d| {
                if mask & (1 << d) != 0 {
                    for bin in 0..NUM_BINS {
                        hist[bin] -= digit_hist[d][bin];
                    }
                }
            });

            for (si, &sh) in start_heights.iter().enumerate() {
                if h < sh {
                    continue;
                }
                let i = idx(mi, si);
                if oracles[i].is_none() {
                    oracles[i] = Some(Oracle::new(
                        seed_bin(sh),
                        Config {
                            exclude_common_round_values: false,
                            ..Default::default()
                        },
                    ));
                }

                let ref_bin = oracles[i].as_mut().unwrap().process_histogram(&hist);

                if h < height_bands.len() {
                    let (high_bin, low_bin) = height_bands[h];
                    if high_bin > 0.0 && low_bin > 0.0 {
                        let err = if ref_bin < high_bin {
                            ref_bin - high_bin
                        } else if ref_bin > low_bin {
                            ref_bin - low_bin
                        } else {
                            0.0
                        };
                        stats[i].update(err);
                    }
                }
            }
        }
    }

    // Print results grouped by start height.
    for (si, &sh) in start_heights.iter().enumerate() {
        println!();
        println!("@ {}k:", sh / 1000);
        println!(
            "  {:<16} {:>8} {:>10} {:>10} {:>6} {:>6} {:>6} {:>8}",
            "Digits", "Blocks", "RMSE%", "Max%", ">5%", ">10%", ">20%", "Bias"
        );
        println!("  {}", "-".repeat(72));
        for (mi, &(_, label)) in masks.iter().enumerate() {
            let s = &stats[idx(mi, si)];
            println!(
                "  {:<16} {:>8}   {:>7.3}%   {:>7.1}% {:>6} {:>6} {:>6} {:>+8.2}",
                label,
                s.total_blocks,
                s.rmse_pct(),
                s.max_pct(),
                s.gt_5pct,
                s.gt_10pct,
                s.gt_20pct,
                s.bias()
            );
        }
    }

    println!("\nDone in {:.1}s", t0.elapsed().as_secs_f64());
}
