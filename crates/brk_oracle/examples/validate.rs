//! Validate oracle accuracy against exchange reference prices.
//!
//! Run with: cargo run -p brk_oracle --example validate --release
//!
//! Requires:
//! - ~/.brk indexed blockchain data (brk_indexer)
//! - examples/height_price_ohlc.json (per-height [open, high, low, close] in dollars)

use std::path::PathBuf;

use brk_indexer::Indexer;
use brk_oracle::{cents_to_bin, sats_to_bin, Config, Oracle, NUM_BINS, PRICES, START_HEIGHT};
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

struct Run {
    label: &'static str,
    start_height: usize,
    oracle: Option<Oracle>,
    stats: Stats,
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

    let manifest_dir = env!("CARGO_MANIFEST_DIR");

    let height_ohlc: Vec<[f64; 4]> = serde_json::from_str(
        &std::fs::read_to_string(format!("{manifest_dir}/examples/height_price_ohlc.json"))
            .expect("Failed to read height_price_ohlc.json"),
    )
    .expect("Failed to parse height OHLC");

    // Pre-compute per-height (high_bin, low_bin) tolerance band.
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

    let mut runs = vec![
        Run { label: "w12 @ 575k", start_height: 575_000, oracle: None, stats: Stats::new() },
        Run { label: "w12 @ 600k", start_height: 600_000, oracle: None, stats: Stats::new() },
        Run { label: "w12 @ 630k", start_height: 630_000, oracle: None, stats: Stats::new() },
    ];

    // Build per-block filtered histograms from the indexer, feeding all oracles in one pass.
    let total_txs = indexer.vecs.transactions.height.len();
    let total_outputs = indexer.vecs.outputs.value.len();

    // Pre-collect height-indexed vecs (small). Transaction-indexed vecs are too large.
    let first_txindex: Vec<TxIndex> = indexer.vecs.transactions.first_txindex.collect();
    let out_first: Vec<TxOutIndex> = indexer.vecs.outputs.first_txoutindex.collect();

    let ref_config = Config::default();

    for h in START_HEIGHT..total_heights {
        let ft = first_txindex[h];
        let next_ft = first_txindex.get(h + 1).copied().unwrap_or(TxIndex::from(total_txs));

        let out_start = if ft.to_usize() + 1 < next_ft.to_usize() {
            indexer.vecs.transactions.first_txoutindex.collect_one(ft + 1).unwrap().to_usize()
        } else {
            out_first.get(h + 1).copied().unwrap_or(TxOutIndex::from(total_outputs)).to_usize()
        };
        let out_end = out_first.get(h + 1).copied().unwrap_or(TxOutIndex::from(total_outputs)).to_usize();

        // Build filtered histogram once for all oracles.
        let values: Vec<Sats> = indexer.vecs.outputs.value.collect_range_at(out_start, out_end);
        let output_types: Vec<OutputType> = indexer.vecs.outputs.outputtype.collect_range_at(out_start, out_end);

        let mut hist = [0u32; NUM_BINS];
        for (sats, output_type) in values.into_iter().zip(output_types) {
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

        for run in &mut runs {
            if h < run.start_height {
                continue;
            }
            if run.oracle.is_none() {
                let config = Config::default();
                run.oracle = Some(Oracle::new(seed_bin(run.start_height), config));
            }
            let ref_bin = run.oracle.as_mut().unwrap().process_histogram(&hist);

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
                    run.stats.update(err);
                }
            }
        }
    }

    // Print results.
    println!();
    println!(
        "{:<14} {:>8} {:>10} {:>10} {:>6} {:>6} {:>6} {:>8}",
        "Config", "Blocks", "RMSE%", "Max%", ">5%", ">10%", ">20%", "Bias"
    );
    println!("{}", "-".repeat(72));
    for run in &runs {
        let s = &run.stats;
        println!(
            "{:<14} {:>8}   {:>7.2}%   {:>7.1}% {:>6} {:>6} {:>6} {:>+8.2}",
            run.label,
            s.total_blocks,
            s.rmse_pct(),
            s.max_pct(),
            s.gt_5pct,
            s.gt_10pct,
            s.gt_20pct,
            s.bias()
        );
    }
    println!();

    // Verify exact counts against reference.
    // Reference: trunc w12 @ 575k: 261 >5%, 40 >10%, 0 >20%
    //            trunc w12 @ 600k: 174 >5%, 31 >10%, 0 >20%
    //            trunc w12 @ 630k:  84 >5%,  9 >10%, 0 >20%
    let expected: &[(&str, u64, u64, u64)] = &[
        ("w12 @ 575k", 237, 22, 0),
        ("w12 @ 600k", 152, 15, 0),
        ("w12 @ 630k", 84, 9, 0),
    ];

    for (run, &(label, exp_5, exp_10, exp_20)) in runs.iter().zip(expected) {
        let s = &run.stats;
        assert_eq!(s.gt_20pct, exp_20, "{label}: expected {exp_20} blocks >20%, got {}", s.gt_20pct);
        assert_eq!(s.gt_10pct, exp_10, "{label}: expected {exp_10} blocks >10%, got {}", s.gt_10pct);
        assert_eq!(s.gt_5pct, exp_5, "{label}: expected {exp_5} blocks >5%, got {}", s.gt_5pct);
    }

    println!("All assertions passed!");
}
