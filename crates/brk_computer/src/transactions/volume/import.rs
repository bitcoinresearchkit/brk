use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{
    indexes,
    internal::{ComputedFromHeightLast, StoredValueRollingWindows, ValueFromHeightLast},
};

impl Vecs {
    pub(crate) fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
    ) -> Result<Self> {
        let v2 = Version::TWO;
        Ok(Self {
            sent_sum: ValueFromHeightLast::forced_import(
                db, "sent_sum", version, indexes,
            )?,
            sent_sum_rolling: StoredValueRollingWindows::forced_import(
                db, "sent_sum", version, indexes,
            )?,
            received_sum: ValueFromHeightLast::forced_import(
                db, "received_sum", version, indexes,
            )?,
            received_sum_rolling: StoredValueRollingWindows::forced_import(
                db, "received_sum", version, indexes,
            )?,
            annualized_volume: ValueFromHeightLast::forced_import(
                db,
                "annualized_volume",
                version,
                indexes,
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
