use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeight;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub btc: ComputedFromHeight<StoredF64, M>,
    pub usd: ComputedFromHeight<StoredF64, M>,
}
