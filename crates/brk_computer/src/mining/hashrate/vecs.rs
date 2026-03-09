use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned16, BasisPointsSigned32, StoredF32, StoredF64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedPerBlock, PercentPerBlock};

#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub hash_rate: ComputedPerBlock<StoredF64, M>,
    pub hash_rate_sma_1w: ComputedPerBlock<StoredF64, M>,
    pub hash_rate_sma_1m: ComputedPerBlock<StoredF64, M>,
    pub hash_rate_sma_2m: ComputedPerBlock<StoredF64, M>,
    pub hash_rate_sma_1y: ComputedPerBlock<StoredF64, M>,
    pub hash_rate_ath: ComputedPerBlock<StoredF64, M>,
    pub hash_rate_drawdown: PercentPerBlock<BasisPointsSigned16, M>,
    pub hash_price_ths: ComputedPerBlock<StoredF32, M>,
    pub hash_price_ths_min: ComputedPerBlock<StoredF32, M>,
    pub hash_price_phs: ComputedPerBlock<StoredF32, M>,
    pub hash_price_phs_min: ComputedPerBlock<StoredF32, M>,
    pub hash_price_rebound: PercentPerBlock<BasisPointsSigned32, M>,
    pub hash_value_ths: ComputedPerBlock<StoredF32, M>,
    pub hash_value_ths_min: ComputedPerBlock<StoredF32, M>,
    pub hash_value_phs: ComputedPerBlock<StoredF32, M>,
    pub hash_value_phs_min: ComputedPerBlock<StoredF32, M>,
    pub hash_value_rebound: PercentPerBlock<BasisPointsSigned32, M>,
}
