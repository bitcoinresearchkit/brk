use brk_traversable::Traversable;
use brk_types::{Height, StoredU64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightLast, Full, RollingFull};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub total_count: Full<Height, StoredU64, M>,
    pub total_count_rolling: RollingFull<StoredU64, M>,
    pub utxo_count: ComputedFromHeightLast<StoredU64, M>,
}
