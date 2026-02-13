use brk_traversable::Traversable;
use brk_types::{CentsUnsigned, DateIndex, Height, OHLCCentsUnsigned, OHLCDollars};
use vecdb::{BytesVec, PcoVec};

use crate::internal::{ComputedOHLC, LazyFromHeightAndDateOHLC};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_cents: PcoVec<Height, CentsUnsigned>,
    pub ohlc_cents: BytesVec<DateIndex, OHLCCentsUnsigned>,
    pub split: ComputedOHLC<CentsUnsigned>,
    pub ohlc: LazyFromHeightAndDateOHLC<OHLCCentsUnsigned>,
    pub ohlc_dollars: LazyFromHeightAndDateOHLC<OHLCDollars>,
}
