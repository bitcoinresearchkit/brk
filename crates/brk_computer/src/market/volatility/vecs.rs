use brk_traversable::Traversable;

use crate::internal::LazyPerBlock;

use brk_types::StoredF32;
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub _1w: LazyPerBlock<StoredF32>,
    pub _1m: LazyPerBlock<StoredF32>,
    pub _1y: LazyPerBlock<StoredF32>,
}
