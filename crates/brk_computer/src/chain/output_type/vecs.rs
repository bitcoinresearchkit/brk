use brk_traversable::Traversable;
use brk_types::StoredU64;

use crate::grouped::ComputedVecsFromHeight;

/// Output type count metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
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
    pub indexes_to_exact_utxo_count: ComputedVecsFromHeight<StoredU64>,
}
