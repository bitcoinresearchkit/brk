use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlock;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub btc: PerBlock<StoredF64, M>,
    pub usd: PerBlock<StoredF64, M>,
}
