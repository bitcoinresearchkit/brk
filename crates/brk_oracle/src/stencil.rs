use crate::{
    scale::{HistogramEma, NUM_BINS},
    shape::ShapeAnchor,
};

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
pub(crate) const N_ARMS: usize = STENCIL_OFFSETS.len();

/// EMA mass at `idx`, or 0.0 when the index falls outside the histogram.
#[inline(always)]
fn bin_value(ema: &HistogramEma, idx: i64) -> f64 {
    if idx >= 0 && (idx as usize) < NUM_BINS {
        ema[idx as usize]
    } else {
        0.0
    }
}

/// Raw EMA mass on each of the 19 stencil arms at `center`.
fn arms_at(ema: &HistogramEma, center: i64) -> [f64; N_ARMS] {
    STENCIL_OFFSETS.map(|offset| bin_value(ema, center + offset as i64))
}

/// [`arms_at`] L1-normalized to sum 1, or `None` when the center carries no mass.
pub(crate) fn normalized_arms_at(ema: &HistogramEma, center: i64) -> Option<[f64; N_ARMS]> {
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

/// Scores each candidate bin in the search window by summing normalized stencil
/// matches across the EMA histogram, then refines with parabolic interpolation.
/// Each candidate also picks up `shape`'s shape-anchoring restoring force, which
/// is inert (adds 0) outside the slow cold-start regime.
pub(crate) fn find_best_bin(
    ema: &HistogramEma,
    prev_bin: f64,
    search_below: usize,
    search_above: usize,
    shape: &ShapeAnchor,
) -> f64 {
    let center = prev_bin.round() as usize;
    let search_start = center.saturating_sub(search_below);
    let search_end = (center + search_above + 1).min(NUM_BINS);

    if search_start >= search_end {
        return prev_bin;
    }

    // Per-offset peak within the search window (for normalization).
    let mut arm_peaks = [0.0f64; N_ARMS];
    for (i, &offset) in STENCIL_OFFSETS.iter().enumerate() {
        for bin in search_start..search_end {
            arm_peaks[i] = arm_peaks[i].max(bin_value(ema, bin as i64 + offset as i64));
        }
    }

    let score = |bin: usize| -> f64 {
        let mut total = 0.0;
        for (i, &offset) in STENCIL_OFFSETS.iter().enumerate() {
            if arm_peaks[i] > 0.0 {
                total += bin_value(ema, bin as i64 + offset as i64) / arm_peaks[i];
            }
        }
        total += shape.score(ema, bin as i64);
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
