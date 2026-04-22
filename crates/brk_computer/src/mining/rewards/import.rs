use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{
        ValuePerBlockCumulative, ValuePerBlockCumulativeRolling, ValuePerBlockFull,
        LazyPercentCumulativeRolling, OneMinusBp16, PercentCumulativeRolling, RatioRollingWindows,
        WindowStartVec, Windows,
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
            PercentCumulativeRolling::forced_import(db, "fee_dominance", version, indexes)?;

        let subsidy_dominance = LazyPercentCumulativeRolling::from_source::<OneMinusBp16>(
            "subsidy_dominance",
            version,
            &fee_dominance,
        );

        Ok(Self {
            coinbase: ValuePerBlockCumulativeRolling::forced_import(
                db,
                "coinbase",
                version,
                indexes,
                cached_starts,
            )?,
            subsidy: ValuePerBlockCumulativeRolling::forced_import(
                db,
                "subsidy",
                version,
                indexes,
                cached_starts,
            )?,
            fees: ValuePerBlockFull::forced_import(db, "fees", version, indexes, cached_starts)?,
            output_volume: EagerVec::forced_import(db, "output_volume", version)?,
            unclaimed: ValuePerBlockCumulative::forced_import(
                db,
                "unclaimed_rewards",
                version,
                indexes,
            )?,
            fee_dominance,
            subsidy_dominance,
            fee_to_subsidy_ratio: RatioRollingWindows::forced_import(
                db,
                "fee_to_subsidy_ratio",
                version,
                indexes,
            )?,
        })
    }
}
