use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, OHLCDollars};
use vecdb::LazyVecFrom1;

use crate::internal::{ComputedHeightDerivedOHLC, ComputedHeightDerivedSplitOHLC};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price: LazyVecFrom1<Height, Dollars, Height, Cents>,
    pub split: ComputedHeightDerivedSplitOHLC<Dollars>,
    pub ohlc: ComputedHeightDerivedOHLC<OHLCDollars>,
}
