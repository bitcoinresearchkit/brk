use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::{
    vecs::{HashPriceValueVecs, HashRateSmaVecs},
    Vecs,
};
use crate::{
    indexes,
    internal::{ComputedPerBlock, PercentPerBlock},
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
            hash_rate: ComputedPerBlock::forced_import(db, "hash_rate", version + v5, indexes)?,
            hash_rate_sma: HashRateSmaVecs {
                _1w: ComputedPerBlock::forced_import(
                    db,
                    "hash_rate_sma_1w",
                    version,
                    indexes,
                )?,
                _1m: ComputedPerBlock::forced_import(
                    db,
                    "hash_rate_sma_1m",
                    version,
                    indexes,
                )?,
                _2m: ComputedPerBlock::forced_import(
                    db,
                    "hash_rate_sma_2m",
                    version,
                    indexes,
                )?,
                _1y: ComputedPerBlock::forced_import(
                    db,
                    "hash_rate_sma_1y",
                    version,
                    indexes,
                )?,
            },
            hash_rate_ath: ComputedPerBlock::forced_import(
                db,
                "hash_rate_ath",
                version,
                indexes,
            )?,
            hash_rate_drawdown: PercentPerBlock::forced_import(
                db,
                "hash_rate_drawdown",
                version,
                indexes,
            )?,
            hash_price: HashPriceValueVecs {
                ths: ComputedPerBlock::forced_import(
                    db,
                    "hash_price_ths",
                    version + v4,
                    indexes,
                )?,
                ths_min: ComputedPerBlock::forced_import(
                    db,
                    "hash_price_ths_min",
                    version + v4,
                    indexes,
                )?,
                phs: ComputedPerBlock::forced_import(
                    db,
                    "hash_price_phs",
                    version + v4,
                    indexes,
                )?,
                phs_min: ComputedPerBlock::forced_import(
                    db,
                    "hash_price_phs_min",
                    version + v4,
                    indexes,
                )?,
                rebound: PercentPerBlock::forced_import(
                    db,
                    "hash_price_rebound",
                    version + v4,
                    indexes,
                )?,
            },
            hash_value: HashPriceValueVecs {
                ths: ComputedPerBlock::forced_import(
                    db,
                    "hash_value_ths",
                    version + v4,
                    indexes,
                )?,
                ths_min: ComputedPerBlock::forced_import(
                    db,
                    "hash_value_ths_min",
                    version + v4,
                    indexes,
                )?,
                phs: ComputedPerBlock::forced_import(
                    db,
                    "hash_value_phs",
                    version + v4,
                    indexes,
                )?,
                phs_min: ComputedPerBlock::forced_import(
                    db,
                    "hash_value_phs_min",
                    version + v4,
                    indexes,
                )?,
                rebound: PercentPerBlock::forced_import(
                    db,
                    "hash_value_rebound",
                    version + v4,
                    indexes,
                )?,
            },
        })
    }
}
