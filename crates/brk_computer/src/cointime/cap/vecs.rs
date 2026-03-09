use brk_traversable::Traversable;
use brk_types::Cents;
use vecdb::{Rw, StorageMode};

use crate::internal::FiatPerBlock;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub thermo_cap: FiatPerBlock<Cents, M>,
    pub investor_cap: FiatPerBlock<Cents, M>,
    pub vaulted_cap: FiatPerBlock<Cents, M>,
    pub active_cap: FiatPerBlock<Cents, M>,
    pub cointime_cap: FiatPerBlock<Cents, M>,
}
