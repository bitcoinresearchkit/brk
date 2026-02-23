use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightLast, StoredValueFromHeightLast, ValueFromHeightFull, ValueFromHeightSumCum},
    prices,
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            coinbase_24h_sum: StoredValueFromHeightLast::forced_import(db, "coinbase_24h_sum", version, indexes)?,
            coinbase_7d_sum: StoredValueFromHeightLast::forced_import(db, "coinbase_7d_sum", version, indexes)?,
            coinbase_30d_sum: StoredValueFromHeightLast::forced_import(db, "coinbase_30d_sum", version, indexes)?,
            coinbase_1y_sum: StoredValueFromHeightLast::forced_import(db, "coinbase_1y_sum", version, indexes)?,
            fee_24h_sum: StoredValueFromHeightLast::forced_import(db, "fee_24h_sum", version, indexes)?,
            fee_7d_sum: StoredValueFromHeightLast::forced_import(db, "fee_7d_sum", version, indexes)?,
            fee_30d_sum: StoredValueFromHeightLast::forced_import(db, "fee_30d_sum", version, indexes)?,
            fee_1y_sum: StoredValueFromHeightLast::forced_import(db, "fee_1y_sum", version, indexes)?,
            coinbase: ValueFromHeightFull::forced_import(db, "coinbase", version, indexes, prices)?,
            subsidy: ValueFromHeightFull::forced_import(db, "subsidy", version, indexes, prices)?,
            unclaimed_rewards: ValueFromHeightSumCum::forced_import(
                db,
                "unclaimed_rewards",
                version,
                indexes,
                prices,
            )?,
            fee_dominance: ComputedFromHeightLast::forced_import(
                db,
                "fee_dominance",
                version,
                indexes,
            )?,
            fee_dominance_24h: ComputedFromHeightLast::forced_import(
                db,
                "fee_dominance_24h",
                version,
                indexes,
            )?,
            fee_dominance_7d: ComputedFromHeightLast::forced_import(
                db,
                "fee_dominance_7d",
                version,
                indexes,
            )?,
            fee_dominance_30d: ComputedFromHeightLast::forced_import(
                db,
                "fee_dominance_30d",
                version,
                indexes,
            )?,
            fee_dominance_1y: ComputedFromHeightLast::forced_import(
                db,
                "fee_dominance_1y",
                version,
                indexes,
            )?,
            subsidy_dominance: ComputedFromHeightLast::forced_import(
                db,
                "subsidy_dominance",
                version,
                indexes,
            )?,
            subsidy_dominance_24h: ComputedFromHeightLast::forced_import(
                db,
                "subsidy_dominance_24h",
                version,
                indexes,
            )?,
            subsidy_dominance_7d: ComputedFromHeightLast::forced_import(
                db,
                "subsidy_dominance_7d",
                version,
                indexes,
            )?,
            subsidy_dominance_30d: ComputedFromHeightLast::forced_import(
                db,
                "subsidy_dominance_30d",
                version,
                indexes,
            )?,
            subsidy_dominance_1y: ComputedFromHeightLast::forced_import(
                db,
                "subsidy_dominance_1y",
                version,
                indexes,
            )?,
            subsidy_usd_1y_sma: ComputedFromHeightLast::forced_import(
                db,
                "subsidy_usd_1y_sma",
                version,
                indexes,
            )?,
        })
    }
}
