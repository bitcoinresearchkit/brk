use brk_traversable::Traversable;
use vecdb::{Rw, StorageMode};

use crate::internal::AmountPerBlockCumulative;

#[derive(Traversable)]
#[traversable(transparent)]
pub struct Vecs<M: StorageMode = Rw> {
    pub total: AmountPerBlockCumulative<M>,
}
