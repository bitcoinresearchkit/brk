use brk_traversable::Traversable;
use brk_types::{Height, Timestamp};
use vecdb::LazyVecFrom1;

use crate::internal::DerivedComputedBlockDistribution;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub height_to_interval: LazyVecFrom1<Height, Timestamp, Height, Timestamp>,
    pub indexes_to_block_interval: DerivedComputedBlockDistribution<Timestamp>,
}
