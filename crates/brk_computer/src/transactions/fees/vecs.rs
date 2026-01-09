use brk_traversable::Traversable;
use brk_types::{FeeRate, Sats, TxIndex};
use vecdb::{EagerVec, PcoVec};

use crate::internal::{ComputedTxDistribution, ValueTxFull};

#[derive(Clone, Traversable)]
pub struct Vecs {
    pub input_value: EagerVec<PcoVec<TxIndex, Sats>>,
    pub output_value: EagerVec<PcoVec<TxIndex, Sats>>,
    pub fee: ValueTxFull,
    pub fee_rate: ComputedTxDistribution<FeeRate>,
}
