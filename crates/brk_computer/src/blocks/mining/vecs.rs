use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredF64};

use crate::internal::{
    ComputedBlockLast, ComputedBlockSum, ComputedDateLast, DerivedComputedBlockLast,
};

/// Mining-related metrics: hash rate, hash price, hash value, difficulty
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub indexes_to_hash_rate: ComputedBlockLast<StoredF64>,
    pub indexes_to_hash_rate_1w_sma: ComputedDateLast<StoredF64>,
    pub indexes_to_hash_rate_1m_sma: ComputedDateLast<StoredF32>,
    pub indexes_to_hash_rate_2m_sma: ComputedDateLast<StoredF32>,
    pub indexes_to_hash_rate_1y_sma: ComputedDateLast<StoredF32>,
    pub indexes_to_hash_price_ths: ComputedBlockLast<StoredF32>,
    pub indexes_to_hash_price_ths_min: ComputedBlockLast<StoredF32>,
    pub indexes_to_hash_price_phs: ComputedBlockLast<StoredF32>,
    pub indexes_to_hash_price_phs_min: ComputedBlockLast<StoredF32>,
    pub indexes_to_hash_price_rebound: ComputedBlockLast<StoredF32>,
    pub indexes_to_hash_value_ths: ComputedBlockLast<StoredF32>,
    pub indexes_to_hash_value_ths_min: ComputedBlockLast<StoredF32>,
    pub indexes_to_hash_value_phs: ComputedBlockLast<StoredF32>,
    pub indexes_to_hash_value_phs_min: ComputedBlockLast<StoredF32>,
    pub indexes_to_hash_value_rebound: ComputedBlockLast<StoredF32>,
    /// Derived from indexer - no height storage needed
    pub indexes_to_difficulty: DerivedComputedBlockLast<StoredF64>,
    pub indexes_to_difficulty_as_hash: ComputedBlockLast<StoredF32>,
    pub indexes_to_difficulty_adjustment: ComputedBlockSum<StoredF32>,
}
