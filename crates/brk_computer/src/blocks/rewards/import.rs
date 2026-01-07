use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedDateLast, ValueBlockFull, ValueBlockSumCum},
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        Ok(Self {
            height_to_24h_coinbase_sum: EagerVec::forced_import(db, "24h_coinbase_sum", version)?,
            height_to_24h_coinbase_usd_sum: EagerVec::forced_import(
                db,
                "24h_coinbase_usd_sum",
                version,
            )?,
            indexes_to_coinbase: ValueBlockFull::forced_import(
                db,
                "coinbase",
                version,
                indexes,
                compute_dollars,
            )?,
            indexes_to_subsidy: ValueBlockFull::forced_import(
                db,
                "subsidy",
                version,
                indexes,
                compute_dollars,
            )?,
            indexes_to_unclaimed_rewards: ValueBlockSumCum::forced_import(
                db,
                "unclaimed_rewards",
                version,
                indexes,
                compute_dollars,
            )?,
            dateindex_to_fee_dominance: EagerVec::forced_import(db, "fee_dominance", version)?,
            dateindex_to_subsidy_dominance: EagerVec::forced_import(
                db,
                "subsidy_dominance",
                version,
            )?,
            indexes_to_subsidy_usd_1y_sma: compute_dollars
                .then(|| {
                    ComputedDateLast::forced_import(db, "subsidy_usd_1y_sma", version, indexes)
                })
                .transpose()?,
        })
    }
}
