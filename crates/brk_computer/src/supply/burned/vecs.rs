use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::AmountPerBlockCumulativeSum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub opreturn: AmountPerBlockCumulativeSum<M>,
    pub unspendable: AmountPerBlockCumulativeSum<M>,
}
