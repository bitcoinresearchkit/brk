use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::AmountFromHeightCumulative;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub opreturn: AmountFromHeightCumulative<M>,
}
