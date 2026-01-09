use brk_traversable::Traversable;
use brk_types::{StoredU32, VSize, Weight};

use crate::internal::LazyTxDistribution;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vsize: LazyTxDistribution<VSize, StoredU32, StoredU32>,
    pub weight: LazyTxDistribution<Weight, StoredU32, StoredU32>,
}
