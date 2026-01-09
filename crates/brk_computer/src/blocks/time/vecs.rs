use brk_traversable::Traversable;
use brk_types::{Date, Height, Timestamp};
use vecdb::{EagerVec, LazyVecFrom1, PcoVec};

use crate::internal::ComputedDerivedBlockFirst;

/// Timestamp and date metrics for blocks
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub date: LazyVecFrom1<Height, Date, Height, Timestamp>,
    pub date_fixed: LazyVecFrom1<Height, Date, Height, Timestamp>,
    pub timestamp_fixed: EagerVec<PcoVec<Height, Timestamp>>,
    pub timestamp: ComputedDerivedBlockFirst<Timestamp>,
}
