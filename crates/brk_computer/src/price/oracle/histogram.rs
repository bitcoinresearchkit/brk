//! Log-scale histogram for UTXOracle price detection.
//! Bins output values on a logarithmic scale to detect periodic patterns
//! From round USD amounts.

use brk_types::Sats;

/// Histogram configuration constants
pub const BINS_PER_DECADE: usize = 200;
pub const MIN_LOG_BTC: f64 = -6.0; // 10^-6 BTC = 100 sats
pub const MAX_LOG_BTC: f64 = 2.0; // 10^2 BTC = 100 BTC
pub const NUM_DECADES: usize = 8; // -6 to +2
pub const TOTAL_BINS: usize = NUM_DECADES * BINS_PER_DECADE; // 1600 bins

/// Minimum output value to consider (~1,000 sats = 0.00001 BTC)
/// Matches Python: zeros bins 0-200 which is 10^-5 BTC
pub const MIN_OUTPUT_SATS: Sats = Sats::_1K;
/// Maximum output value to consider (100 BTC)
/// Matches Python: zeros bins 1601+ which is ~10^2 BTC
pub const MAX_OUTPUT_SATS: Sats = Sats::_100BTC;

/// Round BTC bin indices that should be smoothed to avoid false positives
/// These are bins where round BTC amounts would naturally cluster
const ROUND_BTC_BINS: &[usize] = &[
    201,  // 1k sats (0.00001 BTC)
    401,  // 10k sats (0.0001 BTC)
    461,  // 20k sats
    496,  // 30k sats
    540,  // 50k sats
    601,  // 100k sats (0.001 BTC)
    661,  // 200k sats
    696,  // 300k sats
    740,  // 500k sats
    801,  // 0.01 BTC
    861,  // 0.02 BTC
    896,  // 0.03 BTC
    940,  // 0.04 BTC
    1001, // 0.1 BTC
    1061, // 0.2 BTC
    1096, // 0.3 BTC
    1140, // 0.5 BTC
    1201, // 1 BTC
];

/// Log-scale histogram for output values
#[derive(Clone)]
pub struct Histogram {
    bins: [f64; TOTAL_BINS],
    count: usize,
    /// Running sum of all bin values (tracked incrementally for fast normalize)
    sum: f64,
}

impl Default for Histogram {
    fn default() -> Self {
        Self::new()
    }
}

impl Histogram {
    /// Create a new empty histogram
    pub fn new() -> Self {
        Self {
            bins: [0.0; TOTAL_BINS],
            count: 0,
            sum: 0.0,
        }
    }

    /// Reset the histogram to empty
    #[allow(dead_code)] // Utility for reusing histograms
    pub fn clear(&mut self) {
        self.bins.fill(0.0);
        self.count = 0;
        self.sum = 0.0;
    }

    /// Get the number of samples added
    #[allow(dead_code)] // For v2 confidence scoring
    pub fn count(&self) -> usize {
        self.count
    }

    /// Get the bins array
    pub fn bins(&self) -> &[f64; TOTAL_BINS] {
        &self.bins
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Private helpers for bin operations that maintain sum invariant
    // ─────────────────────────────────────────────────────────────────────────

    /// Add value to a bin, maintaining sum invariant
    #[inline]
    fn bin_add(&mut self, bin: usize, value: f64) {
        self.bins[bin] += value;
        self.sum += value;
    }

    /// Set a bin to a new value, maintaining sum invariant
    #[inline]
    fn bin_set(&mut self, bin: usize, new_value: f64) {
        let old_value = self.bins[bin];
        self.bins[bin] = new_value;
        self.sum += new_value - old_value;
    }

    /// Subtract from a bin (clamped to 0), maintaining sum invariant
    /// Returns the actual amount subtracted
    #[inline]
    fn bin_sub_clamped(&mut self, bin: usize, value: f64) -> f64 {
        let old_value = self.bins[bin];
        let new_value = (old_value - value).max(0.0);
        self.bins[bin] = new_value;
        let removed = old_value - new_value;
        self.sum -= removed;
        removed
    }

    // ─────────────────────────────────────────────────────────────────────────

    /// Convert satoshi value to bin index
    /// Returns None if value is outside the histogram range
    #[inline]
    pub fn sats_to_bin(sats: Sats) -> Option<usize> {
        if sats < MIN_OUTPUT_SATS || sats > MAX_OUTPUT_SATS {
            return None;
        }

        // Convert sats to BTC (log scale)
        let btc = f64::from(sats) / f64::from(Sats::ONE_BTC);
        let log_btc = btc.log10();

        // Map to bin index: log_btc in [-6, 2] -> bin in [0, 1600)
        let normalized = (log_btc - MIN_LOG_BTC) / (MAX_LOG_BTC - MIN_LOG_BTC);
        let bin = (normalized * TOTAL_BINS as f64) as usize;

        if bin < TOTAL_BINS { Some(bin) } else { None }
    }

    /// Convert bin index to approximate satoshi value
    #[allow(dead_code)] // Inverse of sats_to_bin, useful for debugging
    #[inline]
    pub fn bin_to_sats(bin: usize) -> Sats {
        let normalized = bin as f64 / TOTAL_BINS as f64;
        let log_btc = MIN_LOG_BTC + normalized * (MAX_LOG_BTC - MIN_LOG_BTC);
        let btc = 10_f64.powf(log_btc);
        Sats::from((btc * f64::from(Sats::ONE_BTC)) as u64)
    }

    /// Add a value to the histogram with the given weight
    #[allow(dead_code)] // Used in tests and non-sparse paths
    #[inline]
    pub fn add(&mut self, sats: Sats, weight: f64) {
        if let Some(bin) = Self::sats_to_bin(sats) {
            self.bin_add(bin, weight);
            self.count += 1;
        }
    }

    /// Add another histogram to this one
    #[allow(dead_code)] // Non-sparse alternative
    pub fn add_histogram(&mut self, other: &Histogram) {
        for (i, &v) in other.bins.iter().enumerate() {
            if v > 0.0 {
                self.bin_add(i, v);
            }
        }
        self.count += other.count;
    }

    /// Subtract another histogram from this one
    /// Clamps bins to >= 0 to handle floating-point precision issues
    #[allow(dead_code)] // Non-sparse alternative
    pub fn subtract_histogram(&mut self, other: &Histogram) {
        for (i, &v) in other.bins.iter().enumerate() {
            if v > 0.0 {
                self.bin_sub_clamped(i, v);
            }
        }
        self.count = self.count.saturating_sub(other.count);
    }

    /// Add sparse entries to this histogram (O(entries) instead of O(1600))
    #[inline]
    pub fn add_sparse(&mut self, entries: &[(u16, f64)]) {
        for &(bin, value) in entries {
            self.bin_add(bin as usize, value);
        }
        self.count += entries.len();
    }

    /// Subtract sparse entries from this histogram (O(entries) instead of O(1600))
    #[inline]
    pub fn subtract_sparse(&mut self, entries: &[(u16, f64)]) {
        for &(bin, value) in entries {
            self.bin_sub_clamped(bin as usize, value);
        }
        self.count = self.count.saturating_sub(entries.len());
    }

    /// Add a value and return the bin index (for sparse collection)
    #[allow(dead_code)] // Alternative API for hybrid approaches
    #[inline]
    pub fn add_and_get_bin(&mut self, sats: Sats, weight: f64) -> Option<u16> {
        if let Some(bin) = Self::sats_to_bin(sats) {
            self.bin_add(bin, weight);
            self.count += 1;
            Some(bin as u16)
        } else {
            None
        }
    }

    /// Copy from another histogram (avoids allocation vs clone)
    #[inline]
    pub fn copy_from(&mut self, other: &Histogram) {
        self.bins.copy_from_slice(&other.bins);
        self.count = other.count;
        self.sum = other.sum;
    }

    /// Smooth over round BTC amounts to prevent false positives
    /// Replaces each round BTC bin with the average of its neighbors
    pub fn smooth_round_btc(&mut self) {
        for &bin in ROUND_BTC_BINS {
            if bin > 0 && bin < TOTAL_BINS - 1 {
                let new_val = (self.bins[bin - 1] + self.bins[bin + 1]) / 2.0;
                self.bin_set(bin, new_val);
            }
        }
    }

    /// Normalize the histogram so bins sum to 1.0, then cap extremes
    /// Python caps at 0.008 after normalization to remove outliers
    /// Uses pre-tracked sum for O(1) instead of O(1600) sum computation
    pub fn normalize(&mut self) {
        if self.sum > 0.0 {
            let inv_sum = 1.0 / self.sum;
            for bin in &mut self.bins {
                if *bin > 0.0 {
                    *bin *= inv_sum;
                    // Cap extremes (0.008 chosen by historical testing in Python)
                    if *bin > 0.008 {
                        *bin = 0.008;
                    }
                }
            }
        }
    }

    /// Get the value at a specific bin
    #[allow(dead_code)] // Alternative to direct bins() access
    #[inline]
    pub fn get(&self, bin: usize) -> f64 {
        self.bins.get(bin).copied().unwrap_or(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sats_to_bin() {
        // 10k sats should map to early bins
        let bin = Histogram::sats_to_bin(Sats::_10K).unwrap();
        assert!(bin < TOTAL_BINS / 2);

        // 1 BTC should map to later bins
        let bin = Histogram::sats_to_bin(Sats::_1BTC).unwrap();
        assert!(bin > TOTAL_BINS / 2);

        // Below minimum should return None
        assert!(Histogram::sats_to_bin(Sats::_100).is_none());

        // Above maximum should return None
        assert!(Histogram::sats_to_bin(Sats::_100BTC).is_none());
    }

    #[test]
    fn test_bin_to_sats_roundtrip() {
        for sats in [Sats::_10K, Sats::_100K, Sats::_1M, Sats::_10M, Sats::_1BTC] {
            if let Some(bin) = Histogram::sats_to_bin(sats) {
                let recovered = Histogram::bin_to_sats(bin);
                // Should be within ~1% due to binning
                let ratio = f64::from(recovered) / f64::from(sats);
                assert!(
                    ratio > 0.95 && ratio < 1.05,
                    "sats={}, recovered={}",
                    sats,
                    recovered
                );
            }
        }
    }

    #[test]
    fn test_add_and_normalize() {
        let mut hist = Histogram::new();
        hist.add(Sats::_100K, 1.0);
        hist.add(Sats::_1M, 1.0);
        hist.add(Sats::_10M, 1.0);

        assert_eq!(hist.count(), 3);

        hist.normalize();

        // After normalization, all non-zero bins should be capped at 0.008
        // because 1/3 ≈ 0.333 > 0.008
        let non_zero_bins: Vec<f64> = hist.bins().iter().filter(|&&x| x > 0.0).cloned().collect();

        assert_eq!(non_zero_bins.len(), 3);
        for bin in non_zero_bins {
            assert!((bin - 0.008).abs() < 1e-10);
        }
    }

    #[test]
    fn test_normalize_caps_extremes() {
        let mut hist = Histogram::new();
        // Add a single large value - after normalization it would be 1.0
        hist.add(Sats::_100K, 100.0);

        hist.normalize();
        // Should be capped at 0.008
        let max_bin = hist.bins().iter().cloned().fold(0.0_f64, f64::max);
        assert!((max_bin - 0.008).abs() < 1e-10);
    }
}
