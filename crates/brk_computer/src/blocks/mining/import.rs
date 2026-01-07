use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedBlockLast, ComputedBlockSum, ComputedDateLast, DerivedComputedBlockLast},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v4 = Version::new(4);
        let v5 = Version::new(5);

        Ok(Self {
            indexes_to_hash_rate: ComputedBlockLast::forced_import(
                db,
                "hash_rate",
                version + v5,
                indexes,
            )?,
            indexes_to_hash_rate_1w_sma: ComputedDateLast::forced_import(
                db,
                "hash_rate_1w_sma",
                version,
                indexes,
            )?,
            indexes_to_hash_rate_1m_sma: ComputedDateLast::forced_import(
                db,
                "hash_rate_1m_sma",
                version,
                indexes,
            )?,
            indexes_to_hash_rate_2m_sma: ComputedDateLast::forced_import(
                db,
                "hash_rate_2m_sma",
                version,
                indexes,
            )?,
            indexes_to_hash_rate_1y_sma: ComputedDateLast::forced_import(
                db,
                "hash_rate_1y_sma",
                version,
                indexes,
            )?,
            indexes_to_hash_price_ths: ComputedBlockLast::forced_import(
                db,
                "hash_price_ths",
                version + v4,
                indexes,
            )?,
            indexes_to_hash_price_ths_min: ComputedBlockLast::forced_import(
                db,
                "hash_price_ths_min",
                version + v4,
                indexes,
            )?,
            indexes_to_hash_price_phs: ComputedBlockLast::forced_import(
                db,
                "hash_price_phs",
                version + v4,
                indexes,
            )?,
            indexes_to_hash_price_phs_min: ComputedBlockLast::forced_import(
                db,
                "hash_price_phs_min",
                version + v4,
                indexes,
            )?,
            indexes_to_hash_price_rebound: ComputedBlockLast::forced_import(
                db,
                "hash_price_rebound",
                version + v4,
                indexes,
            )?,
            indexes_to_hash_value_ths: ComputedBlockLast::forced_import(
                db,
                "hash_value_ths",
                version + v4,
                indexes,
            )?,
            indexes_to_hash_value_ths_min: ComputedBlockLast::forced_import(
                db,
                "hash_value_ths_min",
                version + v4,
                indexes,
            )?,
            indexes_to_hash_value_phs: ComputedBlockLast::forced_import(
                db,
                "hash_value_phs",
                version + v4,
                indexes,
            )?,
            indexes_to_hash_value_phs_min: ComputedBlockLast::forced_import(
                db,
                "hash_value_phs_min",
                version + v4,
                indexes,
            )?,
            indexes_to_hash_value_rebound: ComputedBlockLast::forced_import(
                db,
                "hash_value_rebound",
                version + v4,
                indexes,
            )?,
            // Derived from external indexer data - no height storage needed
            indexes_to_difficulty: DerivedComputedBlockLast::forced_import(
                db,
                "difficulty",
                indexer.vecs.block.height_to_difficulty.boxed_clone(),
                version,
                indexes,
            )?,
            indexes_to_difficulty_as_hash: ComputedBlockLast::forced_import(
                db,
                "difficulty_as_hash",
                version,
                indexes,
            )?,
            indexes_to_difficulty_adjustment: ComputedBlockSum::forced_import(
                db,
                "difficulty_adjustment",
                version,
                indexes,
            )?,
        })
    }
}
