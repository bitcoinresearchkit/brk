use brk_traversable::Traversable;
use brk_types::{Cents, Height, OHLCCents, OHLCSats, Sats};
use vecdb::LazyVecFrom1;

use crate::internal::{ComputedHeightDerivedLast, LazyEagerIndexes};
use crate::prices::{ohlcs::LazyOhlcVecs, split::SplitOhlc};

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[allow(clippy::type_complexity)]
    pub split: SplitOhlc<
        LazyEagerIndexes<Sats, Cents>,
        LazyEagerIndexes<Sats, Cents>,
        LazyEagerIndexes<Sats, Cents>,
        ComputedHeightDerivedLast<Sats>,
    >,
    pub ohlc: LazyOhlcVecs<OHLCSats, OHLCCents>,
    pub price: LazyVecFrom1<Height, Sats, Height, Cents>,
}
