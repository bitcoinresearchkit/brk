use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredF64};

use crate::internal::{ComputedVecsFromDateIndex, ComputedVecsFromHeight};

/// Mining-related metrics: hash rate, hash price, hash value, difficulty
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_hash_rate: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_hash_rate_1w_sma: ComputedVecsFromDateIndex<StoredF64>,
    pub indexes_to_hash_rate_1m_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_hash_rate_2m_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_hash_rate_1y_sma: ComputedVecsFromDateIndex<StoredF32>,
    pub indexes_to_hash_price_ths: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_price_ths_min: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_price_phs: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_price_phs_min: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_price_rebound: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_value_ths: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_value_ths_min: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_value_phs: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_value_phs_min: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_hash_value_rebound: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_difficulty: ComputedVecsFromHeight<StoredF64>,
    pub indexes_to_difficulty_as_hash: ComputedVecsFromHeight<StoredF32>,
    pub indexes_to_difficulty_adjustment: ComputedVecsFromHeight<StoredF32>,
}
