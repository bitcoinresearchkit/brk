use brk_traversable::Traversable;
use brk_types::Cents;
use vecdb::{Rw, StorageMode};

use crate::internal::FiatFromHeight;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub thermo_cap: FiatFromHeight<Cents, M>,
    pub investor_cap: FiatFromHeight<Cents, M>,
    pub vaulted_cap: FiatFromHeight<Cents, M>,
    pub active_cap: FiatFromHeight<Cents, M>,
    pub cointime_cap: FiatFromHeight<Cents, M>,
}
