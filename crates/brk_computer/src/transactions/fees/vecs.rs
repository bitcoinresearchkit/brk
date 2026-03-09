use brk_traversable::Traversable;
use brk_types::{FeeRate, Sats, TxIndex};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use crate::internal::ComputedPerTxDistribution;

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub input_value: M::Stored<EagerVec<PcoVec<TxIndex, Sats>>>,
    pub output_value: M::Stored<EagerVec<PcoVec<TxIndex, Sats>>>,
    pub fee: ComputedPerTxDistribution<Sats, M>,
    pub fee_rate: ComputedPerTxDistribution<FeeRate, M>,
}
