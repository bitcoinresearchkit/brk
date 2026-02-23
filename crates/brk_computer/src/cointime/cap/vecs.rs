use brk_traversable::Traversable;
use brk_types::Dollars;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeightLast;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub thermo_cap: ComputedFromHeightLast<Dollars, M>,
    pub investor_cap: ComputedFromHeightLast<Dollars, M>,
    pub vaulted_cap: ComputedFromHeightLast<Dollars, M>,
    pub active_cap: ComputedFromHeightLast<Dollars, M>,
    pub cointime_cap: ComputedFromHeightLast<Dollars, M>,
}
