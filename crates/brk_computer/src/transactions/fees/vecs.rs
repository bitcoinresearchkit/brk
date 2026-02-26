use brk_traversable::Traversable;
use brk_types::{Dollars, FeeRate, Height, Sats, TxIndex};
use vecdb::{EagerVec, PcoVec, Rw, StorageMode};

use crate::internal::{ComputedFromHeightFull, Distribution, RollingDistribution};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub input_value: M::Stored<EagerVec<PcoVec<TxIndex, Sats>>>,
    pub output_value: M::Stored<EagerVec<PcoVec<TxIndex, Sats>>>,
    pub fee_txindex: M::Stored<EagerVec<PcoVec<TxIndex, Sats>>>,
    pub fee: ComputedFromHeightFull<Sats, M>,
    pub fee_usd_sum: M::Stored<EagerVec<PcoVec<Height, Dollars>>>,
    pub fee_rate_txindex: M::Stored<EagerVec<PcoVec<TxIndex, FeeRate>>>,
    pub fee_rate: Distribution<Height, FeeRate, M>,
    pub fee_rate_rolling: RollingDistribution<FeeRate, M>,
}
