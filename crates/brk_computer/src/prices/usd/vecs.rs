use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height};
use vecdb::LazyVecFrom1;

use crate::internal::{ComputedHeightDerivedLast, LazyEagerIndexes};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price: LazyVecFrom1<Height, Dollars, Height, Cents>,
    pub open: LazyEagerIndexes<Dollars, Cents>,
    pub high: LazyEagerIndexes<Dollars, Cents>,
    pub low: LazyEagerIndexes<Dollars, Cents>,
    pub close: ComputedHeightDerivedLast<Dollars>,
}
