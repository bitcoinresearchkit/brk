//! Generate detailed oracle accuracy report for README / documentation.
//!
//! Run with: cargo run -p brk_oracle --example report --release

use std::path::PathBuf;

use brk_indexer::Indexer;
use brk_oracle::{
    Config, NUM_BINS, Oracle, PRICES, START_HEIGHT, bin_to_cents, cents_to_bin, sats_to_bin,
};
use brk_types::{OutputType, Sats, TxIndex, TxOutIndex};
use vecdb::{AnyVec, VecIndex, VecIterator};

/// DateIndex 1 = Jan 9, 2009 (block 1). For dates after genesis week:
/// dateindex = floor(timestamp / 86400) - 14252.
const GENESIS_DAY: u32 = 14252;

const BINS_5PCT: f64 = 4.24;
const BINS_10PCT: f64 = 8.28;
const BINS_20PCT: f64 = 15.84;

fn bins_to_pct(bins: f64) -> f64 {
    (10.0_f64.powf(bins / 200.0) - 1.0) * 100.0
}

fn timestamp_to_year(ts: u32) -> u16 {
    let years_since_1970 = ts as f64 / 31557600.0;
    (1970.0 + years_since_1970) as u16
}

struct YearStats {
    year: u16,
    total_sq_err: f64,
    max_err: f64,
    total_blocks: u64,
    gt_5pct: u64,
    gt_10pct: u64,
    gt_20pct: u64,
    min_price: f64,
    max_price: f64,
    errors: Vec<f64>,
}

impl YearStats {
    fn new(year: u16) -> Self {
        Self {
            year,
            total_sq_err: 0.0,
            max_err: 0.0,
            total_blocks: 0,
            gt_5pct: 0,
            gt_10pct: 0,
            gt_20pct: 0,
            min_price: f64::MAX,
            max_price: 0.0,
            errors: Vec::new(),
        }
    }

    fn update(&mut self, err: f64, exchange_high: f64, exchange_low: f64) {
        let abs_err = err.abs();
        self.total_sq_err += err * err;
        self.total_blocks += 1;
        self.errors.push(bins_to_pct(abs_err));
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
        if exchange_high > self.max_price {
            self.max_price = exchange_high;
        }
        if exchange_low > 0.0 && exchange_low < self.min_price {
            self.min_price = exchange_low;
        }
    }

    fn rmse_pct(&self) -> f64 {
        bins_to_pct((self.total_sq_err / self.total_blocks as f64).sqrt())
    }

    fn max_pct(&self) -> f64 {
        bins_to_pct(self.max_err)
    }

    fn median_pct(&mut self) -> f64 {
        self.errors.sort_by(|a, b| a.partial_cmp(b).unwrap());
        let n = self.errors.len();
        if n == 0 { 0.0 } else { self.errors[n / 2] }
    }

    fn percentile(&self, p: f64) -> f64 {
        let n = self.errors.len();
        if n == 0 {
            return 0.0;
        }
        let idx = ((p / 100.0) * (n - 1) as f64).round() as usize;
        self.errors[idx.min(n - 1)]
    }
}

/// Oracle OHLC for a single day, built from per-block prices.
struct DayCandle {
    dateindex: usize,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
}

struct BlockError {
    height: usize,
    oracle_price: f64,
    exchange_low: f64,
    exchange_high: f64,
    error_pct: f64,
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

    let daily_ohlc: Vec<[f64; 4]> = serde_json::from_str(
        &std::fs::read_to_string(format!("{manifest_dir}/examples/date_price_ohlc.json"))
            .expect("Failed to read date_price_ohlc.json"),
    )
    .expect("Failed to parse daily OHLC");

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

    // Read block timestamps for year + dateindex mapping.
    let mut timestamp_iter = indexer.vecs.blocks.timestamp.into_iter();
    let mut height_years: Vec<u16> = Vec::with_capacity(total_heights);
    let mut height_dateindexes: Vec<usize> = Vec::with_capacity(total_heights);
    for h in 0..total_heights {
        let ts: brk_types::Timestamp = timestamp_iter.get_at_unwrap(h);
        let ts_u32 = *ts as u32;
        height_years.push(timestamp_to_year(ts_u32));
        height_dateindexes.push((ts_u32 / 86400).saturating_sub(GENESIS_DAY) as usize);
    }

    let start_price: f64 = PRICES
        .lines()
        .nth(START_HEIGHT - 1)
        .expect("prices.txt too short")
        .parse()
        .expect("Failed to parse seed price");

    let config = Config::default();
    let mut oracle = Oracle::new(cents_to_bin(start_price * 100.0), config);

    let total_txs = indexer.vecs.transactions.height.len();
    let total_outputs = indexer.vecs.outputs.value.len();

    let mut first_txindex_iter = indexer.vecs.transactions.first_txindex.into_iter();
    let mut first_txoutindex_iter = indexer.vecs.transactions.first_txoutindex.into_iter();
    let mut out_first_iter = indexer.vecs.outputs.first_txoutindex.into_iter();
    let mut value_iter = indexer.vecs.outputs.value.into_iter();
    let mut outputtype_iter = indexer.vecs.outputs.outputtype.into_iter();

    let ref_config = Config::default();

    let mut year_stats: Vec<YearStats> = Vec::new();
    let mut overall = YearStats::new(0);
    let mut worst_blocks: Vec<BlockError> = Vec::new();
    let mut total_bias = 0.0f64;

    // Track oracle daily candles.
    let mut oracle_candles: Vec<DayCandle> = Vec::new();
    let mut current_di: Option<usize> = None;

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

        let ref_bin = oracle.process_histogram(&hist);
        let oracle_price = bin_to_cents(ref_bin) as f64 / 100.0;

        // Build oracle daily candle.
        let di = height_dateindexes[h];
        if current_di != Some(di) {
            current_di = Some(di);
            oracle_candles.push(DayCandle {
                dateindex: di,
                open: oracle_price,
                high: oracle_price,
                low: oracle_price,
                close: oracle_price,
            });
        } else {
            let candle = oracle_candles.last_mut().unwrap();
            if oracle_price > candle.high {
                candle.high = oracle_price;
            }
            if oracle_price < candle.low {
                candle.low = oracle_price;
            }
            candle.close = oracle_price;
        }

        // Per-block error stats.
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

                let exchange_high = height_ohlc[h][1];
                let exchange_low = height_ohlc[h][2];

                overall.update(err, exchange_high, exchange_low);
                total_bias += err;

                let year = height_years[h];
                if year_stats.is_empty() || year_stats.last().unwrap().year != year {
                    year_stats.push(YearStats::new(year));
                }
                year_stats
                    .last_mut()
                    .unwrap()
                    .update(err, exchange_high, exchange_low);

                if err.abs() > BINS_5PCT {
                    worst_blocks.push(BlockError {
                        height: h,
                        oracle_price,
                        exchange_low,
                        exchange_high,
                        error_pct: if err < 0.0 {
                            -bins_to_pct(err.abs())
                        } else {
                            bins_to_pct(err.abs())
                        },
                    });
                }
            }
        }
    }

    worst_blocks.sort_by(|a, b| b.error_pct.abs().partial_cmp(&a.error_pct.abs()).unwrap());
    overall.errors.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // Daily candle comparison: oracle OHLC vs exchange OHLC.
    let mut daily_open_errors: Vec<f64> = Vec::new();
    let mut daily_high_errors: Vec<f64> = Vec::new();
    let mut daily_low_errors: Vec<f64> = Vec::new();
    let mut daily_close_errors: Vec<f64> = Vec::new();
    let mut daily_days = 0u64;

    for candle in &oracle_candles {
        let di = candle.dateindex;
        if di >= daily_ohlc.len() {
            continue;
        }
        let ex = &daily_ohlc[di];
        if ex[0] <= 0.0 || ex[3] <= 0.0 {
            continue;
        }
        let ex_open = ex[0];
        let ex_high = ex[1];
        let ex_low = ex[2];
        let ex_close = ex[3];

        // Error as percentage: (oracle - exchange) / exchange * 100
        daily_open_errors.push((candle.open - ex_open) / ex_open * 100.0);
        daily_high_errors.push((candle.high - ex_high) / ex_high * 100.0);
        daily_low_errors.push((candle.low - ex_low) / ex_low * 100.0);
        daily_close_errors.push((candle.close - ex_close) / ex_close * 100.0);
        daily_days += 1;
    }

    fn daily_stats(errors: &mut [f64]) -> (f64, f64, f64) {
        let n = errors.len() as f64;
        let rmse = (errors.iter().map(|e| e * e).sum::<f64>() / n).sqrt();
        errors.sort_by(|a, b| a.abs().partial_cmp(&b.abs()).unwrap());
        let max = errors.last().map(|e| e.abs()).unwrap_or(0.0);
        let median = errors[errors.len() / 2].abs();
        (median, rmse, max)
    }

    let (open_med, open_rmse, open_max) = daily_stats(&mut daily_open_errors);
    let (high_med, high_rmse, high_max) = daily_stats(&mut daily_high_errors);
    let (low_med, low_rmse, low_max) = daily_stats(&mut daily_low_errors);
    let (close_med, close_rmse, close_max) = daily_stats(&mut daily_close_errors);

    // Print report.
    println!();
    println!("  brk_oracle accuracy report");
    println!("  ══════════════════════════");
    println!();
    println!("  Config:       w12, alpha=2/7, search -9/+11, noisy/dust/round-btc filtered");
    println!(
        "  Test range:   height {} .. {} ({} blocks)",
        START_HEIGHT,
        total_heights - 1,
        overall.total_blocks
    );
    println!(
        "  Price range:  ${:.0} .. ${:.0}",
        overall.min_price, overall.max_price
    );

    println!();
    println!("  Per-block accuracy (vs per-height exchange OHLC):");
    println!("    Median:      {:.3}%", overall.percentile(50.0));
    println!("    95th pct:    {:.3}%", overall.percentile(95.0));
    println!("    99th pct:    {:.3}%", overall.percentile(99.0));
    println!("    99.9th pct:  {:.3}%", overall.percentile(99.9));
    println!("    RMSE:        {:.3}%", overall.rmse_pct());
    println!("    Max:         {:.1}%", overall.max_pct());
    println!(
        "    Bias:        {:+.2} bins",
        total_bias / overall.total_blocks as f64
    );
    println!(
        "    > 5%:        {} blocks ({:.3}%)",
        overall.gt_5pct,
        overall.gt_5pct as f64 / overall.total_blocks as f64 * 100.0
    );
    println!("    > 10%:       {} blocks", overall.gt_10pct);
    println!("    > 20%:       {} blocks", overall.gt_20pct);

    println!();
    println!(
        "  Daily candle accuracy ({} days, vs exchange daily OHLC):",
        daily_days
    );
    println!(
        "    {:>8} {:>10} {:>10} {:>10}",
        "", "Median", "RMSE", "Max"
    );
    println!(
        "    {:>8} {:>9.2}% {:>9.2}% {:>9.1}%",
        "Open", open_med, open_rmse, open_max
    );
    println!(
        "    {:>8} {:>9.2}% {:>9.2}% {:>9.1}%",
        "High", high_med, high_rmse, high_max
    );
    println!(
        "    {:>8} {:>9.2}% {:>9.2}% {:>9.1}%",
        "Low", low_med, low_rmse, low_max
    );
    println!(
        "    {:>8} {:>9.2}% {:>9.2}% {:>9.1}%",
        "Close", close_med, close_rmse, close_max
    );

    println!();
    println!("  By year:");
    println!(
        "    {:<6} {:>7} {:>9} {:>9} {:>9} {:>6} {:>5} {:>5} {:>14}",
        "Year", "Blocks", "Median", "RMSE", "Max", ">5%", ">10%", ">20%", "Price range"
    );
    println!("    {}", "-".repeat(80));
    for ys in &mut year_stats {
        let median = ys.median_pct();
        println!(
            "    {:<6} {:>7} {:>8.3}% {:>8.3}% {:>8.1}% {:>6} {:>5} {:>5}   ${:.0}..${:.0}",
            ys.year,
            ys.total_blocks,
            median,
            ys.rmse_pct(),
            ys.max_pct(),
            ys.gt_5pct,
            ys.gt_10pct,
            ys.gt_20pct,
            ys.min_price,
            ys.max_price,
        );
    }

    if !worst_blocks.is_empty() {
        println!();
        println!("  Worst blocks:");
        let show = worst_blocks.len().min(10);
        for wb in &worst_blocks[..show] {
            let dir = if wb.error_pct < 0.0 { "above" } else { "below" };
            println!(
                "    height {:>7}: oracle ${:>9.0}, exchange ${:.0}..${:.0} ({:+.1}%, {})",
                wb.height, wb.oracle_price, wb.exchange_low, wb.exchange_high, wb.error_pct, dir
            );
        }
        if worst_blocks.len() > show {
            println!("    ... and {} more", worst_blocks.len() - show);
        }
    }

    println!();
}
