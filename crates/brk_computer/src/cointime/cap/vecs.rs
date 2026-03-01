use brk_traversable::Traversable;
use brk_types::Cents;
use vecdb::{Rw, StorageMode};

use crate::internal::FiatFromHeightLast;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub thermo_cap: FiatFromHeightLast<Cents, M>,
    pub investor_cap: FiatFromHeightLast<Cents, M>,
    pub vaulted_cap: FiatFromHeightLast<Cents, M>,
    pub active_cap: FiatFromHeightLast<Cents, M>,
    pub cointime_cap: FiatFromHeightLast<Cents, M>,
}
