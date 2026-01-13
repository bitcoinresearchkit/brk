use brk_traversable::Traversable;
use brk_types::{Cents, DateIndex, Height, OHLCCents, StoredU32};
use vecdb::{BytesVec, PcoVec};

/// Vectors storing UTXOracle-derived price data
#[derive(Clone, Traversable)]
pub struct Vecs {
    /// Per-block price estimate in cents
    /// This enables OHLC derivation for any time period
    pub price: PcoVec<Height, Cents>,

    /// Daily OHLC derived from height_to_price
    /// Uses BytesVec because OHLCCents is a complex type
    pub ohlc: BytesVec<DateIndex, OHLCCents>,

    /// Number of qualifying transactions per day (for confidence)
    pub tx_count: PcoVec<DateIndex, StoredU32>,
}
