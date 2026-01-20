//! Playground library for Bitcoin on-chain analysis
//!
//! This crate provides tools for:
//! - Phase histogram analysis of UTXO patterns
//! - Filter-based output selection for price signal extraction
//! - On-chain OHLC price oracle derivation

pub mod anchors;
pub mod conditions;
pub mod constants;
pub mod filters;
pub mod histogram;
pub mod oracle;
pub mod render;
pub mod signal;

pub use anchors::{get_anchor_ohlc, get_anchor_range, Ohlc};
pub use conditions::{out_bits, tx_bits, MappedOutputConditions};
pub use constants::{HeatmapFilter, NUM_BINS, ROUND_USD_AMOUNTS};
pub use filters::FILTERS;
pub use oracle::{
    derive_daily_ohlc, derive_daily_ohlc_with_confidence, derive_height_price,
    derive_height_price_with_confidence, derive_ohlc_from_height_prices,
    derive_ohlc_from_height_prices_with_confidence, derive_price_from_histogram,
    HeightPriceResult, OracleConfig, OracleResult,
};
pub use signal::{compute_expected_bins_per_day, usd_to_bin};
pub use histogram::load_or_compute_output_conditions;
