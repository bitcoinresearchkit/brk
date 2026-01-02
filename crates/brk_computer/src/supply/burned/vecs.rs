use brk_traversable::Traversable;
use brk_types::{Height, Sats};
use vecdb::{EagerVec, PcoVec};

use crate::internal::ComputedValueVecsFromHeight;

/// Burned/unspendable supply metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub height_to_opreturn: EagerVec<PcoVec<Height, Sats>>,
    pub height_to_unspendable: EagerVec<PcoVec<Height, Sats>>,
    pub indexes_to_opreturn: ComputedValueVecsFromHeight,
    pub indexes_to_unspendable: ComputedValueVecsFromHeight,
}
