use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::AmountPerBlock;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted_supply: AmountPerBlock<M>,
    pub active_supply: AmountPerBlock<M>,
}
