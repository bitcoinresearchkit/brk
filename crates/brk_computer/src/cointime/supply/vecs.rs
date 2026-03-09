use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::AmountFromHeight;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vaulted_supply: AmountFromHeight<M>,
    pub active_supply: AmountFromHeight<M>,
}
