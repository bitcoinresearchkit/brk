use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::AmountPerBlockCumulativeRolling;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub op_return: AmountPerBlockCumulativeRolling<M>,
}
