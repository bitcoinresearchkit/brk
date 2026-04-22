use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::ValuePerBlockCumulative;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub op_return: ValuePerBlockCumulative<M>,
}
