use brk_traversable::Traversable;
use brk_types::{FeeRate, Sats, TxIndex};
use vecdb::{EagerVec, PcoVec};

use crate::internal::{ComputedTxDistribution, ValueDerivedTxFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub txindex_to_input_value: EagerVec<PcoVec<TxIndex, Sats>>,
    pub txindex_to_output_value: EagerVec<PcoVec<TxIndex, Sats>>,
    pub txindex_to_fee: EagerVec<PcoVec<TxIndex, Sats>>,
    pub txindex_to_fee_rate: EagerVec<PcoVec<TxIndex, FeeRate>>,
    pub indexes_to_fee: ValueDerivedTxFull,
    pub indexes_to_fee_rate: ComputedTxDistribution<FeeRate>,
}
