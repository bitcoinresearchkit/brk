use brk_traversable::Traversable;
use brk_types::{DateIndex, Height, OHLCCents};
use vecdb::BytesVec;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub dateindex_to_ohlc_in_cents: BytesVec<DateIndex, OHLCCents>,
    pub height_to_ohlc_in_cents: BytesVec<Height, OHLCCents>,
}
