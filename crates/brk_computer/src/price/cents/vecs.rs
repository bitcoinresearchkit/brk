use brk_traversable::Traversable;
use brk_types::{Cents, OHLCCents};

use crate::internal::{HeightDateBytes, HeightDateLazyOHLC};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub split: HeightDateLazyOHLC<Cents, OHLCCents>,
    pub ohlc: HeightDateBytes<OHLCCents>,
}
