use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredU64};

use crate::internal::{BinaryBlockFull, ComputedBlockFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    // Per-type output counts
    pub p2a: ComputedBlockFull<StoredU64>,
    pub p2ms: ComputedBlockFull<StoredU64>,
    pub p2pk33: ComputedBlockFull<StoredU64>,
    pub p2pk65: ComputedBlockFull<StoredU64>,
    pub p2pkh: ComputedBlockFull<StoredU64>,
    pub p2sh: ComputedBlockFull<StoredU64>,
    pub p2tr: ComputedBlockFull<StoredU64>,
    pub p2wpkh: ComputedBlockFull<StoredU64>,
    pub p2wsh: ComputedBlockFull<StoredU64>,
    pub opreturn: ComputedBlockFull<StoredU64>,
    pub emptyoutput: ComputedBlockFull<StoredU64>,
    pub unknownoutput: ComputedBlockFull<StoredU64>,

    // Aggregate counts
    /// SegWit output count (p2wpkh + p2wsh + p2tr)
    pub segwit: ComputedBlockFull<StoredU64>,

    // Adoption ratios
    pub taproot_adoption: BinaryBlockFull<StoredF32, StoredU64, StoredU64>,
    pub segwit_adoption: BinaryBlockFull<StoredF32, StoredU64, StoredU64>,
}
