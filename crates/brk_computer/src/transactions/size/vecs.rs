use brk_traversable::Traversable;
use brk_types::{StoredU32, VSize, Weight};
use vecdb::{Rw, StorageMode};

use crate::internal::{LazyPerTxDistribution, LazyPerTxDistributionTransformed};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub vsize: LazyPerTxDistribution<VSize, StoredU32, StoredU32, M>,
    pub weight: LazyPerTxDistributionTransformed<Weight, StoredU32, StoredU32, VSize>,
}
