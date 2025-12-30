use brk_error::Result;
use brk_indexer::Indexer;
use brk_types::Version;
use vecdb::{Database, IterableCloneableVec};

use super::Vecs;
use crate::{
    grouped::{ComputedVecsFromDateIndex, ComputedVecsFromHeight, Source, VecBuilderOptions},
    indexes,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexer: &Indexer,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v0 = Version::ZERO;
        let v4 = Version::new(4);
        let v5 = Version::new(5);

        let last = || VecBuilderOptions::default().add_last();
        let sum = || VecBuilderOptions::default().add_sum();

        Ok(Self {
            indexes_to_hash_rate: ComputedVecsFromHeight::forced_import(
                db,
                "hash_rate",
                Source::Compute,
                version + v5,
                indexes,
                last(),
            )?,
            indexes_to_hash_rate_1w_sma: ComputedVecsFromDateIndex::forced_import(
                db,
                "hash_rate_1w_sma",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_hash_rate_1m_sma: ComputedVecsFromDateIndex::forced_import(
                db,
                "hash_rate_1m_sma",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_hash_rate_2m_sma: ComputedVecsFromDateIndex::forced_import(
                db,
                "hash_rate_2m_sma",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_hash_rate_1y_sma: ComputedVecsFromDateIndex::forced_import(
                db,
                "hash_rate_1y_sma",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_hash_price_ths: ComputedVecsFromHeight::forced_import(
                db,
                "hash_price_ths",
                Source::Compute,
                version + v4,
                indexes,
                last(),
            )?,
            indexes_to_hash_price_ths_min: ComputedVecsFromHeight::forced_import(
                db,
                "hash_price_ths_min",
                Source::Compute,
                version + v4,
                indexes,
                last(),
            )?,
            indexes_to_hash_price_phs: ComputedVecsFromHeight::forced_import(
                db,
                "hash_price_phs",
                Source::Compute,
                version + v4,
                indexes,
                last(),
            )?,
            indexes_to_hash_price_phs_min: ComputedVecsFromHeight::forced_import(
                db,
                "hash_price_phs_min",
                Source::Compute,
                version + v4,
                indexes,
                last(),
            )?,
            indexes_to_hash_price_rebound: ComputedVecsFromHeight::forced_import(
                db,
                "hash_price_rebound",
                Source::Compute,
                version + v4,
                indexes,
                last(),
            )?,
            indexes_to_hash_value_ths: ComputedVecsFromHeight::forced_import(
                db,
                "hash_value_ths",
                Source::Compute,
                version + v4,
                indexes,
                last(),
            )?,
            indexes_to_hash_value_ths_min: ComputedVecsFromHeight::forced_import(
                db,
                "hash_value_ths_min",
                Source::Compute,
                version + v4,
                indexes,
                last(),
            )?,
            indexes_to_hash_value_phs: ComputedVecsFromHeight::forced_import(
                db,
                "hash_value_phs",
                Source::Compute,
                version + v4,
                indexes,
                last(),
            )?,
            indexes_to_hash_value_phs_min: ComputedVecsFromHeight::forced_import(
                db,
                "hash_value_phs_min",
                Source::Compute,
                version + v4,
                indexes,
                last(),
            )?,
            indexes_to_hash_value_rebound: ComputedVecsFromHeight::forced_import(
                db,
                "hash_value_rebound",
                Source::Compute,
                version + v4,
                indexes,
                last(),
            )?,
            indexes_to_difficulty: ComputedVecsFromHeight::forced_import(
                db,
                "difficulty",
                Source::Vec(indexer.vecs.block.height_to_difficulty.boxed_clone()),
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_difficulty_as_hash: ComputedVecsFromHeight::forced_import(
                db,
                "difficulty_as_hash",
                Source::Compute,
                version + v0,
                indexes,
                last(),
            )?,
            indexes_to_difficulty_adjustment: ComputedVecsFromHeight::forced_import(
                db,
                "difficulty_adjustment",
                Source::Compute,
                version + v0,
                indexes,
                sum(),
            )?,
        })
    }
}
