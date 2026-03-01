use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::ValueFromHeight;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted_supply: ValueFromHeight<M>,
    pub active_supply: ValueFromHeight<M>,
}
