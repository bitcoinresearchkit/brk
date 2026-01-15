use brk_traversable::Traversable;
use brk_types::{
    Cents, DateIndex, Dollars, Height, OHLCCents, OHLCDollars, OracleBins, PairOutputIndex, Sats,
    StoredU32, TxIndex,
};
use vecdb::{BytesVec, LazyVecFrom1, PcoVec};

use crate::internal::{Distribution, LazyTransformDistribution};

/// Vectors storing oracle-derived price data
#[derive(Clone, Traversable)]
pub struct Vecs {
    // ========== Layer 1: Pair identification (requires chain scan) ==========
    /// Maps PairOutputIndex to TxIndex for all 2-output transactions
    /// This is the base index for oracle candidates (~400M entries)
    pub pairoutputindex_to_txindex: PcoVec<PairOutputIndex, TxIndex>,

    /// Maps Height to first PairOutputIndex in that block
    /// Enables efficient per-block iteration over pairs
    pub height_to_first_pairoutputindex: PcoVec<Height, PairOutputIndex>,

    // ========== Layer 3: Output values (enables any price algorithm) ==========
    /// First output value for each pair (index 0)
    pub output0_value: PcoVec<PairOutputIndex, Sats>,

    /// Second output value for each pair (index 1)
    pub output1_value: PcoVec<PairOutputIndex, Sats>,

    // ========== Layer 4: Phase histograms (per block) ==========
    /// Phase histogram per block: frac(log10(sats)) binned into 100 bins
    /// ~200 bytes per block, ~175 MB total
    pub phase_histogram: BytesVec<Height, OracleBins>,

    // ========== Layer 5: Phase Oracle prices (derived from histograms) ==========
    /// Per-block price in cents from phase histogram analysis
    /// Calibrated at block 840,000 (~$63,000)
    /// TODO: Add interpolation for sub-bin precision
    pub phase_price_cents: PcoVec<Height, Cents>,

    /// Daily distribution (min, max, average, percentiles) from phase oracle in cents
    pub phase_daily_cents: Distribution<DateIndex, Cents>,

    /// Daily distribution in dollars (lazy conversion from cents)
    pub phase_daily_dollars: LazyTransformDistribution<DateIndex, Dollars, Cents>,

    // ========== UTXOracle (Python port) ==========
    /// Per-block price estimate in cents (sliding window + stencil matching)
    pub price_cents: PcoVec<Height, Cents>,

    /// Daily OHLC derived from price_cents
    pub ohlc_cents: BytesVec<DateIndex, OHLCCents>,

    /// Daily OHLC in dollars (lazy conversion from cents)
    pub ohlc_dollars: LazyVecFrom1<DateIndex, OHLCDollars, DateIndex, OHLCCents>,

    /// Number of qualifying transactions per day (for confidence)
    pub tx_count: PcoVec<DateIndex, StoredU32>,
}
