use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::AmountPerBlockCumulative;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub opreturn: AmountPerBlockCumulative<M>,
}
