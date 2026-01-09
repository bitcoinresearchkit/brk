use brk_traversable::Traversable;

use crate::internal::ValueBlockSumCum;

/// Burned/unspendable supply metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub opreturn: ValueBlockSumCum,
    pub unspendable: ValueBlockSumCum,
}
