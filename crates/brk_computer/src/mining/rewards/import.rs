use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        AmountPerBlockCumulative, AmountPerBlockCumulativeRolling, AmountPerBlockFull,
        LazyPercentRollingWindows, OneMinusBp16, PercentCumulativeRolling, PercentPerBlock,
        RatioRollingWindows, WindowStartVec, Windows,
    },
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        cached_starts: &Windows<&WindowStartVec>,
    ) -> Result<Self> {
        let fee_dominance =
            PercentCumulativeRolling::forced_import_flat(db, "fee_dominance", version, indexes)?;

        let subsidy_dominance_rolling = LazyPercentRollingWindows::from_rolling::<OneMinusBp16>(
            "subsidy_dominance",
            version,
            &fee_dominance.rolling,
        );

        Ok(Self {
            coinbase: AmountPerBlockCumulativeRolling::forced_import(
                db,
                "coinbase",
                version,
                indexes,
                cached_starts,
            )?,
            subsidy: AmountPerBlockCumulativeRolling::forced_import(
                db,
                "subsidy",
                version,
                indexes,
                cached_starts,
            )?,
            fees: AmountPerBlockFull::forced_import(db, "fees", version, indexes, cached_starts)?,
            output_volume: EagerVec::forced_import(db, "output_volume", version)?,
            unclaimed: AmountPerBlockCumulative::forced_import(
                db,
                "unclaimed_rewards",
                version,
                indexes,
            )?,
            fee_dominance,
            subsidy_dominance: PercentPerBlock::forced_import(
                db,
                "subsidy_dominance",
                version,
                indexes,
            )?,
            subsidy_dominance_rolling,
            fee_to_subsidy_ratio: RatioRollingWindows::forced_import(
                db,
                "fee_to_subsidy_ratio",
                version,
                indexes,
            )?,
        })
    }
}
