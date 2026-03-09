use brk_traversable::Traversable;

use crate::internal::LazyPerBlock;

use brk_types::StoredF32;
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub price_volatility_1w: LazyPerBlock<StoredF32>,
    pub price_volatility_1m: LazyPerBlock<StoredF32>,
    pub price_volatility_1y: LazyPerBlock<StoredF32>,
}
