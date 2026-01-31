use brk_traversable::Traversable;
use brk_types::{CentsUnsigned, OHLCCentsUnsigned};

use crate::internal::{ComputedHeightAndDateBytes, LazyHeightAndDateOHLC};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub split: LazyHeightAndDateOHLC<CentsUnsigned, OHLCCentsUnsigned>,
    pub ohlc: ComputedHeightAndDateBytes<OHLCCentsUnsigned>,
}
