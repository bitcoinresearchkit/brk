//! Pure on-chain BTC/USD price oracle.
//!
//! Detects round-dollar transaction patterns ($1, $5, $10, ... $10,000) in Bitcoin
//! block outputs to derive the current price without any exchange data.

use brk_types::{Cents, Dollars, Histogram, OutputType, Sats};

mod config;

pub use config::Config;
use config::{DEFAULT_EXCLUDED_OUTPUT_TYPES, DEFAULT_MIN_SATS};

/// Oracle algorithm version. Bump on any change that alters computed prices
/// so downstream consumers can invalidate cached results.
pub const VERSION: u32 = 3;

/// Pre-oracle dollar prices, one per line, heights 0..340_000. The last entry
/// seeds the oracle's first on-chain computation at `START_HEIGHT_SLOW`.
pub const PRICES: &str = include_str!("prices.txt");

/// First height the oracle computes on-chain, with the slow cold-start EMA
/// ([`Config::slow`]). Below it, prices come from [`PRICES`].
pub const START_HEIGHT_SLOW: usize = 340_000;

/// Height where the oracle switches slow -> fast EMA ([`Config::default`]).
/// The regimes are complementary: slow resists the round-USD half-price drift
/// that locks fast below here; fast tracks the 2018-2019 crashes that lock slow.
pub const START_HEIGHT: usize = 508_000;

/// A transaction with more than this many outputs is a batch payout (exchange
/// sweep, mixer fan-out), not a round-dollar payment, so it is dropped below
/// [`MAX_OUTPUTS_UNTIL_HEIGHT`].
pub const MAX_OUTPUTS: usize = 100;

/// Height below which the [`MAX_OUTPUTS`] cap applies. The thin 2018-2020
/// signal needs batch payouts removed to stay locked onto the round-dollar
/// pattern. Above this height on-chain volume is dense enough that the cap
/// removes more genuine signal than noise, so it is lifted.
pub const MAX_OUTPUTS_UNTIL_HEIGHT: usize = 630_000;

pub const BINS_PER_DECADE: usize = 200;
const MIN_LOG_BTC: i32 = -8;
const MAX_LOG_BTC: i32 = 4;
pub const NUM_BINS: usize = BINS_PER_DECADE * (MAX_LOG_BTC - MIN_LOG_BTC) as usize;

/// Per-block round-dollar payment counts, one `u32` per log-scale bin: the
/// oracle's ring-buffer element and the `histogram/raw/*` wire payload.
pub type HistogramRaw = Histogram<u32, NUM_BINS>;

/// Smoothed EMA over the window, one `f64` per bin. The stencil search reads it,
/// never serialized (projected to [`HistogramEmaCompact`] for the wire).
pub type HistogramEma = Histogram<f64, NUM_BINS>;

/// Quantized `u16` projection of [`HistogramEma`] for the `histogram/ema/*` wire.
pub type HistogramEmaCompact = Histogram<u16, NUM_BINS>;

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

/// Number of round-USD stencil arms.
const N_ARMS: usize = STENCIL_OFFSETS.len();

/// EMA rate for the adaptive shape template (~250-block time constant), slow
/// enough that a transient octave slide can't corrupt the profile before the
/// pick recovers.
const CORR_BETA: f64 = 0.004;

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

/// Bitmask form of `DEFAULT_EXCLUDED_OUTPUT_TYPES`, evaluated at compile
/// time so `default_eligible_bin` checks membership with a single AND.
const DEFAULT_EXCLUDED_MASK: u16 = {
    let mut mask = 0u16;
    let mut i = 0;
    while i < DEFAULT_EXCLUDED_OUTPUT_TYPES.len() {
        mask |= 1u16 << DEFAULT_EXCLUDED_OUTPUT_TYPES[i] as u8;
        i += 1;
    }
    mask
};

/// Bin index for `(sats, output_type)` under `Config::default()` rules.
/// Returns `None` for excluded types (P2TR/P2WSH), dust, round-BTC values,
/// or out-of-range bins. Mirror of `Oracle::output_to_bin` for callers that
/// can pre-bin outputs at write time and don't have an `Oracle` handle.
#[inline(always)]
pub fn default_eligible_bin(sats: Sats, output_type: OutputType) -> Option<u16> {
    if DEFAULT_EXCLUDED_MASK & (1u16 << output_type as u8) != 0 {
        return None;
    }
    if *sats < DEFAULT_MIN_SATS || sats.is_common_round_value() {
        return None;
    }
    sats_to_bin(sats).map(|b| b as u16)
}

/// The single definition of the on-chain round-dollar payment filter, shared by
/// the indexer warm-up, per-request reconstruction, and the mempool's live
/// histogram so every path bins identically. Calls `emit(bin)` for each eligible
/// output, in order.
///
/// A whole transaction is dropped when it carries any OP_RETURN output (data
/// carriers like consolidations and inscriptions aren't payments and would
/// pollute the signal) or, below [`MAX_OUTPUTS_UNTIL_HEIGHT`], when it has more
/// than [`MAX_OUTPUTS`] outputs (batch payouts). `height` is the block these
/// outputs belong to. The mempool, always past the cap window, passes
/// `usize::MAX`.
#[inline]
pub fn for_each_round_dollar_bin(
    height: usize,
    outputs: impl ExactSizeIterator<Item = (Sats, OutputType)> + Clone,
    mut emit: impl FnMut(u16),
) {
    if height < MAX_OUTPUTS_UNTIL_HEIGHT && outputs.len() > MAX_OUTPUTS {
        return;
    }
    if outputs.clone().any(|(_, ty)| ty == OutputType::OpReturn) {
        return;
    }
    for (sats, ty) in outputs {
        if let Some(bin) = default_eligible_bin(sats, ty) {
            emit(bin);
        }
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

/// Raw EMA mass on each of the 19 stencil arms at `center`.
fn arms_at(ema: &HistogramEma, center: i64) -> [f64; N_ARMS] {
    let mut arms = [0.0; N_ARMS];
    for (i, &offset) in STENCIL_OFFSETS.iter().enumerate() {
        let idx = center + offset as i64;
        if idx >= 0 && (idx as usize) < NUM_BINS {
            arms[i] = ema[idx as usize];
        }
    }
    arms
}

/// [`arms_at`] L1-normalized to sum 1, or `None` when the center carries no mass.
fn normalized_arms_at(ema: &HistogramEma, center: i64) -> Option<[f64; N_ARMS]> {
    let mut arms = arms_at(ema, center);
    let sum: f64 = arms.iter().sum();
    if sum <= 0.0 {
        return None;
    }
    for arm in &mut arms {
        *arm /= sum;
    }
    Some(arms)
}

/// Shape match `1 - L1distance` between the candidate's L1-normalized arm vector
/// and the L1-normalized `profile`. 1.0 is an identical shape and it falls as
/// mass shifts off the round-USD ladder, so it pulls the pick toward the octave
/// whose payment shape looks real. Returns 0 for an empty (no-mass) center.
fn arm_profile_match(ema: &HistogramEma, center: i64, profile: &[f64; N_ARMS]) -> f64 {
    match normalized_arms_at(ema, center) {
        Some(arms) => {
            1.0 - (0..N_ARMS)
                .map(|i| (arms[i] - profile[i]).abs())
                .sum::<f64>()
        }
        None => 0.0,
    }
}

/// Scores each candidate bin in the search window by summing normalized stencil
/// matches across the EMA histogram, then refines with parabolic interpolation.
/// When `corr_weight` is non-zero the [`arm_profile_match`] shape term is added
/// to each candidate's score as an octave-discriminating restoring force.
fn find_best_bin(
    ema: &HistogramEma,
    prev_bin: f64,
    search_below: usize,
    search_above: usize,
    corr_weight: f64,
    profile: &[f64; N_ARMS],
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
        if corr_weight != 0.0 {
            total += corr_weight * arm_profile_match(ema, bin as i64, profile);
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
    let score_left = if best_bin > search_start {
        score(best_bin - 1)
    } else {
        score_center
    };
    let score_right = if best_bin + 1 < search_end {
        score(best_bin + 1)
    } else {
        score_center
    };
    let denom = score_left - 2.0 * score_center + score_right;
    let sub_bin = if denom.abs() > 1e-10 {
        (0.5 * (score_left - score_right) / denom).clamp(-0.5, 0.5)
    } else {
        0.0
    };

    best_bin as f64 + sub_bin
}

#[derive(Clone)]
pub struct Oracle {
    histograms: Vec<HistogramRaw>,
    ema: Box<HistogramEma>,
    cursor: usize,
    filled: usize,
    ref_bin: f64,
    config: Config,
    weights: Vec<f64>,
    excluded_mask: u16,
    warmup: bool,
    /// Adaptive round-USD shape template, re-estimated each non-warmup block from
    /// the arm vector at the pick. Seeded flat (every arm equal) and only
    /// read/updated when `config.corr_weight` is non-zero (the slow cold-start
    /// regime), so the EMA learns the real payment shape within a few hundred
    /// blocks without a hand-tuned starting guess biasing acquisition.
    profile: [f64; N_ARMS],
}

impl Oracle {
    pub fn new(start_bin: f64, config: Config) -> Self {
        let window_size = config.window_size;
        let decay = 1.0 - config.alpha;
        let weights: Vec<f64> = (0..window_size)
            .map(|i| config.alpha * decay.powi(i as i32))
            .collect();
        let excluded_mask = config
            .excluded_output_types
            .iter()
            .fold(0u16, |mask, ot| mask | (1 << *ot as u8));
        Self {
            histograms: vec![HistogramRaw::zeros(); window_size],
            ema: Box::new(HistogramEma::zeros()),
            cursor: 0,
            filled: 0,
            ref_bin: start_bin,
            weights,
            excluded_mask,
            warmup: false,
            config,
            profile: [1.0 / N_ARMS as f64; N_ARMS],
        }
    }

    /// Create an oracle restored from a known price. `fill` should call
    /// `process_histogram` for the warmup blocks; during warmup the ring
    /// fills without recomputing EMA or searching, then we recompute once
    /// at the end so the first non-warmup call has a primed EMA.
    pub fn from_checkpoint(ref_bin: f64, config: Config, fill: impl FnOnce(&mut Self)) -> Self {
        let mut oracle = Self::new(ref_bin, config);
        oracle.warmup = true;
        fill(&mut oracle);
        oracle.warmup = false;
        oracle.recompute_ema();
        oracle
    }

    pub fn process_histogram(&mut self, hist: &HistogramRaw) -> f64 {
        self.histograms[self.cursor] = hist.clone();
        self.cursor = (self.cursor + 1) % self.config.window_size;
        if self.filled < self.config.window_size {
            self.filled += 1;
        }

        if !self.warmup {
            self.recompute_ema();

            self.ref_bin = find_best_bin(
                &self.ema,
                self.ref_bin,
                self.config.search_below,
                self.config.search_above,
                self.config.corr_weight,
                &self.profile,
            );
            if self.config.corr_weight != 0.0 {
                self.update_profile();
            }
        }
        self.ref_bin
    }

    /// Blend the L1-normalized arm shape at the current pick into the adaptive
    /// `profile` (slow EMA, [`CORR_BETA`]). The slow rate lets the template ride
    /// through a transient octave dip without locking onto it. No-op when the
    /// pick carries no mass.
    fn update_profile(&mut self) {
        if let Some(arms) = normalized_arms_at(&self.ema, self.ref_bin.round() as i64) {
            (0..N_ARMS).for_each(|i| {
                self.profile[i] = (1.0 - CORR_BETA) * self.profile[i] + CORR_BETA * arms[i];
            });
        }
    }

    /// Switch EMA regime mid-stream (slow -> fast at [`START_HEIGHT`]) by
    /// re-warming under `config` over the most recent `config.window_size` raw
    /// histograms, so a continuous build and an incremental warm-up reach the
    /// same state; `ref_bin` carries over.
    pub fn reconfigure(&mut self, config: Config) {
        let window = self.config.window_size;
        let kept: Vec<HistogramRaw> = (0..self.filled.min(config.window_size))
            .rev()
            .map(|age| self.histograms[(self.cursor + window - 1 - age) % window].clone())
            .collect();
        *self = Self::from_checkpoint(self.ref_bin, config, |o| {
            kept.iter().for_each(|h| {
                o.process_histogram(h);
            });
        });
    }

    pub fn ref_bin(&self) -> f64 {
        self.ref_bin
    }

    /// The current weighted EMA over the window, one value per log-scale bin.
    /// `ema()[i]` is bin `i` (see `sats_to_bin`); callers transporting it
    /// round/clamp to a smaller type.
    pub fn ema(&self) -> &HistogramEma {
        &self.ema
    }

    pub fn price_cents(&self) -> Cents {
        bin_to_cents(self.ref_bin).into()
    }

    pub fn price_dollars(&self) -> Dollars {
        self.price_cents().into()
    }

    /// Config-aware bin index for `(sats, output_type)`. Returns `None`
    /// for excluded types, dust, round-BTC values, or out-of-range bins.
    /// Callers under `Config::default()` should use `default_eligible_bin`
    /// (free function) to skip the `&self` indirection.
    #[inline(always)]
    pub fn output_to_bin(&self, sats: Sats, output_type: OutputType) -> Option<usize> {
        if self.excluded_mask & (1 << output_type as u8) != 0 {
            return None;
        }
        if *sats < self.config.min_sats
            || (self.config.exclude_common_round_values && sats.is_common_round_value())
        {
            return None;
        }
        sats_to_bin(sats)
    }

    fn recompute_ema(&mut self) {
        self.ema.fill(0.0);
        for age in 0..self.filled {
            let idx = (self.cursor + self.config.window_size - 1 - age) % self.config.window_size;
            let weight = self.weights[age];
            let h = &self.histograms[idx];
            self.ema
                .iter_mut()
                .zip(h.iter())
                .for_each(|(e, &c)| *e += weight * c as f64);
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

    // reconfigure must leave the oracle in the same state as a fresh warm-up
    // over the most recent window of raw histograms; the continuous build and
    // the incremental resume rely on this agreeing at the slow -> fast seam.
    #[test]
    fn reconfigure_matches_fresh_warmup() {
        let hists: Vec<HistogramRaw> = (0..60)
            .map(|i| {
                let mut h = HistogramRaw::zeros();
                h.increment(1200 + i % 7);
                h.increment(1600 + i % 5);
                h
            })
            .collect();

        let fast = Config::default();
        let mut switched = Oracle::new(1600.0, Config::slow());
        hists.iter().for_each(|h| {
            switched.process_histogram(h);
        });
        switched.reconfigure(fast.clone());

        let keep = fast.window_size;
        let fresh = Oracle::from_checkpoint(switched.ref_bin(), fast, |o| {
            hists[hists.len() - keep..].iter().for_each(|h| {
                o.process_histogram(h);
            });
        });

        assert!(switched.ema().iter().eq(fresh.ema().iter()));
    }
}
