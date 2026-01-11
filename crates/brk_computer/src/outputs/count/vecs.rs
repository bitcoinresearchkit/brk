use brk_traversable::Traversable;
use brk_types::StoredU64;

use crate::internal::{ComputedFromHeightLast, TxDerivedFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub total_count: TxDerivedFull<StoredU64>,
    pub utxo_count: ComputedFromHeightLast<StoredU64>,
}
