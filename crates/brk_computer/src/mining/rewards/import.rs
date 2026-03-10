use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{
        AmountPerBlockCumulative, AmountPerBlockCumulativeSum, AmountPerBlockFull,
        FiatPerBlock, PercentPerBlock, PercentRollingWindows, RatioRollingWindows,
    },
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        Ok(Self {
            coinbase: AmountPerBlockCumulativeSum::forced_import(
                db, "coinbase", version, indexes,
            )?,
            subsidy: AmountPerBlockCumulative::forced_import(db, "subsidy", version, indexes)?,
            fees: AmountPerBlockFull::forced_import(db, "fees", version, indexes)?,
            unclaimed_rewards: AmountPerBlockCumulativeSum::forced_import(
                db,
                "unclaimed_rewards",
                version,
                indexes,
            )?,
            fee_dominance: PercentPerBlock::forced_import(db, "fee_dominance", version, indexes)?,
            fee_dominance_rolling: PercentRollingWindows::forced_import(
                db,
                "fee_dominance",
                version,
                indexes,
            )?,
            subsidy_dominance: PercentPerBlock::forced_import(
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
            subsidy_sma_1y: FiatPerBlock::forced_import(db, "subsidy_sma_1y", version, indexes)?,
            fee_ratio_multiple: RatioRollingWindows::forced_import(
                db,
                "fee_ratio_multiple",
                version,
                indexes,
            )?,
        })
    }
}
