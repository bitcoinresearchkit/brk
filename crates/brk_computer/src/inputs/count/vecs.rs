use brk_traversable::Traversable;
use brk_types::{Height, StoredU64};
use vecdb::{Rw, StorageMode};

use crate::internal::{Full, RollingFull};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub height: Full<Height, StoredU64, M>,
    pub rolling: RollingFull<StoredU64, M>,
}
