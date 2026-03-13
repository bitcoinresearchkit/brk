use brk_traversable::Traversable;
use brk_types::{Halving, StoredF32, StoredU32};
use vecdb::{Rw, StorageMode};

use crate::internal::{LazyPerBlock, PerBlock};
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub epoch: PerBlock<Halving, M>,
    pub blocks_before_next: PerBlock<StoredU32, M>,
    pub days_before_next: LazyPerBlock<StoredF32, StoredU32>,
}
