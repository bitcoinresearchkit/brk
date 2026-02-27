use brk_traversable::Traversable;
use brk_types::{Cents, Dollars, Height, OHLCCents, OHLCDollars};
use vecdb::LazyVecFrom1;

use crate::internal::{ComputedHeightDerivedLast, LazyEagerIndexes};
use crate::prices::{ohlcs::LazyOhlcVecs, split::SplitOhlc};

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[allow(clippy::type_complexity)]
    pub split: SplitOhlc<
        LazyEagerIndexes<Dollars, Cents>,
        LazyEagerIndexes<Dollars, Cents>,
        LazyEagerIndexes<Dollars, Cents>,
        ComputedHeightDerivedLast<Dollars>,
    >,
    pub ohlc: LazyOhlcVecs<OHLCDollars, OHLCCents>,
    pub price: LazyVecFrom1<Height, Dollars, Height, Cents>,
}
