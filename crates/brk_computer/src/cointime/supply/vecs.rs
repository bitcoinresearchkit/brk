use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::ValuePerBlock;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted: ValuePerBlock<M>,
    pub active: ValuePerBlock<M>,
}
