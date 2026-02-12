use brk_traversable::Traversable;
use brk_types::{CentsUnsigned, DateIndex, Height, OHLCCentsUnsigned, OHLCDollars};
use vecdb::{BytesVec, LazyVecFrom1, PcoVec};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_cents: PcoVec<Height, CentsUnsigned>,
    pub ohlc_cents: BytesVec<DateIndex, OHLCCentsUnsigned>,
    pub ohlc_dollars: LazyVecFrom1<DateIndex, OHLCDollars, DateIndex, OHLCCentsUnsigned>,
}
