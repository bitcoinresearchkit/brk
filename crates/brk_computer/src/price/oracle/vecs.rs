use brk_traversable::Traversable;
use brk_types::{Cents, DateIndex, Height, OHLCCents, OHLCDollars, StoredU32};
use vecdb::{BytesVec, LazyVecFrom1, PcoVec};

/// Vectors storing UTXOracle-derived price data
#[derive(Clone, Traversable)]
pub struct Vecs {
    /// Per-block price estimate in cents
    /// This enables OHLC derivation for any time period
    pub price_cents: PcoVec<Height, Cents>,

    /// Daily OHLC derived from height_to_price
    /// Uses BytesVec because OHLCCents is a complex type
    pub ohlc_cents: BytesVec<DateIndex, OHLCCents>,

    /// Daily OHLC in dollars (lazy conversion from cents)
    pub ohlc_dollars: LazyVecFrom1<DateIndex, OHLCDollars, DateIndex, OHLCCents>,

    /// Number of qualifying transactions per day (for confidence)
    pub tx_count: PcoVec<DateIndex, StoredU32>,
}
