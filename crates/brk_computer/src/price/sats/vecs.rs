use brk_traversable::Traversable;
use brk_types::{OHLCSats, Sats};

use crate::internal::{ComputedOHLC, LazyFromHeightAndDateOHLC};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub split: ComputedOHLC<Sats>,
    pub ohlc: LazyFromHeightAndDateOHLC<OHLCSats>,
}
