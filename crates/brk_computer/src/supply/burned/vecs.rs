use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::AmountPerBlockCumulativeWithSums;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub opreturn: AmountPerBlockCumulativeWithSums<M>,
    pub unspendable: AmountPerBlockCumulativeWithSums<M>,
}
