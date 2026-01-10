use brk_traversable::Traversable;
use brk_types::{Dollars, OHLCDollars};

use crate::internal::{ComputedOHLC, LazyFromHeightAndDateOHLC};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub split: ComputedOHLC<Dollars>,
    pub ohlc: LazyFromHeightAndDateOHLC<OHLCDollars>,
}
