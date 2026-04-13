use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::PerBlock;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    /// UTXO count per block: `total - inputs - op_return - genesis`.
    pub count: PerBlock<StoredU64, M>,
}
