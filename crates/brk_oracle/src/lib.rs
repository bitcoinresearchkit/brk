//! Pure on-chain BTC/USD price oracle.
//!
//! Detects round-dollar transaction patterns ($1, $5, $10, ... $10,000) in Bitcoin
//! block outputs to derive the current price without any exchange data.

use brk_types::{Block, CentsUnsigned, Dollars, OutputType, Sats};

/// Pre-oracle dollar prices, one per line, heights 0..630_000.
pub const PRICES: &str = include_str!("prices.txt");

/// First height where the oracle computes from on-chain data.
pub const START_HEIGHT: usize = 575_000;

pub const BINS_PER_DECADE: usize = 200;
const MIN_LOG_BTC: i32 = -8;
const MAX_LOG_BTC: i32 = 4;
pub const NUM_BINS: usize = BINS_PER_DECADE * (MAX_LOG_BTC - MIN_LOG_BTC) as usize;

/// Bin offsets for 19 round-USD amounts relative to the $100 reference (offset 0).
/// Each offset = log10(amount / 100) * BINS_PER_DECADE.
const STENCIL_OFFSETS: [i32; 19] = [
    -400, // $1
    -340, // $2
    -305, // $3
    -260, // $5
    -200, // $10
    -165, // $15
    -140, // $20
    -120, // $25
    -105, // $30
    -60,  // $50
    0,    // $100
    35,   // $150
    60,   // $200
    95,   // $300
    140,  // $500
    200,  // $1000
    260,  // $2000
    340,  // $5000
    400,  // $10000
];

/// Maps a satoshi value to its log-scale bin index.
/// bin = round(log10(sats) * BINS_PER_DECADE).
#[inline(always)]
pub fn sats_to_bin(sats: Sats) -> Option<usize> {
    if sats.is_zero() {
        return None;
    }
    let bin = ((*sats as f64).log10() * BINS_PER_DECADE as f64).round() as i64;
    if bin >= 0 && (bin as usize) < NUM_BINS {
        Some(bin as usize)
    } else {
        None
    }
}

/// Converts a fractional bin to a USD price in cents.
/// For a $D output at price P: sats = D * 1e8 / P, so P = 10^(10 - bin/200) dollars,
/// where 10 = log10($100 reference * 1e8 sats/BTC).
#[inline]
pub fn bin_to_cents(bin: f64) -> u64 {
    let dollars = 10.0_f64.powf(10.0 - bin / BINS_PER_DECADE as f64);
    (dollars * 100.0).round() as u64
}

/// Converts a USD price in cents to a fractional bin (inverse of bin_to_cents).
#[inline]
pub fn cents_to_bin(cents: f64) -> f64 {
    (10.0 - (cents / 100.0).log10()) * BINS_PER_DECADE as f64
}

/// Scores each candidate bin in the search window by summing normalized stencil
/// matches across the EMA histogram, then refines with parabolic interpolation.
fn find_best_bin(
    ema: &[f64; NUM_BINS],
    prev_bin: f64,
    search_below: usize,
    search_above: usize,
) -> f64 {
    let center = prev_bin.round() as usize;
    let search_start = center.saturating_sub(search_below);
    let search_end = (center + search_above + 1).min(NUM_BINS);

    if search_start >= search_end {
        return prev_bin;
    }

    // Per-offset peak within the search window (for normalization).
    let mut track_norm = [0.0f64; 19];
    for (i, &offset) in STENCIL_OFFSETS.iter().enumerate() {
        for bin in search_start..search_end {
            let idx = bin as i32 + offset;
            if idx >= 0 && (idx as usize) < NUM_BINS {
                track_norm[i] = track_norm[i].max(ema[idx as usize]);
            }
        }
    }

    let score = |bin: usize| -> f64 {
        let mut total = 0.0;
        for (i, &offset) in STENCIL_OFFSETS.iter().enumerate() {
            let idx = bin as i32 + offset;
            if idx >= 0 && (idx as usize) < NUM_BINS && track_norm[i] > 0.0 {
                total += ema[idx as usize] / track_norm[i];
            }
        }
        total
    };

    let mut best_bin = search_start;
    let mut best_score = score(search_start);
    for bin in (search_start + 1)..search_end {
        let candidate = score(bin);
        if candidate > best_score {
            best_score = candidate;
            best_bin = bin;
        }
    }

    // Parabolic sub-bin interpolation for fractional precision.
    let score_center = best_score;
    let score_left = if best_bin > search_start { score(best_bin - 1) } else { score_center };
    let score_right = if best_bin + 1 < search_end { score(best_bin + 1) } else { score_center };
    let denom = score_left - 2.0 * score_center + score_right;
    let sub_bin = if denom.abs() > 1e-10 {
        (0.5 * (score_left - score_right) / denom).clamp(-0.5, 0.5)
    } else {
        0.0
    };

    best_bin as f64 + sub_bin
}

pub struct Config {
    /// EMA decay: 2/(N+1) where N is span in blocks. 2/7 = 6-block span.
    pub alpha: f64,
    /// Ring buffer depth. 12 blocks for deterministic convergence at any start height.
    pub window_size: usize,
    /// Search window bins below/above previous estimate. Asymmetric for log-scale.
    pub search_below: usize,
    pub search_above: usize,
    /// Minimum output value in sats (dust filter).
    pub min_sats: u64,
    /// Exclude round BTC amounts that create false stencil matches.
    pub exclude_common_round_values: bool,
    /// Output types to ignore (e.g. P2TR, P2WSH are noisy).
    pub excluded_output_types: Vec<OutputType>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            alpha: 2.0 / 7.0,
            window_size: 12,
            search_below: 9,
            search_above: 11,
            min_sats: 1000,
            exclude_common_round_values: true,
            excluded_output_types: vec![OutputType::P2TR, OutputType::P2WSH],
        }
    }
}

pub struct Oracle {
    histograms: Vec<[u32; NUM_BINS]>,
    ema: Box<[f64; NUM_BINS]>,
    weights: Vec<f64>,
    cursor: usize,
    filled: usize,
    ref_bin: f64,
    config: Config,
}

impl Oracle {
    pub fn new(start_bin: f64, config: Config) -> Self {
        let weights: Vec<f64> = (0..config.window_size)
            .map(|age| config.alpha * (1.0 - config.alpha).powi(age as i32))
            .collect();
        let window_size = config.window_size;
        Self {
            histograms: vec![[0u32; NUM_BINS]; window_size],
            ema: Box::new([0.0; NUM_BINS]),
            weights,
            cursor: 0,
            filled: 0,
            ref_bin: start_bin,
            config,
        }
    }

    pub fn process_block(&mut self, block: &Block) -> f64 {
        self.process_outputs(
            block
                .txdata
                .iter()
                .skip(1) // skip coinbase
                .flat_map(|tx| &tx.output)
                .map(|txout| (Sats::from(txout.value), OutputType::from(&txout.script_pubkey))),
        )
    }

    pub fn process_outputs(&mut self, outputs: impl Iterator<Item = (Sats, OutputType)>) -> f64 {
        let mut hist = [0u32; NUM_BINS];
        for (sats, output_type) in outputs {
            if let Some(bin) = self.eligible_bin(sats, output_type) {
                hist[bin] += 1;
            }
        }
        self.ingest(&hist)
    }

    /// Create an oracle restored from a known price.
    /// `fill` should feed warmup blocks to populate the ring buffer.
    /// ref_bin is anchored to the checkpoint regardless of warmup drift.
    pub fn from_checkpoint(ref_bin: f64, config: Config, fill: impl FnOnce(&mut Self)) -> Self {
        let mut oracle = Self::new(ref_bin, config);
        fill(&mut oracle);
        oracle.ref_bin = ref_bin;
        oracle
    }

    pub fn process_histogram(&mut self, hist: &[u32; NUM_BINS]) -> f64 {
        self.ingest(hist)
    }

    pub fn ref_bin(&self) -> f64 {
        self.ref_bin
    }

    pub fn price_cents(&self) -> CentsUnsigned {
        bin_to_cents(self.ref_bin).into()
    }

    pub fn price_dollars(&self) -> Dollars {
        self.price_cents().into()
    }

    #[inline(always)]
    fn eligible_bin(&self, sats: Sats, output_type: OutputType) -> Option<usize> {
        if self.config.excluded_output_types.contains(&output_type) {
            return None;
        }
        if *sats < self.config.min_sats || (self.config.exclude_common_round_values && sats.is_common_round_value()) {
            return None;
        }
        sats_to_bin(sats)
    }

    fn ingest(&mut self, hist: &[u32; NUM_BINS]) -> f64 {
        self.histograms[self.cursor] = *hist;
        self.cursor = (self.cursor + 1) % self.config.window_size;
        if self.filled < self.config.window_size {
            self.filled += 1;
        }

        self.recompute_ema();

        self.ref_bin = find_best_bin(
            &self.ema,
            self.ref_bin,
            self.config.search_below,
            self.config.search_above,
        );
        self.ref_bin
    }

    fn recompute_ema(&mut self) {
        self.ema.fill(0.0);
        for age in 0..self.filled {
            let idx = (self.cursor + self.config.window_size - 1 - age) % self.config.window_size;
            let weight = self.weights[age];
            for bin in 0..NUM_BINS {
                self.ema[bin] += weight * self.histograms[idx][bin] as f64;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sats_to_bin_round_trip() {
        assert_eq!(sats_to_bin(Sats::new(100_000_000)), Some(1600));
        assert_eq!(sats_to_bin(Sats::new(1)), Some(0));
        assert_eq!(sats_to_bin(Sats::ZERO), None);
    }

    #[test]
    fn bin_to_cents_known_values() {
        assert_eq!(bin_to_cents(1600.0), 10000);
        assert_eq!(bin_to_cents(1800.0), 1000);
    }

    #[test]
    fn sats_to_bin_boundary() {
        assert_eq!(sats_to_bin(Sats::new(1_000_000_000_000)), None);
        let sats = 10.0_f64.powf(11.995) as u64;
        assert!(sats_to_bin(Sats::new(sats)).is_some());
    }

    #[test]
    fn oracle_basic() {
        let oracle = Oracle::new(1600.0, Config::default());
        assert_eq!(oracle.ref_bin(), 1600.0);
        assert_eq!(oracle.price_cents(), bin_to_cents(1600.0).into());
    }
}
