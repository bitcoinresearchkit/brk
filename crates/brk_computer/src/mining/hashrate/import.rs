use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::ComputedFromHeightLast,
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
            hash_rate: ComputedFromHeightLast::forced_import(db, "hash_rate", version + v5, indexes)?,
            hash_rate_1w_sma: ComputedFromHeightLast::forced_import(
                db,
                "hash_rate_1w_sma",
                version,
                indexes,
            )?,
            hash_rate_1m_sma: ComputedFromHeightLast::forced_import(
                db,
                "hash_rate_1m_sma",
                version,
                indexes,
            )?,
            hash_rate_2m_sma: ComputedFromHeightLast::forced_import(
                db,
                "hash_rate_2m_sma",
                version,
                indexes,
            )?,
            hash_rate_1y_sma: ComputedFromHeightLast::forced_import(
                db,
                "hash_rate_1y_sma",
                version,
                indexes,
            )?,
            hash_rate_ath: ComputedFromHeightLast::forced_import(
                db,
                "hash_rate_ath",
                version,
                indexes,
            )?,
            hash_rate_drawdown: ComputedFromHeightLast::forced_import(
                db,
                "hash_rate_drawdown",
                version,
                indexes,
            )?,
            hash_price_ths: ComputedFromHeightLast::forced_import(
                db,
                "hash_price_ths",
                version + v4,
                indexes,
            )?,
            hash_price_ths_min: ComputedFromHeightLast::forced_import(
                db,
                "hash_price_ths_min",
                version + v4,
                indexes,
            )?,
            hash_price_phs: ComputedFromHeightLast::forced_import(
                db,
                "hash_price_phs",
                version + v4,
                indexes,
            )?,
            hash_price_phs_min: ComputedFromHeightLast::forced_import(
                db,
                "hash_price_phs_min",
                version + v4,
                indexes,
            )?,
            hash_price_rebound: ComputedFromHeightLast::forced_import(
                db,
                "hash_price_rebound",
                version + v4,
                indexes,
            )?,
            hash_value_ths: ComputedFromHeightLast::forced_import(
                db,
                "hash_value_ths",
                version + v4,
                indexes,
            )?,
            hash_value_ths_min: ComputedFromHeightLast::forced_import(
                db,
                "hash_value_ths_min",
                version + v4,
                indexes,
            )?,
            hash_value_phs: ComputedFromHeightLast::forced_import(
                db,
                "hash_value_phs",
                version + v4,
                indexes,
            )?,
            hash_value_phs_min: ComputedFromHeightLast::forced_import(
                db,
                "hash_value_phs_min",
                version + v4,
                indexes,
            )?,
            hash_value_rebound: ComputedFromHeightLast::forced_import(
                db,
                "hash_value_rebound",
                version + v4,
                indexes,
            )?,
        })
    }
}
