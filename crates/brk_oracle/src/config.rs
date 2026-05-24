use brk_types::OutputType;

/// Dust floor used by `Config::default()` and `default_eligible_bin`.
pub(crate) const DEFAULT_MIN_SATS: u64 = 1000;

/// Output types skipped by `Config::default()` (protocol-dominated) and the
/// source of truth for `default_eligible_bin`'s precomputed exclusion mask.
pub(crate) const DEFAULT_EXCLUDED_OUTPUT_TYPES: &[OutputType] = &[OutputType::P2TR];

#[derive(Clone)]
pub struct Config {
    /// EMA decay: 2/(N+1) where N is span in blocks. 2/7 = 6-block span.
    pub alpha: f64,
    /// Ring buffer depth. 12 blocks for deterministic convergence at any start height.
    pub window_size: usize,
    /// Search window bins below/above previous estimate. Asymmetric for log-scale.
    pub search_below: usize,
    pub search_above: usize,
    /// Weight of the adaptive shape-correlation restoring force added to the
    /// stencil score. `0.0` disables it (mature regime, where the fast EMA
    /// tracks real moves the shape term would resist); the slow cold-start uses
    /// a positive weight to resist round-USD octave aliasing in the thin early
    /// output mix.
    pub corr_weight: f64,
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
            search_below: 12,
            search_above: 11,
            corr_weight: 0.0,
            min_sats: DEFAULT_MIN_SATS,
            exclude_common_round_values: true,
            excluded_output_types: DEFAULT_EXCLUDED_OUTPUT_TYPES.to_vec(),
        }
    }
}

impl Config {
    /// Cold-start config below [`START_HEIGHT`](crate::START_HEIGHT): a slow EMA
    /// (span ~19) that resists the round-USD half-price drift the fast default
    /// octave-locks onto in the thin pre-2018 output mix. Window grows to 40 to
    /// hold the decay, and a shape-correlation restoring force (`corr_weight`)
    /// pulls the pick toward the octave whose arm-shape looks like real payments.
    pub fn slow() -> Self {
        Self {
            alpha: 0.10,
            window_size: 40,
            corr_weight: 8.0,
            ..Self::default()
        }
    }

    /// Config for `height`: [`slow`](Self::slow) below
    /// [`START_HEIGHT`](crate::START_HEIGHT), else [`default`](Self::default).
    pub fn for_height(height: usize) -> Self {
        if height < crate::START_HEIGHT {
            Self::slow()
        } else {
            Self::default()
        }
    }
}
