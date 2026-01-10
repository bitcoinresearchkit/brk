use brk_error::Result;
use brk_types::Version;
use vecdb::Database;

use super::Vecs;
use crate::{indexes, internal::{ComputedFromDateLast, ValueFromHeightSum, ValueFromDateLast}, price};

impl Vecs {
    pub fn forced_import(
        db: &Database,
        version: Version,
        indexes: &indexes::Vecs,
        price: Option<&price::Vecs>,
    ) -> Result<Self> {
        let v2 = Version::TWO;
        let compute_dollars = price.is_some();

        Ok(Self {
            sent_sum: ValueFromHeightSum::forced_import(
                db,
                "sent_sum",
                version,
                indexes,
                price,
            )?,
            annualized_volume: ValueFromDateLast::forced_import(
                db,
                "annualized_volume",
                version,
                compute_dollars,
                indexes,
            )?,
            tx_per_sec: ComputedFromDateLast::forced_import(db, "tx_per_sec", version + v2, indexes)?,
            outputs_per_sec: ComputedFromDateLast::forced_import(
                db,
                "outputs_per_sec",
                version + v2,
                indexes,
            )?,
            inputs_per_sec: ComputedFromDateLast::forced_import(
                db,
                "inputs_per_sec",
                version + v2,
                indexes,
            )?,
        })
    }
}
