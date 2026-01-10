use brk_traversable::Traversable;
use brk_types::{Cents, OHLCCents};

use crate::internal::{ComputedHeightAndDateBytes, LazyHeightAndDateOHLC};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub split: LazyHeightAndDateOHLC<Cents, OHLCCents>,
    pub ohlc: ComputedHeightAndDateBytes<OHLCCents>,
}
