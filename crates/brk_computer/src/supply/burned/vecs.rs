use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::ValuePerBlockCumulative;

#[derive(Traversable)]
#[traversable(transparent)]
pub struct Vecs<M: StorageMode = Rw> {
    pub total: ValuePerBlockCumulative<M>,
}
