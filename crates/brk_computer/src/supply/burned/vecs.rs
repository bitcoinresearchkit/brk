use brk_traversable::Traversable;

use crate::internal::ValueFromHeightSumCum;

/// Burned/unspendable supply metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub opreturn: ValueFromHeightSumCum,
    pub unspendable: ValueFromHeightSumCum,
}
