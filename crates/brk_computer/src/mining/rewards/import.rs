use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{
        ComputedFromHeight, FiatFromHeight, RollingWindows, ValueFromHeightFull,
        ValueFromHeightCumulativeSum,
    },
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            coinbase: ValueFromHeightFull::forced_import(db, "coinbase", version, indexes)?,
            subsidy: ValueFromHeightFull::forced_import(db, "subsidy", version, indexes)?,
            fees: ValueFromHeightFull::forced_import(db, "fees", version, indexes)?,
            unclaimed_rewards: ValueFromHeightCumulativeSum::forced_import(
                db,
                "unclaimed_rewards",
                version,
                indexes,
            )?,
            fee_dominance: ComputedFromHeight::forced_import(
                db,
                "fee_dominance",
                version,
                indexes,
            )?,
            fee_dominance_rolling: RollingWindows::forced_import(
                db,
                "fee_dominance",
                version,
                indexes,
            )?,
            subsidy_dominance: ComputedFromHeight::forced_import(
                db,
                "subsidy_dominance",
                version,
                indexes,
            )?,
            subsidy_dominance_rolling: RollingWindows::forced_import(
                db,
                "subsidy_dominance",
                version,
                indexes,
            )?,
            subsidy_usd_1y_sma: FiatFromHeight::forced_import(
                db,
                "subsidy_usd_1y_sma",
                version,
                indexes,
            )?,
        })
    }
}
