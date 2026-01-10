use brk_traversable::Traversable;
use brk_types::Timestamp;

use crate::internal::LazyFromHeightDistribution;

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(flatten)]
    pub interval: LazyFromHeightDistribution<Timestamp>,
}
