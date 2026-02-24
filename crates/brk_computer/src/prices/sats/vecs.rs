use brk_traversable::Traversable;
use brk_types::{Cents, Height, Sats};
use vecdb::LazyVecFrom1;

use crate::internal::{ComputedHeightDerivedLast, LazyEagerIndexes};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price: LazyVecFrom1<Height, Sats, Height, Cents>,
    pub open: LazyEagerIndexes<Sats, Cents>,
    pub high: LazyEagerIndexes<Sats, Cents>,
    pub low: LazyEagerIndexes<Sats, Cents>,
    pub close: ComputedHeightDerivedLast<Sats>,
}
