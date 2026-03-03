use brk_traversable::Traversable;
use brk_types::{BasisPointsSigned16, StoredF32, StoredF64};
use vecdb::{Rw, StorageMode};

use crate::internal::{ComputedFromHeight, PercentFromHeight};

/// Mining-related metrics: hash rate, hash price, hash value
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub hash_rate: ComputedFromHeight<StoredF64, M>,
    pub hash_rate_sma_1w: ComputedFromHeight<StoredF64, M>,
    pub hash_rate_sma_1m: ComputedFromHeight<StoredF64, M>,
    pub hash_rate_sma_2m: ComputedFromHeight<StoredF64, M>,
    pub hash_rate_sma_1y: ComputedFromHeight<StoredF64, M>,
    pub hash_rate_ath: ComputedFromHeight<StoredF64, M>,
    pub hash_rate_drawdown: PercentFromHeight<BasisPointsSigned16, M>,
    pub hash_price_ths: ComputedFromHeight<StoredF32, M>,
    pub hash_price_ths_min: ComputedFromHeight<StoredF32, M>,
    pub hash_price_phs: ComputedFromHeight<StoredF32, M>,
    pub hash_price_phs_min: ComputedFromHeight<StoredF32, M>,
    pub hash_price_rebound: ComputedFromHeight<StoredF32, M>,
    pub hash_value_ths: ComputedFromHeight<StoredF32, M>,
    pub hash_value_ths_min: ComputedFromHeight<StoredF32, M>,
    pub hash_value_phs: ComputedFromHeight<StoredF32, M>,
    pub hash_value_phs_min: ComputedFromHeight<StoredF32, M>,
    pub hash_value_rebound: ComputedFromHeight<StoredF32, M>,
}
