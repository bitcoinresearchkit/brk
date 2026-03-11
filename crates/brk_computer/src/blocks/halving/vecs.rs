use brk_traversable::Traversable;
use brk_types::{Halving, StoredF32, StoredU32};
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedPerBlock;
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub epoch: ComputedPerBlock<Halving, M>,
    pub blocks_before_next: ComputedPerBlock<StoredU32, M>,
    pub days_before_next: ComputedPerBlock<StoredF32, M>,
}
