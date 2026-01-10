use brk_traversable::Traversable;
use brk_types::{StoredU32, VSize, Weight};

use crate::internal::LazyFromTxDistribution;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub vsize: LazyFromTxDistribution<VSize, StoredU32, StoredU32>,
    pub weight: LazyFromTxDistribution<Weight, StoredU32, StoredU32>,
}
