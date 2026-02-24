use brk_traversable::Traversable;
use brk_types::{Cents, Height};
use vecdb::{PcoVec, Rw, StorageMode};

use crate::internal::{ComputedHeightDerivedLast, EagerIndexes};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub price: M::Stored<PcoVec<Height, Cents>>,
    pub open: EagerIndexes<Cents, M>,
    pub high: EagerIndexes<Cents, M>,
    pub low: EagerIndexes<Cents, M>,
    pub close: ComputedHeightDerivedLast<Cents>,
}
