use brk_traversable::Traversable;
use brk_types::{Cents, Height, OHLCSats, Sats};
use vecdb::LazyVecFrom1;

use crate::internal::{ComputedHeightDerivedOHLC, ComputedHeightDerivedSplitOHLC};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price: LazyVecFrom1<Height, Sats, Height, Cents>,
    pub split: ComputedHeightDerivedSplitOHLC<Sats>,
    pub ohlc: ComputedHeightDerivedOHLC<OHLCSats>,
}
