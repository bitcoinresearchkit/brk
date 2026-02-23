use brk_traversable::Traversable;
use brk_types::{Bitcoin, Dollars, Height, Sats};
use vecdb::{LazyVecFrom1, LazyVecFrom2};

#[derive(Clone, Traversable)]
pub struct LazyDerivedValuesHeight {
    pub btc: LazyVecFrom1<Height, Bitcoin, Height, Sats>,
    pub usd: LazyVecFrom2<Height, Dollars, Height, Dollars, Height, Sats>,
}
