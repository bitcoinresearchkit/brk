use brk_traversable::Traversable;
use brk_types::{BasisPoints32, Cents};
use vecdb::{Rw, StorageMode};

use crate::internal::{FiatPerBlock, RatioPerBlock};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub thermo_cap: FiatPerBlock<Cents, M>,
    pub investor_cap: FiatPerBlock<Cents, M>,
    pub vaulted_cap: FiatPerBlock<Cents, M>,
    pub active_cap: FiatPerBlock<Cents, M>,
    pub cointime_cap: FiatPerBlock<Cents, M>,
    pub aviv: RatioPerBlock<BasisPoints32, M>,
}
