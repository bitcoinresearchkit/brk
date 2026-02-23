use brk_traversable::Traversable;
use brk_types::{Cents, Height, OHLCCents};
use vecdb::{PcoVec, Rw, StorageMode};

use crate::internal::{ComputedHeightDerivedOHLC, ComputedHeightDerivedSplitOHLC};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price: M::Stored<PcoVec<Height, Cents>>,
    pub split: ComputedHeightDerivedSplitOHLC<Cents>,
    pub ohlc: ComputedHeightDerivedOHLC<OHLCCents>,
}
