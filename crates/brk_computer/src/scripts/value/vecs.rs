use brk_traversable::Traversable;
use brk_types::{Height, Sats};
use vecdb::{EagerVec, PcoVec};

use crate::internal::ComputedValueVecsFromHeight;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub height_to_opreturn_value: EagerVec<PcoVec<Height, Sats>>,
    pub indexes_to_opreturn_value: ComputedValueVecsFromHeight,
}
