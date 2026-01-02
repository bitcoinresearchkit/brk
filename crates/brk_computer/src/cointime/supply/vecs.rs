use brk_traversable::Traversable;

use crate::internal::ComputedValueVecsFromHeight;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_vaulted_supply: ComputedValueVecsFromHeight,
    pub indexes_to_active_supply: ComputedValueVecsFromHeight,
}
