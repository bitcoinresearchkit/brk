use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightLast, ValueFromHeightLast, ValueFromHeightSum},
    prices,
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        prices: &prices::Vecs,
    ) -> Result<Self> {
        let v2 = Version::TWO;
        Ok(Self {
            sent_sum: ValueFromHeightSum::forced_import(db, "sent_sum", version, indexes, prices)?,
            received_sum: ValueFromHeightSum::forced_import(
                db,
                "received_sum",
                version,
                indexes,
                prices,
            )?,
            annualized_volume: ValueFromHeightLast::forced_import(
                db,
                "annualized_volume",
                version,
                indexes,
                prices,
            )?,
            tx_per_sec: ComputedFromHeightLast::forced_import(
                db,
                "tx_per_sec",
                version + v2,
                indexes,
            )?,
            outputs_per_sec: ComputedFromHeightLast::forced_import(
                db,
                "outputs_per_sec",
                version + v2,
                indexes,
            )?,
            inputs_per_sec: ComputedFromHeightLast::forced_import(
                db,
                "inputs_per_sec",
                version + v2,
                indexes,
            )?,
        })
    }
}
