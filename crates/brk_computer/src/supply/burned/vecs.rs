use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::ValueFromHeightSumCum;

/// Burned/unspendable supply metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub opreturn: ValueFromHeightSumCum<M>,
    pub unspendable: ValueFromHeightSumCum<M>,
}
