use brk_traversable::Traversable;
use brk_types::Dollars;

use crate::internal::ComputedBlockLast;

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub thermo_cap: ComputedBlockLast<Dollars>,
    pub investor_cap: ComputedBlockLast<Dollars>,
    pub vaulted_cap: ComputedBlockLast<Dollars>,
    pub active_cap: ComputedBlockLast<Dollars>,
    pub cointime_cap: ComputedBlockLast<Dollars>,
}
