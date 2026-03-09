use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedPerBlock;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub btc: ComputedPerBlock<StoredF64, M>,
    pub usd: ComputedPerBlock<StoredF64, M>,
}
