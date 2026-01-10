use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::ComputedFromHeightLast;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub thermo_cap: ComputedFromHeightLast<Dollars>,
    pub investor_cap: ComputedFromHeightLast<Dollars>,
    pub vaulted_cap: ComputedFromHeightLast<Dollars>,
    pub active_cap: ComputedFromHeightLast<Dollars>,
    pub cointime_cap: ComputedFromHeightLast<Dollars>,
}
