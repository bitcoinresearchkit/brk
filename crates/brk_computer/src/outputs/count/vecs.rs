use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightLast, TxDerivedFull};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub total_count: TxDerivedFull<StoredU64, M>,
    pub utxo_count: ComputedFromHeightLast<StoredU64, M>,
}
