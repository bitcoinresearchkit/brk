use brk_traversable::Traversable;
use brk_types::{Date, Height, Timestamp};
use vecdb::{EagerVec, LazyVecFrom1, PcoVec, Rw, StorageMode};

use crate::internal::ComputedHeightDerivedFirst;

/// Timestamp and date metrics for blocks
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub date: LazyVecFrom1<Height, Date, Height, Timestamp>,
    pub timestamp_monotonic: M::Stored<EagerVec<PcoVec<Height, Timestamp>>>,
    pub timestamp: ComputedHeightDerivedFirst<Timestamp>,
}
