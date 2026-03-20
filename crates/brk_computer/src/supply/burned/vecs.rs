use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::AmountPerBlockCumulativeRolling;

#[derive(Traversable)]
#[traversable(transparent)]
pub struct Vecs<M: StorageMode = Rw> {
    pub total: AmountPerBlockCumulativeRolling<M>,
}
