use brk_traversable::Traversable;
use brk_types::StoredU64;
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeightCumulativeSum;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    // Per-type output counts
    pub p2a: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub p2ms: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub p2pk33: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub p2pk65: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub p2pkh: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub p2sh: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub p2tr: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub p2wpkh: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub p2wsh: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub opreturn: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub emptyoutput: ComputedFromHeightCumulativeSum<StoredU64, M>,
    pub unknownoutput: ComputedFromHeightCumulativeSum<StoredU64, M>,

    // Aggregate counts
    /// SegWit output count (p2wpkh + p2wsh + p2tr)
    pub segwit: ComputedFromHeightCumulativeSum<StoredU64, M>,
}
