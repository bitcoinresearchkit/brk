use brk_traversable::Traversable;
use brk_types::{OHLCSats, Sats};

use crate::internal::{OHLCComputedVecs, OHLCPeriodVecs};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub split: OHLCComputedVecs<Sats>,
    pub ohlc: OHLCPeriodVecs<OHLCSats>,
}
