use brk_error::Result;
use brk_types::Version;
use vecdb::{Database, EagerVec, ImportableVec};

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromDateLast, ValueFromHeightFull, ValueHeight, ValueFromHeightSumCum},
    price,
};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let compute_dollars = price.is_some();

        Ok(Self {
            _24h_coinbase_sum: ValueHeight::forced_import(
                db,
                "24h_coinbase_sum",
                version,
                compute_dollars,
            )?,
            coinbase: ValueFromHeightFull::forced_import(db, "coinbase", version, indexes, price)?,
            subsidy: ValueFromHeightFull::forced_import(db, "subsidy", version, indexes, price)?,
            unclaimed_rewards: ValueFromHeightSumCum::forced_import(
                db,
                "unclaimed_rewards",
                version,
                indexes,
                price,
            )?,
            fee_dominance: EagerVec::forced_import(db, "fee_dominance", version)?,
            subsidy_dominance: EagerVec::forced_import(db, "subsidy_dominance", version)?,
            subsidy_usd_1y_sma: compute_dollars
                .then(|| {
                    ComputedFromDateLast::forced_import(db, "subsidy_usd_1y_sma", version, indexes)
                })
                .transpose()?,
        })
    }
}
