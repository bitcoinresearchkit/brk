use brk_traversable::Traversable;
use brk_types::{Dollars, Height};
use vecdb::LazyVecFrom1;

use crate::internal::LazyDateLast;

/// Market cap metrics - lazy references to supply in USD (KISS)
/// (market_cap = circulating_supply * price, already computed in distribution)
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub height: Option<LazyVecFrom1<Height, Dollars, Height, Dollars>>,
    pub indexes: Option<LazyDateLast<Dollars, Dollars>>,
}
