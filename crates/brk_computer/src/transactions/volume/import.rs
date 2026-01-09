use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::{ComputedDateLast, ValueBlockSum, ValueDateLast}};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        compute_dollars: bool,
    ) -> Result<Self> {
        let v2 = Version::TWO;

        Ok(Self {
            sent_sum: ValueBlockSum::forced_import(
                db,
                "sent_sum",
                version,
                indexes,
                compute_dollars,
            )?,
            annualized_volume: ValueDateLast::forced_import(
                db,
                "annualized_volume",
                version,
                compute_dollars,
                indexes,
            )?,
            tx_per_sec: ComputedDateLast::forced_import(db, "tx_per_sec", version + v2, indexes)?,
            outputs_per_sec: ComputedDateLast::forced_import(
                db,
                "outputs_per_sec",
                version + v2,
                indexes,
            )?,
            inputs_per_sec: ComputedDateLast::forced_import(
                db,
                "inputs_per_sec",
                version + v2,
                indexes,
            )?,
        })
    }
}
