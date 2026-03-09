use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::AmountFromHeightCumulativeSum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub opreturn: AmountFromHeightCumulativeSum<M>,
    pub unspendable: AmountFromHeightCumulativeSum<M>,
}
