use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::ValueFromHeightLast;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted_supply: ValueFromHeightLast<M>,
    pub active_supply: ValueFromHeightLast<M>,
}
