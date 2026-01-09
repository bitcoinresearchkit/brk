use brk_traversable::Traversable;
use brk_types::Timestamp;

use crate::internal::LazyBlockDistribution;

#[derive(Clone, Traversable)]
pub struct Vecs {
    #[traversable(flatten)]
    pub interval: LazyBlockDistribution<Timestamp>,
}
