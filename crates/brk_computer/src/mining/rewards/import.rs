use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{
        FiatFromHeight, PercentFromHeight, PercentRollingWindows, ValueFromHeightCumulative,
        ValueFromHeightCumulativeSum, ValueFromHeightFull,
    },
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            coinbase: ValueFromHeightCumulativeSum::forced_import(db, "coinbase", version, indexes)?,
            subsidy: ValueFromHeightCumulative::forced_import(db, "subsidy", version, indexes)?,
            fees: ValueFromHeightFull::forced_import(db, "fees", version, indexes)?,
            unclaimed_rewards: ValueFromHeightCumulativeSum::forced_import(
                db,
                "unclaimed_rewards",
                version,
                indexes,
            )?,
            fee_dominance: PercentFromHeight::forced_import(db, "fee_dominance", version, indexes)?,
            fee_dominance_rolling: PercentRollingWindows::forced_import(
                db,
                "fee_dominance",
                version,
                indexes,
            )?,
            subsidy_dominance: PercentFromHeight::forced_import(
                db,
                "subsidy_dominance",
                version,
                indexes,
            )?,
            subsidy_dominance_rolling: PercentRollingWindows::forced_import(
                db,
                "subsidy_dominance",
                version,
                indexes,
            )?,
            subsidy_sma_1y: FiatFromHeight::forced_import(db, "subsidy_sma_1y", version, indexes)?,
        })
    }
}
