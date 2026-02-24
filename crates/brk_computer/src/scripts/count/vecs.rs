use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredU64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeightCumSum, ComputedFromHeightLast};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    // Per-type output counts
    pub p2a: ComputedFromHeightCumSum<StoredU64, M>,
    pub p2ms: ComputedFromHeightCumSum<StoredU64, M>,
    pub p2pk33: ComputedFromHeightCumSum<StoredU64, M>,
    pub p2pk65: ComputedFromHeightCumSum<StoredU64, M>,
    pub p2pkh: ComputedFromHeightCumSum<StoredU64, M>,
    pub p2sh: ComputedFromHeightCumSum<StoredU64, M>,
    pub p2tr: ComputedFromHeightCumSum<StoredU64, M>,
    pub p2wpkh: ComputedFromHeightCumSum<StoredU64, M>,
    pub p2wsh: ComputedFromHeightCumSum<StoredU64, M>,
    pub opreturn: ComputedFromHeightCumSum<StoredU64, M>,
    pub emptyoutput: ComputedFromHeightCumSum<StoredU64, M>,
    pub unknownoutput: ComputedFromHeightCumSum<StoredU64, M>,

    // Aggregate counts
    /// SegWit output count (p2wpkh + p2wsh + p2tr)
    pub segwit: ComputedFromHeightCumSum<StoredU64, M>,

    // Adoption ratios (stored per-block, lazy period views)
    pub taproot_adoption: ComputedFromHeightLast<StoredF32, M>,
    pub segwit_adoption: ComputedFromHeightLast<StoredF32, M>,
}
