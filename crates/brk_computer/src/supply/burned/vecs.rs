use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::ValueFromHeightSumCumulative;

/// Burned/unspendable supply metrics
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub opreturn: ValueFromHeightSumCumulative<M>,
    pub unspendable: ValueFromHeightSumCumulative<M>,
}
