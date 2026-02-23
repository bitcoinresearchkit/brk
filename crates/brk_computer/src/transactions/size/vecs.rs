use brk_traversable::Traversable;
use brk_types::{StoredU32, VSize, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::LazyFromTxDistribution;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vsize: LazyFromTxDistribution<VSize, StoredU32, StoredU32, M>,
    pub weight: LazyFromTxDistribution<Weight, StoredU32, StoredU32, M>,
}
