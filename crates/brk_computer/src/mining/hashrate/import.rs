use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{Bps16ToFloat, Bps16ToPercent, ComputedFromHeight, PercentFromHeight},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v4 = Version::new(4);
        let v5 = Version::new(5);

        Ok(Self {
            hash_rate: ComputedFromHeight::forced_import(db, "hash_rate", version + v5, indexes)?,
            hash_rate_sma_1w: ComputedFromHeight::forced_import(
                db,
                "hash_rate_sma_1w",
                version,
                indexes,
            )?,
            hash_rate_sma_1m: ComputedFromHeight::forced_import(
                db,
                "hash_rate_sma_1m",
                version,
                indexes,
            )?,
            hash_rate_sma_2m: ComputedFromHeight::forced_import(
                db,
                "hash_rate_sma_2m",
                version,
                indexes,
            )?,
            hash_rate_sma_1y: ComputedFromHeight::forced_import(
                db,
                "hash_rate_sma_1y",
                version,
                indexes,
            )?,
            hash_rate_ath: ComputedFromHeight::forced_import(
                db,
                "hash_rate_ath",
                version,
                indexes,
            )?,
            hash_rate_drawdown: PercentFromHeight::forced_import::<Bps16ToFloat, Bps16ToPercent>(
                db,
                "hash_rate_drawdown",
                version,
                indexes,
            )?,
            hash_price_ths: ComputedFromHeight::forced_import(
                db,
                "hash_price_ths",
                version + v4,
                indexes,
            )?,
            hash_price_ths_min: ComputedFromHeight::forced_import(
                db,
                "hash_price_ths_min",
                version + v4,
                indexes,
            )?,
            hash_price_phs: ComputedFromHeight::forced_import(
                db,
                "hash_price_phs",
                version + v4,
                indexes,
            )?,
            hash_price_phs_min: ComputedFromHeight::forced_import(
                db,
                "hash_price_phs_min",
                version + v4,
                indexes,
            )?,
            hash_price_rebound: ComputedFromHeight::forced_import(
                db,
                "hash_price_rebound",
                version + v4,
                indexes,
            )?,
            hash_value_ths: ComputedFromHeight::forced_import(
                db,
                "hash_value_ths",
                version + v4,
                indexes,
            )?,
            hash_value_ths_min: ComputedFromHeight::forced_import(
                db,
                "hash_value_ths_min",
                version + v4,
                indexes,
            )?,
            hash_value_phs: ComputedFromHeight::forced_import(
                db,
                "hash_value_phs",
                version + v4,
                indexes,
            )?,
            hash_value_phs_min: ComputedFromHeight::forced_import(
                db,
                "hash_value_phs_min",
                version + v4,
                indexes,
            )?,
            hash_value_rebound: ComputedFromHeight::forced_import(
                db,
                "hash_value_rebound",
                version + v4,
                indexes,
            )?,
        })
    }
}
