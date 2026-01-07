use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::ComputedBlockLast;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_thermo_cap: ComputedBlockLast<Dollars>,
    pub indexes_to_investor_cap: ComputedBlockLast<Dollars>,
    pub indexes_to_vaulted_cap: ComputedBlockLast<Dollars>,
    pub indexes_to_active_cap: ComputedBlockLast<Dollars>,
    pub indexes_to_cointime_cap: ComputedBlockLast<Dollars>,
}
