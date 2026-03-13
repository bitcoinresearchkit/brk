use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{
        AmountPerBlockCumulative, AmountPerBlockCumulativeWithSums, AmountPerBlockFull,
        CachedWindowStarts, FiatPerBlock, LazyPercentRollingWindows, OneMinusBp16,
        PercentPerBlock, PercentRollingWindows, RatioRollingWindows,
    },
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &CachedWindowStarts,
    ) -> Result<Self> {
        let fee_dominance_rolling = PercentRollingWindows::forced_import(
            db,
            "fee_dominance",
            version,
            indexes,
        )?;

        let subsidy_dominance_rolling = LazyPercentRollingWindows::from_rolling::<OneMinusBp16>(
            "subsidy_dominance",
            version,
            &fee_dominance_rolling,
        );

        Ok(Self {
            coinbase: AmountPerBlockCumulativeWithSums::forced_import(
                db, "coinbase", version, indexes, cached_starts,
            )?,
            subsidy: AmountPerBlockCumulative::forced_import(db, "subsidy", version, indexes)?,
            fees: AmountPerBlockFull::forced_import(db, "fees", version, indexes, cached_starts)?,
            unclaimed: AmountPerBlockCumulativeWithSums::forced_import(
                db,
                "unclaimed_rewards",
                version,
                indexes,
                cached_starts,
            )?,
            fee_dominance: PercentPerBlock::forced_import(db, "fee_dominance", version, indexes)?,
            fee_dominance_rolling,
            subsidy_dominance: PercentPerBlock::forced_import(
                db,
                "subsidy_dominance",
                version,
                indexes,
            )?,
            subsidy_dominance_rolling,
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
