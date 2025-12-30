use brk_traversable::Traversable;
use brk_types::{FeeRate, Height, Sats, StoredBool, StoredU32, StoredU64, TxIndex, VSize, Weight};
use vecdb::{EagerVec, LazyVecFrom1, LazyVecFrom2, PcoVec};

use crate::grouped::{ComputedValueVecsFromTxindex, ComputedVecsFromHeight, ComputedVecsFromTxindex};

/// Transaction-related metrics
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_tx_count: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v1: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v2: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_v3: ComputedVecsFromHeight<StoredU64>,
    pub indexes_to_tx_vsize: ComputedVecsFromTxindex<VSize>,
    pub indexes_to_tx_weight: ComputedVecsFromTxindex<Weight>,
    pub indexes_to_input_count: ComputedVecsFromTxindex<StoredU64>,
    pub indexes_to_output_count: ComputedVecsFromTxindex<StoredU64>,
    pub txindex_to_is_coinbase: LazyVecFrom2<TxIndex, StoredBool, TxIndex, Height, Height, TxIndex>,
    pub txindex_to_vsize: LazyVecFrom1<TxIndex, VSize, TxIndex, Weight>,
    pub txindex_to_weight: LazyVecFrom2<TxIndex, Weight, TxIndex, StoredU32, TxIndex, StoredU32>,
    /// Value == 0 when Coinbase
    pub txindex_to_input_value: EagerVec<PcoVec<TxIndex, Sats>>,
    pub txindex_to_output_value: EagerVec<PcoVec<TxIndex, Sats>>,
    pub txindex_to_fee: EagerVec<PcoVec<TxIndex, Sats>>,
    pub txindex_to_fee_rate: EagerVec<PcoVec<TxIndex, FeeRate>>,
    pub indexes_to_fee: ComputedValueVecsFromTxindex,
    pub indexes_to_fee_rate: ComputedVecsFromTxindex<FeeRate>,
}
