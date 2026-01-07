use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredU64};

use crate::internal::{ComputedBlockFull, BinaryBlockFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    // Per-type output counts
    pub indexes_to_p2a_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_p2ms_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_p2pk33_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_p2pk65_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_p2pkh_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_p2sh_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_p2tr_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_p2wpkh_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_p2wsh_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_opreturn_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_emptyoutput_count: ComputedBlockFull<StoredU64>,
    pub indexes_to_unknownoutput_count: ComputedBlockFull<StoredU64>,

    // Aggregate counts
    /// SegWit output count (p2wpkh + p2wsh + p2tr)
    pub indexes_to_segwit_count: ComputedBlockFull<StoredU64>,

    // Adoption ratios (lazy)
    // Denominator is outputs.count.indexes_to_count (total output count)
    /// Taproot adoption: p2tr / total_outputs * 100
    pub indexes_to_taproot_adoption: BinaryBlockFull<StoredF32, StoredU64, StoredU64>,
    /// SegWit adoption: segwit / total_outputs * 100
    pub indexes_to_segwit_adoption: BinaryBlockFull<StoredF32, StoredU64, StoredU64>,
}
