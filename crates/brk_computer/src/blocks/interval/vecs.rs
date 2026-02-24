use brk_traversable::Traversable;
use brk_types::Timestamp;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightLast, RollingDistribution};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    #[traversable(flatten)]
    pub interval: ComputedFromHeightLast<Timestamp, M>,
    pub interval_rolling: RollingDistribution<Timestamp, M>,
}
