use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::ComputedVecsFromHeight;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_thermo_cap: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_investor_cap: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_vaulted_cap: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_active_cap: ComputedVecsFromHeight<Dollars>,
    pub indexes_to_cointime_cap: ComputedVecsFromHeight<Dollars>,
}
