use brk_traversable::Traversable;

use crate::internal::ValueBlockSumCum;

/// Burned/unspendable supply metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_opreturn: ValueBlockSumCum,
    pub indexes_to_unspendable: ValueBlockSumCum,
}
