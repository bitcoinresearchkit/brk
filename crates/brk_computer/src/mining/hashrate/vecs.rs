use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredF64};
use vecdb::{Rw, StorageMode};

use crate::internal::ComputedFromHeightLast;

/// Mining-related metrics: hash rate, hash price, hash value
#[derive(Traversable)]
pub struct Vecs<M: StorageMode = Rw> {
    pub hash_rate: ComputedFromHeightLast<StoredF64, M>,
    pub hash_rate_1w_sma: ComputedFromHeightLast<StoredF64, M>,
    pub hash_rate_1m_sma: ComputedFromHeightLast<StoredF32, M>,
    pub hash_rate_2m_sma: ComputedFromHeightLast<StoredF32, M>,
    pub hash_rate_1y_sma: ComputedFromHeightLast<StoredF32, M>,
    pub hash_rate_ath: ComputedFromHeightLast<StoredF64, M>,
    pub hash_rate_drawdown: ComputedFromHeightLast<StoredF32, M>,
    pub hash_price_ths: ComputedFromHeightLast<StoredF32, M>,
    pub hash_price_ths_min: ComputedFromHeightLast<StoredF32, M>,
    pub hash_price_phs: ComputedFromHeightLast<StoredF32, M>,
    pub hash_price_phs_min: ComputedFromHeightLast<StoredF32, M>,
    pub hash_price_rebound: ComputedFromHeightLast<StoredF32, M>,
    pub hash_value_ths: ComputedFromHeightLast<StoredF32, M>,
    pub hash_value_ths_min: ComputedFromHeightLast<StoredF32, M>,
    pub hash_value_phs: ComputedFromHeightLast<StoredF32, M>,
    pub hash_value_phs_min: ComputedFromHeightLast<StoredF32, M>,
    pub hash_value_rebound: ComputedFromHeightLast<StoredF32, M>,
}
