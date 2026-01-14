//! Stencil matching for UTXOracle price detection.
//! Uses two stencils that slide across the histogram:
//! 1. Smooth stencil: Gaussian capturing general spending distribution
//! 2. Spike stencil: Hard-coded weights at known USD amounts

use brk_types::{Cents, Sats};
use rayon::prelude::*;
use rustc_hash::FxHashMap;

use super::histogram::{BINS_PER_DECADE, Histogram, TOTAL_BINS};

/// Number of parallel chunks for stencil sliding
const PARALLEL_CHUNKS: i32 = 4;

/// USD spike stencil entries: (bin offset from $100 center, weight)
/// These represent the expected frequency of round USD amounts in transactions
/// Offset formula: log10(USD/100) * 200 bins/decade
/// Companion spikes at ±2 bins from main spike (Rust 200 bins/decade ≈ Python's ±1 at 180 bins/decade)
/// Matches Python's 29 entries from utxo_oracle.py lines 1013-1041
const SPIKE_STENCIL: &[(i32, f64)] = &[
    // $1 (single)
    (-400, 0.00130),
    // $5 (single)
    (-260, 0.00168),
    // $10 (main + companion)
    (-200, 0.00347),
    (-198, 0.00199),
    // $15 (single)
    (-165, 0.00191),
    // $20 (main + companion)
    (-140, 0.00334),
    (-138, 0.00259),
    // $30 (main + companion)
    (-105, 0.00258),
    (-103, 0.00273),
    // $50 (main + 2 companions)
    (-62, 0.00308),
    (-60, 0.00561),
    (-58, 0.00309),
    // $100 (main + 3 companions) - center
    (-2, 0.00292),
    (0, 0.00617),
    (2, 0.00442),
    (4, 0.00263),
    // $150 (single)
    (35, 0.00286),
    // $200 (main + companion)
    (60, 0.00410),
    (62, 0.00335),
    // $300 (main + companion)
    (95, 0.00252),
    (97, 0.00278),
    // $500 (single)
    (140, 0.00379),
    // $1000 (main + companion)
    (200, 0.00369),
    (202, 0.00239),
    // $1500 (single)
    (235, 0.00128),
    // $2000 (main + companion)
    (260, 0.00165),
    (262, 0.00140),
    // $5000 (single)
    (340, 0.00115),
    // $10000 (single)
    (400, 0.00083),
];

/// Width of the smooth stencil in bins (Gaussian sigma)
/// Python uses std_dev=201 with 803 bins. Our histogram has 1600 bins (2x),
/// so we use 201 * (1600/803) ≈ 400 bins sigma equivalent
const SMOOTH_WIDTH: f64 = 400.0;

/// Linear term coefficient for smooth stencil (per Python: 0.0000005 * x)
/// Scaled for our larger histogram: 0.0000005 * (803/1600) ≈ 0.00000025
const SMOOTH_LINEAR_COEF: f64 = 0.00000025;

/// Weight given to smooth stencil vs spike stencil
const SMOOTH_WEIGHT: f64 = 0.65;
const SPIKE_WEIGHT: f64 = 1.0;

/// Pre-computed Gaussian weights for smooth stencil
/// Index is absolute distance from center (0 to SMOOTH_RANGE)
/// This avoids computing exp() billions of times
const SMOOTH_RANGE: usize = 800;

/// Lazily initialized Gaussian weight lookup table
fn gaussian_weights() -> &'static [f64; SMOOTH_RANGE + 1] {
    use std::sync::OnceLock;
    static WEIGHTS: OnceLock<[f64; SMOOTH_RANGE + 1]> = OnceLock::new();
    WEIGHTS.get_or_init(|| {
        let mut weights = [0.0; SMOOTH_RANGE + 1];
        (0..=SMOOTH_RANGE).for_each(|d| {
            let distance = d as f64;
            weights[d] = (-distance * distance / (2.0 * SMOOTH_WIDTH * SMOOTH_WIDTH)).exp();
        });
        weights
    })
}

/// Find the best price estimate by sliding stencils across the histogram
///
/// # Arguments
/// * `histogram` - The log-scale histogram of output values
/// * `min_slide` - Minimum slide position (higher prices)
/// * `max_slide` - Maximum slide position (lower prices)
///
/// # Returns
/// The estimated price in cents, or None if no valid estimate found
pub fn find_best_price(histogram: &Histogram, min_slide: i32, max_slide: i32) -> Option<Cents> {
    let bins = histogram.bins();

    // Pre-compute the linear term sum (constant for all slide positions)
    // linear_sum = Σ bins[i] * SMOOTH_LINEAR_COEF * i
    let linear_sum: f64 = bins
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, v)| *v > 0.0)
        .map(|(i, v)| v * SMOOTH_LINEAR_COEF * i as f64)
        .sum();

    // Collect non-zero bins: Vec for Gaussian (needs iteration), HashMap for spike (needs lookup)
    let non_zero_bins: Vec<(usize, f64)> = bins
        .iter()
        .copied()
        .enumerate()
        .filter(|(_, v)| *v > 0.0)
        .collect();

    // HashMap for O(1) spike lookups instead of O(n) linear search
    let bin_map: FxHashMap<usize, f64> = non_zero_bins.iter().copied().collect();

    // Slide through possible price positions in parallel chunks
    let range_size = max_slide - min_slide + 1;
    let chunk_size = (range_size + PARALLEL_CHUNKS - 1) / PARALLEL_CHUNKS;

    // Track total score for weighted average computation
    let (best_position, best_score, total_score) = (0..PARALLEL_CHUNKS)
        .into_par_iter()
        .map(|chunk_idx| {
            let chunk_start = min_slide + chunk_idx * chunk_size;
            let chunk_end = (chunk_start + chunk_size - 1).min(max_slide);

            let mut local_best_score = f64::NEG_INFINITY;
            let mut local_best_pos = chunk_start;
            let mut local_total = 0.0;

            for slide in chunk_start..=chunk_end {
                let score = compute_score_fast(&non_zero_bins, &bin_map, linear_sum, slide);
                local_total += score;
                if score > local_best_score {
                    local_best_score = score;
                    local_best_pos = slide;
                }
            }

            (local_best_pos, local_best_score, local_total)
        })
        .reduce(
            || (0, f64::NEG_INFINITY, 0.0),
            |a, b| {
                let total = a.2 + b.2;
                if a.1 > b.1 {
                    (a.0, a.1, total)
                } else {
                    (b.0, b.1, total)
                }
            },
        );

    // Compute neighbor scores for sub-bin interpolation (matches Python behavior)
    let neighbor_up_score = compute_score_fast(&non_zero_bins, &bin_map, linear_sum, best_position + 1);
    let neighbor_down_score = compute_score_fast(&non_zero_bins, &bin_map, linear_sum, best_position - 1);

    // Find best neighbor
    let (best_neighbor_offset, neighbor_score) = if neighbor_up_score > neighbor_down_score {
        (1, neighbor_up_score)
    } else {
        (-1, neighbor_down_score)
    };

    // Weighted average between best position and best neighbor (Python lines 1144-1149)
    // This provides sub-bin precision for the rough estimate
    let avg_score = total_score / range_size as f64;
    let a1 = best_score - avg_score;
    let a2 = (neighbor_score - avg_score).abs();

    if a1 + a2 > 0.0 {
        let w1 = a1 / (a1 + a2);
        let w2 = a2 / (a1 + a2);

        let price_best = i64::from(position_to_cents(best_position)?);
        let price_neighbor = i64::from(position_to_cents(best_position + best_neighbor_offset)?);

        let weighted_price = Cents::from((w1 * price_best as f64 + w2 * price_neighbor as f64) as i64);
        Some(weighted_price)
    } else {
        position_to_cents(best_position)
    }
}

/// Fast score computation using sparse bin representation
fn compute_score_fast(
    non_zero_bins: &[(usize, f64)],
    bin_map: &FxHashMap<usize, f64>,
    linear_sum: f64,
    slide: i32,
) -> f64 {
    let spike_score = compute_spike_score_hash(bin_map, slide);

    // Python: smooth weight only applied for slide < 150
    if slide < 150 {
        let gaussian_score = compute_gaussian_score_sparse(non_zero_bins, slide);
        // Combine Gaussian and linear parts of smooth score
        let smooth_score = 0.0015 * gaussian_score + linear_sum;
        SMOOTH_WEIGHT * smooth_score + SPIKE_WEIGHT * spike_score
    } else {
        SPIKE_WEIGHT * spike_score
    }
}

/// Compute just the Gaussian part of the smooth stencil (sparse iteration)
fn compute_gaussian_score_sparse(non_zero_bins: &[(usize, f64)], slide: i32) -> f64 {
    let center = center_bin() as i32 + slide;
    let weights = gaussian_weights();
    let mut score = 0.0;

    for &(i, bin_value) in non_zero_bins {
        let distance = (i as i32 - center).unsigned_abs() as usize;
        if distance <= SMOOTH_RANGE {
            score += bin_value * weights[distance];
        }
    }

    score
}

/// Compute spike score using HashMap for O(1) bin lookups
/// This is O(29) per slide instead of O(29 × 500) with linear search
#[inline]
fn compute_spike_score_hash(bin_map: &FxHashMap<usize, f64>, slide: i32) -> f64 {
    let center = center_bin() as i32 + slide;
    let mut score = 0.0;

    for &(offset, weight) in SPIKE_STENCIL {
        let bin_idx = (center + offset) as usize;
        if let Some(&bin_value) = bin_map.get(&bin_idx) {
            score += bin_value * weight;
        }
    }

    score
}

/// Get the center bin index (corresponds to ~0.001 BTC baseline)
/// This is approximately where $100 would be at ~$100,000/BTC
/// Python uses center_p001 = 601
#[inline]
fn center_bin() -> usize {
    // 0.001 BTC = 10^-3 BTC
    // In our range of [-6, 2], -3 is at position (3/8) * 1600 = 600
    // Python uses 601 for center_p001, so we match that
    601
}

/// Convert a slide position to price in cents
/// Position 0 = center (~$100,000 at 0.001 BTC)
fn position_to_cents(position: i32) -> Option<Cents> {
    // Each bin represents 1/200 of a decade in log scale
    // Moving the stencil by +1 means the price is lower (outputs are smaller for same USD)
    // Moving by -1 means the price is higher

    // At position 0, we assume the center maps to some reference price
    // The reference: 0.001 BTC = $100 means price is $100,000/BTC

    // Offset per bin in log10 terms: 1/200 decades
    let log_offset = position as f64 / BINS_PER_DECADE as f64;

    // Reference price: $100 at 0.001 BTC = $100,000/BTC = 10,000,000 cents/BTC
    let ref_price_cents: f64 = 10_000_000.0;

    // Price scales inversely with position (higher position = lower price)
    let price = ref_price_cents / 10_f64.powf(log_offset);

    if price > 0.0 && price < 1e12 {
        Some(Cents::from(price as i64))
    } else {
        None
    }
}

/// Round USD amounts for price point collection (in cents)
/// Matches Python: [5, 10, 15, 20, 25, 30, 40, 50, 100, 150, 200, 300, 500, 1000]
const ROUND_USD_CENTS: [f64; 14] = [
    500.0, 1000.0, 1500.0, 2000.0, 2500.0, 3000.0, 4000.0, 5000.0, 10000.0, 15000.0, 20000.0,
    30000.0, 50000.0, 100000.0,
];

/// Check if a sats value is a round amount that should be filtered
/// Matches Python's micro_remove_list with ±0.01% tolerance
/// Uses O(1) modular arithmetic instead of iterating through all round values
#[inline]
pub fn is_round_sats(sats: Sats) -> bool {
    let sats = u64::from(sats);

    // Determine the step size based on the magnitude
    let (step, min_val) = if sats < 10_000 {
        (1_000u64, 5_000u64)
    } else if sats < 100_000 {
        (1_000, 10_000)
    } else if sats < 1_000_000 {
        (10_000, 100_000)
    } else if sats < 10_000_000 {
        (100_000, 1_000_000)
    } else if sats < 100_000_000 {
        (1_000_000, 10_000_000)
    } else {
        return false; // Outside range
    };

    if sats < min_val {
        return false;
    }

    // Find the nearest round value
    let nearest_round = ((sats + step / 2) / step) * step;

    // Check if within ±0.01% tolerance
    let tolerance = nearest_round / 10000;
    sats >= nearest_round.saturating_sub(tolerance) && sats <= nearest_round + tolerance
}

/// Refine a rough price estimate using center-of-mass convergence
/// Matches Python's find_central_output algorithm (geometric median)
///
/// # Arguments
/// * `by_bin` - Pre-built index of non-round sats values grouped by histogram bin (maintained incrementally by compute.rs)
/// * `rough_price_cents` - Initial price estimate from stencil matching
///
/// # Returns
/// Refined price in cents
pub fn refine_price(by_bin: &[Vec<Sats>; TOTAL_BINS], rough_price_cents: Cents) -> Cents {
    if rough_price_cents == Cents::ZERO {
        return rough_price_cents;
    }

    const WIDE_WINDOW: f64 = 0.25; // ±25% for initial collection (per Python)
    const TIGHT_WINDOW: f64 = 0.05; // ±5% for refinement

    let rough_price = i64::from(rough_price_cents) as f64;

    // For each USD amount, scan only the bins that overlap with ±25% window
    let mut price_points: Vec<f64> = Vec::with_capacity(8000);

    (0..14).for_each(|i| {
        let usd_cents = ROUND_USD_CENTS[i];
        let expected_sats = usd_cents * 1e8 / rough_price;
        let sats_low = Sats::from((expected_sats * (1.0 - WIDE_WINDOW)) as u64);
        let sats_high = Sats::from((expected_sats * (1.0 + WIDE_WINDOW)) as u64);

        // Convert bounds to bin range
        let bin_low = Histogram::sats_to_bin(sats_low).unwrap_or(0);
        let bin_high = Histogram::sats_to_bin(sats_high).unwrap_or(TOTAL_BINS - 1);

        // Scan only bins in range
        (bin_low..=bin_high.min(TOTAL_BINS - 1)).for_each(|bin| {
            for &sats in &by_bin[bin] {
                if sats > sats_low && sats < sats_high {
                    price_points.push(usd_cents * 1e8 / f64::from(sats));
                }
            }
        });
    });

    if price_points.is_empty() {
        return rough_price_cents;
    }

    // Step 2: Find geometric median using iterative refinement
    let mut center_price = rough_price;
    // Use fixed array instead of HashSet (max 20 iterations)
    let mut seen_prices = [0u64; 20];
    let mut seen_count = 0usize;

    // Reusable buffer for filtered prices (avoids allocation per iteration)
    let mut filtered: Vec<f64> = Vec::with_capacity(price_points.len());

    for _ in 0..20 {
        let price_low = center_price * (1.0 - TIGHT_WINDOW);
        let price_high = center_price * (1.0 + TIGHT_WINDOW);

        // Reuse filtered buffer
        filtered.clear();
        filtered.extend(
            price_points
                .iter()
                .filter(|&&p| p > price_low && p < price_high),
        );

        if filtered.is_empty() {
            break;
        }

        let new_center = find_geometric_median_inplace(&mut filtered);

        // Check for convergence using fixed array
        let new_center_rounded = new_center as u64;
        if seen_prices[..seen_count].contains(&new_center_rounded) {
            break;
        }
        if seen_count < 20 {
            seen_prices[seen_count] = new_center_rounded;
            seen_count += 1;
        }

        center_price = new_center;
    }

    Cents::from(center_price as i64)
}

/// Find the geometric median (point minimizing sum of absolute distances)
/// Sorts in-place to avoid allocation. Input slice is modified!
fn find_geometric_median_inplace(prices: &mut [f64]) -> f64 {
    if prices.is_empty() {
        return 0.0;
    }
    if prices.len() == 1 {
        return prices[0];
    }

    // Sort in-place
    prices.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let n = prices.len();

    // Compute prefix sums using running total (no allocation needed)
    // We compute total first, then calculate distances on the fly
    let total: f64 = prices.iter().sum();

    // Find point minimizing total distance
    let mut min_dist = f64::MAX;
    let mut best_price = prices[n / 2];
    let mut left_sum = 0.0;

    (0..n).for_each(|i| {
        let x = prices[i];
        let left_count = i as f64;
        let right_count = (n - i - 1) as f64;
        let right_sum = total - left_sum - x;

        let dist = (x * left_count - left_sum) + (right_sum - x * right_count);

        if dist < min_dist {
            min_dist = dist;
            best_price = x;
        }

        left_sum += x;
    });

    best_price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_position_to_cents() {
        // Position 0 should give reference price (~$100,000)
        let cents = position_to_cents(0).unwrap();
        let cents_val = i64::from(cents);
        assert!(cents_val > 5_000_000 && cents_val < 20_000_000);

        // Positive position = lower price
        let lower = position_to_cents(200).unwrap();
        assert!(lower < cents);

        // Negative position = higher price
        let higher = position_to_cents(-200).unwrap();
        assert!(higher > cents);
    }

    #[test]
    fn test_spike_stencil_entries() {
        // Verify stencil has 29 entries matching Python
        assert_eq!(SPIKE_STENCIL.len(), 29);

        // All weights should be positive
        for &(_, weight) in SPIKE_STENCIL {
            assert!(weight > 0.0);
        }
    }
}
