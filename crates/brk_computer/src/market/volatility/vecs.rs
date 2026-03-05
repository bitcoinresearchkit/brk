use brk_traversable::Traversable;

use crate::internal::LazyFromHeight;

use brk_types::StoredF32;
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_volatility_1w: LazyFromHeight<StoredF32>,
    pub price_volatility_1m: LazyFromHeight<StoredF32>,
    pub price_volatility_1y: LazyFromHeight<StoredF32>,
}
