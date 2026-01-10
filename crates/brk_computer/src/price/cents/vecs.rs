use brk_traversable::Traversable;
use brk_types::{Cents, OHLCCents};

use crate::internal::{ComputedHeightDateBytes, LazyHeightDateOHLC};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub split: LazyHeightDateOHLC<Cents, OHLCCents>,
    pub ohlc: ComputedHeightDateBytes<OHLCCents>,
}
