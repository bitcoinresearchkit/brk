use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredU64};

use crate::internal::{ComputedVecsFromHeight, LazyVecsFrom2FromHeight};

#[derive(Clone, Traversable)]
pub struct Vecs {
    // Per-type output counts
    pub indexes_to_p2a_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2ms_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2pk33_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2pk65_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2pkh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2sh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2tr_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2wpkh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_p2wsh_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_opreturn_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_emptyoutput_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_unknownoutput_count: ComputedVecsFromHeight<StoredU64>,

    // Aggregate counts
    /// SegWit output count (p2wpkh + p2wsh + p2tr)
    pub indexes_to_segwit_count: ComputedVecsFromHeight<StoredU64>,

    // Adoption ratios (lazy)
    // Denominator is outputs.count.indexes_to_count (total output count)
    /// Taproot adoption: p2tr / total_outputs * 100
    pub indexes_to_taproot_adoption: LazyVecsFrom2FromHeight<StoredF32, StoredU64, StoredU64>,
    /// SegWit adoption: segwit / total_outputs * 100
    pub indexes_to_segwit_adoption: LazyVecsFrom2FromHeight<StoredF32, StoredU64, StoredU64>,
}
