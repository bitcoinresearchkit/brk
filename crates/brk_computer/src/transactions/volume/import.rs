use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{AmountFromHeight, AmountFromHeightRolling, ComputedFromHeight},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v2 = Version::TWO;
        Ok(Self {
            sent_sum: AmountFromHeightRolling::forced_import(db, "sent_sum", version, indexes)?,
            received_sum: AmountFromHeightRolling::forced_import(
                db,
                "received_sum",
                version,
                indexes,
            )?,
            annualized_volume: AmountFromHeight::forced_import(
                db,
                "annualized_volume",
                version,
                indexes,
            )?,
            tx_per_sec: ComputedFromHeight::forced_import(db, "tx_per_sec", version + v2, indexes)?,
            outputs_per_sec: ComputedFromHeight::forced_import(
                db,
                "outputs_per_sec",
                version + v2,
                indexes,
            )?,
            inputs_per_sec: ComputedFromHeight::forced_import(
                db,
                "inputs_per_sec",
                version + v2,
                indexes,
            )?,
        })
    }
}
