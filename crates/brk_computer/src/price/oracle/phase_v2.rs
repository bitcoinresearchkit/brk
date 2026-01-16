//! Phase Oracle V2 - Round USD Template Cross-Correlation
//!
//! Detects Bitcoin prices by finding where round USD amounts ($1, $5, $10, etc.)
//! cluster in the phase histogram. Uses weekly OHLC anchors to constrain search.
//!
//! ## Algorithm
//!
//! 1. Build 200-bin phase histogram: bin = frac(log10(sats)) * 200
//! 2. Cross-correlate with weighted round USD template
//! 3. Use weekly OHLC anchor to constrain phase search range
//! 4. Return best-matching phase, convert to price
//!
//! ## Key Insight
//!
//! Round USD amounts create a fixed "fingerprint" pattern in phase space:
//! - $1, $10, $100, $1000 → phase 0.00 (weight 10)
//! - $5, $50, $500 → phase 0.70 (weight 9)
//! - $2, $20, $200 → phase 0.30 (weight 7)
//! - etc.
//!
//! The pattern shifts based on price: sats_phase = usd_phase - price_phase (mod 1)
//! Finding the shift that best matches the template reveals the price phase.

use brk_types::Sats;

/// Number of phase bins (0.5% resolution)
pub const PHASE_BINS_V2: usize = 200;

/// Round USD template: (phase, weight) pairs
/// Phase = frac(log10(usd_cents)) for round USD values
/// Weight reflects expected popularity (higher = more common)
pub const ROUND_USD_TEMPLATE: [(f64, u32); 11] = [
    (0.00, 10), // $1, $10, $100, $1000 - VERY common
    (0.18, 3),  // $1.50, $15, $150 - uncommon
    (0.30, 7),  // $2, $20, $200 - common
    (0.40, 4),  // $2.50, $25, $250 - moderate
    (0.48, 5),  // $3, $30, $300 - moderate
    (0.60, 4),  // $4, $40, $400 - moderate
    (0.70, 9),  // $5, $50, $500 - VERY common
    (0.78, 2),  // $6, $60, $600 - rare
    (0.85, 2),  // $7, $70, $700 - rare
    (0.90, 2),  // $8, $80, $800 - rare
    (0.95, 2),  // $9, $90, $900 - rare
];

/// Pre-computed template bins: (bin_index, weight)
pub fn template_bins() -> Vec<(usize, u32)> {
    ROUND_USD_TEMPLATE
        .iter()
        .map(|&(phase, weight)| {
            let bin = ((phase * PHASE_BINS_V2 as f64) as usize) % PHASE_BINS_V2;
            (bin, weight)
        })
        .collect()
}

/// Phase histogram for V2 oracle (200 bins)
#[derive(Clone)]
pub struct PhaseHistogramV2 {
    bins: [u32; PHASE_BINS_V2],
    total: u32,
}

impl Default for PhaseHistogramV2 {
    fn default() -> Self {
        Self::new()
    }
}

impl PhaseHistogramV2 {
    pub fn new() -> Self {
        Self {
            bins: [0; PHASE_BINS_V2],
            total: 0,
        }
    }

    /// Convert sats value to phase bin index
    /// Filters: min 1k sats, max 100k BTC
    #[inline]
    pub fn sats_to_bin(sats: Sats) -> Option<usize> {
        if sats < Sats::_1K || sats > Sats::_100K_BTC {
            return None;
        }
        let log_sats = f64::from(sats).log10();
        let phase = log_sats.fract();
        let phase = if phase < 0.0 { phase + 1.0 } else { phase };
        Some(((phase * PHASE_BINS_V2 as f64) as usize).min(PHASE_BINS_V2 - 1))
    }

    /// Add a sats value to the histogram
    #[inline]
    pub fn add(&mut self, sats: Sats) {
        if let Some(bin) = Self::sats_to_bin(sats) {
            self.bins[bin] = self.bins[bin].saturating_add(1);
            self.total += 1;
        }
    }

    /// Add another histogram to this one
    pub fn add_histogram(&mut self, other: &PhaseHistogramV2) {
        for (i, &count) in other.bins.iter().enumerate() {
            self.bins[i] = self.bins[i].saturating_add(count);
        }
        self.total = self.total.saturating_add(other.total);
    }

    /// Get total count
    pub fn total(&self) -> u32 {
        self.total
    }

    /// Get bins array
    pub fn bins(&self) -> &[u32; PHASE_BINS_V2] {
        &self.bins
    }

    /// Clear the histogram
    pub fn clear(&mut self) {
        self.bins.fill(0);
        self.total = 0;
    }
}

/// Find the best price phase using cross-correlation with weighted template
///
/// # Arguments
/// * `histogram` - Phase histogram to analyze
/// * `tolerance_bins` - Number of bins tolerance for template matching (e.g., 4 = ±2%)
/// * `phase_min` - Optional minimum phase from anchor (0.0-1.0)
/// * `phase_max` - Optional maximum phase from anchor (0.0-1.0)
///
/// # Returns
/// * `(best_phase, best_correlation)` - Best matching phase (0.0-1.0) and correlation score
pub fn find_best_phase(
    histogram: &PhaseHistogramV2,
    tolerance_bins: usize,
    phase_min: Option<f64>,
    phase_max: Option<f64>,
) -> (f64, u64) {
    let template = template_bins();
    let bins = histogram.bins();

    let mut best_phase = 0.0;
    let mut best_corr: u64 = 0;

    // Determine valid shifts based on anchor constraints
    let valid_shifts: Vec<usize> = if let (Some(p_min), Some(p_max)) = (phase_min, phase_max) {
        let min_bin = ((p_min * PHASE_BINS_V2 as f64) as usize) % PHASE_BINS_V2;
        let max_bin = ((p_max * PHASE_BINS_V2 as f64) as usize) % PHASE_BINS_V2;

        if min_bin <= max_bin {
            (min_bin..=max_bin).collect()
        } else {
            // Wraps around
            (min_bin..PHASE_BINS_V2)
                .chain(0..=max_bin)
                .collect()
        }
    } else {
        (0..PHASE_BINS_V2).collect()
    };

    // Cross-correlation: slide template across histogram
    for shift in valid_shifts {
        let mut corr: u64 = 0;

        for &(template_bin, weight) in &template {
            // Where would this template bin appear at this price phase shift?
            let expected_bin = (template_bin + PHASE_BINS_V2 - shift) % PHASE_BINS_V2;

            // Sum bins within tolerance, weighted
            for t in 0..=(2 * tolerance_bins) {
                let check_bin = (expected_bin + PHASE_BINS_V2 - tolerance_bins + t) % PHASE_BINS_V2;
                corr += bins[check_bin] as u64 * weight as u64;
            }
        }

        if corr > best_corr {
            best_corr = corr;
            best_phase = shift as f64 / PHASE_BINS_V2 as f64;
        }
    }

    (best_phase, best_corr)
}

/// Get phase range from price anchor (low, high)
///
/// Returns (phase_min, phase_max) with tolerance added
pub fn phase_range_from_anchor(price_low: f64, price_high: f64, tolerance_pct: f64) -> (f64, f64) {
    let low_adj = price_low * (1.0 - tolerance_pct);
    let high_adj = price_high * (1.0 + tolerance_pct);

    let phase_low = low_adj.log10().fract();
    let phase_high = high_adj.log10().fract();

    let phase_low = if phase_low < 0.0 {
        phase_low + 1.0
    } else {
        phase_low
    };
    let phase_high = if phase_high < 0.0 {
        phase_high + 1.0
    } else {
        phase_high
    };

    (phase_low, phase_high)
}

/// Convert detected phase to price using anchor for decade selection
///
/// The phase alone is ambiguous ($6.3, $63, $630, $6300 all have same phase).
/// Use the anchor price range to select the correct decade.
pub fn phase_to_price(phase: f64, anchor_low: f64, anchor_high: f64) -> f64 {
    // Base price from phase (arbitrary decade, we'll adjust)
    // phase = frac(log10(price)), so price = 10^(decade + phase)
    // Start with decade 0 (prices 1-10)
    let base_price = 10.0_f64.powf(phase);

    // Find which decade puts us in the anchor range
    let anchor_mid = (anchor_low + anchor_high) / 2.0;

    // Try decades -2 to 6 ($0.01 to $1,000,000)
    let mut best_price = base_price;
    let mut best_dist = f64::MAX;

    for decade in -2..=6 {
        let candidate = base_price * 10.0_f64.powi(decade);
        let dist = (candidate - anchor_mid).abs();
        if dist < best_dist {
            best_dist = dist;
            best_price = candidate;
        }
    }

    // Clamp to reasonable range
    best_price.clamp(0.01, 10_000_000.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_template_bins() {
        let template = template_bins();
        assert_eq!(template.len(), 11);

        // Check $1/$10/$100 maps to bin 0
        assert_eq!(template[0].0, 0);
        assert_eq!(template[0].1, 10);

        // Check $5/$50 maps to bin 140 (0.70 * 200)
        assert_eq!(template[6].0, 140);
        assert_eq!(template[6].1, 9);
    }

    #[test]
    fn test_sats_to_bin() {
        // 1 BTC = 100M sats, log10(100M) = 8.0, frac = 0.0 → bin 0
        let bin = PhaseHistogramV2::sats_to_bin(Sats::_1BTC).unwrap();
        assert_eq!(bin, 0);

        // 10M sats, log10(10M) = 7.0, frac = 0.0 → bin 0
        let bin = PhaseHistogramV2::sats_to_bin(Sats::_10M).unwrap();
        assert_eq!(bin, 0);

        // 5M sats, log10(5M) ≈ 6.699, frac ≈ 0.699 → bin ~140
        let bin = PhaseHistogramV2::sats_to_bin(Sats::from(5_000_000u64)).unwrap();
        assert!((138..=142).contains(&bin), "5M sats bin = {}", bin);
    }

    #[test]
    fn test_phase_range_from_anchor() {
        // $6000-$8000 range
        let (p_min, p_max) = phase_range_from_anchor(6000.0, 8000.0, 0.05);

        // $6000 → log10 = 3.778, phase = 0.778
        // $8000 → log10 = 3.903, phase = 0.903
        assert!(p_min > 0.7 && p_min < 0.8, "p_min = {}", p_min);
        assert!(p_max > 0.85 && p_max < 0.95, "p_max = {}", p_max);
    }

    #[test]
    fn test_phase_to_price() {
        // Phase 0.0 with anchor $50-150 should give ~$100
        let price = phase_to_price(0.0, 50.0, 150.0);
        assert!(price > 80.0 && price < 120.0, "price = {}", price);

        // Phase 0.70 with anchor $4000-6000 should give ~$5000
        let price = phase_to_price(0.70, 4000.0, 6000.0);
        assert!(price > 4000.0 && price < 6000.0, "price = {}", price);
    }
}
