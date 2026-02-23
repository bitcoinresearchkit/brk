use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredU64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightFull, LazyBinaryFromHeightFull};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    // Per-type output counts
    pub p2a: ComputedFromHeightFull<StoredU64, M>,
    pub p2ms: ComputedFromHeightFull<StoredU64, M>,
    pub p2pk33: ComputedFromHeightFull<StoredU64, M>,
    pub p2pk65: ComputedFromHeightFull<StoredU64, M>,
    pub p2pkh: ComputedFromHeightFull<StoredU64, M>,
    pub p2sh: ComputedFromHeightFull<StoredU64, M>,
    pub p2tr: ComputedFromHeightFull<StoredU64, M>,
    pub p2wpkh: ComputedFromHeightFull<StoredU64, M>,
    pub p2wsh: ComputedFromHeightFull<StoredU64, M>,
    pub opreturn: ComputedFromHeightFull<StoredU64, M>,
    pub emptyoutput: ComputedFromHeightFull<StoredU64, M>,
    pub unknownoutput: ComputedFromHeightFull<StoredU64, M>,

    // Aggregate counts
    /// SegWit output count (p2wpkh + p2wsh + p2tr)
    pub segwit: ComputedFromHeightFull<StoredU64, M>,

    // Adoption ratios
    pub taproot_adoption: LazyBinaryFromHeightFull<StoredF32, StoredU64, StoredU64>,
    pub segwit_adoption: LazyBinaryFromHeightFull<StoredF32, StoredU64, StoredU64>,
}
