use brk_traversable::Traversable;
use brk_types::{StoredU32, VSize, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::LazyPerTxDistribution;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vsize: LazyPerTxDistribution<VSize, StoredU32, StoredU32, M>,
    pub weight: LazyPerTxDistribution<Weight, StoredU32, StoredU32, M>,
}
