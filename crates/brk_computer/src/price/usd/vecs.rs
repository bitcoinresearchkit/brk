use brk_traversable::Traversable;
use brk_types::{Dollars, OHLCDollars};

use crate::internal::{OHLCComputedVecs, OHLCPeriodVecs};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub split: OHLCComputedVecs<Dollars>,
    pub ohlc: OHLCPeriodVecs<OHLCDollars>,
}
