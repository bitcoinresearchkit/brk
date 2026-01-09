use brk_traversable::Traversable;
use brk_types::{StoredF32, StoredF64};

use crate::internal::{
    ComputedBlockLast, ComputedBlockSum, ComputedDateLast, ComputedDerivedBlockLast,
};

/// Mining-related metrics: hash rate, hash price, hash value, difficulty
#[derive(Clone, Traversable)]
pub struct Vecs {
    pub hash_rate: ComputedBlockLast<StoredF64>,
    pub hash_rate_1w_sma: ComputedDateLast<StoredF64>,
    pub hash_rate_1m_sma: ComputedDateLast<StoredF32>,
    pub hash_rate_2m_sma: ComputedDateLast<StoredF32>,
    pub hash_rate_1y_sma: ComputedDateLast<StoredF32>,
    pub hash_price_ths: ComputedBlockLast<StoredF32>,
    pub hash_price_ths_min: ComputedBlockLast<StoredF32>,
    pub hash_price_phs: ComputedBlockLast<StoredF32>,
    pub hash_price_phs_min: ComputedBlockLast<StoredF32>,
    pub hash_price_rebound: ComputedBlockLast<StoredF32>,
    pub hash_value_ths: ComputedBlockLast<StoredF32>,
    pub hash_value_ths_min: ComputedBlockLast<StoredF32>,
    pub hash_value_phs: ComputedBlockLast<StoredF32>,
    pub hash_value_phs_min: ComputedBlockLast<StoredF32>,
    pub hash_value_rebound: ComputedBlockLast<StoredF32>,
    /// Derived from indexer - no height storage needed
    pub difficulty: ComputedDerivedBlockLast<StoredF64>,
    pub difficulty_as_hash: ComputedBlockLast<StoredF32>,
    pub difficulty_adjustment: ComputedBlockSum<StoredF32>,
}
