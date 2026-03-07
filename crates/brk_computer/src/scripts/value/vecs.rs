use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::ValueFromHeightCumulative;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub opreturn: ValueFromHeightCumulative<M>,
}
