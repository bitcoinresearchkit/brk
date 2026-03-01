use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::ValueFromHeightCumulativeSum;

/// Burned/unspendable supply metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub opreturn: ValueFromHeightCumulativeSum<M>,
    pub unspendable: ValueFromHeightCumulativeSum<M>,
}
