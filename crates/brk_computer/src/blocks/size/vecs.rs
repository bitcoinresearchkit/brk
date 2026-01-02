use brk_traversable::Traversable;
use brk_types::{Height, StoredU64, Weight};
use vecdb::LazyVecFrom1;

use crate::internal::ComputedVecsFromHeight;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub height_to_vbytes: LazyVecFrom1<Height, StoredU64, Height, Weight>,
    pub indexes_to_block_size: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_block_vbytes: ComputedVecsFromHeight<StoredU64>,
}
