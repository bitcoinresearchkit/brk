use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredF64};

use crate::internal::{ComputedFromHeightLast, ComputedFromDateLast};

/// Mining-related metrics: hash rate, hash price, hash value
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub hash_rate: ComputedFromHeightLast<StoredF64>,
    pub hash_rate_1w_sma: ComputedFromDateLast<StoredF64>,
    pub hash_rate_1m_sma: ComputedFromDateLast<StoredF32>,
    pub hash_rate_2m_sma: ComputedFromDateLast<StoredF32>,
    pub hash_rate_1y_sma: ComputedFromDateLast<StoredF32>,
    pub hash_price_ths: ComputedFromHeightLast<StoredF32>,
    pub hash_price_ths_min: ComputedFromHeightLast<StoredF32>,
    pub hash_price_phs: ComputedFromHeightLast<StoredF32>,
    pub hash_price_phs_min: ComputedFromHeightLast<StoredF32>,
    pub hash_price_rebound: ComputedFromHeightLast<StoredF32>,
    pub hash_value_ths: ComputedFromHeightLast<StoredF32>,
    pub hash_value_ths_min: ComputedFromHeightLast<StoredF32>,
    pub hash_value_phs: ComputedFromHeightLast<StoredF32>,
    pub hash_value_phs_min: ComputedFromHeightLast<StoredF32>,
    pub hash_value_rebound: ComputedFromHeightLast<StoredF32>,
}
