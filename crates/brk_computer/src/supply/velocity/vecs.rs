use brk_traversable::Traversable;
use brk_types::StoredF64;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlock;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub native: PerBlock<StoredF64, M>,
    pub fiat: PerBlock<StoredF64, M>,
}
