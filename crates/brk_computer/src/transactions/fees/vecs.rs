use brk_traversable::Traversable;
use brk_types::{FeeRate, Sats, TxIndex};
use vecdb::{EagerVec, PcoVec};

use crate::internal::{ComputedValueVecsFromTxindex, ComputedVecsFromTxindex};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub txindex_to_input_value: EagerVec<PcoVec<TxIndex, Sats>>,
    pub txindex_to_output_value: EagerVec<PcoVec<TxIndex, Sats>>,
    pub txindex_to_fee: EagerVec<PcoVec<TxIndex, Sats>>,
    pub txindex_to_fee_rate: EagerVec<PcoVec<TxIndex, FeeRate>>,
    pub indexes_to_fee: ComputedValueVecsFromTxindex,
    pub indexes_to_fee_rate: ComputedVecsFromTxindex<FeeRate>,
}
